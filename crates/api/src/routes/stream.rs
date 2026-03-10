//! Stream proxy handler
//!
//! GET /api/stream - Proxy video stream from source to client

use axum::body::Body;
use axum::extract::Query;
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use serde::Deserialize;
use std::sync::{Arc, OnceLock};
use tokio::sync::mpsc;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tracing::{debug, error, info};

use crate::limit_profiles::backend_limit_profile;
use proxy::anti_bot::AntiBotError;
use proxy::client::ProxyError;
use proxy::client::{parse_range_header, validate_stream_url, ProxyClient, Range};
use proxy::stream::forward_stream_headers;
use proxy::Platform;

/// Chunk size for YouTube CDN throttle bypass: 9.5 MB.
///
/// YouTube CDN throttles full-file requests to ~2 Mbps (governed by `initcwndbps`).
/// Each sub-range request is served at full line speed. Fetching in ≤9.5 MB explicit
/// range chunks bypasses the per-file throttle entirely — same technique as yt-dlp.
const YOUTUBE_CHUNK_SIZE: u64 = 9_500_000;
const NO_STORE_CACHE_CONTROL: &str = "no-store, no-cache, must-revalidate";
const BACKPRESSURE_RETRY_AFTER_SECS: u64 = 2;

static STREAM_SEMAPHORE: OnceLock<Option<Arc<Semaphore>>> = OnceLock::new();
static STREAM_URL_REFRESH_MAX_ATTEMPTS: OnceLock<Option<usize>> = OnceLock::new();

/// Query parameters for stream proxy.
#[derive(Debug, Deserialize)]
pub struct StreamParams {
    /// URL of the stream to proxy (URL-encoded)
    pub url: String,
    /// Original watch URL used to extract formats (for refresh-on-auth-failure)
    pub source_url: Option<String>,
    /// Selected yt-dlp format_id (for refresh-on-auth-failure)
    pub format_id: Option<String>,
    /// Video title for Content-Disposition header
    pub title: Option<String>,
    /// File format extension
    pub format: Option<String>,
}

/// API error type.
#[derive(Debug)]
pub struct ApiError {
    message: String,
    status: StatusCode,
    retry_after_secs: Option<u64>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = format!(r#"{{"error": "{}"}}"#, self.message);
        let mut builder = Response::builder()
            .status(self.status)
            .header("content-type", "application/json")
            .header(header::CACHE_CONTROL, NO_STORE_CACHE_CONTROL);

        if let Some(seconds) = self.retry_after_secs {
            if let Ok(value) = HeaderValue::from_str(&seconds.to_string()) {
                builder = builder.header(header::RETRY_AFTER, value);
            }
        }

        builder.body(Body::from(body)).unwrap()
    }
}

fn stream_semaphore() -> Option<Arc<Semaphore>> {
    STREAM_SEMAPHORE
        .get_or_init(|| {
            backend_limit_profile()
                .stream_max_concurrent_value()
                .map(|limit| Arc::new(Semaphore::new(limit)))
        })
        .clone()
}

fn stream_url_refresh_max_attempts() -> Option<usize> {
    *STREAM_URL_REFRESH_MAX_ATTEMPTS
        .get_or_init(|| backend_limit_profile().stream_url_refresh_max_attempts_value())
}

fn within_retry_limit(attempts: usize, max_attempts: Option<usize>) -> bool {
    max_attempts
        .map(|max_attempts| attempts < max_attempts)
        .unwrap_or(true)
}

fn acquire_backpressure_permit(
    semaphore: Option<Arc<Semaphore>>,
    endpoint: &'static str,
) -> Result<Option<OwnedSemaphorePermit>, ApiError> {
    match semaphore {
        Some(semaphore) => semaphore
            .try_acquire_owned()
            .map(Some)
            .map_err(|_| ApiError {
                message: format!(
                    "Server is busy for {}. Please retry after a short delay.",
                    endpoint
                ),
                status: StatusCode::SERVICE_UNAVAILABLE,
                retry_after_secs: Some(BACKPRESSURE_RETRY_AFTER_SECS),
            }),
        None => Ok(None),
    }
}

fn stream_with_permit_guard<E, S>(
    stream: S,
    permit: Option<OwnedSemaphorePermit>,
) -> impl Stream<Item = Result<Bytes, E>> + Send
where
    E: Send + 'static,
    S: Stream<Item = Result<Bytes, E>> + Send + 'static,
{
    async_stream::stream! {
        let _permit = permit;
        let stream = stream;
        futures::pin_mut!(stream);
        while let Some(item) = stream.next().await {
            yield item;
        }
    }
}

fn proxy_error_status(error: &ProxyError) -> Option<StatusCode> {
    match error {
        ProxyError::RequestFailed(err) => err.status(),
        ProxyError::AntiBotFailed(AntiBotError::RequestFailed(err)) => err.status(),
        _ => None,
    }
}

fn is_upstream_auth_status(status: StatusCode) -> bool {
    status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN
}

fn is_auth_like_error_message(message: &str) -> bool {
    let normalized = message.to_ascii_lowercase();
    normalized.contains("401 unauthorized")
        || normalized.contains("403 forbidden")
        || normalized.contains("status client error (401")
        || normalized.contains("status client error (403")
        || normalized.contains("http status client error (401")
        || normalized.contains("http status client error (403")
}

fn is_auth_like_proxy_error(error: &ProxyError) -> bool {
    proxy_error_status(error)
        .map(is_upstream_auth_status)
        .unwrap_or(false)
        || is_auth_like_error_message(&error.to_string())
}

fn find_refreshed_format_url(
    formats: &[extractor::VideoFormat],
    format_id: Option<&str>,
    fallback_url: &str,
    expected_audio_only: Option<bool>,
    expected_has_audio: Option<bool>,
    expected_ext: Option<&str>,
) -> Option<String> {
    if let Some(id) = format_id {
        if let Some(found) = formats.iter().find(|f| f.format_id == id) {
            return Some(found.url.clone());
        }
    }

    let fallback_ext_owned = expected_ext.map(|ext| ext.to_string()).or_else(|| {
        reqwest::Url::parse(fallback_url)
            .ok()
            .and_then(|url| {
                url.query_pairs()
                    .find(|(k, _)| k == "mime")
                    .map(|(_, v)| v.to_string())
            })
            .and_then(|mime| mime.split('/').nth(1).map(|v| v.to_lowercase()))
    });
    let fallback_ext = fallback_ext_owned.as_deref();

    formats
        .iter()
        .filter(|format| {
            if let Some(audio_only) = expected_audio_only {
                if format.is_audio_only != audio_only {
                    return false;
                }
            }
            if let Some(has_audio) = expected_has_audio {
                if format.has_audio != has_audio {
                    return false;
                }
            }
            if let Some(ext) = fallback_ext {
                if !format.ext.eq_ignore_ascii_case(ext) {
                    return false;
                }
            }
            true
        })
        .map(|format| format.url.clone())
        .next()
}

async fn refresh_stream_url(
    source_url: &str,
    format_id: Option<&str>,
    fallback_url: &str,
    expected_audio_only: Option<bool>,
    expected_has_audio: Option<bool>,
    expected_ext: Option<&str>,
) -> Option<String> {
    let refreshed = extractor::extract(source_url).await.ok()?;
    find_refreshed_format_url(
        &refreshed.formats,
        format_id,
        fallback_url,
        expected_audio_only,
        expected_has_audio,
        expected_ext,
    )
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
    let permit = acquire_backpressure_permit(stream_semaphore(), "/api/stream")?;

    let parsed_url = validate_stream_url(&params.url).map_err(|e| ApiError {
        message: format!("URL validation failed: {}", e),
        status: StatusCode::BAD_REQUEST,
        retry_after_secs: None,
    })?;

    let platform = detect_platform(&params.url);
    let browser_range = extract_range_from_headers(&headers);

    // Mode 1: browser Range header present (seek / resume) → single range request
    if let Some(range) = browser_range {
        return proxy_single_request(&params, Some(range), platform, permit).await;
    }

    // Mode 2: YouTube CDN with clen → chunked download (throttle bypass)
    if let Some(total_size) = extract_clen(&parsed_url) {
        info!(
            "Chunked download: {} bytes in ~{}MB chunks",
            total_size,
            YOUTUBE_CHUNK_SIZE / 1_000_000
        );
        return proxy_chunked(&params, &parsed_url, total_size, platform, permit).await;
    }

    // Mode 3: fallback single request
    proxy_single_request(&params, None, platform, permit).await
}

/// Proxy a single (possibly ranged) request — used for seek/resume or non-CDN URLs.
async fn proxy_single_request(
    params: &StreamParams,
    range: Option<Range>,
    platform: Platform,
    permit: Option<OwnedSemaphorePermit>,
) -> Result<Response, ApiError> {
    let mut target_url = params.url.clone();
    let max_refresh_attempts = stream_url_refresh_max_attempts();
    let mut refresh_attempts = 0usize;
    loop {
        let pinned_proxy = extractor::resolve_stream_proxy(&target_url).await;
        let client = ProxyClient::new_with_proxy(platform, pinned_proxy).map_err(|e| ApiError {
            message: format!("Failed to create proxy client: {}", e),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            retry_after_secs: None,
        })?;

        match client.fetch_stream_with_headers(&target_url, range).await {
            Ok((source_headers, byte_stream)) => {
                let mut response_headers = forward_stream_headers(&source_headers);
                let filename = build_filename(&params.title, &params.format);
                add_content_disposition(&mut response_headers, &filename);
                add_cors_headers(&mut response_headers);
                add_no_store_header(&mut response_headers);

                let guarded_stream = stream_with_permit_guard(byte_stream, permit);
                let body = Body::from_stream(guarded_stream);
                let mut rb = Response::builder();
                for (k, v) in response_headers.iter() {
                    rb = rb.header(k.as_str(), v.as_bytes());
                }
                return rb.body(body).map_err(|e| ApiError {
                    message: format!("Failed to build response: {}", e),
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    retry_after_secs: None,
                });
            }
            Err(e) => {
                let can_refresh = within_retry_limit(refresh_attempts, max_refresh_attempts)
                    && is_auth_like_proxy_error(&e)
                    && params.source_url.is_some();

                if can_refresh {
                    refresh_attempts += 1;
                    if let Some(source_url) = params.source_url.as_deref() {
                        if let Some(new_url) = refresh_stream_url(
                            source_url,
                            params.format_id.as_deref(),
                            &target_url,
                            None,
                            None,
                            params.format.as_deref(),
                        )
                        .await
                        {
                            info!(
                                attempt = refresh_attempts,
                                max_attempts = ?max_refresh_attempts,
                                "Refreshed stream URL after upstream auth error"
                            );
                            target_url = new_url;
                            continue;
                        }
                    }
                }

                error!("Failed to fetch stream: {}", e);
                return Err(ApiError {
                    message: format!("Failed to fetch stream: {}", e),
                    status: StatusCode::BAD_GATEWAY,
                    retry_after_secs: None,
                });
            }
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
    permit: Option<OwnedSemaphorePermit>,
) -> Result<Response, ApiError> {
    let content_type =
        extract_mime_type(parsed_url).unwrap_or_else(|| "application/octet-stream".to_string());

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

    let stream = chunked_stream(
        params.url.clone(),
        total_size,
        0,
        platform,
        params.source_url.clone(),
        params.format_id.clone(),
        params.format.clone(),
    );
    let guarded_stream = stream_with_permit_guard(stream, permit);
    let body = Body::from_stream(guarded_stream);

    let mut rb = Response::builder();
    for (k, v) in response_headers.iter() {
        rb = rb.header(k.as_str(), v.as_bytes());
    }
    rb.body(body).map_err(|e| ApiError {
        message: format!("Failed to build response: {}", e),
        status: StatusCode::INTERNAL_SERVER_ERROR,
        retry_after_secs: None,
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
    source_url: Option<String>,
    format_id: Option<String>,
    preferred_format: Option<String>,
) -> impl Stream<Item = Result<Bytes, std::io::Error>> + Send + 'static {
    let (tx, rx) = mpsc::channel::<Result<Bytes, std::io::Error>>(8);

    tokio::spawn(async move {
        let mut active_url = url;
        let mut active_total_size = total_size;
        let max_refresh_attempts = stream_url_refresh_max_attempts();
        let mut refresh_attempts = 0usize;
        let mut offset = start_offset;

        let mut forced_proxy = extractor::resolve_stream_proxy(&active_url).await;
        let mut client = match ProxyClient::new_with_proxy(platform, forced_proxy.clone()) {
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

        while offset < active_total_size {
            let end = (offset + YOUTUBE_CHUNK_SIZE - 1).min(active_total_size - 1);
            let range = Range {
                start: offset,
                end: Some(end),
            };
            debug!("Chunk: bytes={}-{} / {}", offset, end, active_total_size);

            match client
                .fetch_stream_with_headers(&active_url, Some(range))
                .await
            {
                Ok((_, mut chunk_stream)) => {
                    while let Some(item) = chunk_stream.next().await {
                        let result = item.map_err(|e| std::io::Error::other(e.to_string()));
                        if tx.send(result).await.is_err() {
                            return; // Client disconnected — abort silently
                        }
                    }
                }
                Err(e) => {
                    let can_refresh = within_retry_limit(refresh_attempts, max_refresh_attempts)
                        && is_auth_like_proxy_error(&e)
                        && source_url.is_some();
                    if can_refresh {
                        refresh_attempts += 1;
                        if let Some(source) = source_url.as_deref() {
                            if let Some(new_url) = refresh_stream_url(
                                source,
                                format_id.as_deref(),
                                &active_url,
                                None,
                                None,
                                preferred_format.as_deref(),
                            )
                            .await
                            {
                                info!(
                                    attempt = refresh_attempts,
                                    max_attempts = ?max_refresh_attempts,
                                    "Refreshed chunked stream URL after upstream auth error"
                                );
                                active_url = new_url;
                                if let Ok(parsed) = reqwest::Url::parse(&active_url) {
                                    if let Some(clen) = extract_clen(&parsed) {
                                        active_total_size = clen;
                                    }
                                }
                                let refreshed_proxy =
                                    extractor::resolve_stream_proxy(&active_url).await;
                                if refreshed_proxy.is_some() {
                                    forced_proxy = refreshed_proxy;
                                }
                                client = match ProxyClient::new_with_proxy(
                                    platform,
                                    forced_proxy.clone(),
                                ) {
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
                                continue;
                            }
                        }
                    }

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
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
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
