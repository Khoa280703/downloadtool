//! GPU Pipeline crate - NVDEC/NVENC hardware acceleration
//!
//! This crate provides GPU-accelerated video transcoding capabilities
//! using NVIDIA's NVDEC (decode) and NVENC (encode) hardware.
//!
//! Note: This crate is only available on the Home Server deployment
//! and requires NVIDIA GPU with CUDA support.

#[cfg(feature = "gpu-support")]
pub mod decoder;
#[cfg(feature = "gpu-support")]
pub mod encoder;
#[cfg(feature = "gpu-support")]
pub mod frame_queue;
#[cfg(feature = "gpu-support")]
pub mod pipeline;
#[cfg(feature = "gpu-support")]
pub mod watermark;
#[cfg(feature = "gpu-support")]
pub mod ffi;

#[cfg(feature = "gpu-support")]
pub use decoder::NvDecoder;
#[cfg(feature = "gpu-support")]
pub use encoder::NvEncoder;
#[cfg(feature = "gpu-support")]
pub use pipeline::GpuPipeline;
#[cfg(feature = "gpu-support")]
pub use frame_queue::FrameQueue;
#[cfg(feature = "gpu-support")]
pub use watermark::Watermark;

use thiserror::Error;

/// Errors that can occur during GPU operations.
#[derive(Debug, Error)]
pub enum GpuError {
    /// CUDA initialization failed.
    #[error("CUDA initialization failed: {0}")]
    CudaInitFailed(String),

    /// NVDEC decoder error.
    #[error("NVDEC error: {0}")]
    NvDecError(String),

    /// NVENC encoder error.
    #[error("NVENC error: {0}")]
    NvEncError(String),

    /// No compatible GPU found.
    #[error("No compatible NVIDIA GPU found")]
    NoGpuFound,

    /// Invalid video format.
    #[error("Invalid video format: {0}")]
    InvalidFormat(String),

    /// Memory allocation failed.
    #[error("GPU memory allocation failed: {0}")]
    MemoryError(String),

    /// Feature not implemented.
    #[error("Feature not implemented")]
    NotImplemented,
}

/// Video codec types supported by the GPU pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoCodec {
    /// H.264 / AVC
    H264,
    /// H.265 / HEVC
    H265,
    /// AV1
    AV1,
    /// VP9
    VP9,
}

impl VideoCodec {
    /// Get the codec name as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            VideoCodec::H264 => "h264",
            VideoCodec::H265 => "hevc",
            VideoCodec::AV1 => "av1",
            VideoCodec::VP9 => "vp9",
        }
    }
}

/// Video resolution.
#[derive(Debug, Clone, Copy)]
pub struct Resolution {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
}

impl Resolution {
    /// Create a new resolution.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Get 1080p resolution.
    pub fn p1080() -> Self {
        Self::new(1920, 1080)
    }

    /// Get 720p resolution.
    pub fn p720() -> Self {
        Self::new(1280, 720)
    }

    /// Get 480p resolution.
    pub fn p480() -> Self {
        Self::new(854, 480)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_codec_as_str() {
        assert_eq!(VideoCodec::H264.as_str(), "h264");
        assert_eq!(VideoCodec::H265.as_str(), "hevc");
    }

    #[test]
    fn test_resolution_creation() {
        let res = Resolution::p1080();
        assert_eq!(res.width, 1920);
        assert_eq!(res.height, 1080);
    }

    #[test]
    fn test_error_display() {
        let err = GpuError::NoGpuFound;
        assert_eq!(err.to_string(), "No compatible NVIDIA GPU found");
    }
}
