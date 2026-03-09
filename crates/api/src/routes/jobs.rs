use std::io::ErrorKind;

use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{header, HeaderValue, Response, StatusCode};
use axum::response::{IntoResponse, Redirect};
use axum::{Extension, Json};
use futures::TryStreamExt;
use job_system::{JobStatus, MuxJobRequest};
use serde::{Deserialize, Serialize};
use tokio_util::io::ReaderStream;
use utoipa::ToSchema;

use crate::auth::authenticated_user::AuthenticatedUser;
use crate::AppState;

const NO_STORE_CACHE_CONTROL: &str = "no-store, no-cache, must-revalidate";
const JOB_BUSY_RETRY_AFTER_SECS: u64 = 2;
const JOB_POLL_RETRY_AFTER_SECS: u64 = 1;

#[derive(Debug)]
pub struct JobsApiError {
    message: String,
    status: StatusCode,
    retry_after_secs: Option<u64>,
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
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
    pub file_size_bytes: Option<u64>,
    pub error: Option<String>,
    pub file_ticket_url: Option<String>,
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
    Json(payload): Json<CreateJobRequest>,
) -> Result<(StatusCode, Json<CreateJobResponse>), JobsApiError> {
    let user_id = require_authenticated_user(&user)?;
    validate_create_payload(&payload)?;

    let request = MuxJobRequest {
        video_url: payload.video_url,
        audio_url: payload.audio_url,
        source_url: payload.source_url,
        video_format_id: payload.video_format_id,
        audio_format_id: payload.audio_format_id,
        title: payload.title,
    };

    let created = state
        .job_control_plane
        .create_job(user_id, request)
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
    Path(job_id): Path<String>,
) -> Result<Json<JobStatusResponse>, JobsApiError> {
    let user_id = require_authenticated_user(&user)?;
    let snapshot = state
        .job_control_plane
        .get_job_for_user(&job_id, user_id)
        .await
        .map_err(map_control_plane_error)?
        .ok_or_else(not_found_error)?;

    Ok(Json(JobStatusResponse {
        job_id: snapshot.job_id,
        status: snapshot.status.as_str().to_string(),
        created_at_ms: snapshot.created_at_ms,
        updated_at_ms: snapshot.updated_at_ms,
        file_size_bytes: snapshot.file_size_bytes,
        error: snapshot.error,
        file_ticket_url: (snapshot.status == JobStatus::Ready)
            .then(|| build_file_ticket_url(&job_id)),
    }))
}

pub async fn job_file_ticket_handler(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(job_id): Path<String>,
) -> Result<Json<JobFileTicketResponse>, JobsApiError> {
    let user_id = require_authenticated_user(&user)?;
    let job = state
        .job_control_plane
        .get_job_for_user(&job_id, user_id)
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
    Path(job_id): Path<String>,
) -> Result<axum::response::Response, JobsApiError> {
    let user_id = require_authenticated_user(&user)?;
    let job = state
        .job_control_plane
        .get_job_for_user(&job_id, user_id)
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
    Path(job_id): Path<String>,
) -> Result<Json<JobReleaseResponse>, JobsApiError> {
    let user_id = require_authenticated_user(&user)?;
    let released = state
        .job_control_plane
        .touch_release(&job_id, user_id)
        .await
        .map_err(map_control_plane_error)?;
    Ok(Json(JobReleaseResponse { released }))
}

fn require_authenticated_user(user: &AuthenticatedUser) -> Result<&str, JobsApiError> {
    if !user.is_authenticated() {
        return Err(JobsApiError {
            message: "Authentication required".to_string(),
            status: StatusCode::UNAUTHORIZED,
            retry_after_secs: None,
        });
    }

    user.user_id.as_deref().ok_or(JobsApiError {
        message: "Authentication required".to_string(),
        status: StatusCode::UNAUTHORIZED,
        retry_after_secs: None,
    })
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
