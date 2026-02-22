//! Stream proxy handler
//!
//! GET /api/stream - Proxy video stream from source to client
//! GET /api/stream/muxed - Mux audio+video streams into fMP4

use axum::body::Body;
use axum::extract::Query;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use futures::Stream;
use serde::Deserialize;
use tracing::{debug, error, info, warn};

use proxy::client::{parse_range_header, validate_stream_url, ProxyClient, Range};
use proxy::cookie_store::Platform;
use proxy::stream::forward_stream_headers;

use muxer::codec::Codec;
use muxer::mux_router::{MuxRouter, StreamSource};
use muxer::stream_fetcher::StreamFetcher;
use muxer::{mux_streams, MuxerError};

/// Query parameters for stream proxy.
#[derive(Debug, Deserialize)]
pub struct StreamParams {
    /// URL of the stream to proxy (URL-encoded)
    pub url: String,
    /// Video title for Content-Disposition header
    pub title: Option<String>,
    /// File format extension
    pub format: Option<String>,
}

/// Query parameters for muxed stream.
#[derive(Debug, Deserialize)]
pub struct MuxedStreamParams {
    /// Video stream URL (URL-encoded)
    pub video_url: String,
    /// Audio stream URL (URL-encoded)
    pub audio_url: String,
    /// Video codec (e.g., "h264", "vp9")
    pub video_codec: Option<String>,
    /// Audio codec (e.g., "aac", "opus")
    pub audio_codec: Option<String>,
    /// Video title for Content-Disposition header
    pub title: Option<String>,
}

/// API error type.
#[derive(Debug)]
pub struct ApiError {
    message: String,
    status: StatusCode,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = format!(r#"{{"error": "{}"}}"#, self.message);
        Response::builder()
            .status(self.status)
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap()
    }
}

impl From<MuxerError> for ApiError {
    fn from(err: MuxerError) -> Self {
        Self {
            message: format!("Muxing error: {}", err),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// Stream proxy endpoint.
///
/// GET /api/stream?url=<encoded>&title=<encoded>&format=mp4
///
/// Validates URL against allowlist, fetches from source CDN,
/// and pipes bytes to browser with zero-copy streaming.
pub async fn stream_handler(
    Query(params): Query<StreamParams>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    info!("Stream request for URL: {}", params.url);

    // Validate URL against allowlist
    let parsed_url = validate_stream_url(&params.url).map_err(|e| ApiError {
        message: format!("URL validation failed: {}", e),
        status: StatusCode::BAD_REQUEST,
    })?;

    info!(
        "Validated stream URL: {}",
        parsed_url.host_str().unwrap_or("unknown")
    );

    // Parse Range header from request (if present) for resume support
    let range = extract_range_from_headers(&headers);

    // Create proxy client and fetch stream
    let client = ProxyClient::new().map_err(|e| ApiError {
        message: format!("Failed to create proxy client: {}", e),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    match client.fetch_stream_with_headers(&params.url, range).await {
        Ok((source_headers, byte_stream)) => {
            info!("Successfully connected to source stream");

            // Build response headers
            let mut response_headers = forward_stream_headers(&source_headers);

            // Add Content-Disposition for download
            let filename = build_filename(&params.title, &params.format);
            add_content_disposition(&mut response_headers, &filename);

            // Add CORS headers
            add_cors_headers(&mut response_headers);

            // Create response body from stream
            let body = Body::from_stream(byte_stream);

            let mut response_builder = Response::builder();

            // Copy headers to response
            for (key, value) in response_headers.iter() {
                response_builder = response_builder.header(key.as_str(), value.as_bytes());
            }

            let response = response_builder.body(body).map_err(|e| ApiError {
                message: format!("Failed to build response: {}", e),
                status: StatusCode::INTERNAL_SERVER_ERROR,
            })?;

            Ok(response)
        }
        Err(e) => {
            error!("Failed to fetch stream: {}", e);
            Err(ApiError {
                message: format!("Failed to fetch stream: {}", e),
                status: StatusCode::BAD_GATEWAY,
            })
        }
    }
}

/// Muxed stream endpoint.
///
/// GET /api/stream/muxed?video_url=<encoded>&audio_url=<encoded>&title=<encoded>
///
/// Fetches separate video and audio streams, muxes them into fMP4,
/// and returns as a single stream.
pub async fn muxed_stream_handler(
    Query(params): Query<MuxedStreamParams>,
) -> Result<Response, ApiError> {
    info!("Muxed stream request");
    debug!("Video URL: {}", params.video_url);
    debug!("Audio URL: {}", params.audio_url);

    // Validate URLs
    let _ = validate_stream_url(&params.video_url).map_err(|e| ApiError {
        message: format!("Video URL validation failed: {}", e),
        status: StatusCode::BAD_REQUEST,
    })?;

    let _ = validate_stream_url(&params.audio_url).map_err(|e| ApiError {
        message: format!("Audio URL validation failed: {}", e),
        status: StatusCode::BAD_REQUEST,
    })?;

    // Detect platform from URLs
    let platform = detect_platform(&params.video_url);

    // Fetch both streams concurrently
    let (video_stream, audio_stream) = StreamFetcher::fetch_both(
        &params.video_url,
        &params.audio_url,
        platform,
    )
    .await
    .map_err(|e| ApiError {
        message: format!("Failed to fetch streams: {}", e),
        status: StatusCode::BAD_GATEWAY,
    })?;

    // Parse codecs
    let video_codec = params
        .video_codec
        .as_deref()
        .and_then(Codec::from_string)
        .unwrap_or(Codec::H264);

    let audio_codec = params
        .audio_codec
        .as_deref()
        .and_then(Codec::from_string)
        .unwrap_or(Codec::AAC);

    info!(
        "Starting mux with video: {:?}, audio: {:?}",
        video_codec, audio_codec
    );

    // Create muxed stream
    let muxed_stream = mux_streams(video_stream, audio_stream, video_codec, audio_codec);

    // Build response headers
    let mut response_headers = HeaderMap::new();

    // Set Content-Type for MP4
    response_headers.insert(
        "Content-Type",
        HeaderValue::from_static("video/mp4"),
    );

    // Add Content-Disposition for download
    let filename = build_filename(&params.title, &Some("mp4".to_string()));
    add_content_disposition(&mut response_headers, &filename);

    // Add CORS headers
    add_cors_headers(&mut response_headers);

    // Note: No Content-Length as we don't know the final size
    // This triggers chunked transfer encoding

    // Create response body from muxed stream
    // Convert MuxerError to std::io::Error for axum compatibility
    let compat_stream = stream_with_muxer_error(muxed_stream);
    let body = Body::from_stream(compat_stream);

    let mut response_builder = Response::builder();

    // Copy headers to response
    for (key, value) in response_headers.iter() {
        response_builder = response_builder.header(key.as_str(), value.as_bytes());
    }

    let response = response_builder.body(body).map_err(|e| ApiError {
        message: format!("Failed to build response: {}", e),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(response)
}

/// Convert a muxer error stream to std::io::Error for axum compatibility.
fn stream_with_muxer_error(
    stream: impl Stream<Item = Result<Bytes, MuxerError>> + Send + 'static,
) -> impl Stream<Item = Result<Bytes, std::io::Error>> + Send {
    futures::stream::unfold(stream, |mut stream| async move {
        match stream.next().await {
            Some(Ok(bytes)) => Some((Ok(bytes), stream)),
            Some(Err(e)) => Some((
                Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())),
                stream,
            )),
            None => None,
        }
    })
}

/// Detect platform from URL.
fn detect_platform(url: &str) -> Platform {
    let url_lower = url.to_lowercase();

    if url_lower.contains("googlevideo.com")
        || url_lower.contains("youtube.com")
        || url_lower.contains("youtu.be")
    {
        Platform::YouTube
    } else if url_lower.contains("tiktok") {
        Platform::TikTok
    } else {
        Platform::YouTube // Default
    }
}

/// Add Content-Disposition header for file download.
fn add_content_disposition(headers: &mut HeaderMap, filename: &str) {
    let disposition = format!(r#"attachment; filename="{}""#, filename);
    headers.insert(
        "Content-Disposition",
        HeaderValue::from_str(&disposition).unwrap_or_else(|_| {
            HeaderValue::from_static("attachment")
        }),
    );
}

/// Add CORS headers to response.
fn add_cors_headers(headers: &mut HeaderMap) {
    headers.insert(
        "Access-Control-Allow-Origin",
        HeaderValue::from_static("*"),
    );
    headers.insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("GET, HEAD, OPTIONS"),
    );
}

/// Build filename from title and format.
fn build_filename(title: &Option<String>, format: &Option<String>) -> String {
    let base = title
        .as_ref()
        .map(|t| sanitize_filename(t))
        .unwrap_or_else(|| "video".to_string());

    let ext = format
        .as_ref()
        .map(|f| f.trim_start_matches('.').to_string())
        .unwrap_or_else(|| "mp4".to_string());

    format!("{}.{}" , base, ext)
}

/// Sanitize filename by removing/replacing invalid characters.
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            c => c,
        })
        .take(100) // Limit length
        .collect::<String>()
        .trim()
        .to_string()
}

/// Parse Range header from incoming request headers.
pub fn extract_range_from_headers(headers: &HeaderMap) -> Option<Range> {
    headers
        .get("Range")
        .and_then(|h| h.to_str().ok())
        .and_then(parse_range_header)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_filename() {
        assert_eq!(
            build_filename(&Some("My Video".to_string()), &Some("mp4".to_string())),
            "My Video.mp4"
        );
        assert_eq!(
            build_filename(&None, &Some("webm".to_string())),
            "video.webm"
        );
        assert_eq!(
            build_filename(&Some("Test".to_string()), &None),
            "Test.mp4"
        );
    }

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
    fn test_extract_range_from_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("Range", HeaderValue::from_static("bytes=0-1023"));

        let range = extract_range_from_headers(&headers).unwrap();
        assert_eq!(range.start, 0);
        assert_eq!(range.end, Some(1023));
    }

    #[test]
    fn test_detect_platform_youtube() {
        assert_eq!(
            detect_platform("https://rr1---sn-abc.googlevideo.com/videoplayback"),
            Platform::YouTube
        );
        assert_eq!(
            detect_platform("https://youtube.com/watch?v=abc"),
            Platform::YouTube
        );
    }

    #[test]
    fn test_detect_platform_tiktok() {
        assert_eq!(
            detect_platform("https://v16-webapp.tiktokcdn.com/video"),
            Platform::TikTok
        );
    }

    #[test]
    fn test_add_content_disposition() {
        let mut headers = HeaderMap::new();
        add_content_disposition(&mut headers, "My Video.mp4");

        let value = headers.get("Content-Disposition").unwrap();
        assert!(value.to_str().unwrap().contains("My Video.mp4"));
    }

    #[test]
    fn test_add_cors_headers() {
        let mut headers = HeaderMap::new();
        add_cors_headers(&mut headers);

        assert_eq!(
            headers.get("Access-Control-Allow-Origin").unwrap(),
            "*"
        );
        assert_eq!(
            headers.get("Access-Control-Allow-Methods").unwrap(),
            "GET, HEAD, OPTIONS"
        );
    }
}
