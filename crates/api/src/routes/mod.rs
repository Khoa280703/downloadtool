//! HTTP route handlers for the API server

pub mod batch;
pub mod extract;
pub mod jobs;
pub mod openapi;
pub mod playlist_jobs;
pub mod static_files;
pub mod stream;
pub mod whop_webhook;

pub use batch::batch_handler;
pub use extract::extract_handler;
pub use jobs::{
    create_job_handler, job_events_handler, job_file_handler, job_file_ticket_handler,
    job_status_handler, release_job_handler,
};
pub use openapi::openapi_handler;
pub use playlist_jobs::{
    cancel_playlist_job_handler, create_playlist_job_handler, get_playlist_job_handler,
    playlist_job_events_handler, start_playlist_job_handler,
};
pub use static_files::{bm_js_handler, userscript_handler};
pub use stream::stream_handler;
pub use whop_webhook::whop_webhook_handler;

use axum::http::StatusCode;

/// Health check endpoint.
/// Returns 200 OK when the server is running.
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let status = health_check().await;
        assert_eq!(status, StatusCode::OK);
    }
}
