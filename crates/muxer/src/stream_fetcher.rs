//! Concurrent stream fetching for audio and video
//!
//! Fetches both streams simultaneously using the AntiBotClient.

use bytes::Bytes;
use futures::Stream;
use proxy::anti_bot::{AntiBotClient, AntiBotError};
use proxy::cookie_store::Platform;
use std::pin::Pin;
use std::task::{Context, Poll};
use tracing::{debug, error, info, warn};

/// Type alias for a pinned byte stream with AntiBotError.
pub type ByteStream = Pin<Box<dyn Stream<Item = Result<Bytes, AntiBotError>> + Send>>;

/// Fetches video and audio streams concurrently.
pub struct StreamFetcher;

impl StreamFetcher {
    /// Fetch both video and audio streams concurrently.
    ///
    /// # Arguments
    /// * `video_url` - The video stream URL.
    /// * `audio_url` - The audio stream URL.
    /// * `platform` - The platform for anti-bot configuration.
    ///
    /// # Returns
    /// A tuple of (video_stream, audio_stream) on success.
    ///
    /// # Errors
    /// Returns an error if either stream fails to initialize.
    pub async fn fetch_both(
        video_url: &str,
        audio_url: &str,
        platform: Platform,
    ) -> Result<(ByteStream, ByteStream), AntiBotError> {
        info!("Fetching video stream from: {}", video_url);
        info!("Fetching audio stream from: {}", audio_url);

        let client = AntiBotClient::new(platform)?;

        // Fetch both streams concurrently
        let (video_result, audio_result) = tokio::join!(
            Self::fetch_stream_with_client(&client, video_url),
            Self::fetch_stream_with_client(&client, audio_url)
        );

        // Check results and propagate errors
        let video_stream = video_result?;
        let audio_stream = audio_result?;

        debug!("Both streams initialized successfully");
        Ok((video_stream, audio_stream))
    }

    /// Fetch a single stream using the provided client.
    async fn fetch_stream_with_client(
        client: &AntiBotClient,
        url: &str,
    ) -> Result<ByteStream, AntiBotError> {
        let stream = client.fetch_stream(url, None).await?;
        Ok(Box::pin(stream))
    }

    /// Fetch a video stream with range support.
    ///
    /// # Arguments
    /// * `url` - The video stream URL.
    /// * `range` - Optional byte range for partial content.
    /// * `platform` - The platform for anti-bot configuration.
    ///
    /// # Returns
    /// A stream of bytes on success.
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
    ///
    /// # Arguments
    /// * `url` - The audio stream URL.
    /// * `range` - Optional byte range for partial content.
    /// * `platform` - The platform for anti-bot configuration.
    ///
    /// # Returns
    /// A stream of bytes on success.
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

/// A combined stream that yields from both video and audio sources.
///
/// This is used internally by the muxer to interleave chunks.
pub struct CombinedStream {
    video: ByteStream,
    audio: ByteStream,
    video_buffer: Option<Bytes>,
    audio_buffer: Option<Bytes>,
    video_done: bool,
    audio_done: bool,
}

impl CombinedStream {
    /// Create a new combined stream from video and audio sources.
    pub fn new(video: ByteStream, audio: ByteStream) -> Self {
        Self {
            video,
            audio,
            video_buffer: None,
            audio_buffer: None,
            video_done: false,
            audio_done: false,
        }
    }

    /// Check if both streams are complete.
    pub fn is_complete(&self) -> bool {
        self.video_done && self.audio_done
    }

    /// Get the next video chunk if available.
    pub fn poll_video(&mut self, cx: &mut Context<'_>) -> Poll<Option<Result<Bytes, AntiBotError>>> {
        if self.video_done {
            return Poll::Ready(None);
        }

        self.video.as_mut().poll_next(cx)
    }

    /// Get the next audio chunk if available.
    pub fn poll_audio(&mut self, cx: &mut Context<'_>) -> Poll<Option<Result<Bytes, AntiBotError>>> {
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

    // Note: These tests would require mocking the AntiBotClient
    // For now, we just verify the types compile correctly

    #[test]
    fn test_combined_stream_creation() {
        // Create dummy streams for type checking
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
}
