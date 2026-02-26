//! Transcoding types — includes tonic proto-generated code + hand-written helpers

// Include tonic-generated types from transcode.proto
tonic::include_proto!("transcode");

use bytes::Bytes;

/// High-level transcoding request (wraps proto types for internal use).
#[derive(Debug, Clone)]
pub struct TranscodeRequest {
    pub data: Bytes,
    pub options: TranscodeOptions,
    pub eof: bool,
}

/// High-level transcoding response.
#[derive(Debug, Clone)]
pub struct TranscodeResponse {
    pub data: Bytes,
    pub eof: bool,
    pub error: Option<String>,
}

impl TranscodeResponse {
    pub fn success(data: Bytes, eof: bool) -> Self {
        Self { data, eof, error: None }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self { data: Bytes::new(), eof: true, error: Some(message.into()) }
    }

    pub fn is_success(&self) -> bool {
        self.error.is_none()
    }
}
