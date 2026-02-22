//! Transcode handler for GPU-accelerated video processing
//!
//! POST /api/transcode - Transcode video with GPU acceleration

use axum::body::Body;
use axum::extract::Json;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn};

/// Transcode request body.
#[derive(Debug, Deserialize)]
pub struct TranscodeRequest {
    /// URL of the video to transcode
    pub url: String,
    /// Transcoding mode
    pub mode: TranscodeMode,
    /// Output options
    pub options: Option<TranscodeOptions>,
}

/// Transcoding mode.
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TranscodeMode {
    /// Watermark overlay
    Watermark,
    /// Recompress at different bitrate/resolution
    Recompress,
    /// Watermark + recompress
    WatermarkAndRecompress,
}

/// Transcoding options.
#[derive(Debug, Deserialize)]
pub struct TranscodeOptions {
    /// Target resolution (e.g., "1080p", "720p")
    pub resolution: Option<String>,
    /// Target bitrate in kbps
    pub bitrate_kbps: Option<u32>,
    /// Output format (e.g., "mp4", "webm")
    pub format: Option<String>,
}

/// API error response.
#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
    pub message: String,
}

/// API error type.
#[derive(Debug)]
pub struct ApiError {
    message: String,
    status: StatusCode,
}

impl ApiError {
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        Self {
            message: message.into(),
            status,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&ApiErrorResponse {
            error: format!("{:?}", self.status),
            message: self.message,
        })
        .unwrap_or_else(|_| r#"{"error": "Internal Server Error"}"#.to_string());

        Response::builder()
            .status(self.status)
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap()
    }
}

/// Transcode video endpoint.
///
/// POST /api/transcode
///
/// Transcodes video using GPU acceleration (if available on the Home Server)
/// or falls back to CPU transcoding.
///
/// # Request Body
/// ```json
/// {
///     "url": "https://example.com/video.mp4",
///     "mode": "watermark",
///     "options": {
///         "resolution": "1080p",
///         "bitrate_kbps": 5000,
///         "format": "mp4"
///     }
/// }
/// ```
///
/// # Response
/// Returns a streaming response with the transcoded video data.
pub async fn transcode_handler(
    Json(body): Json<TranscodeRequest>,
) -> Result<Response, ApiError> {
    info!("Transcode request received: url={}, mode={:?}", body.url, body.mode);

    // Validate URL
    if body.url.is_empty() {
        return Err(ApiError::new("URL is required", StatusCode::BAD_REQUEST));
    }

    // For now, return a 503 indicating GPU transcoding is not yet available
    // This will be implemented when the gRPC client is integrated
    warn!("GPU transcoding requested but not fully implemented");

    // TODO: Implement gRPC client to communicate with Home Server
    // 1. Establish gRPC connection to Home Server via WireGuard tunnel
    // 2. Stream video chunks to GPU worker
    // 3. Receive transcoded chunks and stream to client

    Err(ApiError::new(
        "GPU transcoding is not yet available. Please use /api/stream for direct streaming.",
        StatusCode::SERVICE_UNAVAILABLE,
    ))
}

/// Check if GPU transcoding is available.
///
/// GET /api/transcode/health
///
/// Returns the status of GPU transcoding service.
pub async fn transcode_health_check() -> impl IntoResponse {
    // TODO: Check gRPC connection to Home Server
    let response = serde_json::json!({
        "available": false,
        "reason": "GPU transcoding not yet implemented",
        "gpu_info": null
    });

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(response.to_string()))
        .unwrap()
}

/// Build response headers for transcoded video.
fn build_response_headers(
    title: Option<&str>,
    format: &str,
) -> HeaderMap {
    let mut headers = HeaderMap::new();

    // Content-Type based on format
    let content_type = match format {
        "mp4" | "m4v" => "video/mp4",
        "webm" => "video/webm",
        "mkv" => "video/x-matroska",
        _ => "video/mp4",
    };
    headers.insert("Content-Type", HeaderValue::from_static(content_type));

    // Content-Disposition for download
    let filename = title.unwrap_or("video");
    let sanitized = sanitize_filename(filename);
    let disposition = format!(r#"attachment; filename="{}.{}")"#, sanitized, format);
    headers.insert(
        "Content-Disposition",
        HeaderValue::from_str(&disposition).unwrap_or_else(|_| {
            HeaderValue::from_static("attachment")
        }),
    );

    // CORS headers
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));

    headers
}

/// Sanitize filename for Content-Disposition header.
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            c => c,
        })
        .take(100)
        .collect::<String>()
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(
            sanitize_filename("My:Video|Name?"),
            "My_Video_Name_"
        );
        assert_eq!(
            sanitize_filename("normal_name"),
            "normal_name"
        );
    }

    #[test]
    fn test_build_response_headers() {
        let headers = build_response_headers(Some("Test Video"), "mp4");

        assert_eq!(
            headers.get("Content-Type").unwrap(),
            "video/mp4"
        );
        assert!(
            headers.get("Content-Disposition").unwrap()
                .to_str().unwrap()
                .contains("Test Video")
        );
    }

    #[test]
    fn test_api_error_response() {
        let error = ApiError::new("Test error", StatusCode::BAD_REQUEST);
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
