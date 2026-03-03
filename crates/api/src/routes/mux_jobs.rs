use std::io::ErrorKind;

use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{header, HeaderValue, Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use tokio_util::io::ReaderStream;
use utoipa::ToSchema;

use crate::services::muxed_job_queue::{
    MuxJobQueueError, MuxJobRequest, MuxJobStatus,
};
use crate::AppState;

const NO_STORE_CACHE_CONTROL: &str = "no-store, no-cache, must-revalidate";
const JOB_BUSY_RETRY_AFTER_SECS: u64 = 2;
const JOB_POLL_RETRY_AFTER_SECS: u64 = 1;

#[derive(Debug)]
pub struct MuxJobsApiError {
    message: String,
    status: StatusCode,
    retry_after_secs: Option<u64>,
}

impl IntoResponse for MuxJobsApiError {
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
pub struct CreateMuxJobRequest {
    pub video_url: String,
    pub audio_url: String,
    pub source_url: Option<String>,
    pub video_format_id: Option<String>,
    pub audio_format_id: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateMuxJobResponse {
    pub job_id: String,
    pub status: String,
    pub status_url: String,
    pub file_url: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MuxJobStatusResponse {
    pub job_id: String,
    pub status: String,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
    pub file_size_bytes: Option<u64>,
    pub error: Option<String>,
    pub file_url: Option<String>,
}

pub async fn create_muxed_job_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateMuxJobRequest>,
) -> Result<(StatusCode, Json<CreateMuxJobResponse>), MuxJobsApiError> {
    validate_create_payload(&payload)?;

    let request = MuxJobRequest {
        video_url: payload.video_url,
        audio_url: payload.audio_url,
        source_url: payload.source_url,
        video_format_id: payload.video_format_id,
        audio_format_id: payload.audio_format_id,
        title: payload.title,
    };

    let job_id = state.mux_jobs.enqueue(request).await.map_err(map_queue_error)?;
    let response = CreateMuxJobResponse {
        status_url: build_status_url(&job_id),
        file_url: build_file_url(&job_id),
        status: "queued".to_string(),
        job_id,
    };

    Ok((StatusCode::ACCEPTED, Json(response)))
}

pub async fn muxed_job_status_handler(
    State(state): State<AppState>,
    Path(job_id): Path<String>,
) -> Result<Json<MuxJobStatusResponse>, MuxJobsApiError> {
    let snapshot = state
        .mux_jobs
        .get_snapshot(&job_id)
        .await
        .map_err(map_queue_error)?;

    let is_ready = snapshot.status == MuxJobStatus::Ready;
    let response = MuxJobStatusResponse {
        job_id: snapshot.job_id,
        status: snapshot.status.as_str().to_string(),
        created_at_ms: snapshot.created_at_ms,
        updated_at_ms: snapshot.updated_at_ms,
        file_size_bytes: snapshot.file_size_bytes,
        error: snapshot.error,
        file_url: if is_ready {
            Some(build_file_url(&job_id))
        } else {
            None
        },
    };

    Ok(Json(response))
}

pub async fn muxed_job_file_handler(
    State(state): State<AppState>,
    Path(job_id): Path<String>,
) -> Result<axum::response::Response, MuxJobsApiError> {
    let ready = state
        .mux_jobs
        .get_ready_file(&job_id)
        .await
        .map_err(map_queue_error)?;

    let file = tokio::fs::File::open(&ready.file_path)
        .await
        .map_err(|error| match error.kind() {
            ErrorKind::NotFound => MuxJobsApiError {
                message: "Muxed output file no longer exists".to_string(),
                status: StatusCode::GONE,
                retry_after_secs: None,
            },
            _ => MuxJobsApiError {
                message: format!("Failed to open muxed output file: {error}"),
                status: StatusCode::INTERNAL_SERVER_ERROR,
                retry_after_secs: None,
            },
        })?;

    let stream = ReaderStream::new(file).map_err(std::io::Error::other);
    let body = Body::from_stream(stream);

    let filename = build_filename(
        ready
            .title
            .as_deref()
            .map(sanitize_filename)
            .unwrap_or_else(|| "video".to_string()),
    );
    let disposition = build_content_disposition(&filename);

    let mut response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "video/mp4")
        .header(header::CACHE_CONTROL, NO_STORE_CACHE_CONTROL)
        .header(header::CONTENT_LENGTH, ready.file_size_bytes.to_string())
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, HEAD, OPTIONS")
        .header(header::CONTENT_DISPOSITION, disposition)
        .body(body)
        .map_err(|error| MuxJobsApiError {
            message: format!("Failed to build muxed file response: {error}"),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            retry_after_secs: None,
        })?;

    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static(NO_STORE_CACHE_CONTROL),
    );
    Ok(response)
}

fn validate_create_payload(payload: &CreateMuxJobRequest) -> Result<(), MuxJobsApiError> {
    proxy::client::validate_stream_url(&payload.video_url).map_err(|error| MuxJobsApiError {
        message: format!("Video URL validation failed: {error}"),
        status: StatusCode::BAD_REQUEST,
        retry_after_secs: None,
    })?;
    proxy::client::validate_stream_url(&payload.audio_url).map_err(|error| MuxJobsApiError {
        message: format!("Audio URL validation failed: {error}"),
        status: StatusCode::BAD_REQUEST,
        retry_after_secs: None,
    })?;

    if is_webm_mime(&payload.video_url, "video") {
        return Err(MuxJobsApiError {
            message: "WebM video streams cannot be muxed into fMP4. Select an MP4 video stream instead.".into(),
            status: StatusCode::UNPROCESSABLE_ENTITY,
            retry_after_secs: None,
        });
    }
    if is_webm_mime(&payload.audio_url, "audio") {
        return Err(MuxJobsApiError {
            message: "WebM audio streams cannot be muxed into fMP4. Select an M4A audio stream instead.".into(),
            status: StatusCode::UNPROCESSABLE_ENTITY,
            retry_after_secs: None,
        });
    }

    Ok(())
}

fn map_queue_error(error: MuxJobQueueError) -> MuxJobsApiError {
    match error {
        MuxJobQueueError::QueueFull => MuxJobsApiError {
            message: "Mux queue is full. Please retry shortly.".to_string(),
            status: StatusCode::SERVICE_UNAVAILABLE,
            retry_after_secs: Some(JOB_BUSY_RETRY_AFTER_SECS),
        },
        MuxJobQueueError::QueueOverloaded {
            retry_after_secs,
            estimated_wait_secs,
        } => MuxJobsApiError {
            message: format!(
                "Mux queue is busy (estimated wait ~{}s). Please retry shortly.",
                estimated_wait_secs
            ),
            status: StatusCode::SERVICE_UNAVAILABLE,
            retry_after_secs: Some(retry_after_secs),
        },
        MuxJobQueueError::QueueUnavailable => MuxJobsApiError {
            message: "Mux queue is unavailable right now".to_string(),
            status: StatusCode::SERVICE_UNAVAILABLE,
            retry_after_secs: Some(JOB_BUSY_RETRY_AFTER_SECS),
        },
        MuxJobQueueError::NotFound => MuxJobsApiError {
            message: "Mux job not found".to_string(),
            status: StatusCode::NOT_FOUND,
            retry_after_secs: None,
        },
        MuxJobQueueError::NotReady(status) => MuxJobsApiError {
            message: format!("Mux job is not ready (status: {})", status.as_str()),
            status: StatusCode::CONFLICT,
            retry_after_secs: Some(JOB_POLL_RETRY_AFTER_SECS),
        },
    }
}

fn build_status_url(job_id: &str) -> String {
    format!("/api/stream/muxed/jobs/{job_id}")
}

fn build_file_url(job_id: &str) -> String {
    format!("/api/stream/muxed/jobs/{job_id}/file")
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

fn build_filename(base: String) -> String {
    if base.is_empty() {
        "video.mp4".to_string()
    } else {
        format!("{base}.mp4")
    }
}

fn build_content_disposition(filename: &str) -> String {
    let ascii_name: String = filename
        .chars()
        .map(|c| if c.is_ascii() { c } else { '_' })
        .collect();
    let encoded: String = filename
        .bytes()
        .flat_map(|b| {
            if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
                vec![b as char]
            } else {
                format!("%{:02X}", b).chars().collect::<Vec<_>>()
            }
        })
        .collect();
    format!(
        r#"attachment; filename="{}"; filename*=UTF-8''{}"#,
        ascii_name, encoded
    )
}

fn is_webm_mime(url: &str, media_kind: &str) -> bool {
    let encoded = format!("mime={}%2Fwebm", media_kind);
    let plain = format!("mime={}/webm", media_kind);
    url.contains(&encoded) || url.contains(&plain)
}
