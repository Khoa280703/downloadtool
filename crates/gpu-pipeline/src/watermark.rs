//! Watermark overlay using CUDA filters
//!
//! This module provides GPU-accelerated watermark overlay functionality
//! using FFmpeg's overlay_cuda filter.

use std::path::Path;
use bytes::Bytes;
use tracing::{debug, error, info, warn};

use crate::{GpuError, Resolution};

/// Watermark position on the video.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatermarkPosition {
    /// Top-left corner
    TopLeft,
    /// Top-right corner
    TopRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-right corner (default)
    BottomRight,
    /// Center
    Center,
    /// Custom position (x, y)
    Custom(u32, u32),
}

impl WatermarkPosition {
    /// Get the overlay filter expression for this position.
    ///
    /// Returns x and y coordinates as FFmpeg expressions.
    pub fn to_filter_expr(&self, margin: u32) -> (String, String) {
        match self {
            WatermarkPosition::TopLeft => {
                (margin.to_string(), margin.to_string())
            }
            WatermarkPosition::TopRight => {
                (format!("W-w-{}", margin), margin.to_string())
            }
            WatermarkPosition::BottomLeft => {
                (margin.to_string(), format!("H-h-{}", margin))
            }
            WatermarkPosition::BottomRight => {
                (format!("W-w-{}", margin), format!("H-h-{}", margin))
            }
            WatermarkPosition::Center => {
                ("(W-w)/2".to_string(), "(H-h)/2".to_string())
            }
            WatermarkPosition::Custom(x, y) => {
                (x.to_string(), y.to_string())
            }
        }
    }
}

impl Default for WatermarkPosition {
    fn default() -> Self {
        WatermarkPosition::BottomRight
    }
}

/// Watermark configuration.
#[derive(Debug, Clone)]
pub struct WatermarkConfig {
    /// Path to the logo image (PNG recommended)
    pub logo_path: String,
    /// Position on the video
    pub position: WatermarkPosition,
    /// Margin from edges in pixels
    pub margin: u32,
    /// Opacity (0.0 - 1.0)
    pub opacity: f32,
    /// Scale factor relative to video width (0.0 - 1.0)
    pub scale: f32,
}

impl Default for WatermarkConfig {
    fn default() -> Self {
        Self {
            logo_path: String::new(),
            position: WatermarkPosition::default(),
            margin: 10,
            opacity: 0.8,
            scale: 0.15,
        }
    }
}

/// GPU-accelerated watermark overlay.
///
/// Loads a logo image into GPU memory and applies it to video frames
/// using the overlay_cuda filter.
pub struct Watermark {
    config: WatermarkConfig,
    logo_data: Option<Bytes>,
    logo_resolution: Option<Resolution>,
}

impl Watermark {
    /// Create a new watermark instance.
    ///
    /// The logo is not loaded until `load()` is called.
    pub fn new(config: WatermarkConfig) -> Self {
        Self {
            config,
            logo_data: None,
            logo_resolution: None,
        }
    }

    /// Load the logo image into memory.
    ///
    /// This should be called once at startup. The logo data will be
    /// uploaded to GPU memory when processing begins.
    ///
    /// # Errors
    /// Returns an error if the logo file cannot be read.
    pub fn load(&mut self) -> Result<(), GpuError> {
        let path = Path::new(&self.config.logo_path);

        if !path.exists() {
            return Err(GpuError::InvalidFormat(
                format!("Logo file not found: {}", self.config.logo_path)
            ));
        }

        info!("Loading watermark logo from: {}", self.config.logo_path);

        // Read logo file into memory
        match std::fs::read(path) {
            Ok(data) => {
                let bytes = Bytes::from(data);
                debug!("Loaded logo: {} bytes", bytes.len());

                // Parse image dimensions (basic PNG/JPEG detection)
                self.logo_resolution = Self::detect_image_resolution(&bytes);

                if let Some(res) = self.logo_resolution {
                    debug!("Logo resolution: {}x{}", res.width, res.height);
                }

                self.logo_data = Some(bytes);
                Ok(())
            }
            Err(e) => {
                error!("Failed to load logo: {}", e);
                Err(GpuError::InvalidFormat(
                    format!("Failed to read logo file: {}", e)
                ))
            }
        }
    }

    /// Detect image resolution from file bytes.
    /// Supports PNG and JPEG formats.
    fn detect_image_resolution(data: &[u8]) -> Option<Resolution> {
        // Check for PNG signature
        if data.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
            // PNG: width is at bytes 16-19, height at 20-23 (big-endian)
            if data.len() >= 24 {
                let width = u32::from_be_bytes([
                    data[16], data[17], data[18], data[19]
                ]);
                let height = u32::from_be_bytes([
                    data[20], data[21], data[22], data[23]
                ]);
                return Some(Resolution { width, height });
            }
        }

        // Check for JPEG SOI marker
        if data.starts_with(&[0xFF, 0xD8]) {
            // JPEG: scan for SOF0-SOF3 markers
            let mut i = 2;
            while i < data.len() - 1 {
                if data[i] == 0xFF {
                    match data[i + 1] {
                        0xC0 | 0xC1 | 0xC2 | 0xC3 => {
                            // SOF markers: height at i+5,6; width at i+7,8 (big-endian)
                            if data.len() >= i + 9 {
                                let height = u16::from_be_bytes([
                                    data[i + 5], data[i + 6]
                                ]) as u32;
                                let width = u16::from_be_bytes([
                                    data[i + 7], data[i + 8]
                                ]) as u32;
                                return Some(Resolution { width, height });
                            }
                        }
                        0xD9 => break, // EOI marker
                        _ => {}
                    }
                }
                i += 1;
            }
        }

        None
    }

    /// Get the overlay filter string for FFmpeg.
    ///
    /// This generates the filter graph string for overlay_cuda.
    pub fn get_filter_string(&self, video_width: u32, video_height: u32) -> String {
        let (x, y) = self.config.position.to_filter_expr(self.config.margin);

        // Calculate scaled logo dimensions
        let scale = self.config.scale;
        let scaled_width = (video_width as f32 * scale) as u32;

        format!(
            "[1:v]scale={}:{}:force_original_aspect_ratio=decrease[logo]; \
             [0:v][logo]overlay_cuda=x={}:y={}:alpha={}",
            scaled_width,
            scaled_width, // height will be calculated to maintain aspect ratio
            x, y,
            self.config.opacity
        )
    }

    /// Check if the watermark is loaded and ready.
    pub fn is_loaded(&self) -> bool {
        self.logo_data.is_some()
    }

    /// Get the logo data if loaded.
    pub fn logo_data(&self) -> Option<&Bytes> {
        self.logo_data.as_ref()
    }

    /// Get the logo resolution if detected.
    pub fn logo_resolution(&self) -> Option<&Resolution> {
        self.logo_resolution.as_ref()
    }

    /// Get the watermark configuration.
    pub fn config(&self) -> &WatermarkConfig {
        &self.config
    }

    /// Create a watermark from a logo path with default settings.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        Self::new(WatermarkConfig {
            logo_path: path.as_ref().to_string_lossy().to_string(),
            ..Default::default()
        })
    }
}

/// Watermark overlay processor that applies the watermark to frames.
///
/// This is a higher-level interface that manages the filter graph
/// and applies watermarks to decoded frames.
pub struct WatermarkProcessor {
    watermark: Watermark,
    filter_graph_initialized: bool,
}

impl WatermarkProcessor {
    /// Create a new watermark processor.
    pub fn new(watermark: Watermark) -> Self {
        Self {
            watermark,
            filter_graph_initialized: false,
        }
    }

    /// Initialize the filter graph for processing.
    ///
    /// This must be called before processing frames.
    pub fn initialize(&mut self, video_width: u32, video_height: u32) -> Result<(), GpuError> {
        if !self.watermark.is_loaded() {
            return Err(GpuError::InvalidFormat(
                "Watermark not loaded".to_string()
            ));
        }

        debug!(
            "Initializing watermark filter graph for {}x{}",
            video_width, video_height
        );

        let _filter_str = self.watermark.get_filter_string(video_width, video_height);
        debug!("Filter graph: {}", _filter_str);

        // Actual filter graph initialization will be done with FFmpeg
        self.filter_graph_initialized = true;

        info!("Watermark processor initialized");
        Ok(())
    }

    /// Check if the processor is initialized.
    pub fn is_initialized(&self) -> bool {
        self.filter_graph_initialized
    }

    /// Get the watermark instance.
    pub fn watermark(&self) -> &Watermark {
        &self.watermark
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watermark_position_expr() {
        let pos = WatermarkPosition::TopLeft;
        let (x, y) = pos.to_filter_expr(10);
        assert_eq!(x, "10");
        assert_eq!(y, "10");

        let pos = WatermarkPosition::BottomRight;
        let (x, y) = pos.to_filter_expr(10);
        assert_eq!(x, "W-w-10");
        assert_eq!(y, "H-h-10");

        let pos = WatermarkPosition::Center;
        let (x, y) = pos.to_filter_expr(0);
        assert_eq!(x, "(W-w)/2");
        assert_eq!(y, "(H-h)/2");
    }

    #[test]
    fn test_detect_png_resolution() {
        // Create a minimal PNG header
        let mut png_data = vec![0x89, 0x50, 0x4E, 0x47]; // PNG signature
        png_data.extend(vec![0u8; 12]); // IHDR chunk header
        // Width: 1920 (big-endian)
        png_data.push(0x00);
        png_data.push(0x00);
        png_data.push(0x07);
        png_data.push(0x80);
        // Height: 1080 (big-endian)
        png_data.push(0x00);
        png_data.push(0x00);
        png_data.push(0x04);
        png_data.push(0x38);

        let res = Watermark::detect_image_resolution(&png_data);
        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(res.width, 1920);
        assert_eq!(res.height, 1080);
    }

    #[test]
    fn test_watermark_config_default() {
        let config = WatermarkConfig::default();
        assert_eq!(config.position, WatermarkPosition::BottomRight);
        assert_eq!(config.margin, 10);
        assert_eq!(config.opacity, 0.8);
        assert_eq!(config.scale, 0.15);
    }

    #[test]
    fn test_get_filter_string() {
        let watermark = Watermark::new(WatermarkConfig {
            logo_path: "/tmp/logo.png".to_string(),
            position: WatermarkPosition::BottomRight,
            margin: 10,
            opacity: 0.8,
            scale: 0.15,
        });

        let filter = watermark.get_filter_string(1920, 1080);
        assert!(filter.contains("overlay_cuda"));
        assert!(filter.contains("scale="));
        assert!(filter.contains("alpha=0.8"));
    }
}
