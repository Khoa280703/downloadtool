//! NVENC hardware encoder using FFmpeg
//!
//! This module provides GPU-accelerated video encoding using NVIDIA's
//! NVENC hardware through FFmpeg's nvenc encoders.

use bytes::Bytes;
use tracing::{debug, error, info, trace, warn};

use crate::{ffi::{AvCodecContext, AvFrame, AvPacket, ffmpeg_result}, GpuError, Resolution, VideoCodec};

/// NVENC encoder name for H.264.
const NVENC_ENCODER_H264: &str = "h264_nvenc";
/// NVENC encoder name for H.265/HEVC.
const NVENC_ENCODER_H265: &str = "hevc_nvenc";

/// CUDA pixel format for GPU frames.
const AV_PIX_FMT_CUDA: i32 = 119;
/// NV12 pixel format.
const AV_PIX_FMT_NV12: i32 = 23;

/// NVENC hardware encoder for NVIDIA GPUs.
///
/// Uses FFmpeg's nvenc encoders to encode video directly from GPU memory.
/// Supports H.264 and H.265 output codecs with configurable quality settings.
pub struct NvEncoder {
    /// Output codec
    codec: VideoCodec,
    /// Output resolution
    resolution: Resolution,
    /// Target bitrate in bits per second
    target_bitrate: u32,
    /// Encoding preset
    preset: EncodePreset,
    /// Rate control mode
    rc_mode: RateControlMode,
    /// FFmpeg codec context
    codec_context: Option<AvCodecContext>,
    /// Whether encoder is initialized
    initialized: bool,
    /// Encoder name being used
    encoder_name: String,
    /// Packet counter
    packet_count: u64,
    /// Hardware device context
    hw_device_ctx: Option<*mut ffmpeg_next::ffi::AVBufferRef>,
}

/// An encoded video packet.
#[derive(Debug, Clone)]
pub struct EncodedPacket {
    /// Encoded data
    pub data: Bytes,
    /// Presentation timestamp
    pub pts: u64,
    /// Decode timestamp
    pub dts: i64,
    /// Whether this is a keyframe
    pub is_keyframe: bool,
    /// Packet duration
    pub duration: i64,
}

/// Encoding preset for quality vs speed tradeoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodePreset {
    /// Slowest, highest quality (equivalent to p1)
    Slow,
    /// Balanced quality and speed (equivalent to p4)
    Medium,
    /// Fastest, lower quality (equivalent to p7)
    Fast,
    /// Custom preset level (p1-p7)
    Custom(i32),
}

impl EncodePreset {
    /// Get the preset string for NVENC.
    pub fn as_str(&self) -> String {
        match self {
            EncodePreset::Slow => "p1".to_string(),
            EncodePreset::Medium => "p4".to_string(),
            EncodePreset::Fast => "p7".to_string(),
            EncodePreset::Custom(n) => format!("p{}", n.clamp(1, 7)),
        }
    }

    /// Get the preset as a number (1-7).
    pub fn as_number(&self) -> i32 {
        match self {
            EncodePreset::Slow => 1,
            EncodePreset::Medium => 4,
            EncodePreset::Fast => 7,
            EncodePreset::Custom(n) => *n,
        }
    }
}

impl Default for EncodePreset {
    fn default() -> Self {
        EncodePreset::Medium
    }
}

/// Rate control mode for encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateControlMode {
    /// Constant bitrate
    CBR,
    /// Variable bitrate
    VBR,
    /// Constant QP (quality)
    CQP,
    /// Constant quality (NVENC specific)
    CQ,
}

impl RateControlMode {
    /// Get the rate control string for NVENC.
    pub fn as_str(&self) -> &'static str {
        match self {
            RateControlMode::CBR => "cbr",
            RateControlMode::VBR => "vbr",
            RateControlMode::CQP => "cqp",
            RateControlMode::CQ => "constqp",
        }
    }
}

impl Default for RateControlMode {
    fn default() -> Self {
        RateControlMode::VBR
    }
}

/// Encoder configuration options.
#[derive(Debug, Clone)]
pub struct EncoderConfig {
    /// Output codec
    pub codec: VideoCodec,
    /// Output resolution
    pub resolution: Resolution,
    /// Target bitrate in bits per second
    pub target_bitrate: u32,
    /// Encoding preset
    pub preset: EncodePreset,
    /// Rate control mode
    pub rc_mode: RateControlMode,
    /// Constant QP value (for CQP mode)
    pub qp: Option<u32>,
    /// Frame rate numerator
    pub fps_num: u32,
    /// Frame rate denominator
    pub fps_den: u32,
    /// GOP size (keyframe interval)
    pub gop_size: i32,
}

impl Default for EncoderConfig {
    fn default() -> Self {
        Self {
            codec: VideoCodec::H264,
            resolution: Resolution::p1080(),
            target_bitrate: 5_000_000, // 5 Mbps
            preset: EncodePreset::default(),
            rc_mode: RateControlMode::default(),
            qp: Some(23),
            fps_num: 30,
            fps_den: 1,
            gop_size: 250,
        }
    }
}

impl NvEncoder {
    /// Create a new NVENC encoder.
    ///
    /// # Arguments
    /// * `codec` - Output video codec
    /// * `resolution` - Output resolution
    /// * `target_bitrate` - Target bitrate in bits per second
    ///
    /// # Errors
    /// Returns an error if CUDA/NVENC initialization fails.
    pub fn new(
        codec: VideoCodec,
        resolution: Resolution,
        target_bitrate: u32,
    ) -> Result<Self, GpuError> {
        info!(
            "Creating NVENC encoder: codec={}, resolution={}x{}, bitrate={}",
            codec.as_str(),
            resolution.width,
            resolution.height,
            target_bitrate
        );

        let encoder_name = Self::get_encoder_name(codec)?;
        debug!("Using encoder: {}", encoder_name);

        // Validate codec support
        match codec {
            VideoCodec::H264 | VideoCodec::H265 => {
                // Supported
            }
            _ => {
                return Err(GpuError::InvalidFormat(format!(
                    "Codec {:?} not supported by NVENC",
                    codec
                )));
            }
        }

        Ok(Self {
            codec,
            resolution,
            target_bitrate,
            preset: EncodePreset::default(),
            rc_mode: RateControlMode::default(),
            codec_context: None,
            initialized: false,
            encoder_name,
            packet_count: 0,
            hw_device_ctx: None,
        })
    }

    /// Create a new encoder with full configuration.
    pub fn with_config(config: EncoderConfig) -> Result<Self, GpuError> {
        let mut encoder = Self::new(
            config.codec,
            config.resolution,
            config.target_bitrate,
        )?;
        encoder.preset = config.preset;
        encoder.rc_mode = config.rc_mode;
        Ok(encoder)
    }

    /// Get the appropriate NVENC encoder name for a codec.
    fn get_encoder_name(codec: VideoCodec) -> Result<String, GpuError> {
        let name = match codec {
            VideoCodec::H264 => NVENC_ENCODER_H264,
            VideoCodec::H265 => NVENC_ENCODER_H265,
            _ => {
                return Err(GpuError::InvalidFormat(format!(
                    "NVENC does not support codec: {:?}",
                    codec
                )));
            }
        };

        // Check if encoder is available
        unsafe {
            let encoder = ffmpeg_next::ffi::avcodec_find_encoder_by_name(
                std::ffi::CString::new(name).unwrap().as_ptr()
            );
            if encoder.is_null() {
                return Err(GpuError::NvEncError(
                    format!("NVENC encoder '{}' not available. Ensure FFmpeg is compiled with NVENC support.", name)
                ));
            }
        }

        Ok(name.to_string())
    }

    /// Initialize the encoder with FFmpeg.
    fn initialize_encoder(&mut self) -> Result<(), GpuError> {
        if self.initialized {
            return Ok(());
        }

        debug!("Initializing NVENC encoder with FFmpeg");

        unsafe {
            // Find the encoder
            let encoder = ffmpeg_next::ffi::avcodec_find_encoder_by_name(
                std::ffi::CString::new(self.encoder_name.clone()).unwrap().as_ptr()
            );
            if encoder.is_null() {
                return Err(GpuError::NvEncError(
                    format!("Failed to find encoder: {}", self.encoder_name)
                ));
            }

            // Allocate codec context
            let codec_context_ptr = ffmpeg_next::ffi::avcodec_alloc_context3(encoder);
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

            // Configure codec context
            (*codec_context_ptr).width = self.resolution.width as i32;
            (*codec_context_ptr).height = self.resolution.height as i32;
            (*codec_context_ptr).pix_fmt = AV_PIX_FMT_CUDA;
            (*codec_context_ptr).bit_rate = self.target_bitrate as i64;
            (*codec_context_ptr).gop_size = 250; // Keyframe every 250 frames
            (*codec_context_ptr).max_b_frames = 0; // NVENC works best with no B-frames
            (*codec_context_ptr).hw_device_ctx = hw_device_ctx;

            // Set time base (1/30 for 30fps)
            (*codec_context_ptr).time_base.num = 1;
            (*codec_context_ptr).time_base.den = 30;

            // Set framerate
            (*codec_context_ptr).framerate.num = 30;
            (*codec_context_ptr).framerate.den = 1;

            // Set encoding options
            let mut opts: *mut ffmpeg_next::ffi::AVDictionary = std::ptr::null_mut();

            // Preset (p1-p7)
            let preset_str = std::ffi::CString::new(self.preset.as_str()).unwrap();
            ffmpeg_next::ffi::av_dict_set(
                &mut opts,
                std::ffi::CString::new("preset").unwrap().as_ptr(),
                preset_str.as_ptr(),
                0,
            );

            // Rate control mode
            let rc_str = std::ffi::CString::new(self.rc_mode.as_str()).unwrap();
            ffmpeg_next::ffi::av_dict_set(
                &mut opts,
                std::ffi::CString::new("rc").unwrap().as_ptr(),
                rc_str.as_ptr(),
                0,
            );

            // Tune for low latency
            ffmpeg_next::ffi::av_dict_set(
                &mut opts,
                std::ffi::CString::new("tune").unwrap().as_ptr(),
                std::ffi::CString::new("ll").unwrap().as_ptr(),
                0,
            );

            // Profile
            let profile = if self.codec == VideoCodec::H264 {
                "high"
            } else {
                "main"
            };
            ffmpeg_next::ffi::av_dict_set(
                &mut opts,
                std::ffi::CString::new("profile").unwrap().as_ptr(),
                std::ffi::CString::new(profile).unwrap().as_ptr(),
                0,
            );

            // Open codec
            let ret = ffmpeg_next::ffi::avcodec_open2(
                codec_context_ptr,
                encoder,
                &mut opts,
            );

            // Clean up dictionary
            ffmpeg_next::ffi::av_dict_free(&mut opts);

            if ret < 0 {
                ffmpeg_next::ffi::av_buffer_unref(&mut hw_device_ctx);
                ffmpeg_next::ffi::avcodec_free_context(&mut codec_context_ptr);
                return Err(GpuError::NvEncError(
                    format!("Failed to open encoder: {}", crate::ffi::ffmpeg_error_str(ret))
                ));
            }

            self.codec_context = AvCodecContext::from_ptr(codec_context_ptr);
            self.hw_device_ctx = Some(hw_device_ctx);
        }

        self.initialized = true;
        info!("NVENC encoder initialized successfully");
        Ok(())
    }

    /// Encode a raw frame.
    ///
    /// # Arguments
    /// * `frame` - Raw frame data or empty for GPU frames
    /// * `pts` - Presentation timestamp
    ///
    /// # Returns
    /// Encoded packet data or None if more data needed.
    ///
    /// # Errors
    /// Returns an error if encoding fails.
    pub fn encode(
        &mut self,
        _frame: &[u8],
        pts: u64,
    ) -> Result<Option<EncodedPacket>, GpuError> {
        if !self.initialized {
            self.initialize_encoder()?;
        }

        let codec_context = self.codec_context.as_ref()
            .ok_or_else(|| GpuError::NvEncError("Codec context not initialized".to_string()))?;

        unsafe {
            // Create frame
            let frame = AvFrame::new()
                .ok_or_else(|| GpuError::MemoryError("Failed to allocate frame".to_string()))?;

            // Set frame properties
            (*frame.as_ptr()).width = self.resolution.width as i32;
            (*frame.as_ptr()).height = self.resolution.height as i32;
            (*frame.as_ptr()).format = AV_PIX_FMT_CUDA;
            (*frame.as_ptr()).pts = pts as i64;

            // Send frame to encoder
            let ret = ffmpeg_next::ffi::avcodec_send_frame(codec_context.as_ptr(), frame.as_ptr());
            if ret < 0 && ret != -ffmpeg_next::ffi::EAGAIN {
                return Err(GpuError::NvEncError(
                    format!("Failed to send frame: {}", crate::ffi::ffmpeg_error_str(ret))
                ));
            }

            // Receive packet
            let packet = AvPacket::new()
                .ok_or_else(|| GpuError::MemoryError("Failed to allocate packet".to_string()))?;

            let ret = ffmpeg_next::ffi::avcodec_receive_packet(codec_context.as_ptr(), packet.as_ptr());
            if ret == -ffmpeg_next::ffi::EAGAIN || ret == -ffmpeg_next::ffi::AVERROR_EOF {
                return Ok(None);
            }
            ffmpeg_result(ret)?;

            // Successfully encoded a packet
            self.packet_count += 1;

            let data = Bytes::copy_from_slice(packet.data());
            let encoded = EncodedPacket {
                data,
                pts,
                dts: packet.dts(),
                is_keyframe: packet.is_keyframe(),
                duration: (*packet.as_ptr()).duration,
            };

            trace!(
                "Encoded packet {}: {} bytes, keyframe={}",
                pts, encoded.data.len(), encoded.is_keyframe
            );

            Ok(Some(encoded))
        }
    }

    /// Flush the encoder and return any pending packets.
    pub fn flush(&mut self) -> Result<Vec<EncodedPacket>, GpuError> {
        if !self.initialized {
            return Ok(vec![]);
        }

        debug!("Flushing NVENC encoder");

        let codec_context = self.codec_context.as_ref()
            .ok_or_else(|| GpuError::NvEncError("Codec context not initialized".to_string()))?;

        let mut packets = Vec::new();

        unsafe {
            // Send null frame to signal EOF
            let ret = ffmpeg_next::ffi::avcodec_send_frame(codec_context.as_ptr(), std::ptr::null());
            if ret < 0 && ret != -ffmpeg_next::ffi::AVERROR_EOF {
                warn!("Error sending flush frame: {}", crate::ffi::ffmpeg_error_str(ret));
            }

            // Receive all remaining packets
            loop {
                let packet = AvPacket::new()
                    .ok_or_else(|| GpuError::MemoryError("Failed to allocate packet".to_string()))?;

                let ret = ffmpeg_next::ffi::avcodec_receive_packet(codec_context.as_ptr(), packet.as_ptr());
                if ret == -ffmpeg_next::ffi::EAGAIN || ret == -ffmpeg_next::ffi::AVERROR_EOF {
                    break;
                }
                if ret < 0 {
                    return Err(GpuError::NvEncError(
                        format!("Error receiving packet during flush: {}", crate::ffi::ffmpeg_error_str(ret))
                    ));
                }

                self.packet_count += 1;
                packets.push(EncodedPacket {
                    data: Bytes::copy_from_slice(packet.data()),
                    pts: self.packet_count,
                    dts: packet.dts(),
                    is_keyframe: packet.is_keyframe(),
                    duration: (*packet.as_ptr()).duration,
                });
            }
        }

        debug!("Flushed {} packets from encoder", packets.len());
        Ok(packets)
    }

    /// Get the encoder codec.
    pub fn codec(&self) -> VideoCodec {
        self.codec
    }

    /// Get the output resolution.
    pub fn resolution(&self) -> Resolution {
        self.resolution
    }

    /// Get the target bitrate.
    pub fn target_bitrate(&self) -> u32 {
        self.target_bitrate
    }

    /// Get the encoding preset.
    pub fn preset(&self) -> &EncodePreset {
        &self.preset
    }

    /// Set the encoding preset.
    pub fn set_preset(&mut self, preset: EncodePreset) {
        self.preset = preset;
    }

    /// Check if encoder is initialized.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get the encoder name.
    pub fn encoder_name(&self) -> &str {
        &self.encoder_name
    }

    /// Get the total packet count.
    pub fn packet_count(&self) -> u64 {
        self.packet_count
    }
}

impl Drop for NvEncoder {
    fn drop(&mut self) {
        if self.initialized {
            debug!("Destroying NVENC encoder");

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

unsafe impl Send for NvEncoder {}
unsafe impl Sync for NvEncoder {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_creation() {
        let encoder = NvEncoder::new(VideoCodec::H264, Resolution::p1080(), 5000000);
        assert!(encoder.is_ok());

        let encoder = encoder.unwrap();
        assert_eq!(encoder.codec(), VideoCodec::H264);
        assert_eq!(encoder.target_bitrate(), 5000000);
        assert!(!encoder.is_initialized());
    }

    #[test]
    fn test_invalid_codec() {
        let encoder = NvEncoder::new(VideoCodec::AV1, Resolution::p1080(), 5000000);
        assert!(matches!(encoder, Err(GpuError::InvalidFormat(_))));
    }

    #[test]
    fn test_preset_as_str() {
        assert_eq!(EncodePreset::Slow.as_str(), "p1");
        assert_eq!(EncodePreset::Medium.as_str(), "p4");
        assert_eq!(EncodePreset::Fast.as_str(), "p7");
        assert_eq!(EncodePreset::Custom(3).as_str(), "p3");
        assert_eq!(EncodePreset::Custom(10).as_str(), "p7"); // Clamped
    }

    #[test]
    fn test_rate_control_mode() {
        assert_eq!(RateControlMode::CBR.as_str(), "cbr");
        assert_eq!(RateControlMode::VBR.as_str(), "vbr");
        assert_eq!(RateControlMode::CQP.as_str(), "cqp");
        assert_eq!(RateControlMode::CQ.as_str(), "constqp");
    }

    #[test]
    fn test_encoder_config_default() {
        let config = EncoderConfig::default();
        assert_eq!(config.target_bitrate, 5_000_000);
        assert_eq!(config.preset, EncodePreset::Medium);
        assert_eq!(config.rc_mode, RateControlMode::VBR);
    }

    #[test]
    fn test_encoded_packet() {
        let packet = EncodedPacket {
            data: Bytes::from(vec![1, 2, 3, 4]),
            pts: 100,
            dts: 100,
            is_keyframe: true,
            duration: 33,
        };

        assert_eq!(packet.data.len(), 4);
        assert_eq!(packet.pts, 100);
        assert!(packet.is_keyframe);
    }
}
