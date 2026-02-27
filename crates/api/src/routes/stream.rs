//! Stream proxy handler
//!
//! GET /api/stream - Proxy video stream from source to client
//! GET /api/stream/muxed - Mux audio+video streams into fMP4

use axum::body::Body;
use axum::extract::Query;
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use serde::Deserialize;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use proxy::client::{parse_range_header, validate_stream_url, ProxyClient, Range};
use proxy::cookie_store::Platform;
use proxy::stream::forward_stream_headers;

use muxer::stream_fetcher::StreamFetcher;
use muxer::{remux_streams, MuxerError};

/// Chunk size for YouTube CDN throttle bypass: 9.5 MB.
///
/// YouTube CDN throttles full-file requests to ~2 Mbps (governed by `initcwndbps`).
/// Each sub-range request is served at full line speed. Fetching in ≤9.5 MB explicit
/// range chunks bypasses the per-file throttle entirely — same technique as yt-dlp.
const YOUTUBE_CHUNK_SIZE: u64 = 9_500_000;
const NO_STORE_CACHE_CONTROL: &str = "no-store, no-cache, must-revalidate";

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
#[derive(Debug, Deserialize, utoipa::ToSchema)]
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
            .header(header::CACHE_CONTROL, NO_STORE_CACHE_CONTROL)
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
/// Three modes:
/// 1. Browser sent `Range` header (seek/resume) → single proxied range request.
/// 2. YouTube CDN URL with `clen` param → chunked download to bypass throttle.
/// 3. Fallback → single proxied request.
pub async fn stream_handler(
    Query(params): Query<StreamParams>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    info!("Stream request for URL: {}", params.url);

    let parsed_url = validate_stream_url(&params.url).map_err(|e| ApiError {
        message: format!("URL validation failed: {}", e),
        status: StatusCode::BAD_REQUEST,
    })?;

    let platform = detect_platform(&params.url);
    let browser_range = extract_range_from_headers(&headers);

    // Mode 1: browser Range header present (seek / resume) → single range request
    if let Some(range) = browser_range {
        return proxy_single_request(&params, Some(range), platform).await;
    }

    // Mode 2: YouTube CDN with clen → chunked download (throttle bypass)
    if let Some(total_size) = extract_clen(&parsed_url) {
        info!(
            "Chunked download: {} bytes in ~{}MB chunks",
            total_size,
            YOUTUBE_CHUNK_SIZE / 1_000_000
        );
        return proxy_chunked(&params, &parsed_url, total_size, platform).await;
    }

    // Mode 3: fallback single request
    proxy_single_request(&params, None, platform).await
}

/// Proxy a single (possibly ranged) request — used for seek/resume or non-CDN URLs.
async fn proxy_single_request(
    params: &StreamParams,
    range: Option<Range>,
    platform: Platform,
) -> Result<Response, ApiError> {
    let client = ProxyClient::new(platform).map_err(|e| ApiError {
        message: format!("Failed to create proxy client: {}", e),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    match client.fetch_stream_with_headers(&params.url, range).await {
        Ok((source_headers, byte_stream)) => {
            let mut response_headers = forward_stream_headers(&source_headers);
            let filename = build_filename(&params.title, &params.format);
            add_content_disposition(&mut response_headers, &filename);
            add_cors_headers(&mut response_headers);
            add_no_store_header(&mut response_headers);

            let body = Body::from_stream(byte_stream);
            let mut rb = Response::builder();
            for (k, v) in response_headers.iter() {
                rb = rb.header(k.as_str(), v.as_bytes());
            }
            rb.body(body).map_err(|e| ApiError {
                message: format!("Failed to build response: {}", e),
                status: StatusCode::INTERNAL_SERVER_ERROR,
            })
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

/// Proxy via sequential chunked download — bypasses YouTube CDN full-file throttle.
///
/// Sets `Content-Length` from `clen` so Chrome shows a real progress bar.
async fn proxy_chunked(
    params: &StreamParams,
    parsed_url: &reqwest::Url,
    total_size: u64,
    platform: Platform,
) -> Result<Response, ApiError> {
    let content_type = extract_mime_type(parsed_url)
        .unwrap_or_else(|| "application/octet-stream".to_string());

    let mut response_headers = HeaderMap::new();
    response_headers.insert(
        "Content-Type",
        HeaderValue::from_str(&content_type)
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")),
    );
    // Explicit Content-Length lets Chrome show download progress
    response_headers.insert(
        "Content-Length",
        HeaderValue::from_str(&total_size.to_string()).expect("u64 fits header"),
    );
    response_headers.insert("Accept-Ranges", HeaderValue::from_static("bytes"));

    let filename = build_filename(&params.title, &params.format);
    add_content_disposition(&mut response_headers, &filename);
    add_cors_headers(&mut response_headers);
    add_no_store_header(&mut response_headers);

    let stream = chunked_stream(params.url.clone(), total_size, 0, platform);
    let body = Body::from_stream(stream);

    let mut rb = Response::builder();
    for (k, v) in response_headers.iter() {
        rb = rb.header(k.as_str(), v.as_bytes());
    }
    rb.body(body).map_err(|e| ApiError {
        message: format!("Failed to build response: {}", e),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    })
}

/// Build a byte stream that downloads `url` in YOUTUBE_CHUNK_SIZE chunks.
///
/// Each chunk is requested with an explicit `Range: bytes=start-end` header.
/// YouTube CDN delivers these at full line speed regardless of file size.
/// A background task drives the downloads; bytes are forwarded through a channel.
fn chunked_stream(
    url: String,
    total_size: u64,
    start_offset: u64,
    platform: Platform,
) -> impl Stream<Item = Result<Bytes, std::io::Error>> + Send + 'static {
    let (tx, rx) = mpsc::channel::<Result<Bytes, std::io::Error>>(8);

    tokio::spawn(async move {
        let client = match ProxyClient::new(platform) {
            Ok(c) => c,
            Err(e) => {
                let _ = tx
                    .send(Err(std::io::Error::other(format!(
                        "Failed to create proxy client: {}",
                        e
                    ))))
                    .await;
                return;
            }
        };

        let mut offset = start_offset;
        while offset < total_size {
            let end = (offset + YOUTUBE_CHUNK_SIZE - 1).min(total_size - 1);
            let range = Range {
                start: offset,
                end: Some(end),
            };
            debug!("Chunk: bytes={}-{} / {}", offset, end, total_size);

            match client.fetch_stream_with_headers(&url, Some(range)).await {
                Ok((_, mut chunk_stream)) => {
                    while let Some(item) = chunk_stream.next().await {
                        let result =
                            item.map_err(|e| std::io::Error::other(e.to_string()));
                        if tx.send(result).await.is_err() {
                            return; // Client disconnected — abort silently
                        }
                    }
                }
                Err(e) => {
                    error!("Chunk fetch failed bytes={}-{}: {}", offset, end, e);
                    let _ = tx
                        .send(Err(std::io::Error::other(format!(
                            "Chunk fetch failed: {}",
                            e
                        ))))
                        .await;
                    return;
                }
            }

            offset = end + 1;
        }
    });

    // Convert mpsc::Receiver to Stream
    futures::stream::unfold(rx, |mut rx| async move {
        rx.recv().await.map(|item| (item, rx))
    })
}

/// Muxed stream endpoint.
///
/// GET /api/stream/muxed?video_url=<encoded>&audio_url=<encoded>&title=<encoded>
pub async fn muxed_stream_handler(
    Query(params): Query<MuxedStreamParams>,
) -> Result<Response, ApiError> {
    info!("Muxed stream request");
    debug!("Video URL: {}", params.video_url);
    debug!("Audio URL: {}", params.audio_url);
    if let Some(video_codec) = params.video_codec.as_deref() {
        debug!("Requested video codec: {}", video_codec);
    }
    if let Some(audio_codec) = params.audio_codec.as_deref() {
        debug!("Requested audio codec: {}", audio_codec);
    }

    let _ = validate_stream_url(&params.video_url).map_err(|e| ApiError {
        message: format!("Video URL validation failed: {}", e),
        status: StatusCode::BAD_REQUEST,
    })?;

    let _ = validate_stream_url(&params.audio_url).map_err(|e| ApiError {
        message: format!("Audio URL validation failed: {}", e),
        status: StatusCode::BAD_REQUEST,
    })?;

    // Detect WebM video streams early — before headers are sent.
    // WebM uses EBML container, not ISO BMFF, so fMP4 remuxing is not supported.
    // YouTube embeds mime=video%2Fwebm (URL-encoded) in the stream URL for VP9 streams.
    if params.video_url.contains("mime=video%2Fwebm")
        || params.video_url.contains("mime=video/webm")
    {
        return Err(ApiError {
            message: "WebM video streams cannot be muxed into fMP4. Select an H.264/AV1 (MP4) video stream instead.".into(),
            status: StatusCode::UNPROCESSABLE_ENTITY,
        });
    }

    let platform = detect_platform(&params.video_url);

    let (video_stream, audio_stream) =
        StreamFetcher::fetch_both(&params.video_url, &params.audio_url, platform)
            .await
            .map_err(|e| ApiError {
                message: format!("Failed to fetch streams: {}", e),
                status: StatusCode::BAD_GATEWAY,
            })?;

    info!("Starting remux (copy-based fMP4 box remuxer)");

    let muxed_stream = remux_streams(video_stream, audio_stream);

    let mut response_headers = HeaderMap::new();
    response_headers.insert("Content-Type", HeaderValue::from_static("video/mp4"));

    let filename = build_filename(&params.title, &Some("mp4".to_string()));
    add_content_disposition(&mut response_headers, &filename);
    add_cors_headers(&mut response_headers);
    add_no_store_header(&mut response_headers);

    let compat_stream = stream_with_muxer_error(muxed_stream);
    let body = Body::from_stream(compat_stream);

    let mut rb = Response::builder();
    for (k, v) in response_headers.iter() {
        rb = rb.header(k.as_str(), v.as_bytes());
    }
    rb.body(body).map_err(|e| ApiError {
        message: format!("Failed to build response: {}", e),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    })
}

/// Convert a muxer error stream to std::io::Error for axum compatibility.
fn stream_with_muxer_error(
    stream: impl Stream<Item = Result<Bytes, MuxerError>> + Send + Unpin + 'static,
) -> impl Stream<Item = Result<Bytes, std::io::Error>> + Send {
    futures::stream::unfold(stream, |mut stream| async move {
        match stream.next().await {
            Some(Ok(bytes)) => Some((Ok(bytes), stream)),
            Some(Err(e)) => Some((
                Err(std::io::Error::other(e.to_string())),
                stream,
            )),
            None => None,
        }
    })
}

/// Detect platform from URL (always YouTube for now).
fn detect_platform(_url: &str) -> Platform {
    Platform::YouTube
}

/// Extract `clen` (content length) from CDN URL query params.
fn extract_clen(url: &reqwest::Url) -> Option<u64> {
    url.query_pairs()
        .find(|(k, _)| k == "clen")
        .and_then(|(_, v)| v.parse().ok())
}

/// Extract MIME type from CDN URL `mime` query param (already percent-decoded by reqwest).
fn extract_mime_type(url: &reqwest::Url) -> Option<String> {
    url.query_pairs()
        .find(|(k, _)| k == "mime")
        .map(|(_, v)| v.into_owned())
}

/// Add Content-Disposition header for file download.
/// Uses RFC 5987 encoding for non-ASCII filenames (e.g. Vietnamese titles).
fn add_content_disposition(headers: &mut HeaderMap, filename: &str) {
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

    let disposition = format!(
        r#"attachment; filename="{}"; filename*=UTF-8''{}"#,
        ascii_name, encoded
    );

    headers.insert(
        "Content-Disposition",
        HeaderValue::from_str(&disposition)
            .unwrap_or_else(|_| HeaderValue::from_static("attachment")),
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

/// Disable response caching for dynamic API payloads.
fn add_no_store_header(headers: &mut HeaderMap) {
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static(NO_STORE_CACHE_CONTROL),
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

    format!("{}.{}", base, ext)
}

/// Sanitize filename by removing/replacing invalid characters.
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
        assert_eq!(build_filename(&Some("Test".to_string()), &None), "Test.mp4");
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("My:Video|Name?"), "My_Video_Name_");
        assert_eq!(sanitize_filename("normal_name"), "normal_name");
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
        assert_eq!(headers.get("Access-Control-Allow-Origin").unwrap(), "*");
        assert_eq!(
            headers.get("Access-Control-Allow-Methods").unwrap(),
            "GET, HEAD, OPTIONS"
        );
    }

    #[test]
    fn test_extract_clen() {
        let url = reqwest::Url::parse(
            "https://rr1.googlevideo.com/videoplayback?clen=20971520&mime=video%2Fmp4",
        )
        .unwrap();
        assert_eq!(extract_clen(&url), Some(20_971_520));
    }

    #[test]
    fn test_extract_mime_type() {
        let url = reqwest::Url::parse(
            "https://rr1.googlevideo.com/videoplayback?clen=1234&mime=video%2Fmp4",
        )
        .unwrap();
        assert_eq!(extract_mime_type(&url).as_deref(), Some("video/mp4"));
    }
}
