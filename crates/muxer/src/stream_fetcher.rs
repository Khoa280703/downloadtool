//! Concurrent stream fetching for audio and video
//!
//! Fetches both streams simultaneously using the AntiBotClient.
//! Supports chunked CDN bypass for YouTube throttle circumvention.

use bytes::Bytes;
use futures::{Stream, StreamExt};
use proxy::anti_bot::{AntiBotClient, AntiBotError};
use proxy::cookie_store::Platform;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
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

/// Fetches video and audio streams concurrently.
pub struct StreamFetcher;

impl StreamFetcher {
    /// Fetch both video and audio streams concurrently with CDN chunked bypass.
    ///
    /// If the URL contains a `clen` param (YouTube CDN), uses sequential range
    /// chunk requests to bypass the per-file throttle. Otherwise falls back to
    /// a single request.
    pub async fn fetch_both(
        video_url: &str,
        audio_url: &str,
        platform: Platform,
    ) -> Result<(ByteStream, ByteStream), AntiBotError> {
        info!("Fetching video stream from: {}", video_url);
        info!("Fetching audio stream from: {}", audio_url);

        let (video_result, audio_result) = tokio::join!(
            fetch_stream_chunked(platform, video_url),
            fetch_stream_chunked(platform, audio_url)
        );

        let video_stream = video_result?;
        let audio_stream = audio_result?;

        debug!("Both streams initialized successfully");
        Ok((video_stream, audio_stream))
    }

    /// Fetch a video stream with range support.
    pub async fn fetch_video(
        url: &str,
        range: Option<String>,
        platform: Platform,
    ) -> Result<ByteStream, AntiBotError> {
        let client = AntiBotClient::new(platform)?;
        let stream = client.fetch_stream(url, range).await?;
        Ok(Box::pin(stream))
    }

    /// Fetch an audio stream with range support.
    pub async fn fetch_audio(
        url: &str,
        range: Option<String>,
        platform: Platform,
    ) -> Result<ByteStream, AntiBotError> {
        let client = AntiBotClient::new(platform)?;
        let stream = client.fetch_stream(url, range).await?;
        Ok(Box::pin(stream))
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

/// Fetch a stream with CDN chunked bypass if clen is present.
///
/// If `clen` is found in URL params, fetches via sequential 9.5MB range chunks.
/// Otherwise falls back to a single request.
async fn fetch_stream_chunked(
    platform: Platform,
    url: &str,
) -> Result<ByteStream, AntiBotError> {
    let total_size = match extract_clen_from_url(url) {
        Some(size) => size,
        None => {
            // Fallback: single request
            let client = AntiBotClient::new(platform)?;
            let stream = client.fetch_stream(url, None).await?;
            return Ok(Box::pin(stream));
        }
    };

    info!(
        "Chunked fetch: {} bytes in ~{}MB chunks",
        total_size,
        YOUTUBE_CHUNK_SIZE / 1_000_000
    );

    let (tx, rx) = tokio::sync::mpsc::channel::<Result<Bytes, AntiBotError>>(8);
    let url_owned = url.to_string();

    tokio::spawn(async move {
        type PrefetchHandle = tokio::task::JoinHandle<Result<ByteStream, AntiBotError>>;

        // Create new client inside spawned task
        let task_client = match AntiBotClient::new(platform) {
            Ok(c) => c,
            Err(e) => {
                let _ = tx.send(Err(e)).await;
                return;
            }
        };
        let task_client = Arc::new(task_client);

        let mut offset = 0u64;
        let mut prefetch: Option<PrefetchHandle> = None;
        let mut next_stream: Option<ByteStream> = None;

        while offset < total_size {
            let chunk_end = (offset + YOUTUBE_CHUNK_SIZE - 1).min(total_size - 1);
            let next_offset = chunk_end + 1;
            let has_next_chunk = next_offset < total_size;

            // Sliding window prefetch: while streaming chunk N, keep chunk N+1 request in flight.
            // This overlaps connection/setup RTT with current chunk transfer.
            if has_next_chunk && prefetch.is_none() {
                let next_end = (next_offset + YOUTUBE_CHUNK_SIZE - 1).min(total_size - 1);
                let next_range = format!("bytes={}-{}", next_offset, next_end);
                let pf_url = url_owned.clone();
                let pf_client = Arc::clone(&task_client);
                prefetch = Some(tokio::spawn(async move {
                    let pf_stream = tokio::time::timeout(
                        CHUNK_REQUEST_TIMEOUT,
                        pf_client.fetch_stream(&pf_url, Some(next_range)),
                    )
                    .await
                    .map_err(|_| {
                        AntiBotError::MaxRetriesExceeded(format!(
                            "Prefetch request timed out for range bytes={}-{}",
                            next_offset, next_end
                        ))
                    })??;
                    Ok::<ByteStream, AntiBotError>(Box::pin(pf_stream))
                }));
            }

            // Retry loop per chunk: tracks fetch_start to resume mid-stream
            let mut retry_count = 0u32;
            let mut fetch_start = offset;

            'retry: loop {
                let range = format!("bytes={}-{}", fetch_start, chunk_end);
                debug!("Chunk fetch: bytes={}-{} / {} (attempt {})", fetch_start, chunk_end, total_size, retry_count + 1);

                let mut chunk_stream: ByteStream = if retry_count == 0 {
                    if let Some(stream) = next_stream.take() {
                        stream
                    } else {
                        let stream_result = tokio::time::timeout(
                            CHUNK_REQUEST_TIMEOUT,
                            task_client.fetch_stream(&url_owned, Some(range)),
                        )
                        .await;
                        match stream_result {
                            Ok(Ok(s)) => Box::pin(s),
                            Ok(Err(e)) => {
                                retry_count += 1;
                                if retry_count > CHUNK_MAX_RETRIES {
                                    error!("Chunk fetch failed after {} retries bytes={}-{}: {}", CHUNK_MAX_RETRIES, fetch_start, chunk_end, e);
                                    if let Some(handle) = prefetch.take() {
                                        handle.abort();
                                    }
                                    let _ = tx.send(Err(e)).await;
                                    return;
                                }
                                let delay = std::time::Duration::from_secs(1 << (retry_count - 1));
                                warn!("Chunk request failed (retry {}/{}): {} — retrying in {}s", retry_count, CHUNK_MAX_RETRIES, e, delay.as_secs());
                                tokio::time::sleep(delay).await;
                                continue 'retry;
                            }
                            Err(_) => {
                                retry_count += 1;
                                if retry_count > CHUNK_MAX_RETRIES {
                                    let err = AntiBotError::MaxRetriesExceeded(format!(
                                        "Chunk request timed out for range bytes={}-{}",
                                        fetch_start, chunk_end
                                    ));
                                    error!(
                                        "Chunk request timeout after {} retries bytes={}-{}",
                                        CHUNK_MAX_RETRIES, fetch_start, chunk_end
                                    );
                                    if let Some(handle) = prefetch.take() {
                                        handle.abort();
                                    }
                                    let _ = tx.send(Err(err)).await;
                                    return;
                                }
                                let delay = std::time::Duration::from_secs(1 << (retry_count - 1));
                                warn!(
                                    "Chunk request timeout (retry {}/{}), bytes={}-{}, retrying in {}s",
                                    retry_count,
                                    CHUNK_MAX_RETRIES,
                                    fetch_start,
                                    chunk_end,
                                    delay.as_secs()
                                );
                                tokio::time::sleep(delay).await;
                                continue 'retry;
                            }
                        }
                    }
                } else {
                    let stream_result = tokio::time::timeout(
                        CHUNK_REQUEST_TIMEOUT,
                        task_client.fetch_stream(&url_owned, Some(range)),
                    )
                    .await;
                    match stream_result {
                        Ok(Ok(s)) => Box::pin(s),
                        Ok(Err(e)) => {
                            retry_count += 1;
                            if retry_count > CHUNK_MAX_RETRIES {
                                error!("Chunk fetch failed after {} retries bytes={}-{}: {}", CHUNK_MAX_RETRIES, fetch_start, chunk_end, e);
                                if let Some(handle) = prefetch.take() {
                                    handle.abort();
                                }
                                let _ = tx.send(Err(e)).await;
                                return;
                            }
                            let delay = std::time::Duration::from_secs(1 << (retry_count - 1));
                            warn!("Chunk request failed (retry {}/{}): {} — retrying in {}s", retry_count, CHUNK_MAX_RETRIES, e, delay.as_secs());
                            tokio::time::sleep(delay).await;
                            continue 'retry;
                        }
                        Err(_) => {
                            retry_count += 1;
                            if retry_count > CHUNK_MAX_RETRIES {
                                let err = AntiBotError::MaxRetriesExceeded(format!(
                                    "Chunk request timed out for range bytes={}-{}",
                                    fetch_start, chunk_end
                                ));
                                error!(
                                    "Chunk request timeout after {} retries bytes={}-{}",
                                    CHUNK_MAX_RETRIES, fetch_start, chunk_end
                                );
                                if let Some(handle) = prefetch.take() {
                                    handle.abort();
                                }
                                let _ = tx.send(Err(err)).await;
                                return;
                            }
                            let delay = std::time::Duration::from_secs(1 << (retry_count - 1));
                            warn!(
                                "Chunk request timeout (retry {}/{}), bytes={}-{}, retrying in {}s",
                                retry_count,
                                CHUNK_MAX_RETRIES,
                                fetch_start,
                                chunk_end,
                                delay.as_secs()
                            );
                            tokio::time::sleep(delay).await;
                            continue 'retry;
                        }
                    }
                };

                // Stream bytes from this range, tracking position for mid-stream retry
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
                                return; // Receiver dropped — abort
                            }
                        }
                        Ok(Some(Err(e))) => {
                            retry_count += 1;
                            if retry_count > CHUNK_MAX_RETRIES {
                                error!("Stream interrupted after {} retries at byte {}: {}", CHUNK_MAX_RETRIES, fetch_start, e);
                                if let Some(handle) = prefetch.take() {
                                    handle.abort();
                                }
                                let _ = tx.send(Err(e)).await;
                                return;
                            }
                            let delay = std::time::Duration::from_secs(1 << (retry_count - 1));
                            warn!("Stream interrupted at byte {} (retry {}/{}): {} — retrying in {}s", fetch_start, retry_count, CHUNK_MAX_RETRIES, e, delay.as_secs());
                            tokio::time::sleep(delay).await;
                            stream_ok = false;
                            break;
                        }
                        Ok(None) => break,
                        Err(_) => {
                            retry_count += 1;
                            if retry_count > CHUNK_MAX_RETRIES {
                                let err = AntiBotError::MaxRetriesExceeded(format!(
                                    "Idle timeout while reading range bytes={}-{}",
                                    fetch_start, chunk_end
                                ));
                                error!(
                                    "Chunk idle-read timeout after {} retries bytes={}-{}",
                                    CHUNK_MAX_RETRIES, fetch_start, chunk_end
                                );
                                if let Some(handle) = prefetch.take() {
                                    handle.abort();
                                }
                                let _ = tx.send(Err(err)).await;
                                return;
                            }
                            let delay = std::time::Duration::from_secs(1 << (retry_count - 1));
                            warn!(
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

                // Some servers can terminate the body early without surfacing an explicit error.
                // If we did not reach the expected byte boundary for this chunk, treat as retryable.
                let expected_next = chunk_end.saturating_add(1);
                if stream_ok && fetch_start < expected_next {
                    retry_count += 1;
                    if retry_count > CHUNK_MAX_RETRIES {
                        let err = AntiBotError::MaxRetriesExceeded(format!(
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
                        let _ = tx.send(Err(err)).await;
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
                    break 'retry; // chunk complete, advance to next chunk
                }
                // else: mid-stream failure, retry from fetch_start
            }

            offset = chunk_end + 1;

            // If a prefetch for the next chunk exists, consume it now.
            // On failure, fall back to normal fetch path in the next retry loop.
            if let Some(handle) = prefetch.take() {
                let mut handle = handle;
                let timeout = tokio::time::sleep(PREFETCH_AWAIT_TIMEOUT);
                tokio::pin!(timeout);

                let prefetch_result = tokio::select! {
                    _ = tx.closed() => {
                        // Receiver dropped while waiting prefetch completion.
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
                        Ok(Err(e)) => {
                            warn!(
                                "Prefetch failed for next chunk at offset {}: {} (fallback to normal fetch)",
                                offset, e
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

        // Defensive cleanup in case loop exits with an in-flight prefetch task.
        if let Some(handle) = prefetch.take() {
            handle.abort();
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    Ok(Box::pin(stream))
}

/// A combined stream that yields from both video and audio sources.
///
/// This is used internally by the muxer to interleave chunks.
pub struct CombinedStream {
    video: ByteStream,
    audio: ByteStream,
    video_done: bool,
    audio_done: bool,
}

impl CombinedStream {
    /// Create a new combined stream from video and audio sources.
    pub fn new(video: ByteStream, audio: ByteStream) -> Self {
        Self {
            video,
            audio,
            video_done: false,
            audio_done: false,
        }
    }

    /// Check if both streams are complete.
    pub fn is_complete(&self) -> bool {
        self.video_done && self.audio_done
    }

    /// Get the next video chunk if available.
    pub fn poll_video(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, AntiBotError>>> {
        if self.video_done {
            return Poll::Ready(None);
        }
        self.video.as_mut().poll_next(cx)
    }

    /// Get the next audio chunk if available.
    pub fn poll_audio(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, AntiBotError>>> {
        if self.audio_done {
            return Poll::Ready(None);
        }
        self.audio.as_mut().poll_next(cx)
    }
}

/// Error type for combined stream operations.
#[derive(Debug)]
pub enum CombinedStreamError {
    /// Video stream error.
    VideoError(AntiBotError),
    /// Audio stream error.
    AudioError(AntiBotError),
    /// Both streams failed.
    BothFailed {
        /// Video error.
        video: AntiBotError,
        /// Audio error.
        audio: AntiBotError,
    },
}

impl std::fmt::Display for CombinedStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CombinedStreamError::VideoError(e) => write!(f, "Video stream error: {}", e),
            CombinedStreamError::AudioError(e) => write!(f, "Audio stream error: {}", e),
            CombinedStreamError::BothFailed { video, audio } => {
                write!(f, "Both streams failed - video: {}, audio: {}", video, audio)
            }
        }
    }
}

impl std::error::Error for CombinedStreamError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combined_stream_creation() {
        let video: ByteStream = Box::pin(futures::stream::empty());
        let audio: ByteStream = Box::pin(futures::stream::empty());
        let combined = CombinedStream::new(video, audio);
        assert!(!combined.is_complete());
    }

    #[test]
    fn test_combined_stream_error_display() {
        let err = CombinedStreamError::VideoError(AntiBotError::InvalidUrl("test".to_string()));
        assert!(err.to_string().contains("Video stream error"));

        let err = CombinedStreamError::AudioError(AntiBotError::InvalidUrl("test".to_string()));
        assert!(err.to_string().contains("Audio stream error"));
    }

    #[test]
    fn test_extract_clen_from_url() {
        assert_eq!(
            extract_clen_from_url(
                "https://rr1.googlevideo.com/videoplayback?clen=20971520&mime=video%2Fmp4"
            ),
            Some(20_971_520)
        );
        assert_eq!(extract_clen_from_url("https://example.com/video?foo=bar"), None);
        assert_eq!(extract_clen_from_url("not-a-url"), None);
    }
}
