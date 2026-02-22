//! Transcoding request/response types for gRPC

use bytes::Bytes;

/// Transcoding request from client.
#[derive(Debug, Clone)]
pub struct TranscodeRequest {
    /// Video data chunk
    pub data: Bytes,
    /// Transcoding options
    pub options: TranscodeOptions,
    /// Whether this is the final chunk
    pub eof: bool,
}

/// Transcoding options.
#[derive(Debug, Clone)]
pub struct TranscodeOptions {
    /// Transcode mode (e.g., "h264_to_h265", "remux")
    pub mode: String,
    /// Target bitrate in bits per second
    pub target_bitrate: u32,
    /// Output resolution (optional)
    pub resolution: Option<(u32, u32)>,
}

impl Default for TranscodeOptions {
    fn default() -> Self {
        Self {
            mode: "passthrough".to_string(),
            target_bitrate: 5000000,
            resolution: None,
        }
    }
}

/// Transcoding response to client.
#[derive(Debug, Clone)]
pub struct TranscodeResponse {
    /// Transcoded data chunk
    pub data: Bytes,
    /// Whether this is the final chunk
    pub eof: bool,
    /// Error message if transcoding failed
    pub error: Option<String>,
}

impl TranscodeResponse {
    /// Create a successful response with data.
    pub fn success(data: Bytes, eof: bool) -> Self {
        Self {
            data,
            eof,
            error: None,
        }
    }

    /// Create an error response.
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            data: Bytes::new(),
            eof: true,
            error: Some(message.into()),
        }
    }

    /// Check if the response is successful.
    pub fn is_success(&self) -> bool {
        self.error.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcode_options_default() {
        let opts = TranscodeOptions::default();
        assert_eq!(opts.mode, "passthrough");
        assert_eq!(opts.target_bitrate, 5000000);
        assert!(opts.resolution.is_none());
    }

    #[test]
    fn test_response_success() {
        let data = Bytes::from_static(b"test");
        let resp = TranscodeResponse::success(data.clone(), false);

        assert!(resp.is_success());
        assert!(!resp.eof);
        assert_eq!(resp.data, data);
        assert!(resp.error.is_none());
    }

    #[test]
    fn test_response_error() {
        let resp = TranscodeResponse::error("transcoding failed");

        assert!(!resp.is_success());
        assert!(resp.eof);
        assert!(resp.error.is_some());
        assert_eq!(resp.error.unwrap(), "transcoding failed");
    }
}
