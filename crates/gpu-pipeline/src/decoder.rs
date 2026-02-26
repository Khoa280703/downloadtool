//! NVDEC hardware decoder using FFmpeg CUDA decoders
//!
//! This module provides GPU-accelerated video decoding using NVIDIA's
//! NVDEC hardware through FFmpeg's cuvid decoders.

use bytes::Bytes;
use tracing::{debug, error, info, trace, warn};

use crate::{ffi::{AvCodecContext, AvFrame, AvPacket, ffmpeg_result}, GpuError, Resolution, VideoCodec};

/// NVDEC decoder names for different codecs.
const NVDEC_DECODER_H264: &str = "h264_cuvid";
const NVDEC_DECODER_H265: &str = "hevc_cuvid";
const NVDEC_DECODER_VP9: &str = "vp9_cuvid";
const NVDEC_DECODER_AV1: &str = "av1_cuvid";

/// CUDA pixel format for GPU frames.
const AV_PIX_FMT_CUDA: i32 = 119; // From FFmpeg pixel format definitions

/// NVDEC hardware decoder for NVIDIA GPUs.
///
/// Uses FFmpeg's cuvid decoders to decode video directly into GPU memory.
/// The decoded frames remain in CUDA memory for efficient processing.
pub struct NvDecoder {
    /// Input codec
    codec: VideoCodec,
    /// Output resolution (may differ from input for scaling)
    resolution: Resolution,
    /// FFmpeg codec context
    codec_context: Option<AvCodecContext>,
    /// Whether decoder is initialized
    initialized: bool,
    /// Decoder name being used
    decoder_name: String,
    /// Frame counter for PTS generation
    frame_count: u64,
    /// Hardware device context
    hw_device_ctx: Option<*mut ffmpeg_next::ffi::AVBufferRef>,
}

/// A decoded video frame in GPU memory.
#[derive(Debug, Clone)]
pub struct DecodedFrame {
    /// Frame data reference (in GPU memory if using CUDA)
    pub data: Bytes,
    /// Presentation timestamp
    pub pts: u64,
    /// Frame width
    pub width: u32,
    /// Frame height
    pub height: u32,
    /// Pixel format
    pub format: i32,
    /// Whether this frame is in GPU memory
    pub is_gpu_frame: bool,
}

impl NvDecoder {
    /// Create a new NVDEC decoder.
    ///
    /// # Arguments
    /// * `codec` - Input video codec
    /// * `resolution` - Output resolution (may differ from input for scaling)
    ///
    /// # Errors
    /// Returns an error if CUDA/NVDEC initialization fails.
    pub fn new(codec: VideoCodec, resolution: Resolution) -> Result<Self, GpuError> {
        info!(
            "Creating NVDEC decoder: codec={}, resolution={}x{}",
            codec.as_str(),
            resolution.width,
            resolution.height
        );

        let decoder_name = Self::get_decoder_name(codec)?;
        debug!("Using decoder: {}", decoder_name);

        Ok(Self {
            codec,
            resolution,
            codec_context: None,
            initialized: false,
            decoder_name,
            frame_count: 0,
            hw_device_ctx: None,
        })
    }

    /// Get the appropriate NVDEC decoder name for a codec.
    fn get_decoder_name(codec: VideoCodec) -> Result<String, GpuError> {
        let name = match codec {
            VideoCodec::H264 => NVDEC_DECODER_H264,
            VideoCodec::H265 => NVDEC_DECODER_H265,
            VideoCodec::VP9 => NVDEC_DECODER_VP9,
            VideoCodec::AV1 => NVDEC_DECODER_AV1,
        };

        // Check if decoder is available
        unsafe {
            let decoder = ffmpeg_next::ffi::avcodec_find_decoder_by_name(
                std::ffi::CString::new(name).unwrap().as_ptr()
            );
            if decoder.is_null() {
                return Err(GpuError::NvDecError(
                    format!("NVDEC decoder '{}' not available. Ensure FFmpeg is compiled with NVDEC support.", name)
                ));
            }
        }

        Ok(name.to_string())
    }

    /// Initialize the decoder with FFmpeg.
    ///
    /// This sets up the CUDA hardware device and codec context.
    fn initialize_decoder(&mut self) -> Result<(), GpuError> {
        if self.initialized {
            return Ok(());
        }

        debug!("Initializing NVDEC decoder with FFmpeg");

        unsafe {
            // Find the decoder
            let decoder = ffmpeg_next::ffi::avcodec_find_decoder_by_name(
                std::ffi::CString::new(self.decoder_name.clone()).unwrap().as_ptr()
            );
            if decoder.is_null() {
                return Err(GpuError::NvDecError(
                    format!("Failed to find decoder: {}", self.decoder_name)
                ));
            }

            // Allocate codec context
            let codec_context_ptr = ffmpeg_next::ffi::avcodec_alloc_context3(decoder);
            if codec_context_ptr.is_null() {
                return Err(GpuError::MemoryError(
                    "Failed to allocate codec context".to_string()
                ));
            }

            // Create CUDA device context
            let mut hw_device_ctx: *mut ffmpeg_next::ffi::AVBufferRef = std::ptr::null_mut();
            let ret = ffmpeg_next::ffi::av_hwdevice_ctx_create(
                &mut hw_device_ctx,
                ffmpeg_next::ffi::AVHWDeviceType_AV_HWDEVICE_TYPE_CUDA,
                std::ptr::null(),
                std::ptr::null_mut(),
                0,
            );
            ffmpeg_result(ret)?;

            // Set up codec context
            (*codec_context_ptr).width = self.resolution.width as i32;
            (*codec_context_ptr).height = self.resolution.height as i32;
            (*codec_context_ptr).pix_fmt = AV_PIX_FMT_CUDA;
            (*codec_context_ptr).hw_device_ctx = hw_device_ctx;

            // Open codec
            let ret = ffmpeg_next::ffi::avcodec_open2(
                codec_context_ptr,
                decoder,
                std::ptr::null_mut(),
            );
            if ret < 0 {
                ffmpeg_next::ffi::av_buffer_unref(&mut hw_device_ctx);
                ffmpeg_next::ffi::avcodec_free_context(&mut codec_context_ptr);
                return Err(GpuError::NvDecError(
                    format!("Failed to open codec: {}", crate::ffi::ffmpeg_error_str(ret))
                ));
            }

            self.codec_context = AvCodecContext::from_ptr(codec_context_ptr);
            self.hw_device_ctx = Some(hw_device_ctx);
        }

        self.initialized = true;
        info!("NVDEC decoder initialized successfully");
        Ok(())
    }

    /// Decode a video packet.
    ///
    /// # Arguments
    /// * `data` - Encoded video packet
    ///
    /// # Returns
    /// Decoded frame data or None if more data needed.
    ///
    /// # Errors
    /// Returns an error if decoding fails.
    pub fn decode(
        &mut self,
        data: Bytes,
    ) -> Result<Option<DecodedFrame>, GpuError> {
        if !self.initialized {
            self.initialize_decoder()?;
        }

        if data.is_empty() {
            return Ok(None);
        }

        trace!("Decoding packet of {} bytes", data.len());

        let codec_context = self.codec_context.as_ref()
            .ok_or_else(|| GpuError::NvDecError("Codec context not initialized".to_string()))?;

        unsafe {
            // Create packet from data
            let packet = AvPacket::new()
                .ok_or_else(|| GpuError::MemoryError("Failed to allocate packet".to_string()))?;

            let ret = ffmpeg_next::ffi::av_new_packet(packet.as_ptr(), data.len() as i32);
            ffmpeg_result(ret)?;

            // Copy data into packet
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                (*packet.as_ptr()).data,
                data.len(),
            );

            // Send packet to decoder
            let ret = ffmpeg_next::ffi::avcodec_send_packet(codec_context.as_ptr(), packet.as_ptr());
            if ret < 0 && ret != -ffmpeg_next::ffi::EAGAIN {
                return Err(GpuError::NvDecError(
                    format!("Failed to send packet: {}", crate::ffi::ffmpeg_error_str(ret))
                ));
            }

            // Receive frame
            let frame = AvFrame::new()
                .ok_or_else(|| GpuError::MemoryError("Failed to allocate frame".to_string()))?;

            let ret = ffmpeg_next::ffi::avcodec_receive_frame(codec_context.as_ptr(), frame.as_ptr());
            if ret == -ffmpeg_next::ffi::EAGAIN || ret == -ffmpeg_next::ffi::AVERROR_EOF {
                return Ok(None);
            }
            ffmpeg_result(ret)?;

            // Successfully decoded a frame
            self.frame_count += 1;
            let pts = self.frame_count;

            // For GPU frames, we return metadata only
            // The actual GPU frame reference would be passed to the encoder
            let decoded = DecodedFrame {
                data: Bytes::new(), // GPU frames don't have CPU-accessible data here
                pts,
                width: frame.width() as u32,
                height: frame.height() as u32,
                format: frame.format(),
                is_gpu_frame: frame.format() == AV_PIX_FMT_CUDA,
            };

            trace!(
                "Decoded frame {}: {}x{}, format={}",
                pts, decoded.width, decoded.height, decoded.format
            );

            Ok(Some(decoded))
        }
    }

    /// Flush the decoder and return any pending frames.
    ///
    /// Call this when the input stream ends to retrieve any
    /// buffered frames from the decoder.
    pub fn flush(&mut self) -> Result<Vec<DecodedFrame>, GpuError> {
        if !self.initialized {
            return Ok(vec![]);
        }

        debug!("Flushing NVDEC decoder");

        let codec_context = self.codec_context.as_ref()
            .ok_or_else(|| GpuError::NvDecError("Codec context not initialized".to_string()))?;

        let mut frames = Vec::new();

        unsafe {
            // Send null packet to signal EOF
            let ret = ffmpeg_next::ffi::avcodec_send_packet(codec_context.as_ptr(), std::ptr::null());
            if ret < 0 && ret != -ffmpeg_next::ffi::AVERROR_EOF {
                warn!("Error sending flush packet: {}", crate::ffi::ffmpeg_error_str(ret));
            }

            // Receive all remaining frames
            loop {
                let frame = AvFrame::new()
                    .ok_or_else(|| GpuError::MemoryError("Failed to allocate frame".to_string()))?;

                let ret = ffmpeg_next::ffi::avcodec_receive_frame(codec_context.as_ptr(), frame.as_ptr());
                if ret == -ffmpeg_next::ffi::EAGAIN || ret == -ffmpeg_next::ffi::AVERROR_EOF {
                    break;
                }
                if ret < 0 {
                    return Err(GpuError::NvDecError(
                        format!("Error receiving frame during flush: {}", crate::ffi::ffmpeg_error_str(ret))
                    ));
                }

                self.frame_count += 1;
                frames.push(DecodedFrame {
                    data: Bytes::new(),
                    pts: self.frame_count,
                    width: frame.width() as u32,
                    height: frame.height() as u32,
                    format: frame.format(),
                    is_gpu_frame: frame.format() == AV_PIX_FMT_CUDA,
                });
            }
        }

        debug!("Flushed {} frames from decoder", frames.len());
        Ok(frames)
    }

    /// Get the decoder codec.
    pub fn codec(&self) -> VideoCodec {
        self.codec
    }

    /// Get the output resolution.
    pub fn resolution(&self) -> Resolution {
        self.resolution
    }

    /// Check if decoder is initialized.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get the decoder name.
    pub fn decoder_name(&self) -> &str {
        &self.decoder_name
    }

    /// Get the total frame count.
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }
}

impl Drop for NvDecoder {
    fn drop(&mut self) {
        if self.initialized {
            debug!("Destroying NVDEC decoder");

            unsafe {
                // Clean up hardware device context
                if let Some(hw_ctx) = self.hw_device_ctx.take() {
                    ffmpeg_next::ffi::av_buffer_unref(&mut (hw_ctx as *mut _));
                }
            }

            // Codec context will be freed by AvCodecContext Drop impl
        }
    }
}

unsafe impl Send for NvDecoder {}
unsafe impl Sync for NvDecoder {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoder_creation() {
        let decoder = NvDecoder::new(VideoCodec::H264, Resolution::p1080());
        assert!(decoder.is_ok());

        let decoder = decoder.unwrap();
        assert_eq!(decoder.codec(), VideoCodec::H264);
        assert!(!decoder.is_initialized());
        assert_eq!(decoder.decoder_name(), "h264_cuvid");
    }

    #[test]
    fn test_decoder_resolution() {
        let decoder = NvDecoder::new(VideoCodec::H265, Resolution::p720()).unwrap();
        let res = decoder.resolution();
        assert_eq!(res.width, 1280);
        assert_eq!(res.height, 720);
    }

    #[test]
    fn test_get_decoder_name() {
        assert_eq!(
            NvDecoder::get_decoder_name(VideoCodec::H264).unwrap(),
            "h264_cuvid"
        );
        assert_eq!(
            NvDecoder::get_decoder_name(VideoCodec::H265).unwrap(),
            "hevc_cuvid"
        );
        assert_eq!(
            NvDecoder::get_decoder_name(VideoCodec::VP9).unwrap(),
            "vp9_cuvid"
        );
        assert_eq!(
            NvDecoder::get_decoder_name(VideoCodec::AV1).unwrap(),
            "av1_cuvid"
        );
    }

    #[test]
    fn test_decoded_frame_creation() {
        let frame = DecodedFrame {
            data: Bytes::from(vec![1, 2, 3]),
            pts: 100,
            width: 1920,
            height: 1080,
            format: AV_PIX_FMT_CUDA,
            is_gpu_frame: true,
        };

        assert_eq!(frame.pts, 100);
        assert_eq!(frame.width, 1920);
        assert!(frame.is_gpu_frame);
    }
}
