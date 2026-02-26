//! Video extraction handler
//!
//! POST /api/extract - Extract video information from a URL

use axum::{extract::Json as ExtractJson, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

use extractor::VideoFormat;

/// Request body for video extraction.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct ExtractRequest {
    /// URL of the video to extract
    pub url: String,
    /// Optional quality preference (e.g., "1080p", "720p")
    pub quality: Option<String>,
    /// Optional format preference (e.g., "mp4", "webm")
    pub format: Option<String>,
}

/// Response for video extraction.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ExtractResponse {
    /// Status of the extraction
    pub status: String,
    /// Video metadata if successful
    pub metadata: Option<VideoMetadata>,
    /// Selected stream URL (best match for requested quality)
    pub selected_stream_url: Option<String>,
    /// Error message if extraction failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Video metadata structure.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct VideoMetadata {
    /// Video title
    pub title: String,
    /// Channel/author name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// View count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_count: Option<u64>,
    /// Video description (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,
    /// Thumbnail URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    /// Available formats
    pub formats: Vec<StreamFormat>,
    /// Original URL
    pub original_url: String,
}

/// Stream format information for response.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct StreamFormat {
    /// Format ID
    pub format_id: String,
    /// Quality label (e.g., "1080p (video only)", "Audio 128kbps")
    pub quality: String,
    /// File extension
    pub ext: String,
    /// Direct stream URL
    pub url: String,
    /// Whether stream has an audio track
    pub has_audio: bool,
    /// Whether stream is audio-only
    pub is_audio_only: bool,
    /// Human-readable codec label (e.g., "H.264", "VP9", "AV1", "AAC")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_label: Option<String>,
    /// Bitrate in bits per second
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u64>,
    /// File size in bytes (if known)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesize: Option<u64>,
}

/// API error response.
#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
    pub status: u16,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = axum::Json(self);
        (status, body).into_response()
    }
}

/// Extract video metadata endpoint.
///
/// POST /api/extract
/// Body: { url: string, quality?: string, format?: string }
///
/// Validates URL (youtube.com only), calls extractor,
/// and returns stream list + recommended stream URL.
pub async fn extract_handler(
    ExtractJson(body): ExtractJson<ExtractRequest>,
) -> Result<Json<ExtractResponse>, ApiError> {
    info!("Extracting video from URL: {}", body.url);

    // Validate URL against allowed platforms
    if !is_valid_video_url(&body.url) {
        warn!("Invalid or unsupported URL: {}", body.url);
        return Err(ApiError {
            error: "Invalid or unsupported URL. Only YouTube URLs are supported.".to_string(),
            status: 400,
        });
    }

    // Call extractor
    match extractor::extract(&body.url, None).await {
        Ok(video_info) => {
            info!("Successfully extracted video: {}", video_info.title);

            // Convert to response format
            let metadata = VideoMetadata {
                title: video_info.title.clone(),
                channel: video_info.channel,
                view_count: video_info.view_count,
                description: video_info.description,
                duration: video_info.duration,
                thumbnail: video_info.thumbnail,
                formats: video_info.formats.iter().map(convert_format).collect(),
                original_url: video_info.original_url,
            };

            // Select best stream based on quality preference
            let selected_stream_url = select_best_stream(
                &video_info.formats,
                body.quality.as_deref(),
                body.format.as_deref(),
            );

            let response = ExtractResponse {
                status: "success".to_string(),
                metadata: Some(metadata),
                selected_stream_url,
                error: None,
            };

            Ok(Json(response))
        }
        Err(e) => {
            error!("Extraction failed: {}", e);
            Err(ApiError {
                error: format!("Extraction failed: {}", e),
                status: 500,
            })
        }
    }
}

/// Check if URL is a valid YouTube URL.
fn is_valid_video_url(url: &str) -> bool {
    let url_lower = url.to_lowercase();
    url_lower.contains("youtube.com") || url_lower.contains("youtu.be")
}

/// Convert VideoFormat to StreamFormat for response.
fn convert_format(format: &VideoFormat) -> StreamFormat {
    StreamFormat {
        format_id: format.format_id.clone(),
        quality: format.quality.clone(),
        ext: format.ext.clone(),
        url: format.url.clone(),
        has_audio: format.has_audio,
        is_audio_only: format.is_audio_only,
        codec_label: format.codec_label.clone(),
        bitrate: format.bitrate,
        filesize: format.filesize,
    }
}

/// Select the best stream URL based on quality preference.
fn select_best_stream(
    formats: &[VideoFormat],
    quality_pref: Option<&str>,
    format_pref: Option<&str>,
) -> Option<String> {
    if formats.is_empty() {
        return None;
    }

    // Optional extension preference (e.g., mp4/webm)
    let filtered: Vec<&VideoFormat> = if let Some(pref) = format_pref {
        let ext = pref.trim_start_matches('.').to_lowercase();
        let list: Vec<&VideoFormat> = formats
            .iter()
            .filter(|f| f.ext.eq_ignore_ascii_case(&ext))
            .collect();
        if list.is_empty() {
            formats.iter().collect()
        } else {
            list
        }
    } else {
        formats.iter().collect()
    };

    // If quality preference is specified, try to match it
    if let Some(pref) = quality_pref {
        let pref_lower = pref.to_lowercase();
        // Extract numeric part (e.g., "1080p" -> 1080)
        let target_height: u32 = pref_lower
            .trim_end_matches('p')
            .parse()
            .unwrap_or(1080);

        // Find closest match
        let best_match = filtered
            .iter()
            .filter(|f| f.height.is_some())
            .min_by_key(|f| f.height.unwrap_or(0).abs_diff(target_height));

        if let Some(f) = best_match {
            return Some(f.url.clone());
        }
    }

    // Default: return the format with highest resolution
    filtered
        .iter()
        .filter(|f| f.height.is_some())
        .max_by_key(|f| f.height.unwrap_or(0))
        .map(|f| f.url.clone())
        .or_else(|| filtered.first().map(|f| f.url.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_video_url() {
        assert!(is_valid_video_url("https://youtube.com/watch?v=abc123"));
        assert!(is_valid_video_url("https://youtu.be/abc123"));
        assert!(!is_valid_video_url("https://vimeo.com/123456"));
        assert!(!is_valid_video_url("https://example.com/video"));
    }

    #[test]
    fn test_select_best_stream() {
        let formats = vec![
            VideoFormat {
                format_id: "1".to_string(),
                quality: "1080p (video only)".to_string(),
                vcodec: Some("avc1".to_string()),
                acodec: None,
                codec_label: Some("H.264".to_string()),
                has_audio: false,
                is_audio_only: false,
                width: Some(1920),
                height: Some(1080),
                fps: None,
                bitrate: None,
                ext: "mp4".to_string(),
                url: "http://example.com/1080".to_string(),
                filesize: None,
            },
            VideoFormat {
                format_id: "2".to_string(),
                quality: "720p (video only)".to_string(),
                vcodec: Some("avc1".to_string()),
                acodec: None,
                codec_label: Some("H.264".to_string()),
                has_audio: false,
                is_audio_only: false,
                width: Some(1280),
                height: Some(720),
                fps: None,
                bitrate: None,
                ext: "mp4".to_string(),
                url: "http://example.com/720".to_string(),
                filesize: None,
            },
        ];

        // Should return 1080p for "1080p" preference
        assert_eq!(
            select_best_stream(&formats, Some("1080p"), None),
            Some("http://example.com/1080".to_string())
        );

        // Should return 720p for "720p" preference
        assert_eq!(
            select_best_stream(&formats, Some("720p"), None),
            Some("http://example.com/720".to_string())
        );

        // Should return highest resolution without preference
        assert_eq!(
            select_best_stream(&formats, None, None),
            Some("http://example.com/1080".to_string())
        );
    }
}
