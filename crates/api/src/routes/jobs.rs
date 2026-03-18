use std::io::ErrorKind;
use std::time::Duration;

use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{header, HeaderMap, HeaderValue, Response, StatusCode};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Redirect};
use axum::{Extension, Json};
use futures::{StreamExt, TryStreamExt};
use job_system::{JobOwner, JobProgressPhase, JobProgressSnapshot, JobStatus, MuxJobRequest};
use serde::{Deserialize, Serialize};
use tokio_util::io::ReaderStream;
use tracing::warn;
use utoipa::ToSchema;

use crate::auth::authenticated_user::AuthenticatedUser;
use crate::AppState;

const NO_STORE_CACHE_CONTROL: &str = "no-store, no-cache, must-revalidate";
const JOB_BUSY_RETRY_AFTER_SECS: u64 = 2;
const JOB_POLL_RETRY_AFTER_SECS: u64 = 1;
const DOWNLOAD_SESSION_HEADER: &str = "x-download-session-id";

#[derive(Debug)]
pub struct JobsApiError {
    pub message: String,
    pub status: StatusCode,
    pub retry_after_secs: Option<u64>,
}

impl IntoResponse for JobsApiError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(serde_json::json!({ "error": self.message }));
        let mut response = (self.status, body).into_response();
        response.headers_mut().insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static(NO_STORE_CACHE_CONTROL),
        );
        if let Some(seconds) = self.retry_after_secs {
            if let Ok(value) = HeaderValue::from_str(&seconds.to_string()) {
                response.headers_mut().insert(header::RETRY_AFTER, value);
            }
        }
        response
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateJobRequest {
    pub video_url: String,
    pub audio_url: String,
    pub source_url: Option<String>,
    pub video_format_id: Option<String>,
    pub audio_format_id: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateJobResponse {
    pub job_id: String,
    pub status: String,
    pub poll_url: String,
    pub file_ticket_url: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JobStatusResponse {
    pub job_id: String,
    pub status: String,
    pub queue_position: Option<u64>,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
    pub file_size_bytes: Option<u64>,
    pub error: Option<String>,
    pub file_ticket_url: Option<String>,
    pub progress: Option<JobProgressResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JobProgressResponse {
    pub phase: String,
    pub percent: Option<f32>,
    pub uploaded_bytes: u64,
    pub total_bytes: Option<u64>,
    pub updated_at_ms: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JobFileTicketResponse {
    pub download_url: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JobReleaseResponse {
    pub released: bool,
}

pub async fn create_job_handler(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    headers: HeaderMap,
    Json(payload): Json<CreateJobRequest>,
) -> Result<(StatusCode, Json<CreateJobResponse>), JobsApiError> {
    let owner = resolve_job_owner(&user, &headers)?;
    validate_create_payload(&payload)?;

    let preferred_video_proxy = extractor::resolve_stream_proxy(&payload.video_url).await;
    let preferred_audio_proxy = extractor::resolve_stream_proxy(&payload.audio_url).await;

    let request = MuxJobRequest {
        video_url: payload.video_url,
        audio_url: payload.audio_url,
        source_url: payload.source_url,
        video_format_id: payload.video_format_id,
        audio_format_id: payload.audio_format_id,
        title: payload.title,
        preferred_video_proxy,
        preferred_audio_proxy,
    };

    let created = state
        .job_control_plane
        .create_job(&owner, request)
        .await
        .map_err(map_control_plane_error)?;

    Ok((
        if created.reused_existing {
            StatusCode::OK
        } else {
            StatusCode::ACCEPTED
        },
        Json(CreateJobResponse {
            job_id: created.job_id.clone(),
            status: created.status.as_str().to_string(),
            poll_url: build_status_url(&created.job_id),
            file_ticket_url: build_file_ticket_url(&created.job_id),
        }),
    ))
}

pub async fn job_status_handler(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    headers: HeaderMap,
    Path(job_id): Path<String>,
) -> Result<Json<JobStatusResponse>, JobsApiError> {
    let owner = resolve_job_owner(&user, &headers)?;
    let snapshot = state
        .job_control_plane
        .get_job_for_user(&job_id, &owner)
        .await
        .map_err(map_control_plane_error)?
        .ok_or_else(not_found_error)?;
    let progress = load_job_progress(
        &state,
        &snapshot.job_id,
        snapshot.status,
        snapshot.updated_at_ms,
    )
    .await;

    Ok(Json(build_job_status_response(snapshot, progress)))
}

pub async fn job_events_handler(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    headers: HeaderMap,
    Path(job_id): Path<String>,
) -> Result<Sse<impl futures::Stream<Item = Result<Event, std::convert::Infallible>>>, JobsApiError>
{
    let owner = resolve_job_owner(&user, &headers)?;
    let initial_snapshot = state
        .job_control_plane
        .get_job_for_user(&job_id, &owner)
        .await
        .map_err(map_control_plane_error)?
        .ok_or_else(not_found_error)?;
    let initial_progress = load_job_progress(
        &state,
        &initial_snapshot.job_id,
        initial_snapshot.status,
        initial_snapshot.updated_at_ms,
    )
    .await;
    let initial_event = build_job_status_response(initial_snapshot, initial_progress);

    let mut pubsub = state
        .job_progress_store
        .subscribe(&job_id)
        .await
        .map_err(map_control_plane_error)?;

    let stream = async_stream::stream! {
        yield Ok(sse_event("status", &initial_event));

        if is_terminal_status(&initial_event.status) {
            return;
        }

        let mut messages = pubsub.on_message();
        while let Some(message) = messages.next().await {
            let payload: Result<String, _> = message.get_payload();
            let Ok(payload) = payload else {
                warn!(job_id, "Failed to decode job progress pubsub payload");
                continue;
            };

            let Ok(snapshot) = serde_json::from_str::<JobProgressSnapshot>(&payload) else {
                warn!(job_id, "Failed to parse job progress pubsub payload");
                continue;
            };

            let event_payload = build_job_status_event(snapshot);
            let terminal = is_terminal_status(&event_payload.status);
            yield Ok(sse_event("status", &event_payload));
            if terminal {
                break;
            }
        }
    };

    Ok(Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keepalive"),
    ))
}

fn build_job_status_response(
    snapshot: crate::services::job_control_plane::JobStatusRecord,
    progress: Option<JobProgressResponse>,
) -> JobStatusResponse {
    let status = snapshot.status;
    let job_id = snapshot.job_id;
    JobStatusResponse {
        job_id: job_id.clone(),
        status: status.as_str().to_string(),
        queue_position: snapshot.queue_position,
        created_at_ms: snapshot.created_at_ms,
        updated_at_ms: snapshot.updated_at_ms,
        file_size_bytes: snapshot.file_size_bytes,
        error: snapshot.error,
        file_ticket_url: (status == JobStatus::Ready).then(|| build_file_ticket_url(&job_id)),
        progress,
    }
}

fn build_job_status_event(snapshot: JobProgressSnapshot) -> JobStatusResponse {
    let status = phase_to_job_status(snapshot.phase);
    let updated_at_ms = snapshot.updated_at_ms.max(0) as u64;
    JobStatusResponse {
        job_id: snapshot.job_id.clone(),
        status: status.as_str().to_string(),
        queue_position: None,
        created_at_ms: updated_at_ms,
        updated_at_ms,
        file_size_bytes: None,
        error: None,
        file_ticket_url: (status == JobStatus::Ready)
            .then(|| build_file_ticket_url(&snapshot.job_id)),
        progress: Some(map_job_progress(snapshot)),
    }
}

pub async fn job_file_ticket_handler(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    headers: HeaderMap,
    Path(job_id): Path<String>,
) -> Result<Json<JobFileTicketResponse>, JobsApiError> {
    let owner = resolve_job_owner(&user, &headers)?;
    let job = state
        .job_control_plane
        .get_job_for_user(&job_id, &owner)
        .await
        .map_err(map_control_plane_error)?
        .ok_or_else(not_found_error)?;

    if job.status != JobStatus::Ready {
        return Err(not_ready_error(job.status));
    }

    let ticket = state
        .storage_ticket_service
        .build_ticket(&job, state.mux_direct_download)
        .await
        .map_err(map_control_plane_error)?;

    Ok(Json(JobFileTicketResponse {
        download_url: ticket.download_url,
    }))
}

pub async fn job_file_handler(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    headers: HeaderMap,
    Path(job_id): Path<String>,
) -> Result<axum::response::Response, JobsApiError> {
    let owner = resolve_job_owner(&user, &headers)?;
    let job = state
        .job_control_plane
        .get_job_for_user(&job_id, &owner)
        .await
        .map_err(map_control_plane_error)?
        .ok_or_else(not_found_error)?;

    if job.status != JobStatus::Ready {
        return Err(not_ready_error(job.status));
    }

    if !state
        .storage_ticket_service
        .supports_proxy_file_stream(&job)
    {
        let ticket = state
            .storage_ticket_service
            .build_ticket(&job, true)
            .await
            .map_err(map_control_plane_error)?;
        return Ok(Redirect::temporary(&ticket.download_url).into_response());
    }

    let local_path =
        crate::services::storage_ticket_service::StorageTicketService::ensure_local_path(&job)
            .map_err(map_control_plane_error)?;

    let file = tokio::fs::File::open(&local_path)
        .await
        .map_err(|error| match error.kind() {
            ErrorKind::NotFound => JobsApiError {
                message: "Muxed output file no longer exists".to_string(),
                status: StatusCode::GONE,
                retry_after_secs: None,
            },
            _ => JobsApiError {
                message: format!("Failed to open muxed output file: {error}"),
                status: StatusCode::INTERNAL_SERVER_ERROR,
                retry_after_secs: None,
            },
        })?;

    let stream = ReaderStream::new(file).map_err(std::io::Error::other);
    let body = Body::from_stream(stream);
    let filename = sanitize_filename(job.title.as_deref().unwrap_or("video"));
    let disposition = format!("attachment; filename=\"{}.mp4\"", filename);

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "video/mp4")
        .header(header::CACHE_CONTROL, NO_STORE_CACHE_CONTROL)
        .header(
            header::CONTENT_LENGTH,
            job.file_size_bytes.unwrap_or_default().to_string(),
        )
        .header(header::CONTENT_DISPOSITION, disposition)
        .body(body)
        .map_err(|error| JobsApiError {
            message: format!("Failed to build muxed file response: {error}"),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            retry_after_secs: None,
        })
}

pub async fn release_job_handler(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    headers: HeaderMap,
    Path(job_id): Path<String>,
) -> Result<Json<JobReleaseResponse>, JobsApiError> {
    let owner = resolve_job_owner(&user, &headers)?;
    let released = state
        .job_control_plane
        .touch_release(&job_id, &owner)
        .await
        .map_err(map_control_plane_error)?;
    Ok(Json(JobReleaseResponse { released }))
}

async fn load_job_progress(
    state: &AppState,
    job_id: &str,
    status: JobStatus,
    updated_at_ms: u64,
) -> Option<JobProgressResponse> {
    let snapshot = match state.job_progress_store.read_snapshot(job_id).await {
        Ok(snapshot) => snapshot,
        Err(error) => {
            warn!(job_id, err = %error, "Failed to load job progress snapshot");
            None
        }
    };

    snapshot
        .map(map_job_progress)
        .or_else(|| synthesize_terminal_progress(status, updated_at_ms))
}

fn map_job_progress(snapshot: JobProgressSnapshot) -> JobProgressResponse {
    JobProgressResponse {
        phase: snapshot.phase.as_str().to_string(),
        percent: snapshot.percent.map(|value| value.clamp(0.0, 100.0)),
        uploaded_bytes: snapshot.uploaded_bytes,
        total_bytes: snapshot.total_bytes,
        updated_at_ms: snapshot.updated_at_ms.max(0) as u64,
    }
}

fn phase_to_job_status(phase: JobProgressPhase) -> JobStatus {
    match phase {
        JobProgressPhase::Retrying => JobStatus::Queued,
        JobProgressPhase::Ready => JobStatus::Ready,
        JobProgressPhase::Failed => JobStatus::Failed,
        JobProgressPhase::Starting
        | JobProgressPhase::FetchingStreams
        | JobProgressPhase::MuxingUploading
        | JobProgressPhase::CompletingUpload => JobStatus::Processing,
    }
}

fn is_terminal_status(status: &str) -> bool {
    matches!(status, "ready" | "failed" | "expired")
}

fn sse_event(event: &str, payload: &JobStatusResponse) -> Event {
    let data =
        serde_json::to_string(payload).unwrap_or_else(|_| "{\"status\":\"failed\"}".to_string());
    Event::default().event(event).data(data)
}

fn synthesize_terminal_progress(
    status: JobStatus,
    updated_at_ms: u64,
) -> Option<JobProgressResponse> {
    let phase = match status {
        JobStatus::Ready => JobProgressPhase::Ready,
        JobStatus::Failed => JobProgressPhase::Failed,
        _ => return None,
    };

    Some(JobProgressResponse {
        phase: phase.as_str().to_string(),
        percent: (status == JobStatus::Ready).then_some(100.0),
        uploaded_bytes: 0,
        total_bytes: None,
        updated_at_ms,
    })
}

fn resolve_job_owner(
    user: &AuthenticatedUser,
    headers: &HeaderMap,
) -> Result<JobOwner, JobsApiError> {
    if let Some(user_id) = user
        .user_id
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        return Ok(JobOwner {
            user_id: Some(user_id.to_string()),
            session_id: None,
            scope_key: format!("user:{user_id}"),
        });
    }

    let Some(session_id) = headers
        .get(DOWNLOAD_SESSION_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| is_valid_session_id(value))
    else {
        return Err(JobsApiError {
            message: "Missing download session".to_string(),
            status: StatusCode::UNAUTHORIZED,
            retry_after_secs: None,
        });
    };

    Ok(JobOwner {
        user_id: None,
        session_id: Some(session_id.to_string()),
        scope_key: format!("session:{session_id}"),
    })
}

fn is_valid_session_id(value: &str) -> bool {
    let len = value.len();
    (16..=128).contains(&len)
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
}

fn validate_create_payload(payload: &CreateJobRequest) -> Result<(), JobsApiError> {
    proxy::client::validate_stream_url(&payload.video_url).map_err(|error| JobsApiError {
        message: format!("Video URL validation failed: {error}"),
        status: StatusCode::BAD_REQUEST,
        retry_after_secs: None,
    })?;
    proxy::client::validate_stream_url(&payload.audio_url).map_err(|error| JobsApiError {
        message: format!("Audio URL validation failed: {error}"),
        status: StatusCode::BAD_REQUEST,
        retry_after_secs: None,
    })?;

    if is_webm_mime(&payload.video_url, "video") {
        return Err(JobsApiError {
            message:
                "WebM video streams cannot be muxed into fMP4. Select an MP4 video stream instead."
                    .into(),
            status: StatusCode::UNPROCESSABLE_ENTITY,
            retry_after_secs: None,
        });
    }
    if is_webm_mime(&payload.audio_url, "audio") {
        return Err(JobsApiError {
            message:
                "WebM audio streams cannot be muxed into fMP4. Select an M4A audio stream instead."
                    .into(),
            status: StatusCode::UNPROCESSABLE_ENTITY,
            retry_after_secs: None,
        });
    }

    Ok(())
}

fn map_control_plane_error(error: anyhow::Error) -> JobsApiError {
    let message = error.to_string();
    let lower = message.to_ascii_lowercase();
    if lower.contains("queue is full")
        || lower.contains("queue overloaded")
        || lower.contains("queue unavailable")
    {
        return JobsApiError {
            message,
            status: StatusCode::SERVICE_UNAVAILABLE,
            retry_after_secs: Some(JOB_BUSY_RETRY_AFTER_SECS),
        };
    }

    JobsApiError {
        message,
        status: StatusCode::INTERNAL_SERVER_ERROR,
        retry_after_secs: None,
    }
}

fn not_found_error() -> JobsApiError {
    JobsApiError {
        message: "Mux job not found".to_string(),
        status: StatusCode::NOT_FOUND,
        retry_after_secs: None,
    }
}

fn not_ready_error(status: JobStatus) -> JobsApiError {
    JobsApiError {
        message: format!("Mux job is not ready (status: {})", status.as_str()),
        status: StatusCode::CONFLICT,
        retry_after_secs: Some(JOB_POLL_RETRY_AFTER_SECS),
    }
}

fn build_status_url(job_id: &str) -> String {
    format!("/api/jobs/{job_id}")
}

fn build_file_ticket_url(job_id: &str) -> String {
    format!("/api/jobs/{job_id}/file-ticket")
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            c if c.is_control() => '_',
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            c => c,
        })
        .take(100)
        .collect::<String>()
        .trim()
        .to_string()
}

fn is_webm_mime(url: &str, media_kind: &str) -> bool {
    let encoded = format!("mime={}%2Fwebm", media_kind);
    let plain = format!("mime={}/webm", media_kind);
    url.contains(&encoded) || url.contains(&plain)
}
