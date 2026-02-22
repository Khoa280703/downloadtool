//! Muxer crate - CPU-based fMP4 muxing
//!
//! Provides fragment MP4 muxing capabilities for HLS/DASH streaming
//! without requiring GPU acceleration.

pub mod fmp4_muxer;
pub mod mux_router;
pub mod stream_fetcher;
pub mod codec;

pub use fmp4_muxer::{mux_streams, MuxedStream};
pub use mux_router::{MuxRouter, StreamSource};
pub use stream_fetcher::StreamFetcher;
pub use codec::Codec;

use thiserror::Error;

/// Errors that can occur during muxing operations.
#[derive(Debug, Error)]
pub enum MuxerError {
    /// Invalid input data.
    #[error("Invalid input data: {0}")]
    InvalidInput(String),

    /// Muxing operation failed.
    #[error("Muxing failed: {0}")]
    MuxingFailed(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Feature not implemented.
    #[error("Feature not implemented")]
    NotImplemented,

    /// Stream fetch error.
    #[error("Stream fetch error: {0}")]
    StreamFetchError(String),

    /// Invalid codec.
    #[error("Invalid codec: {0}")]
    InvalidCodec(String),

    /// Proxy error.
    #[error("Proxy error: {0}")]
    ProxyError(String),
}

/// Segment information for a muxed fragment.
#[derive(Debug, Clone)]
pub struct SegmentInfo {
    /// Segment sequence number
    pub sequence_number: u64,
    /// Start timestamp in milliseconds
    pub start_time: u64,
    /// Duration in milliseconds
    pub duration: u64,
    /// Segment data
    pub data: bytes::Bytes,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = MuxerError::NotImplemented;
        assert_eq!(err.to_string(), "Feature not implemented");
    }

    #[test]
    fn test_segment_info_creation() {
        let segment = SegmentInfo {
            sequence_number: 1,
            start_time: 0,
            duration: 2000,
            data: bytes::Bytes::new(),
        };

        assert_eq!(segment.sequence_number, 1);
        assert_eq!(segment.duration, 2000);
    }

    #[test]
    fn test_muxer_error_variants() {
        let err = MuxerError::InvalidInput("test".to_string());
        assert!(err.to_string().contains("Invalid input data"));

        let err = MuxerError::StreamFetchError("fetch failed".to_string());
        assert!(err.to_string().contains("Stream fetch error"));

        let err = MuxerError::InvalidCodec("h266".to_string());
        assert!(err.to_string().contains("Invalid codec"));
    }
}
