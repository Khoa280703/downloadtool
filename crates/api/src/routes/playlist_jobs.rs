//! Playlist job API routes
//!
//! POST /api/playlist-jobs          - Create a new playlist job
//! GET  /api/playlist-jobs/:id      - Get playlist job status + items
//! GET  /api/playlist-jobs/:id/events - SSE stream of playlist job progress
//! POST /api/playlist-jobs/:id/start - Start processing selected playlist items
//! POST /api/playlist-jobs/:id/cancel - Cancel a playlist job

use std::collections::HashSet;
use std::convert::Infallible;
use std::time::Duration;

use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::Json;
use futures::future::join_all;
use job_system::{
    CreatePlaylistJobRequest, JobProgressPhase, JobProgressSnapshot, PlaylistJobItemRecord,
    PlaylistJobRecord, PlaylistJobStatus,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::auth::authenticated_user::AuthenticatedUser;
use crate::routes::jobs::JobsApiError;
use axum::Extension;

const DOWNLOAD_SESSION_HEADER: &str = "x-download-session-id";
const PLAYLIST_EVENTS_POLL_INTERVAL_MS: u64 = 500;

// -- Request / Response types --

#[derive(Debug, Deserialize)]
pub struct CreatePlaylistJobPayload {
    pub url: String,
    pub quality: Option<String>,
    pub mode: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PlaylistJobResponse {
    pub job_id: String,
    pub status: String,
    pub source_url: String,
    pub title: Option<String>,
    pub total_items: i32,
    pub completed_items: i32,
    pub failed_items: i32,
    pub requested_quality: String,
    pub requested_mode: String,
    pub created_at_ms: i64,
    pub updated_at_ms: i64,
    pub items: Vec<PlaylistJobItemResponse>,
}

#[derive(Debug, Serialize)]
pub struct PlaylistJobItemResponse {
    pub id: String,
    pub video_id: String,
    pub title: Option<String>,
    pub thumbnail: Option<String>,
    pub ordinal: i32,
    pub status: String,
    pub attempt_count: i32,
    pub last_error: Option<String>,
    pub selected: bool,
    pub mux_job_id: Option<String>,
    pub download_url: Option<String>,
    pub progress: Option<PlaylistJobItemProgressResponse>,
}

#[derive(Debug, Serialize)]
pub struct PlaylistJobItemProgressResponse {
    pub phase: String,
    pub percent: Option<f32>,
    pub uploaded_bytes: u64,
    pub total_bytes: Option<u64>,
    pub updated_at_ms: u64,
}

#[derive(Debug, Deserialize)]
pub struct StartPlaylistJobPayload {
    pub selected_video_ids: Vec<String>,
    pub quality: Option<String>,
    pub mode: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreatePlaylistJobResponse {
    pub job_id: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct CancelPlaylistJobResponse {
    pub cancelled: bool,
    pub cancelled_items: i32,
}

#[derive(Debug, Serialize)]
pub struct StartPlaylistJobResponse {
    pub started: bool,
    pub selected_items: i32,
}

// -- Handlers --

pub async fn create_playlist_job_handler(
    State(state): State<crate::AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    headers: HeaderMap,
    Json(payload): Json<CreatePlaylistJobPayload>,
) -> Result<(StatusCode, Json<CreatePlaylistJobResponse>), JobsApiError> {
    let (user_id, session_id) = resolve_playlist_owner(&user, &headers)?;

    if payload.url.trim().is_empty() {
        return Err(JobsApiError {
            message: "URL is required".to_string(),
            status: StatusCode::BAD_REQUEST,
            retry_after_secs: None,
        });
    }

    // Host-based validation prevents SSRF — substring checks are insufficient.
    if !crate::validation::is_valid_youtube_url(&payload.url) {
        return Err(JobsApiError {
            message: "Invalid YouTube URL".to_string(),
            status: StatusCode::BAD_REQUEST,
            retry_after_secs: None,
        });
    }

    let request_ip = headers
        .get("cf-connecting-ip")
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    let request = CreatePlaylistJobRequest {
        source_url: payload.url,
        requested_quality: payload.quality,
        requested_mode: payload.mode,
    };

    let job = state
        .playlist_job_repository
        .create_job(
            &request,
            user_id.as_deref(),
            session_id.as_deref(),
            request_ip.as_deref(),
        )
        .await
        .map_err(|e| JobsApiError {
            message: format!("Failed to create playlist job: {e}"),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            retry_after_secs: None,
        })?;

    info!(job_id = %job.id, source_url = %job.source_url, "Playlist job created");

    // Spawn background processor
    crate::services::playlist_processor::spawn_playlist_discovery(
        job.clone(),
        state.playlist_job_repository.clone(),
        state.job_control_plane.clone(),
    );

    Ok((
        StatusCode::ACCEPTED,
        Json(CreatePlaylistJobResponse {
            job_id: job.id,
            status: job.status.as_str().to_string(),
        }),
    ))
}

pub async fn get_playlist_job_handler(
    State(state): State<crate::AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    headers: HeaderMap,
    Path(job_id): Path<String>,
) -> Result<Json<PlaylistJobResponse>, JobsApiError> {
    let (user_id, session_id) = resolve_playlist_owner(&user, &headers)?;

    let job = state
        .playlist_job_repository
        .get_job(&job_id, user_id.as_deref(), session_id.as_deref())
        .await
        .map_err(map_repo_error)?
        .ok_or_else(|| JobsApiError {
            message: "Playlist job not found".to_string(),
            status: StatusCode::NOT_FOUND,
            retry_after_secs: None,
        })?;

    let items = state
        .playlist_job_repository
        .get_items(&job_id)
        .await
        .map_err(map_repo_error)?;

    Ok(Json(build_playlist_response(&state, job, items).await))
}

pub async fn playlist_job_events_handler(
    State(state): State<crate::AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    headers: HeaderMap,
    Path(job_id): Path<String>,
) -> Result<Sse<impl futures::Stream<Item = Result<Event, Infallible>>>, JobsApiError> {
    let (user_id, session_id) = resolve_playlist_owner(&user, &headers)?;

    // Verify the job exists and belongs to the caller
    let job = state
        .playlist_job_repository
        .get_job(&job_id, user_id.as_deref(), session_id.as_deref())
        .await
        .map_err(map_repo_error)?
        .ok_or_else(|| JobsApiError {
            message: "Playlist job not found".to_string(),
            status: StatusCode::NOT_FOUND,
            retry_after_secs: None,
        })?;

    let repo = state.playlist_job_repository.clone();
    let jid = job_id.clone();

    let stream = async_stream::stream! {
        // Send initial snapshot
        let items = repo.get_items(&jid).await.unwrap_or_default();
        let snapshot = build_playlist_response(&state, job, items).await;
        yield Ok(sse_event("status", &snapshot));

        if snapshot.status == "completed" || snapshot.status == "failed" || snapshot.status == "cancelled" {
            return;
        }

        // Poll snapshots fast enough to show overlapping item progress in UI.
        loop {
            tokio::time::sleep(Duration::from_millis(PLAYLIST_EVENTS_POLL_INTERVAL_MS)).await;

            let Some(current_job) = repo.get_job(&jid, None, None).await.ok().flatten() else {
                break;
            };
            let current_items = repo.get_items(&jid).await.unwrap_or_default();
            let resp = build_playlist_response(&state, current_job.clone(), current_items).await;
            let terminal = current_job.status.is_terminal();

            yield Ok(sse_event("status", &resp));

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

pub async fn start_playlist_job_handler(
    State(state): State<crate::AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    headers: HeaderMap,
    Path(job_id): Path<String>,
    Json(payload): Json<StartPlaylistJobPayload>,
) -> Result<Json<StartPlaylistJobResponse>, JobsApiError> {
    let (user_id, session_id) = resolve_playlist_owner(&user, &headers)?;

    let job = state
        .playlist_job_repository
        .get_job(&job_id, user_id.as_deref(), session_id.as_deref())
        .await
        .map_err(map_repo_error)?
        .ok_or_else(|| JobsApiError {
            message: "Playlist job not found".to_string(),
            status: StatusCode::NOT_FOUND,
            retry_after_secs: None,
        })?;

    if job.status == PlaylistJobStatus::Processing || job.status == PlaylistJobStatus::Completed {
        return Ok(Json(StartPlaylistJobResponse {
            started: false,
            selected_items: job.total_items,
        }));
    }

    if job.status != PlaylistJobStatus::Ready {
        return Err(JobsApiError {
            message: "Playlist job is not ready to start".to_string(),
            status: StatusCode::CONFLICT,
            retry_after_secs: None,
        });
    }

    let unique_selected_ids: HashSet<String> = payload
        .selected_video_ids
        .into_iter()
        .filter(|video_id| !video_id.trim().is_empty())
        .collect();
    if unique_selected_ids.is_empty() {
        return Err(JobsApiError {
            message: "Select at least one video before starting".to_string(),
            status: StatusCode::BAD_REQUEST,
            retry_after_secs: None,
        });
    }

    let selected_video_ids: Vec<String> = unique_selected_ids.into_iter().collect();
    let selected_total = selected_video_ids.len() as i32;
    let requested_quality = payload
        .quality
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(job.requested_quality.as_str());
    let requested_mode = payload
        .mode
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(job.requested_mode.as_str());

    state
        .playlist_job_repository
        .prepare_items_for_start(&job_id, &selected_video_ids)
        .await
        .map_err(map_repo_error)?;

    state
        .playlist_job_repository
        .start_processing(&job_id, selected_total, requested_quality, requested_mode)
        .await
        .map_err(map_repo_error)?;

    let refreshed_job = state
        .playlist_job_repository
        .get_job(&job_id, user_id.as_deref(), session_id.as_deref())
        .await
        .map_err(map_repo_error)?
        .ok_or_else(|| JobsApiError {
            message: "Playlist job disappeared".to_string(),
            status: StatusCode::NOT_FOUND,
            retry_after_secs: None,
        })?;

    crate::services::playlist_processor::spawn_playlist_processor(
        refreshed_job,
        state.playlist_job_repository.clone(),
        state.job_control_plane.clone(),
        state.job_progress_store.clone(),
    );

    info!(job_id = %job_id, selected_total, "Playlist job started");

    Ok(Json(StartPlaylistJobResponse {
        started: true,
        selected_items: selected_total,
    }))
}

pub async fn cancel_playlist_job_handler(
    State(state): State<crate::AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    headers: HeaderMap,
    Path(job_id): Path<String>,
) -> Result<Json<CancelPlaylistJobResponse>, JobsApiError> {
    let (user_id, session_id) = resolve_playlist_owner(&user, &headers)?;

    let job = state
        .playlist_job_repository
        .get_job(&job_id, user_id.as_deref(), session_id.as_deref())
        .await
        .map_err(map_repo_error)?
        .ok_or_else(|| JobsApiError {
            message: "Playlist job not found".to_string(),
            status: StatusCode::NOT_FOUND,
            retry_after_secs: None,
        })?;

    if job.status.is_terminal() {
        return Ok(Json(CancelPlaylistJobResponse {
            cancelled: false,
            cancelled_items: 0,
        }));
    }

    let cancelled_items = state
        .playlist_job_repository
        .cancel_pending_items(&job_id)
        .await
        .map_err(map_repo_error)?;

    state
        .playlist_job_repository
        .update_job_status(&job_id, PlaylistJobStatus::Cancelled)
        .await
        .map_err(map_repo_error)?;

    info!(job_id = %job_id, cancelled_items, "Playlist job cancelled");

    Ok(Json(CancelPlaylistJobResponse {
        cancelled: true,
        cancelled_items,
    }))
}

// -- Helpers --

fn resolve_playlist_owner(
    user: &AuthenticatedUser,
    headers: &HeaderMap,
) -> Result<(Option<String>, Option<String>), JobsApiError> {
    if let Some(uid) = user.user_id.as_deref().filter(|v| !v.trim().is_empty()) {
        return Ok((Some(uid.to_string()), None));
    }

    let session_id = headers
        .get(DOWNLOAD_SESSION_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|v| v.len() >= 16 && v.len() <= 128)
        .map(String::from);

    if session_id.is_none() {
        return Err(JobsApiError {
            message: "Missing download session".to_string(),
            status: StatusCode::UNAUTHORIZED,
            retry_after_secs: None,
        });
    }

    Ok((None, session_id))
}

async fn build_playlist_response(
    state: &crate::AppState,
    job: PlaylistJobRecord,
    items: Vec<PlaylistJobItemRecord>,
) -> PlaylistJobResponse {
    let items = join_all(items.into_iter().map(|item| map_item_response(state, item))).await;

    PlaylistJobResponse {
        job_id: job.id,
        status: job.status.as_str().to_string(),
        source_url: job.source_url,
        title: job.title,
        total_items: job.total_items,
        completed_items: job.completed_items,
        failed_items: job.failed_items,
        requested_quality: job.requested_quality,
        requested_mode: job.requested_mode,
        created_at_ms: job.created_at_ms,
        updated_at_ms: job.updated_at_ms,
        items,
    }
}

async fn map_item_response(
    state: &crate::AppState,
    item: PlaylistJobItemRecord,
) -> PlaylistJobItemResponse {
    let selected = item_selected(&item);
    let progress = load_playlist_item_progress(state, &item).await;
    PlaylistJobItemResponse {
        id: item.id,
        video_id: item.video_id,
        title: item.title,
        thumbnail: item.thumbnail,
        ordinal: item.ordinal,
        status: item.status.as_str().to_string(),
        attempt_count: item.attempt_count,
        last_error: item.last_error,
        selected,
        mux_job_id: item.mux_job_id,
        download_url: item.download_url,
        progress,
    }
}

async fn load_playlist_item_progress(
    state: &crate::AppState,
    item: &PlaylistJobItemRecord,
) -> Option<PlaylistJobItemProgressResponse> {
    let Some(mux_job_id) = item.mux_job_id.as_deref() else {
        return None;
    };

    match state.job_progress_store.read_snapshot(mux_job_id).await {
        Ok(Some(snapshot)) => Some(map_playlist_item_progress(snapshot)),
        Ok(None) => synthesize_playlist_item_progress(item),
        Err(_) => synthesize_playlist_item_progress(item),
    }
}

fn map_playlist_item_progress(snapshot: JobProgressSnapshot) -> PlaylistJobItemProgressResponse {
    PlaylistJobItemProgressResponse {
        phase: snapshot.phase.as_str().to_string(),
        percent: snapshot.percent.map(|value| value.clamp(0.0, 100.0)),
        uploaded_bytes: snapshot.uploaded_bytes,
        total_bytes: snapshot.total_bytes,
        updated_at_ms: snapshot.updated_at_ms.max(0) as u64,
    }
}

fn synthesize_playlist_item_progress(
    item: &PlaylistJobItemRecord,
) -> Option<PlaylistJobItemProgressResponse> {
    let phase = match item.status.as_str() {
        "queued_mux" => JobProgressPhase::Retrying,
        "ready" | "completed" => JobProgressPhase::Ready,
        "failed" => JobProgressPhase::Failed,
        _ => return None,
    };

    Some(PlaylistJobItemProgressResponse {
        phase: phase.as_str().to_string(),
        percent: matches!(phase, JobProgressPhase::Ready).then_some(100.0),
        uploaded_bytes: 0,
        total_bytes: None,
        updated_at_ms: item.updated_at_ms.max(0) as u64,
    })
}

fn item_selected(item: &PlaylistJobItemRecord) -> bool {
    item.selected_stream_meta
        .as_ref()
        .and_then(|value| value.get("selected"))
        .and_then(|value| value.as_bool())
        .unwrap_or(true)
}

fn sse_event(event_type: &str, payload: &PlaylistJobResponse) -> Event {
    let data =
        serde_json::to_string(payload).unwrap_or_else(|_| r#"{"status":"failed"}"#.to_string());
    Event::default().event(event_type).data(data)
}

fn map_repo_error(e: anyhow::Error) -> JobsApiError {
    JobsApiError {
        message: e.to_string(),
        status: StatusCode::INTERNAL_SERVER_ERROR,
        retry_after_secs: None,
    }
}
