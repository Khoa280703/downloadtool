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
use std::sync::{Arc, OnceLock};
use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tracing::{debug, error, info};

use proxy::client::{parse_range_header, validate_stream_url, ProxyClient, Range};
use proxy::cookie_store::Platform;
use proxy::stream::forward_stream_headers;
use proxy::anti_bot::AntiBotError;
use proxy::client::ProxyError;

use muxer::stream_fetcher::{FetchBothRefreshOptions, StreamFetcher, StreamUrlRefreshContext};
use muxer::{remux_streams, MuxerError};

/// Chunk size for YouTube CDN throttle bypass: 9.5 MB.
///
/// YouTube CDN throttles full-file requests to ~2 Mbps (governed by `initcwndbps`).
/// Each sub-range request is served at full line speed. Fetching in ≤9.5 MB explicit
/// range chunks bypasses the per-file throttle entirely — same technique as yt-dlp.
const YOUTUBE_CHUNK_SIZE: u64 = 9_500_000;
const NO_STORE_CACHE_CONTROL: &str = "no-store, no-cache, must-revalidate";
const MUX_PREFLIGHT_TIMEOUT_DEFAULT_SECS: u64 = 15;
const STREAM_MAX_CONCURRENT_DEFAULT: usize = 128;
const MUX_MAX_CONCURRENT_DEFAULT: usize = 20;
const STREAM_URL_REFRESH_MAX_ATTEMPTS_DEFAULT: usize = 3;
const MUX_URL_REFRESH_MAX_ATTEMPTS_DEFAULT: usize = 3;
const BACKPRESSURE_RETRY_AFTER_SECS: u64 = 2;

static STREAM_SEMAPHORE: OnceLock<Arc<Semaphore>> = OnceLock::new();
static MUX_SEMAPHORE: OnceLock<Arc<Semaphore>> = OnceLock::new();
static MUX_PREFLIGHT_TIMEOUT: OnceLock<Duration> = OnceLock::new();
static STREAM_URL_REFRESH_MAX_ATTEMPTS: OnceLock<usize> = OnceLock::new();
static MUX_URL_REFRESH_MAX_ATTEMPTS: OnceLock<usize> = OnceLock::new();

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

/// Query parameters for muxed stream.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct MuxedStreamParams {
    /// Video stream URL (URL-encoded)
    pub video_url: String,
    /// Audio stream URL (URL-encoded)
    pub audio_url: String,
    /// Original watch URL used to extract formats (for refresh-on-auth-failure)
    pub source_url: Option<String>,
    /// Selected video format_id
    pub video_format_id: Option<String>,
    /// Selected audio format_id
    pub audio_format_id: Option<String>,
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

impl From<MuxerError> for ApiError {
    fn from(err: MuxerError) -> Self {
        Self {
            message: format!("Muxing error: {}", err),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            retry_after_secs: None,
        }
    }
}

fn read_env_usize(name: &str, default_value: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default_value)
}

fn read_env_u64(name: &str, default_value: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default_value)
}

fn stream_semaphore() -> Arc<Semaphore> {
    STREAM_SEMAPHORE
        .get_or_init(|| {
            Arc::new(Semaphore::new(read_env_usize(
                "STREAM_MAX_CONCURRENT",
                STREAM_MAX_CONCURRENT_DEFAULT,
            )))
        })
        .clone()
}

fn mux_semaphore() -> Arc<Semaphore> {
    MUX_SEMAPHORE
        .get_or_init(|| {
            Arc::new(Semaphore::new(read_env_usize(
                "MUX_MAX_CONCURRENT",
                MUX_MAX_CONCURRENT_DEFAULT,
            )))
        })
        .clone()
}

fn mux_preflight_timeout() -> Duration {
    *MUX_PREFLIGHT_TIMEOUT.get_or_init(|| {
        Duration::from_secs(read_env_u64(
            "MUX_PREFLIGHT_TIMEOUT_SECS",
            MUX_PREFLIGHT_TIMEOUT_DEFAULT_SECS,
        ))
    })
}

fn stream_url_refresh_max_attempts() -> usize {
    *STREAM_URL_REFRESH_MAX_ATTEMPTS.get_or_init(|| {
        read_env_usize(
            "STREAM_URL_REFRESH_MAX_ATTEMPTS",
            STREAM_URL_REFRESH_MAX_ATTEMPTS_DEFAULT,
        )
    })
}

fn mux_url_refresh_max_attempts() -> usize {
    *MUX_URL_REFRESH_MAX_ATTEMPTS.get_or_init(|| {
        read_env_usize(
            "MUX_URL_REFRESH_MAX_ATTEMPTS",
            MUX_URL_REFRESH_MAX_ATTEMPTS_DEFAULT,
        )
    })
}

fn acquire_backpressure_permit(
    semaphore: Arc<Semaphore>,
    endpoint: &'static str,
) -> Result<OwnedSemaphorePermit, ApiError> {
    semaphore
        .try_acquire_owned()
        .map_err(|_| ApiError {
            message: format!(
                "Server is busy for {}. Please retry after a short delay.",
                endpoint
            ),
            status: StatusCode::SERVICE_UNAVAILABLE,
            retry_after_secs: Some(BACKPRESSURE_RETRY_AFTER_SECS),
        })
}

fn stream_with_permit_guard<E, S>(
    stream: S,
    permit: OwnedSemaphorePermit,
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

fn anti_bot_error_status(error: &AntiBotError) -> Option<StatusCode> {
    match error {
        AntiBotError::RequestFailed(err) => err.status(),
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

fn is_auth_like_antibot_error(error: &AntiBotError) -> bool {
    anti_bot_error_status(error)
        .map(is_upstream_auth_status)
        .unwrap_or(false)
        || is_auth_like_error_message(&error.to_string())
}

fn is_auth_like_muxer_error(error: &MuxerError) -> bool {
    is_auth_like_error_message(&error.to_string())
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

    let fallback_ext_owned = expected_ext
        .map(|ext| ext.to_string())
        .or_else(|| {
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
    let refreshed = extractor::extract(source_url, None).await.ok()?;
    find_refreshed_format_url(
        &refreshed.formats,
        format_id,
        fallback_url,
        expected_audio_only,
        expected_has_audio,
        expected_ext,
    )
}

async fn refresh_mux_stream_urls(
    source_url: &str,
    video_format_id: Option<&str>,
    audio_format_id: Option<&str>,
    fallback_video_url: &str,
    fallback_audio_url: &str,
) -> Option<(String, String)> {
    let refreshed = extractor::extract(source_url, None).await.ok()?;
    let next_video = find_refreshed_format_url(
        &refreshed.formats,
        video_format_id,
        fallback_video_url,
        Some(false),
        Some(false),
        None,
    )?;
    let next_audio = find_refreshed_format_url(
        &refreshed.formats,
        audio_format_id,
        fallback_audio_url,
        Some(true),
        Some(true),
        None,
    )?;
    Some((next_video, next_audio))
}

fn build_mux_refresh_options(
    params: &MuxedStreamParams,
    max_refresh_attempts: usize,
) -> FetchBothRefreshOptions {
    let Some(source_url) = params.source_url.clone() else {
        return FetchBothRefreshOptions::default();
    };

    let video = StreamUrlRefreshContext {
        source_url: source_url.clone(),
        format_id: params.video_format_id.clone(),
        expected_audio_only: Some(false),
        expected_has_audio: Some(false),
        expected_ext: None,
        max_refresh_attempts,
    };

    let audio = StreamUrlRefreshContext {
        source_url,
        format_id: params.audio_format_id.clone(),
        expected_audio_only: Some(true),
        expected_has_audio: Some(true),
        expected_ext: None,
        max_refresh_attempts,
    };

    FetchBothRefreshOptions {
        video: Some(video),
        audio: Some(audio),
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
    permit: OwnedSemaphorePermit,
) -> Result<Response, ApiError> {
    let client = ProxyClient::new(platform).map_err(|e| ApiError {
        message: format!("Failed to create proxy client: {}", e),
        status: StatusCode::INTERNAL_SERVER_ERROR,
        retry_after_secs: None,
    })?;

    let mut target_url = params.url.clone();
    let max_refresh_attempts = stream_url_refresh_max_attempts();
    let mut refresh_attempts = 0usize;
    loop {
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
            let can_refresh = refresh_attempts < max_refresh_attempts
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
                            "Refreshed stream URL after upstream auth error (attempt {}/{})",
                            refresh_attempts, max_refresh_attempts
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
    permit: OwnedSemaphorePermit,
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

        let mut active_url = url;
        let mut active_total_size = total_size;
        let max_refresh_attempts = stream_url_refresh_max_attempts();
        let mut refresh_attempts = 0usize;
        let mut offset = start_offset;
        while offset < active_total_size {
            let end = (offset + YOUTUBE_CHUNK_SIZE - 1).min(active_total_size - 1);
            let range = Range {
                start: offset,
                end: Some(end),
            };
            debug!("Chunk: bytes={}-{} / {}", offset, end, active_total_size);

            match client.fetch_stream_with_headers(&active_url, Some(range)).await {
                Ok((_, mut chunk_stream)) => {
                    while let Some(item) = chunk_stream.next().await {
                        let result = item.map_err(|e| std::io::Error::other(e.to_string()));
                        if tx.send(result).await.is_err() {
                            return; // Client disconnected — abort silently
                        }
                    }
                }
                Err(e) => {
                    let can_refresh = refresh_attempts < max_refresh_attempts
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
                                    "Refreshed chunked stream URL after upstream auth error (attempt {}/{})",
                                    refresh_attempts, max_refresh_attempts
                                );
                                active_url = new_url;
                                if let Ok(parsed) = reqwest::Url::parse(&active_url) {
                                    if let Some(clen) = extract_clen(&parsed) {
                                        active_total_size = clen;
                                    }
                                }
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

/// Muxed stream endpoint.
///
/// GET /api/stream/muxed?video_url=<encoded>&audio_url=<encoded>&title=<encoded>
pub async fn muxed_stream_handler(
    Query(params): Query<MuxedStreamParams>,
) -> Result<Response, ApiError> {
    info!("Muxed stream request");
    let permit = acquire_backpressure_permit(mux_semaphore(), "/api/stream/muxed")?;
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
        retry_after_secs: None,
    })?;

    let _ = validate_stream_url(&params.audio_url).map_err(|e| ApiError {
        message: format!("Audio URL validation failed: {}", e),
        status: StatusCode::BAD_REQUEST,
        retry_after_secs: None,
    })?;

    // Detect WebM video streams early — before headers are sent.
    // WebM uses EBML container, not ISO BMFF, so fMP4 remuxing is not supported.
    // YouTube embeds mime=video%2Fwebm (URL-encoded) in the stream URL for VP9 streams.
    if is_webm_mime(&params.video_url, "video") {
        return Err(ApiError {
            message: "WebM video streams cannot be muxed into fMP4. Select an H.264/AV1 (MP4) video stream instead.".into(),
            status: StatusCode::UNPROCESSABLE_ENTITY,
            retry_after_secs: None,
        });
    }
    if is_webm_mime(&params.audio_url, "audio") {
        return Err(ApiError {
            message:
                "WebM audio streams cannot be muxed into fMP4. Select an AAC/M4A audio stream instead."
                    .into(),
            status: StatusCode::UNPROCESSABLE_ENTITY,
            retry_after_secs: None,
        });
    }

    let platform = detect_platform(&params.video_url);
    let mut video_url = params.video_url.clone();
    let mut audio_url = params.audio_url.clone();
    let max_refresh_attempts = mux_url_refresh_max_attempts();
    let fetch_refresh_options = build_mux_refresh_options(&params, max_refresh_attempts);
    let mut refresh_attempts = 0usize;
    let preflight_timeout = mux_preflight_timeout();
    let (first_chunk, muxed_stream) = loop {
        let (video_stream, audio_stream) = match StreamFetcher::fetch_both_with_refresh(
            &video_url,
            &audio_url,
            platform,
            fetch_refresh_options.clone(),
        )
        .await
        {
            Ok(streams) => streams,
            Err(error) => {
                let can_refresh = refresh_attempts < max_refresh_attempts
                    && is_auth_like_antibot_error(&error)
                    && params.source_url.is_some();

                if can_refresh {
                    refresh_attempts += 1;
                    if let Some(source_url) = params.source_url.as_deref() {
                        if let Some((next_video, next_audio)) = refresh_mux_stream_urls(
                            source_url,
                            params.video_format_id.as_deref(),
                            params.audio_format_id.as_deref(),
                            &video_url,
                            &audio_url,
                        )
                        .await
                        {
                            info!(
                                "Refreshed muxed URLs after upstream auth error (attempt {}/{})",
                                refresh_attempts, max_refresh_attempts
                            );
                            video_url = next_video;
                            audio_url = next_audio;
                            continue;
                        }
                    }
                }

                return Err(ApiError {
                    message: format!("Failed to fetch streams: {}", error),
                    status: StatusCode::BAD_GATEWAY,
                    retry_after_secs: None,
                });
            }
        };

        info!("Starting remux (copy-based fMP4 box remuxer)");
        let mut candidate_muxed_stream = remux_streams(video_stream, audio_stream);

        // Force a first-frame preflight so we don't commit HTTP 200 for a broken mux pipeline.
        match tokio::time::timeout(preflight_timeout, candidate_muxed_stream.next()).await {
            Ok(Some(Ok(chunk))) => break (chunk, candidate_muxed_stream),
            Ok(Some(Err(error))) => {
                let can_refresh = refresh_attempts < max_refresh_attempts
                    && is_auth_like_muxer_error(&error)
                    && params.source_url.is_some();

                if can_refresh {
                    refresh_attempts += 1;
                    if let Some(source_url) = params.source_url.as_deref() {
                        if let Some((next_video, next_audio)) = refresh_mux_stream_urls(
                            source_url,
                            params.video_format_id.as_deref(),
                            params.audio_format_id.as_deref(),
                            &video_url,
                            &audio_url,
                        )
                        .await
                        {
                            info!(
                                "Refreshed muxed URLs after mux preflight auth error (attempt {}/{})",
                                refresh_attempts, max_refresh_attempts
                            );
                            video_url = next_video;
                            audio_url = next_audio;
                            continue;
                        }
                    }
                }

                error!("Mux preflight failed before response commit: {}", error);
                return Err(ApiError {
                    message: format!("Muxing error: {}", error),
                    status: StatusCode::BAD_GATEWAY,
                    retry_after_secs: None,
                });
            }
            Ok(None) => {
                return Err(ApiError {
                    message: "Muxing produced an empty stream".into(),
                    status: StatusCode::BAD_GATEWAY,
                    retry_after_secs: None,
                });
            }
            Err(_) => {
                return Err(ApiError {
                    message: format!(
                        "Mux preflight timed out after {}s",
                        preflight_timeout.as_secs()
                    ),
                    status: StatusCode::GATEWAY_TIMEOUT,
                    retry_after_secs: None,
                });
            }
        }
    };

    let mut response_headers = HeaderMap::new();
    response_headers.insert("Content-Type", HeaderValue::from_static("video/mp4"));

    let filename = build_filename(&params.title, &Some("mp4".to_string()));
    add_content_disposition(&mut response_headers, &filename);
    add_cors_headers(&mut response_headers);
    add_no_store_header(&mut response_headers);

    let preface = futures::stream::once(async move { Ok::<Bytes, std::io::Error>(first_chunk) });
    let compat_stream = preface.chain(stream_with_muxer_error(muxed_stream));
    let guarded_stream = stream_with_permit_guard(compat_stream, permit);
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

/// Convert a muxer error stream to std::io::Error for axum compatibility.
fn stream_with_muxer_error(
    stream: impl Stream<Item = Result<Bytes, MuxerError>> + Send + Unpin + 'static,
) -> impl Stream<Item = Result<Bytes, std::io::Error>> + Send {
    futures::stream::unfold(stream, |mut stream| async move {
        match stream.next().await {
            Some(Ok(bytes)) => Some((Ok(bytes), stream)),
            Some(Err(e)) => Some((Err(std::io::Error::other(e.to_string())), stream)),
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

fn is_webm_mime(url: &str, media_kind: &str) -> bool {
    let encoded = format!("mime={}%2Fwebm", media_kind);
    let plain = format!("mime={}/webm", media_kind);
    url.contains(&encoded) || url.contains(&plain)
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

    #[test]
    fn test_is_webm_mime() {
        assert!(is_webm_mime(
            "https://x.test/videoplayback?mime=video%2Fwebm&itag=247",
            "video"
        ));
        assert!(is_webm_mime(
            "https://x.test/videoplayback?mime=audio/webm&itag=251",
            "audio"
        ));
        assert!(!is_webm_mime(
            "https://x.test/videoplayback?mime=audio%2Fmp4&itag=140",
            "audio"
        ));
    }
}
