//! Types for video extraction

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during video extraction.
#[derive(Debug, Error)]
pub enum ExtractionError {
    /// The scripts directory was not found.
    #[error("Scripts directory not found: {0}")]
    ScriptsDirectoryNotFound(String),

    /// The extractor script failed to execute.
    #[error("Script execution failed: {0}")]
    ScriptExecutionFailed(String),

    /// The video URL is invalid or unsupported.
    #[error("Invalid or unsupported URL: {0}")]
    InvalidUrl(String),

    /// Network error during extraction.
    #[error("Network error: {0}")]
    NetworkError(String),

    /// JavaScript runtime error.
    #[error("JavaScript error: {0}")]
    JavaScriptError(String),

    /// Feature not yet implemented.
    #[error("Feature not implemented")]
    NotImplemented,
}

/// Video information extracted from a URL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    /// Video title
    pub title: String,
    /// Video description (optional)
    pub description: Option<String>,
    /// Video duration in seconds
    pub duration: Option<u64>,
    /// Thumbnail URL
    pub thumbnail: Option<String>,
    /// Available video formats
    pub formats: Vec<VideoFormat>,
    /// Original URL
    pub original_url: String,
}

/// Video format information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFormat {
    /// Format ID used by the extractor
    pub format_id: String,
    /// Quality label from extractor (e.g., "1080p (video only)", "Audio 128kbps")
    pub quality: String,
    /// Video codec
    pub vcodec: Option<String>,
    /// Audio codec
    pub acodec: Option<String>,
    /// Human-readable codec label (e.g., "H.264", "VP9", "AV1", "AAC")
    pub codec_label: Option<String>,
    /// Whether this stream has an audio track
    pub has_audio: bool,
    /// Whether this stream is audio-only
    pub is_audio_only: bool,
    /// Width in pixels
    pub width: Option<u32>,
    /// Height in pixels
    pub height: Option<u32>,
    /// Frame rate
    pub fps: Option<f32>,
    /// Bitrate in bits per second
    pub bitrate: Option<u64>,
    /// File extension
    pub ext: String,
    /// Direct download URL
    pub url: String,
    /// File size in bytes (if known)
    pub filesize: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ExtractionError::NotImplemented;
        assert_eq!(err.to_string(), "Feature not implemented");
    }

    #[test]
    fn test_video_info_serialization() {
        let info = VideoInfo {
            title: "Test Video".to_string(),
            description: None,
            duration: Some(120),
            thumbnail: None,
            formats: vec![],
            original_url: "https://example.com/video".to_string(),
        };

        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("Test Video"));
    }
}
