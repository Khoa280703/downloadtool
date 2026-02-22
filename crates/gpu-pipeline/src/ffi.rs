//! FFI wrappers for FFmpeg types with RAII safety
//!
//! This module provides safe Rust wrappers around FFmpeg's C types,
//! ensuring proper resource cleanup through Drop implementations.

use std::ptr::NonNull;

/// Safe wrapper for FFmpeg AVCodecContext.
pub struct AvCodecContext {
    ptr: NonNull<ffmpeg_next::ffi::AVCodecContext>,
}

impl AvCodecContext {
    /// Create a new codec context wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be a valid, non-null AVCodecContext pointer.
    pub unsafe fn from_ptr(ptr: *mut ffmpeg_next::ffi::AVCodecContext) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    /// Get the raw pointer to the codec context.
    pub fn as_ptr(&self) -> *mut ffmpeg_next::ffi::AVCodecContext {
        self.ptr.as_ptr()
    }

    /// Get a mutable pointer to the codec context pointer.
    /// Used for avcodec_free_context which takes a double pointer.
    pub fn as_mut_ptr(&mut self) -> *mut *mut ffmpeg_next::ffi::AVCodecContext {
        &mut self.ptr.as_ptr()
    }
}

impl Drop for AvCodecContext {
    fn drop(&mut self) {
        unsafe {
            let mut ptr = self.ptr.as_ptr();
            ffmpeg_next::ffi::avcodec_free_context(&mut ptr);
        }
    }
}

unsafe impl Send for AvCodecContext {}
unsafe impl Sync for AvCodecContext {}

/// Safe wrapper for FFmpeg AVFrame.
pub struct AvFrame {
    ptr: NonNull<ffmpeg_next::ffi::AVFrame>,
}

impl AvFrame {
    /// Allocate a new AVFrame.
    pub fn new() -> Option<Self> {
        unsafe {
            let ptr = ffmpeg_next::ffi::av_frame_alloc();
            NonNull::new(ptr).map(|ptr| Self { ptr })
        }
    }

    /// Create a wrapper from an existing raw pointer.
    ///
    /// # Safety
    /// The pointer must be a valid, non-null AVFrame pointer.
    pub unsafe fn from_ptr(ptr: *mut ffmpeg_next::ffi::AVFrame) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    /// Get the raw pointer to the frame.
    pub fn as_ptr(&self) -> *mut ffmpeg_next::ffi::AVFrame {
        self.ptr.as_ptr()
    }

    /// Get the width of the frame.
    pub fn width(&self) -> i32 {
        unsafe { (*self.ptr.as_ptr()).width }
    }

    /// Get the height of the frame.
    pub fn height(&self) -> i32 {
        unsafe { (*self.ptr.as_ptr()).height }
    }

    /// Get the presentation timestamp.
    pub fn pts(&self) -> i64 {
        unsafe { (*self.ptr.as_ptr()).pts }
    }

    /// Set the presentation timestamp.
    pub fn set_pts(&mut self, pts: i64) {
        unsafe {
            (*self.ptr.as_ptr()).pts = pts;
        }
    }

    /// Get the pixel format.
    pub fn format(&self) -> i32 {
        unsafe { (*self.ptr.as_ptr()).format }
    }

    /// Set the pixel format.
    pub fn set_format(&mut self, format: i32) {
        unsafe {
            (*self.ptr.as_ptr()).format = format;
        }
    }

    /// Set the width.
    pub fn set_width(&mut self, width: i32) {
        unsafe {
            (*self.ptr.as_ptr()).width = width;
        }
    }

    /// Set the height.
    pub fn set_height(&mut self, height: i32) {
        unsafe {
            (*self.ptr.as_ptr()).height = height;
        }
    }
}

impl Drop for AvFrame {
    fn drop(&mut self) {
        unsafe {
            let mut ptr = self.ptr.as_ptr();
            ffmpeg_next::ffi::av_frame_free(&mut ptr);
        }
    }
}

unsafe impl Send for AvFrame {}
unsafe impl Sync for AvFrame {}

impl Default for AvFrame {
    fn default() -> Self {
        Self::new().expect("Failed to allocate AVFrame")
    }
}

/// Safe wrapper for FFmpeg AVPacket.
pub struct AvPacket {
    ptr: NonNull<ffmpeg_next::ffi::AVPacket>,
}

impl AvPacket {
    /// Allocate a new AVPacket.
    pub fn new() -> Option<Self> {
        unsafe {
            let ptr = ffmpeg_next::ffi::av_packet_alloc();
            NonNull::new(ptr).map(|ptr| Self { ptr })
        }
    }

    /// Create a wrapper from an existing raw pointer.
    ///
    /// # Safety
    /// The pointer must be a valid, non-null AVPacket pointer.
    pub unsafe fn from_ptr(ptr: *mut ffmpeg_next::ffi::AVPacket) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    /// Get the raw pointer to the packet.
    pub fn as_ptr(&self) -> *mut ffmpeg_next::ffi::AVPacket {
        self.ptr.as_ptr()
    }

    /// Get the packet data as a byte slice.
    pub fn data(&self) -> &[u8] {
        unsafe {
            let pkt = self.ptr.as_ptr();
            if (*pkt).data.is_null() || (*pkt).size <= 0 {
                return &[];
            }
            std::slice::from_raw_parts((*pkt).data, (*pkt).size as usize)
        }
    }

    /// Get the presentation timestamp.
    pub fn pts(&self) -> i64 {
        unsafe { (*self.ptr.as_ptr()).pts }
    }

    /// Get the decode timestamp.
    pub fn dts(&self) -> i64 {
        unsafe { (*self.ptr.as_ptr()).dts }
    }

    /// Check if this is a keyframe.
    pub fn is_keyframe(&self) -> bool {
        unsafe { (*self.ptr.as_ptr()).flags & ffmpeg_next::ffi::AV_PKT_FLAG_KEY as i32 != 0 }
    }
}

impl Drop for AvPacket {
    fn drop(&mut self) {
        unsafe {
            let mut ptr = self.ptr.as_ptr();
            ffmpeg_next::ffi::av_packet_free(&mut ptr);
        }
    }
}

unsafe impl Send for AvPacket {}
unsafe impl Sync for AvPacket {}

impl Default for AvPacket {
    fn default() -> Self {
        Self::new().expect("Failed to allocate AVPacket")
    }
}

/// Safe wrapper for FFmpeg AVFormatContext (input).
pub struct AvFormatContext {
    ptr: NonNull<ffmpeg_next::ffi::AVFormatContext>,
}

impl AvFormatContext {
    /// Create a wrapper from an existing raw pointer.
    ///
    /// # Safety
    /// The pointer must be a valid, non-null AVFormatContext pointer.
    pub unsafe fn from_ptr(ptr: *mut ffmpeg_next::ffi::AVFormatContext) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    /// Get the raw pointer.
    pub fn as_ptr(&self) -> *mut ffmpeg_next::ffi::AVFormatContext {
        self.ptr.as_ptr()
    }

    /// Get a mutable pointer for closing.
    pub fn as_mut_ptr(&mut self) -> *mut *mut ffmpeg_next::ffi::AVFormatContext {
        &mut self.ptr.as_ptr()
    }
}

impl Drop for AvFormatContext {
    fn drop(&mut self) {
        unsafe {
            let mut ptr = self.ptr.as_ptr();
            ffmpeg_next::ffi::avformat_close_input(&mut ptr);
        }
    }
}

unsafe impl Send for AvFormatContext {}
unsafe impl Sync for AvFormatContext {}

/// Safe wrapper for FFmpeg AVFilterGraph.
pub struct AvFilterGraph {
    ptr: NonNull<ffmpeg_next::ffi::AVFilterGraph>,
}

impl AvFilterGraph {
    /// Allocate a new filter graph.
    pub fn new() -> Option<Self> {
        unsafe {
            let ptr = ffmpeg_next::ffi::avfilter_graph_alloc();
            NonNull::new(ptr).map(|ptr| Self { ptr })
        }
    }

    /// Get the raw pointer.
    pub fn as_ptr(&self) -> *mut ffmpeg_next::ffi::AVFilterGraph {
        self.ptr.as_ptr()
    }

    /// Get a mutable pointer for freeing.
    pub fn as_mut_ptr(&mut self) -> *mut *mut ffmpeg_next::ffi::AVFilterGraph {
        &mut self.ptr.as_ptr()
    }
}

impl Drop for AvFilterGraph {
    fn drop(&mut self) {
        unsafe {
            let mut ptr = self.ptr.as_ptr();
            ffmpeg_next::ffi::avfilter_graph_free(&mut ptr);
        }
    }
}

unsafe impl Send for AvFilterGraph {}
unsafe impl Sync for AvFilterGraph {}

impl Default for AvFilterGraph {
    fn default() -> Self {
        Self::new().expect("Failed to allocate AVFilterGraph")
    }
}

/// Get FFmpeg error string from error code.
pub fn ffmpeg_error_str(errnum: i32) -> String {
    let mut buf = [0u8; 256];
    unsafe {
        ffmpeg_next::ffi::av_strerror(
            errnum,
            buf.as_mut_ptr() as *mut i8,
            buf.len(),
        );
    }
    let c_str = std::ffi::CStr::from_bytes_until_nul(&buf)
        .unwrap_or_default();
    c_str.to_string_lossy().to_string()
}

/// Check FFmpeg return code and return error if negative.
pub fn ffmpeg_result(code: i32) -> Result<i32, crate::GpuError> {
    if code < 0 {
        Err(crate::GpuError::NvDecError(ffmpeg_error_str(code)))
    } else {
        Ok(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffmpeg_error_str() {
        // AVERROR(EINVAL) = -22
        let err_str = ffmpeg_error_str(-22);
        assert!(!err_str.is_empty());
    }

    #[test]
    fn test_ffmpeg_result_ok() {
        assert_eq!(ffmpeg_result(0).unwrap(), 0);
        assert_eq!(ffmpeg_result(100).unwrap(), 100);
    }

    #[test]
    fn test_ffmpeg_result_err() {
        let result = ffmpeg_result(-22);
        assert!(result.is_err());
    }
}
