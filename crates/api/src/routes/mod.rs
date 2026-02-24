//! HTTP route handlers for the API server

pub mod batch;
pub mod extract;
pub mod openapi;
pub mod static_files;
pub mod stream;
pub mod transcode;

pub use batch::batch_handler;
pub use extract::extract_handler;
pub use openapi::openapi_handler;
pub use static_files::{bm_js_handler, userscript_handler};
pub use stream::{muxed_stream_handler, stream_handler};
pub use transcode::{transcode_handler, transcode_health_check};

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
