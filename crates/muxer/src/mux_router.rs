//! Mux routing decision logic
//!
//! Determines whether to proxy a stream directly or mux audio+video together.

use extractor::types::{VideoFormat, VideoInfo};

/// Source type for stream delivery.
#[derive(Debug, Clone)]
pub enum StreamSource {
    /// Direct single-stream proxy (audio+video already muxed).
    Direct {
        /// The direct stream URL.
        url: String,
        /// Video format information.
        format: VideoFormat,
    },
    /// Separate audio and video streams that need muxing.
    Mux {
        /// Video stream URL.
        video_url: String,
        /// Audio stream URL.
        audio_url: String,
        /// Video format information.
        video_format: VideoFormat,
        /// Audio format information.
        audio_format: VideoFormat,
    },
}

/// Router that decides how to handle video streams.
pub struct MuxRouter;

impl MuxRouter {
    /// Route a video extraction result to determine stream handling strategy.
    ///
    /// # Arguments
    /// * `video_info` - The extracted video information.
    /// * `format_id` - Optional specific format ID to use.
    ///
    /// # Returns
    /// `StreamSource` indicating how to handle the stream.
    pub fn route(video_info: &VideoInfo, format_id: Option<&str>) -> Option<StreamSource> {
        // If a specific format is requested, try to use it directly
        if let Some(fid) = format_id {
            if let Some(format) = video_info.formats.iter().find(|f| f.format_id == fid) {
                // Check if this format has both audio and video
                let has_video = format.vcodec.as_ref().map(|c| !c.is_empty() && c != "none").unwrap_or(false);
                let has_audio = format.acodec.as_ref().map(|c| !c.is_empty() && c != "none").unwrap_or(false);

                if has_video && has_audio {
                    // Format already has both - direct proxy
                    return Some(StreamSource::Direct {
                        url: format.url.clone(),
                        format: format.clone(),
                    });
                }
            }
        }

        // Find best video-only and audio-only formats
        let video_format = Self::find_best_video_format(&video_info.formats);
        let audio_format = Self::find_best_audio_format(&video_info.formats);

        match (video_format, audio_format) {
            (Some(video), Some(audio)) => {
                // Separate streams - need muxing
                Some(StreamSource::Mux {
                    video_url: video.url.clone(),
                    audio_url: audio.url.clone(),
                    video_format: video.clone(),
                    audio_format: audio.clone(),
                })
            }
            (Some(video), None) => {
                // Video only - direct proxy
                Some(StreamSource::Direct {
                    url: video.url.clone(),
                    format: video.clone(),
                })
            }
            (None, Some(audio)) => {
                // Audio only - direct proxy
                Some(StreamSource::Direct {
                    url: audio.url.clone(),
                    format: audio.clone(),
                })
            }
            (None, None) => None,
        }
    }

    /// Check if muxing is needed for the given formats.
    ///
    /// Returns true if separate audio and video streams are detected.
    pub fn needs_mux(formats: &[VideoFormat]) -> bool {
        let has_video_only = formats.iter().any(|f| {
            let has_video = f.vcodec.as_ref().map(|c| !c.is_empty() && c != "none").unwrap_or(false);
            let has_audio = f.acodec.as_ref().map(|c| !c.is_empty() && c != "none").unwrap_or(false);
            has_video && !has_audio
        });

        let has_audio_only = formats.iter().any(|f| {
            let has_video = f.vcodec.as_ref().map(|c| !c.is_empty() && c != "none").unwrap_or(false);
            let has_audio = f.acodec.as_ref().map(|c| !c.is_empty() && c != "none").unwrap_or(false);
            !has_video && has_audio
        });

        has_video_only && has_audio_only
    }

    /// Find the best video-only format (highest quality).
    fn find_best_video_format(formats: &[VideoFormat]) -> Option<&VideoFormat> {
        formats
            .iter()
            .filter(|f| {
                let has_video = f.vcodec.as_ref().map(|c| !c.is_empty() && c != "none").unwrap_or(false);
                let has_audio = f.acodec.as_ref().map(|c| !c.is_empty() && c != "none").unwrap_or(false);
                has_video && !has_audio
            })
            .max_by_key(|f| {
                // Score by resolution and bitrate
                let height_score = f.height.unwrap_or(0);
                let bitrate_score = f.bitrate.unwrap_or(0) / 1000;
                height_score + bitrate_score as u32
            })
    }

    /// Find the best audio-only format (highest quality).
    fn find_best_audio_format(formats: &[VideoFormat]) -> Option<&VideoFormat> {
        formats
            .iter()
            .filter(|f| {
                let has_video = f.vcodec.as_ref().map(|c| !c.is_empty() && c != "none").unwrap_or(false);
                let has_audio = f.acodec.as_ref().map(|c| !c.is_empty() && c != "none").unwrap_or(false);
                !has_video && has_audio
            })
            .max_by_key(|f| f.bitrate.unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_video_format(id: &str, vcodec: Option<&str>, acodec: Option<&str>, height: Option<u32>, bitrate: Option<u64>) -> VideoFormat {
        let is_audio_only = acodec.is_some() && vcodec.is_none();
        let has_audio = acodec.is_some() || vcodec.map(|v| v.contains("mp4a")).unwrap_or(false);
        VideoFormat {
            format_id: id.to_string(),
            quality: height.map(|h| format!("{}p", h)).unwrap_or_else(|| "Audio".to_string()),
            vcodec: vcodec.map(|s| s.to_string()),
            acodec: acodec.map(|s| s.to_string()),
            codec_label: None,
            has_audio,
            is_audio_only,
            width: height.map(|h| (h as f32 * 16.0 / 9.0) as u32),
            height,
            fps: Some(30.0),
            bitrate,
            ext: "mp4".to_string(),
            url: format!("https://example.com/{}" , id),
            filesize: None,
        }
    }

    fn create_video_info(formats: Vec<VideoFormat>) -> VideoInfo {
        VideoInfo {
            title: "Test Video".to_string(),
            description: None,
            duration: Some(120),
            thumbnail: None,
            formats,
            original_url: "https://example.com/video".to_string(),
        }
    }

    #[test]
    fn test_needs_mux_with_separate_streams() {
        let formats = vec![
            create_video_format("137", Some("avc1.640028"), Some("none"), Some(1080), Some(5000000)),
            create_video_format("140", Some("none"), Some("mp4a.40.2"), None, Some(128000)),
        ];

        assert!(MuxRouter::needs_mux(&formats));
    }

    #[test]
    fn test_needs_mux_with_combined_stream() {
        let formats = vec![
            create_video_format("18", Some("avc1.42001E"), Some("mp4a.40.2"), Some(360), Some(500000)),
        ];

        assert!(!MuxRouter::needs_mux(&formats));
    }

    #[test]
    fn test_route_with_separate_streams() {
        let formats = vec![
            create_video_format("137", Some("avc1.640028"), None, Some(1080), Some(5000000)),
            create_video_format("140", None, Some("mp4a.40.2"), None, Some(128000)),
        ];
        let info = create_video_info(formats);

        let source = MuxRouter::route(&info, None);

        match source {
            Some(StreamSource::Mux { video_url, audio_url, .. }) => {
                assert!(video_url.contains("137"));
                assert!(audio_url.contains("140"));
            }
            _ => panic!("Expected Mux source"),
        }
    }

    #[test]
    fn test_route_with_combined_stream() {
        let formats = vec![
            create_video_format("18", Some("avc1.42001E"), Some("mp4a.40.2"), Some(360), Some(500000)),
        ];
        let info = create_video_info(formats);

        let source = MuxRouter::route(&info, Some("18"));

        match source {
            Some(StreamSource::Direct { url, .. }) => {
                assert!(url.contains("18"));
            }
            _ => panic!("Expected Direct source"),
        }
    }

    #[test]
    fn test_find_best_video_format() {
        let formats = vec![
            create_video_format("136", Some("avc1.64001F"), None, Some(720), Some(2500000)),
            create_video_format("137", Some("avc1.640028"), None, Some(1080), Some(5000000)),
            create_video_format("135", Some("avc1.64001E"), None, Some(480), Some(1000000)),
        ];

        let best = MuxRouter::find_best_video_format(&formats);
        assert_eq!(best.unwrap().format_id, "137");
    }

    #[test]
    fn test_find_best_audio_format() {
        let formats = vec![
            create_video_format("139", None, Some("mp4a.40.5"), None, Some(48000)),
            create_video_format("140", None, Some("mp4a.40.2"), None, Some(128000)),
            create_video_format("141", None, Some("mp4a.40.2"), None, Some(256000)),
        ];

        let best = MuxRouter::find_best_audio_format(&formats);
        assert_eq!(best.unwrap().format_id, "141");
    }
}
