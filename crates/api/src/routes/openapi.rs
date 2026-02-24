//! OpenAPI spec endpoint
//!
//! GET /openapi.json - Returns OpenAPI 3.x spec generated from utoipa annotations

use axum::Json;
use utoipa::OpenApi;

use crate::routes::extract::{ExtractRequest, ExtractResponse, StreamFormat, VideoMetadata};
use crate::routes::stream::MuxedStreamParams;

/// OpenAPI document definition for the YouTube Downloader API.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "YouTube Downloader API",
        version = "0.1.0",
        description = "API for extracting and streaming YouTube videos"
    ),
    components(schemas(
        ExtractRequest,
        ExtractResponse,
        VideoMetadata,
        StreamFormat,
        MuxedStreamParams,
    ))
)]
pub struct ApiDoc;

/// Return OpenAPI 3.x spec as JSON.
///
/// GET /openapi.json
pub async fn openapi_handler() -> Json<serde_json::Value> {
    Json(serde_json::to_value(ApiDoc::openapi()).unwrap_or_default())
}
