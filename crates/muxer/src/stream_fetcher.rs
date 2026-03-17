//! Concurrent stream fetching for audio and video
//!
//! Fetches both streams simultaneously using the AntiBotClient.
//! Supports chunked CDN bypass for YouTube throttle circumvention.

use bytes::Bytes;
use extractor::VideoFormat;
use futures::{Stream, StreamExt};
use proxy::anti_bot::{AntiBotClient, AntiBotError};
use proxy::Platform;
use reqwest::StatusCode;
use std::pin::Pin;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

/// Chunk size for YouTube CDN throttle bypass: 9.5 MB.
///
/// YouTube CDN throttles full-file requests to ~2 Mbps.
/// Sub-range requests are served at full line speed.
const YOUTUBE_CHUNK_SIZE: u64 = 9_500_000;

/// Maximum retries per chunk on request failure or mid-stream interruption.
/// Uses exponential backoff: 1s, 2s, 4s.
const CHUNK_MAX_RETRIES: u32 = 3;
/// Timeout for establishing a chunk range stream request.
const CHUNK_REQUEST_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
/// Max idle time waiting for the next bytes from a chunk stream.
const CHUNK_READ_IDLE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(20);
/// Upper bound for waiting a prefetch task after current chunk completes.
/// If exceeded, abort prefetch and fall back to normal fetch path.
const PREFETCH_AWAIT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

/// Type alias for a pinned byte stream with AntiBotError.
pub type ByteStream = Pin<Box<dyn Stream<Item = Result<Bytes, AntiBotError>> + Send>>;

/// Per-stream refresh context used when upstream returns auth-like errors.
#[derive(Debug, Clone)]
pub struct StreamUrlRefreshContext {
    pub source_url: String,
    pub format_id: Option<String>,
    pub expected_audio_only: Option<bool>,
    pub expected_has_audio: Option<bool>,
    pub expected_ext: Option<String>,
    pub max_refresh_attempts: usize,
    pub preferred_proxy: Option<String>,
}

/// Refresh options for concurrent video/audio fetching.
#[derive(Debug, Clone, Default)]
pub struct FetchBothRefreshOptions {
    pub video: Option<StreamUrlRefreshContext>,
    pub audio: Option<StreamUrlRefreshContext>,
}

/// Fetches video and audio streams concurrently.
pub struct StreamFetcher;

impl StreamFetcher {
    /// Fetch both streams with optional refresh contexts for auth-like upstream failures.
    pub async fn fetch_both_with_refresh(
        video_url: &str,
        audio_url: &str,
        platform: Platform,
        refresh: FetchBothRefreshOptions,
    ) -> Result<(ByteStream, ByteStream), AntiBotError> {
        Self::fetch_both_with_refresh_and_proxy(video_url, audio_url, platform, refresh, None, None)
            .await
    }

    /// Fetch both streams with optional refresh contexts and pinned proxies.
    pub async fn fetch_both_with_refresh_and_proxy(
        video_url: &str,
        audio_url: &str,
        platform: Platform,
        refresh: FetchBothRefreshOptions,
        video_proxy: Option<String>,
        audio_proxy: Option<String>,
    ) -> Result<(ByteStream, ByteStream), AntiBotError> {
        debug!(
            video_url_len = video_url.len(),
            audio_url_len = audio_url.len(),
            "Initializing concurrent stream fetch"
        );

        let (video_result, audio_result) = tokio::join!(
            fetch_stream_chunked(platform, video_url, refresh.video, video_proxy),
            fetch_stream_chunked(platform, audio_url, refresh.audio, audio_proxy)
        );

        let video_stream = video_result?;
        let audio_stream = audio_result?;

        debug!("Both streams initialized successfully");
        Ok((video_stream, audio_stream))
    }
}

/// Extract `clen` (content length) from YouTube CDN URL query params.
fn extract_clen_from_url(url: &str) -> Option<u64> {
    reqwest::Url::parse(url)
        .ok()?
        .query_pairs()
        .find(|(k, _)| k == "clen")
        .and_then(|(_, v)| v.parse::<u64>().ok())
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

fn is_auth_like_antibot_error(error: &AntiBotError) -> bool {
    anti_bot_error_status(error)
        .map(is_upstream_auth_status)
        .unwrap_or(false)
        || is_auth_like_error_message(&error.to_string())
}

fn active_proxy_label(client: &AntiBotClient) -> String {
    sanitize_proxy_label(client.active_proxy())
}

fn sanitize_proxy_label(proxy: Option<&str>) -> String {
    let Some(raw) = proxy else {
        return String::new();
    };

    let Ok(mut parsed) = reqwest::Url::parse(raw) else {
        return mask_proxy_credential_segment(raw);
    };

    let has_credentials = !parsed.username().is_empty() || parsed.password().is_some();
    if has_credentials {
        let _ = parsed.set_username("***");
        let _ = parsed.set_password(Some("***"));
    }

    parsed.to_string()
}

fn mask_proxy_credential_segment(raw: &str) -> String {
    let Some((prefix, suffix)) = raw.rsplit_once('@') else {
        return raw.to_string();
    };

    let scheme = prefix
        .split_once("://")
        .map(|(value, _)| value)
        .unwrap_or("proxy");
    format!("{scheme}://***:***@{suffix}")
}

fn find_refreshed_format_url(
    formats: &[VideoFormat],
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
        .find(|format| {
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
}

async fn refresh_stream_url(
    context: &StreamUrlRefreshContext,
    fallback_url: &str,
) -> Option<String> {
    let refreshed = extractor::extract_with_options_and_proxy(
        &context.source_url,
        true,
        context.preferred_proxy.as_deref(),
    )
    .await
    .ok()?;
    find_refreshed_format_url(
        &refreshed.formats,
        context.format_id.as_deref(),
        fallback_url,
        context.expected_audio_only,
        context.expected_has_audio,
        context.expected_ext.as_deref(),
    )
}

async fn open_chunk_stream_with_timeout(
    client: &AntiBotClient,
    url: &str,
    range: &str,
) -> Result<ByteStream, AntiBotError> {
    let stream_result = tokio::time::timeout(
        CHUNK_REQUEST_TIMEOUT,
        client.fetch_stream(url, Some(range.to_string())),
    )
    .await;

    match stream_result {
        Ok(Ok(stream)) => Ok(Box::pin(stream)),
        Ok(Err(err)) => Err(err),
        Err(_) => Err(AntiBotError::MaxRetriesExceeded(format!(
            "Chunk request timed out for range {}",
            range
        ))),
    }
}

/// Fetch a stream with CDN chunked bypass if clen is present.
///
/// If `clen` is found in URL params, fetches via sequential 9.5MB range chunks.
/// Otherwise falls back to a single request.
async fn fetch_stream_chunked(
    platform: Platform,
    url: &str,
    refresh_context: Option<StreamUrlRefreshContext>,
    mut forced_proxy: Option<String>,
) -> Result<ByteStream, AntiBotError> {
    // Fallback: non-CDN URL (no `clen`) -> single request, with optional auth-refresh.
    if extract_clen_from_url(url).is_none() {
        let mut active_url = url.to_string();
        if forced_proxy.is_none() {
            forced_proxy = extractor::resolve_stream_proxy(&active_url).await;
        }
        let mut client = AntiBotClient::new_with_proxy(platform, forced_proxy.clone())?;
        info!(
            proxy = active_proxy_label(&client),
            "Opening single-stream fetch client"
        );
        let max_refresh_attempts = refresh_context
            .as_ref()
            .map(|context| context.max_refresh_attempts)
            .unwrap_or(0);
        let mut refresh_attempts = 0usize;

        loop {
            match client.fetch_stream(&active_url, None).await {
                Ok(stream) => return Ok(Box::pin(stream)),
                Err(error) => {
                    let can_refresh = refresh_attempts < max_refresh_attempts
                        && is_auth_like_antibot_error(&error)
                        && refresh_context.is_some();

                    if can_refresh {
                        if let Some(context) = refresh_context.as_ref() {
                            refresh_attempts += 1;
                            if let Some(new_url) = refresh_stream_url(context, &active_url).await {
                                info!(
                                    proxy = sanitize_proxy_label(forced_proxy.as_deref()),
                                    "Refreshed single stream URL after auth error (attempt {}/{})",
                                    refresh_attempts,
                                    max_refresh_attempts
                                );
                                active_url = new_url;
                                if forced_proxy.is_none() {
                                    forced_proxy =
                                        extractor::resolve_stream_proxy(&active_url).await;
                                }
                                client =
                                    AntiBotClient::new_with_proxy(platform, forced_proxy.clone())?;
                                info!(
                                    proxy = active_proxy_label(&client),
                                    "Rebuilt single-stream client after URL refresh"
                                );
                                continue;
                            }
                        }
                    }
                    return Err(error);
                }
            }
        }
    }

    let total_size = extract_clen_from_url(url).expect("checked above");
    info!(
        "Chunked fetch: {} bytes in ~{}MB chunks",
        total_size,
        YOUTUBE_CHUNK_SIZE / 1_000_000
    );

    let (tx, rx) = tokio::sync::mpsc::channel::<Result<Bytes, AntiBotError>>(8);
    let initial_url = url.to_string();

    tokio::spawn(async move {
        type PrefetchHandle = tokio::task::JoinHandle<Result<ByteStream, AntiBotError>>;

        if forced_proxy.is_none() {
            forced_proxy = extractor::resolve_stream_proxy(&initial_url).await;
        }
        let mut task_client = match AntiBotClient::new_with_proxy(platform, forced_proxy.clone()) {
            Ok(client) => Arc::new(client),
            Err(error) => {
                let _ = tx.send(Err(error)).await;
                return;
            }
        };
        info!(
            proxy = active_proxy_label(task_client.as_ref()),
            total_size, "Starting chunked stream fetch worker"
        );

        let max_refresh_attempts = refresh_context
            .as_ref()
            .map(|context| context.max_refresh_attempts)
            .unwrap_or(0);
        let mut refresh_attempts = 0usize;
        let mut active_url = initial_url;
        let mut active_total_size = total_size;
        let mut offset = 0u64;
        let mut prefetch: Option<PrefetchHandle> = None;
        let mut next_stream: Option<ByteStream> = None;

        while offset < active_total_size {
            let chunk_end = (offset + YOUTUBE_CHUNK_SIZE - 1).min(active_total_size - 1);
            let next_offset = chunk_end + 1;
            let has_next_chunk = next_offset < active_total_size;

            if has_next_chunk && prefetch.is_none() {
                let next_end = (next_offset + YOUTUBE_CHUNK_SIZE - 1).min(active_total_size - 1);
                let next_range = format!("bytes={}-{}", next_offset, next_end);
                let prefetch_url = active_url.clone();
                let prefetch_client = Arc::clone(&task_client);
                prefetch = Some(tokio::spawn(async move {
                    let stream = open_chunk_stream_with_timeout(
                        prefetch_client.as_ref(),
                        &prefetch_url,
                        &next_range,
                    )
                    .await?;
                    Ok::<ByteStream, AntiBotError>(stream)
                }));
            }

            let mut retry_count = 0u32;
            let mut fetch_start = offset;

            'retry: loop {
                let range = format!("bytes={}-{}", fetch_start, chunk_end);
                debug!(
                    proxy = active_proxy_label(task_client.as_ref()),
                    "Chunk fetch: bytes={}-{} / {} (attempt {})",
                    fetch_start,
                    chunk_end,
                    active_total_size,
                    retry_count + 1
                );

                let mut chunk_stream: ByteStream = if retry_count == 0 {
                    if let Some(stream) = next_stream.take() {
                        stream
                    } else {
                        match open_chunk_stream_with_timeout(
                            task_client.as_ref(),
                            &active_url,
                            &range,
                        )
                        .await
                        {
                            Ok(stream) => stream,
                            Err(error) => {
                                let can_refresh = refresh_attempts < max_refresh_attempts
                                    && is_auth_like_antibot_error(&error)
                                    && refresh_context.is_some();
                                if can_refresh {
                                    if let Some(context) = refresh_context.as_ref() {
                                        refresh_attempts += 1;
                                        if let Some(new_url) =
                                            refresh_stream_url(context, &active_url).await
                                        {
                                            info!(
                                                proxy = sanitize_proxy_label(forced_proxy.as_deref()),
                                                "Refreshed chunk URL after auth error (attempt {}/{})",
                                                refresh_attempts, max_refresh_attempts
                                            );
                                            active_url = new_url;
                                            if forced_proxy.is_none() {
                                                forced_proxy =
                                                    extractor::resolve_stream_proxy(&active_url)
                                                        .await;
                                            }
                                            task_client = match AntiBotClient::new_with_proxy(
                                                platform,
                                                forced_proxy.clone(),
                                            ) {
                                                Ok(client) => Arc::new(client),
                                                Err(error) => {
                                                    let _ = tx.send(Err(error)).await;
                                                    return;
                                                }
                                            };
                                            info!(
                                                proxy = active_proxy_label(task_client.as_ref()),
                                                "Rebuilt chunk fetch client after URL refresh"
                                            );
                                            if let Some(clen) = extract_clen_from_url(&active_url) {
                                                active_total_size = clen;
                                            }
                                            if let Some(handle) = prefetch.take() {
                                                handle.abort();
                                            }
                                            next_stream = None;
                                            continue 'retry;
                                        }
                                    }
                                }

                                retry_count += 1;
                                if retry_count > CHUNK_MAX_RETRIES {
                                    error!(
                                        proxy = active_proxy_label(task_client.as_ref()),
                                        "Chunk fetch failed after {} retries bytes={}-{}: {}",
                                        CHUNK_MAX_RETRIES,
                                        fetch_start,
                                        chunk_end,
                                        error
                                    );
                                    if let Some(handle) = prefetch.take() {
                                        handle.abort();
                                    }
                                    let _ = tx.send(Err(error)).await;
                                    return;
                                }
                                let delay = std::time::Duration::from_secs(1 << (retry_count - 1));
                                warn!(
                                    proxy = active_proxy_label(task_client.as_ref()),
                                    "Chunk request failed (retry {}/{}): {} — retrying in {}s",
                                    retry_count,
                                    CHUNK_MAX_RETRIES,
                                    error,
                                    delay.as_secs()
                                );
                                tokio::time::sleep(delay).await;
                                continue 'retry;
                            }
                        }
                    }
                } else {
                    match open_chunk_stream_with_timeout(task_client.as_ref(), &active_url, &range)
                        .await
                    {
                        Ok(stream) => stream,
                        Err(error) => {
                            let can_refresh = refresh_attempts < max_refresh_attempts
                                && is_auth_like_antibot_error(&error)
                                && refresh_context.is_some();
                            if can_refresh {
                                if let Some(context) = refresh_context.as_ref() {
                                    refresh_attempts += 1;
                                    if let Some(new_url) =
                                        refresh_stream_url(context, &active_url).await
                                    {
                                        info!(
                                            proxy = sanitize_proxy_label(forced_proxy.as_deref()),
                                            "Refreshed chunk URL after auth error (attempt {}/{})",
                                            refresh_attempts,
                                            max_refresh_attempts
                                        );
                                        active_url = new_url;
                                        if forced_proxy.is_none() {
                                            forced_proxy =
                                                extractor::resolve_stream_proxy(&active_url).await;
                                        }
                                        task_client = match AntiBotClient::new_with_proxy(
                                            platform,
                                            forced_proxy.clone(),
                                        ) {
                                            Ok(client) => Arc::new(client),
                                            Err(error) => {
                                                let _ = tx.send(Err(error)).await;
                                                return;
                                            }
                                        };
                                        info!(
                                            proxy = active_proxy_label(task_client.as_ref()),
                                            "Rebuilt chunk fetch client after retry URL refresh"
                                        );
                                        if let Some(clen) = extract_clen_from_url(&active_url) {
                                            active_total_size = clen;
                                        }
                                        if let Some(handle) = prefetch.take() {
                                            handle.abort();
                                        }
                                        next_stream = None;
                                        continue 'retry;
                                    }
                                }
                            }

                            retry_count += 1;
                            if retry_count > CHUNK_MAX_RETRIES {
                                error!(
                                    proxy = active_proxy_label(task_client.as_ref()),
                                    "Chunk fetch failed after {} retries bytes={}-{}: {}",
                                    CHUNK_MAX_RETRIES,
                                    fetch_start,
                                    chunk_end,
                                    error
                                );
                                if let Some(handle) = prefetch.take() {
                                    handle.abort();
                                }
                                let _ = tx.send(Err(error)).await;
                                return;
                            }
                            let delay = std::time::Duration::from_secs(1 << (retry_count - 1));
                            warn!(
                                proxy = active_proxy_label(task_client.as_ref()),
                                "Chunk request failed (retry {}/{}): {} — retrying in {}s",
                                retry_count,
                                CHUNK_MAX_RETRIES,
                                error,
                                delay.as_secs()
                            );
                            tokio::time::sleep(delay).await;
                            continue 'retry;
                        }
                    }
                };

                let mut stream_ok = true;
                loop {
                    let next_item =
                        tokio::time::timeout(CHUNK_READ_IDLE_TIMEOUT, chunk_stream.next()).await;
                    match next_item {
                        Ok(Some(Ok(bytes))) => {
                            fetch_start += bytes.len() as u64;
                            if tx.send(Ok(bytes)).await.is_err() {
                                if let Some(handle) = prefetch.take() {
                                    handle.abort();
                                }
                                return;
                            }
                        }
                        Ok(Some(Err(error))) => {
                            let can_refresh = refresh_attempts < max_refresh_attempts
                                && is_auth_like_antibot_error(&error)
                                && refresh_context.is_some();
                            if can_refresh {
                                if let Some(context) = refresh_context.as_ref() {
                                    refresh_attempts += 1;
                                    if let Some(new_url) =
                                        refresh_stream_url(context, &active_url).await
                                    {
                                        info!(
                                            proxy = sanitize_proxy_label(forced_proxy.as_deref()),
                                            "Refreshed stream URL after mid-chunk auth error (attempt {}/{})",
                                            refresh_attempts, max_refresh_attempts
                                        );
                                        active_url = new_url;
                                        if forced_proxy.is_none() {
                                            forced_proxy =
                                                extractor::resolve_stream_proxy(&active_url).await;
                                        }
                                        task_client = match AntiBotClient::new_with_proxy(
                                            platform,
                                            forced_proxy.clone(),
                                        ) {
                                            Ok(client) => Arc::new(client),
                                            Err(error) => {
                                                let _ = tx.send(Err(error)).await;
                                                return;
                                            }
                                        };
                                        info!(
                                            proxy = active_proxy_label(task_client.as_ref()),
                                            "Rebuilt chunk fetch client after mid-chunk URL refresh"
                                        );
                                        if let Some(clen) = extract_clen_from_url(&active_url) {
                                            active_total_size = clen;
                                        }
                                        if let Some(handle) = prefetch.take() {
                                            handle.abort();
                                        }
                                        next_stream = None;
                                        continue 'retry;
                                    }
                                }
                            }

                            retry_count += 1;
                            if retry_count > CHUNK_MAX_RETRIES {
                                error!(
                                    proxy = active_proxy_label(task_client.as_ref()),
                                    "Stream interrupted after {} retries at byte {}: {}",
                                    CHUNK_MAX_RETRIES,
                                    fetch_start,
                                    error
                                );
                                if let Some(handle) = prefetch.take() {
                                    handle.abort();
                                }
                                let _ = tx.send(Err(error)).await;
                                return;
                            }
                            let delay = std::time::Duration::from_secs(1 << (retry_count - 1));
                            warn!(
                                proxy = active_proxy_label(task_client.as_ref()),
                                "Stream interrupted at byte {} (retry {}/{}): {} — retrying in {}s",
                                fetch_start,
                                retry_count,
                                CHUNK_MAX_RETRIES,
                                error,
                                delay.as_secs()
                            );
                            tokio::time::sleep(delay).await;
                            stream_ok = false;
                            break;
                        }
                        Ok(None) => break,
                        Err(_) => {
                            retry_count += 1;
                            if retry_count > CHUNK_MAX_RETRIES {
                                let error = AntiBotError::MaxRetriesExceeded(format!(
                                    "Idle timeout while reading range bytes={}-{}",
                                    fetch_start, chunk_end
                                ));
                                error!(
                                    proxy = active_proxy_label(task_client.as_ref()),
                                    "Chunk idle-read timeout after {} retries bytes={}-{}",
                                    CHUNK_MAX_RETRIES,
                                    fetch_start,
                                    chunk_end
                                );
                                if let Some(handle) = prefetch.take() {
                                    handle.abort();
                                }
                                let _ = tx.send(Err(error)).await;
                                return;
                            }
                            let delay = std::time::Duration::from_secs(1 << (retry_count - 1));
                            warn!(
                                proxy = active_proxy_label(task_client.as_ref()),
                                "Chunk idle-read timeout (retry {}/{}), bytes={}-{}, retrying in {}s",
                                retry_count,
                                CHUNK_MAX_RETRIES,
                                fetch_start,
                                chunk_end,
                                delay.as_secs()
                            );
                            tokio::time::sleep(delay).await;
                            stream_ok = false;
                            break;
                        }
                    }
                }

                let expected_next = chunk_end.saturating_add(1);
                if stream_ok && fetch_start < expected_next {
                    retry_count += 1;
                    if retry_count > CHUNK_MAX_RETRIES {
                        let error = AntiBotError::MaxRetriesExceeded(format!(
                            "Premature EOF for range bytes={}-{} (received until byte {})",
                            offset,
                            chunk_end,
                            fetch_start.saturating_sub(1)
                        ));
                        error!(
                            "Premature EOF after {} retries for bytes={}-{} (fetch_start={})",
                            CHUNK_MAX_RETRIES, offset, chunk_end, fetch_start
                        );
                        if let Some(handle) = prefetch.take() {
                            handle.abort();
                        }
                        let _ = tx.send(Err(error)).await;
                        return;
                    }

                    let delay = std::time::Duration::from_secs(1 << (retry_count - 1));
                    warn!(
                        "Premature EOF at byte {} for bytes={}-{} (retry {}/{}), retrying in {}s",
                        fetch_start,
                        offset,
                        chunk_end,
                        retry_count,
                        CHUNK_MAX_RETRIES,
                        delay.as_secs()
                    );
                    tokio::time::sleep(delay).await;
                    continue 'retry;
                }

                if stream_ok {
                    break 'retry;
                }
            }

            offset = chunk_end + 1;

            if let Some(handle) = prefetch.take() {
                let mut handle = handle;
                let timeout = tokio::time::sleep(PREFETCH_AWAIT_TIMEOUT);
                tokio::pin!(timeout);

                let prefetch_result = tokio::select! {
                    _ = tx.closed() => {
                        handle.abort();
                        return;
                    }
                    _ = &mut timeout => {
                        handle.abort();
                        warn!(
                            "Prefetch timed out after {:?} at offset {} (fallback to normal fetch)",
                            PREFETCH_AWAIT_TIMEOUT, offset
                        );
                        None
                    }
                    result = &mut handle => Some(result),
                };

                if let Some(result) = prefetch_result {
                    match result {
                        Ok(Ok(stream)) => {
                            next_stream = Some(stream);
                        }
                        Ok(Err(error)) => {
                            warn!(
                                "Prefetch failed for next chunk at offset {}: {} (fallback to normal fetch)",
                                offset, error
                            );
                        }
                        Err(join_err) => {
                            warn!(
                                "Prefetch task join error at offset {}: {} (fallback to normal fetch)",
                                offset, join_err
                            );
                        }
                    }
                }
            }
        }

        if let Some(handle) = prefetch.take() {
            handle.abort();
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    Ok(Box::pin(stream))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_clen_from_url() {
        assert_eq!(
            extract_clen_from_url(
                "https://rr1.googlevideo.com/videoplayback?clen=20971520&mime=video%2Fmp4"
            ),
            Some(20_971_520)
        );
        assert_eq!(
            extract_clen_from_url("https://example.com/video?foo=bar"),
            None
        );
        assert_eq!(extract_clen_from_url("not-a-url"), None);
    }

    #[test]
    fn test_sanitize_proxy_label_masks_credentials() {
        assert_eq!(
            sanitize_proxy_label(Some("socks5h://user:pass@127.0.0.1:1080")),
            "socks5h://***:***@127.0.0.1:1080"
        );
    }

    #[test]
    fn test_sanitize_proxy_label_leaves_host_only_proxy() {
        assert_eq!(
            sanitize_proxy_label(Some("socks5h://127.0.0.1:1080")),
            "socks5h://127.0.0.1:1080"
        );
    }
}
