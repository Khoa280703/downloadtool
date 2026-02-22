//! GPU Worker crate - gRPC server for Home Server
//!
//! This crate provides a gRPC server that runs on the Home Server
//! and handles transcoding requests from the VPS via GPU acceleration.

pub mod server;
pub mod transcode;

pub use server::GpuWorkerServer;
pub use transcode::{TranscodeRequest, TranscodeResponse};

use thiserror::Error;

/// Errors that can occur in the GPU worker.
#[derive(Debug, Error)]
pub enum WorkerError {
    /// gRPC server error.
    #[error("gRPC server error: {0}")]
    ServerError(String),

    /// Transcoding error.
    #[error("Transcoding error: {0}")]
    TranscodeError(String),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = WorkerError::ConfigError("test".to_string());
        assert_eq!(err.to_string(), "Configuration error: test");
    }
}
