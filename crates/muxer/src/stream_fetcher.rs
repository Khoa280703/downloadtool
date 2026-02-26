//! Concurrent stream fetching for audio and video
//!
//! Fetches both streams simultaneously using the AntiBotClient.
//! Supports chunked CDN bypass for YouTube throttle circumvention.

use bytes::Bytes;
use futures::{Stream, StreamExt};
use proxy::anti_bot::{AntiBotClient, AntiBotError};
use proxy::cookie_store::Platform;
use std::pin::Pin;
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
        // Create new client inside spawned task
        let task_client = match AntiBotClient::new(platform) {
            Ok(c) => c,
            Err(e) => {
                let _ = tx.send(Err(e)).await;
                return;
            }
        };

        let mut offset = 0u64;
        while offset < total_size {
            let chunk_end = (offset + YOUTUBE_CHUNK_SIZE - 1).min(total_size - 1);

            // Retry loop per chunk: tracks fetch_start to resume mid-stream
            let mut retry_count = 0u32;
            let mut fetch_start = offset;

            'retry: loop {
                let range = format!("bytes={}-{}", fetch_start, chunk_end);
                debug!("Chunk fetch: bytes={}-{} / {} (attempt {})", fetch_start, chunk_end, total_size, retry_count + 1);

                let stream_result = task_client.fetch_stream(&url_owned, Some(range)).await;
                let mut chunk_stream = match stream_result {
                    Ok(s) => s,
                    Err(e) => {
                        retry_count += 1;
                        if retry_count > CHUNK_MAX_RETRIES {
                            error!("Chunk fetch failed after {} retries bytes={}-{}: {}", CHUNK_MAX_RETRIES, fetch_start, chunk_end, e);
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                        let delay = std::time::Duration::from_secs(1 << (retry_count - 1));
                        warn!("Chunk request failed (retry {}/{}): {} — retrying in {}s", retry_count, CHUNK_MAX_RETRIES, e, delay.as_secs());
                        tokio::time::sleep(delay).await;
                        continue 'retry;
                    }
                };

                // Stream bytes from this range, tracking position for mid-stream retry
                let mut stream_ok = true;
                while let Some(item) = chunk_stream.next().await {
                    match item {
                        Ok(bytes) => {
                            fetch_start += bytes.len() as u64;
                            if tx.send(Ok(bytes)).await.is_err() {
                                return; // Receiver dropped — abort
                            }
                        }
                        Err(e) => {
                            retry_count += 1;
                            if retry_count > CHUNK_MAX_RETRIES {
                                error!("Stream interrupted after {} retries at byte {}: {}", CHUNK_MAX_RETRIES, fetch_start, e);
                                let _ = tx.send(Err(e)).await;
                                return;
                            }
                            let delay = std::time::Duration::from_secs(1 << (retry_count - 1));
                            warn!("Stream interrupted at byte {} (retry {}/{}): {} — retrying in {}s", fetch_start, retry_count, CHUNK_MAX_RETRIES, e, delay.as_secs());
                            tokio::time::sleep(delay).await;
                            stream_ok = false;
                            break;
                        }
                    }
                }

                if stream_ok {
                    break 'retry; // chunk complete, advance to next chunk
                }
                // else: mid-stream failure, retry from fetch_start
            }

            offset = chunk_end + 1;
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
