//! Codec detection and representation
//!
//! Provides codec enum and MIME type parsing for video/audio formats.

/// Supported video and audio codecs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Codec {
    /// H.264/AVC video codec
    H264,
    /// H.265/HEVC video codec
    H265,
    /// VP9 video codec
    VP9,
    /// AV1 video codec
    AV1,
    /// AAC audio codec
    AAC,
    /// Opus audio codec
    Opus,
}

impl Codec {
    /// Parse codec from MIME type string.
    ///
    /// Supports common MIME type formats like:
    /// - "video/mp4; codecs=avc1.42E01E"
    /// - "video/webm; codecs=vp9"
    /// - "audio/mp4; codecs=mp4a.40.2"
    /// - "audio/webm; codecs=opus"
    ///
    /// # Arguments
    /// * `mime` - The MIME type string to parse
    ///
    /// # Returns
    /// Some(Codec) if recognized, None otherwise
    pub fn from_mime(mime: &str) -> Option<Self> {
        let mime_lower = mime.to_lowercase();

        // Check for video codecs
        if mime_lower.contains("avc1") || mime_lower.contains("h264") {
            return Some(Codec::H264);
        }
        if mime_lower.contains("hev1") || mime_lower.contains("hvc1") || mime_lower.contains("h265") || mime_lower.contains("hevc") {
            return Some(Codec::H265);
        }
        if mime_lower.contains("vp9") {
            return Some(Codec::VP9);
        }
        if mime_lower.contains("av01") || mime_lower.contains("av1") {
            return Some(Codec::AV1);
        }

        // Check for audio codecs
        if mime_lower.contains("mp4a") || mime_lower.contains("aac") {
            return Some(Codec::AAC);
        }
        if mime_lower.contains("opus") {
            return Some(Codec::Opus);
        }

        None
    }

    /// Parse codec from codec string (e.g., "avc1.64001F", "vp9", "opus")
    ///
    /// # Arguments
    /// * `codec_str` - The codec string to parse
    ///
    /// # Returns
    /// Some(Codec) if recognized, None otherwise
    pub fn from_string(codec_str: &str) -> Option<Self> {
        let lower = codec_str.to_lowercase();

        if lower.starts_with("avc1") || lower == "h264" {
            return Some(Codec::H264);
        }
        if lower.starts_with("hev1") || lower.starts_with("hvc1") || lower == "h265" || lower == "hevc" {
            return Some(Codec::H265);
        }
        if lower.contains("vp9") || lower == "vp09" {
            return Some(Codec::VP9);
        }
        if lower.starts_with("av01") || lower == "av1" {
            return Some(Codec::AV1);
        }
        if lower.starts_with("mp4a") || lower == "aac" {
            return Some(Codec::AAC);
        }
        if lower == "opus" {
            return Some(Codec::Opus);
        }

        None
    }

    /// Check if this is a video codec.
    pub fn is_video(&self) -> bool {
        matches!(self, Codec::H264 | Codec::H265 | Codec::VP9 | Codec::AV1)
    }

    /// Check if this is an audio codec.
    pub fn is_audio(&self) -> bool {
        matches!(self, Codec::AAC | Codec::Opus)
    }

    /// Get the codec name as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Codec::H264 => "h264",
            Codec::H265 => "h265",
            Codec::VP9 => "vp9",
            Codec::AV1 => "av1",
            Codec::AAC => "aac",
            Codec::Opus => "opus",
        }
    }

    /// Get the MIME type for this codec.
    pub fn mime_type(&self) -> &'static str {
        match self {
            Codec::H264 => "video/mp4",
            Codec::H265 => "video/mp4",
            Codec::VP9 => "video/webm",
            Codec::AV1 => "video/mp4",
            Codec::AAC => "audio/mp4",
            Codec::Opus => "audio/webm",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codec_from_mime() {
        // H264
        assert_eq!(Codec::from_mime("video/mp4; codecs=avc1.42E01E"), Some(Codec::H264));
        assert_eq!(Codec::from_mime("video/mp4; codecs=avc1.64001F"), Some(Codec::H264));

        // H265
        assert_eq!(Codec::from_mime("video/mp4; codecs=hev1.1.6.L93.B0"), Some(Codec::H265));
        assert_eq!(Codec::from_mime("video/mp4; codecs=hvc1.1.6.L93.B0"), Some(Codec::H265));

        // VP9
        assert_eq!(Codec::from_mime("video/webm; codecs=vp9"), Some(Codec::VP9));

        // AV1
        assert_eq!(Codec::from_mime("video/mp4; codecs=av01.0.04M.08"), Some(Codec::AV1));

        // AAC
        assert_eq!(Codec::from_mime("audio/mp4; codecs=mp4a.40.2"), Some(Codec::AAC));

        // Opus
        assert_eq!(Codec::from_mime("audio/webm; codecs=opus"), Some(Codec::Opus));

        // Unknown
        assert_eq!(Codec::from_mime("video/mp4; codecs=unknown"), None);
    }

    #[test]
    fn test_codec_from_string() {
        assert_eq!(Codec::from_string("avc1.64001F"), Some(Codec::H264));
        assert_eq!(Codec::from_string("h264"), Some(Codec::H264));
        assert_eq!(Codec::from_string("vp9"), Some(Codec::VP9));
        assert_eq!(Codec::from_string("opus"), Some(Codec::Opus));
        assert_eq!(Codec::from_string("unknown"), None);
    }

    #[test]
    fn test_codec_properties() {
        assert!(Codec::H264.is_video());
        assert!(Codec::H265.is_video());
        assert!(Codec::VP9.is_video());
        assert!(Codec::AV1.is_video());
        assert!(!Codec::H264.is_audio());

        assert!(Codec::AAC.is_audio());
        assert!(Codec::Opus.is_audio());
        assert!(!Codec::AAC.is_video());
    }

    #[test]
    fn test_codec_as_str() {
        assert_eq!(Codec::H264.as_str(), "h264");
        assert_eq!(Codec::VP9.as_str(), "vp9");
        assert_eq!(Codec::Opus.as_str(), "opus");
    }
}
