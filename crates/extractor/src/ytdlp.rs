//! yt-dlp subprocess extractor
//!
//! Calls `yt-dlp -J --no-playlist` to get video metadata + stream URLs.
//! yt-dlp handles PO Token, signature decryption, and throttle bypass automatically.

use crate::types::{ExtractionError, VideoFormat, VideoInfo};
use moka::future::Cache;
use serde_json::Value;
use std::process::Stdio;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::process::Command;
use tokio::sync::Semaphore;
use tracing::{debug, warn};

const MAX_CONCURRENT_YTDLP: usize = 10;
const EXTRACT_CACHE_MAX_CAPACITY: u64 = 500;
const EXTRACT_CACHE_TTL_SECONDS: u64 = 300;

static YTDLP_SEMAPHORE: OnceLock<Arc<Semaphore>> = OnceLock::new();
static EXTRACT_CACHE: OnceLock<Cache<String, Arc<VideoInfo>>> = OnceLock::new();

fn get_semaphore() -> &'static Arc<Semaphore> {
    YTDLP_SEMAPHORE.get_or_init(|| Arc::new(Semaphore::new(MAX_CONCURRENT_YTDLP)))
}

fn get_cache() -> &'static Cache<String, Arc<VideoInfo>> {
    EXTRACT_CACHE.get_or_init(|| {
        Cache::builder()
            .max_capacity(EXTRACT_CACHE_MAX_CAPACITY)
            .time_to_live(Duration::from_secs(EXTRACT_CACHE_TTL_SECONDS))
            .build()
    })
}

fn resolve_ytdlp_binary() -> String {
    std::env::var("YTDLP_PATH")
        .ok()
        .filter(|path| !path.trim().is_empty())
        .unwrap_or_else(|| "yt-dlp".to_string())
}

/// Normalize YouTube URL to canonical cache key.
/// Falls back to the raw URL if no standard video ID is found.
fn normalize_cache_key(url: &str) -> String {
    if let Some(video_id) = extract_video_id(url) {
        return format!("https://www.youtube.com/watch?v={video_id}");
    }

    url.to_string()
}

fn extract_video_id(url: &str) -> Option<&str> {
    if let Some(v_pos) = url.find("v=") {
        let id_start = v_pos + 2;
        let rest = &url[id_start..];
        let id_end = rest
            .find(['&', '#'])
            .map(|idx| id_start + idx)
            .unwrap_or(url.len());
        let video_id = &url[id_start..id_end];
        if video_id.len() == 11 {
            return Some(video_id);
        }
    }

    if let Some(path_start) = url.find("youtu.be/") {
        let id_start = path_start + "youtu.be/".len();
        let rest = &url[id_start..];
        let id_end = rest
            .find(['?', '&', '#', '/'])
            .map(|idx| id_start + idx)
            .unwrap_or(url.len());
        let video_id = &url[id_start..id_end];
        if video_id.len() == 11 {
            return Some(video_id);
        }
    }

    if let Some(path_start) = url.find("/shorts/") {
        let id_start = path_start + "/shorts/".len();
        let rest = &url[id_start..];
        let id_end = rest
            .find(['?', '&', '#', '/'])
            .map(|idx| id_start + idx)
            .unwrap_or(url.len());
        let video_id = &url[id_start..id_end];
        if video_id.len() == 11 {
            return Some(video_id);
        }
    }

    None
}

/// Extract video info via `yt-dlp -J --no-playlist`
pub async fn extract_via_ytdlp(url: &str) -> Result<VideoInfo, ExtractionError> {
    debug!("yt-dlp extracting: {}", url);

    let cache_key = normalize_cache_key(url);

    get_cache()
        .try_get_with(cache_key, extract_subprocess(url.to_string()))
        .await
        .map(|video_info| (*video_info).clone())
        .map_err(|error: Arc<ExtractionError>| (*error).clone())
}

async fn extract_subprocess(url: String) -> Result<Arc<VideoInfo>, ExtractionError> {
    let _permit = get_semaphore().acquire().await.map_err(|error| {
        ExtractionError::ScriptExecutionFailed(format!("semaphore acquire failed: {error}"))
    })?;

    let mut cmd = build_command(&url);
    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| ExtractionError::ScriptExecutionFailed(format!("yt-dlp launch failed: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!("yt-dlp failed for {}: {}", url, stderr.trim());
        return Err(ExtractionError::ScriptExecutionFailed(
            format!("yt-dlp error: {}", stderr.trim())
        ));
    }

    let json: Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| ExtractionError::ScriptExecutionFailed(format!("yt-dlp JSON parse error: {}", e)))?;

    let video_info = parse_ytdlp_json(json, &url)?;
    Ok(Arc::new(video_info))
}

/// Build the yt-dlp Command with appropriate flags
fn build_command(url: &str) -> Command {
    let mut cmd = Command::new(resolve_ytdlp_binary());
    cmd.args([
        "-J",                  // Dump JSON metadata to stdout
        "--no-playlist",       // Single video only (ignore playlist)
        "--no-warnings",       // Suppress non-fatal warnings
        "--extractor-args", "youtube:player_client=android_embedded,web",
        "--socket-timeout", "15",
        "--no-check-certificates",
    ]);

    // Route through SOCKS5 proxy if configured
    if let Ok(proxy) = std::env::var("SOCKS5_PROXY_URL") {
        if !proxy.is_empty() {
            cmd.args(["--proxy", &proxy]);
            debug!("yt-dlp routing through proxy: {}", proxy);
        }
    }

    cmd.arg(url);
    cmd
}

/// Parse yt-dlp `-J` JSON output into VideoInfo
fn parse_ytdlp_json(json: Value, original_url: &str) -> Result<VideoInfo, ExtractionError> {
    let obj = json.as_object().ok_or_else(|| {
        ExtractionError::ScriptExecutionFailed("yt-dlp output is not a JSON object".to_string())
    })?;

    let title = obj
        .get("title")
        .and_then(Value::as_str)
        .unwrap_or("Unknown")
        .to_string();

    let channel = obj
        .get("uploader")
        .or_else(|| obj.get("channel"))
        .and_then(Value::as_str)
        .map(String::from);

    let view_count = obj
        .get("view_count")
        .and_then(Value::as_u64);

    let description = obj
        .get("description")
        .and_then(Value::as_str)
        .map(String::from);

    let duration = obj
        .get("duration")
        .and_then(Value::as_f64)
        .map(|d| d as u64);

    let thumbnail = obj
        .get("thumbnail")
        .and_then(Value::as_str)
        .map(String::from);

    let formats_raw = obj
        .get("formats")
        .and_then(Value::as_array)
        .ok_or_else(|| ExtractionError::ScriptExecutionFailed("yt-dlp: no formats array".to_string()))?;

    let mut formats: Vec<VideoFormat> = formats_raw
        .iter()
        .enumerate()
        .filter_map(|(idx, fmt)| parse_format(fmt, idx))
        .collect();

    if formats.is_empty() {
        return Err(ExtractionError::ScriptExecutionFailed(
            "yt-dlp: no usable formats found".to_string(),
        ));
    }

    // Sort: highest resolution video first, then audio-only by bitrate
    formats.sort_by(|a, b| {
        b.height.unwrap_or(0).cmp(&a.height.unwrap_or(0))
    });

    Ok(VideoInfo {
        title,
        channel,
        view_count,
        description,
        duration,
        thumbnail,
        formats,
        original_url: original_url.to_string(),
    })
}

/// Parse a single yt-dlp format entry into VideoFormat
fn parse_format(fmt: &Value, idx: usize) -> Option<VideoFormat> {
    let obj = fmt.as_object()?;

    // Must have a direct URL (skip manifest/DASH formats without direct URL)
    let url = obj.get("url").and_then(Value::as_str)?;

    // Skip non-http protocols (m3u8, dash, etc.)
    let protocol = obj.get("protocol").and_then(Value::as_str).unwrap_or("https");
    if protocol.contains("m3u8") || protocol == "mhtml" {
        return None;
    }

    let format_id = obj
        .get("format_id")
        .and_then(Value::as_str)
        .unwrap_or(&format!("{}", idx))
        .to_string();

    let ext = obj
        .get("ext")
        .and_then(Value::as_str)
        .unwrap_or("mp4")
        .to_string();

    // Codec fields
    let vcodec = obj.get("vcodec").and_then(Value::as_str).unwrap_or("none");
    let acodec = obj.get("acodec").and_then(Value::as_str).unwrap_or("none");

    let has_video = vcodec != "none";
    let has_audio_codec = acodec != "none";

    let is_audio_only = !has_video && has_audio_codec;
    let has_audio = has_audio_codec;

    let width = obj.get("width").and_then(Value::as_u64).map(|v| v as u32);
    let height = obj.get("height").and_then(Value::as_u64).map(|v| v as u32);
    let fps = obj.get("fps").and_then(Value::as_f64).map(|v| v as f32);

    // Bitrate: prefer tbr (total), then vbr, then abr (in kbps → convert to bps)
    let bitrate = obj
        .get("tbr")
        .or_else(|| obj.get("vbr"))
        .or_else(|| obj.get("abr"))
        .and_then(Value::as_f64)
        .map(|kbps| (kbps * 1000.0) as u64);

    // Filesize: prefer exact, fallback to approx
    let filesize = obj
        .get("filesize")
        .and_then(Value::as_u64)
        .or_else(|| obj.get("filesize_approx").and_then(Value::as_u64));

    // Quality label
    let quality = build_quality_label(obj, height, is_audio_only, fps, bitrate);

    // Codec label
    let codec_label = if is_audio_only {
        codec_label_audio(acodec)
    } else if has_video {
        codec_label_video(vcodec)
    } else {
        None
    };

    let vcodec_str = if has_video { Some(vcodec.to_string()) } else { None };
    let acodec_str = if has_audio_codec { Some(acodec.to_string()) } else { None };

    Some(VideoFormat {
        format_id,
        quality,
        vcodec: vcodec_str,
        acodec: acodec_str,
        codec_label,
        has_audio,
        is_audio_only,
        width,
        height,
        fps,
        bitrate,
        ext,
        url: url.to_string(),
        filesize,
    })
}

/// Build a human-readable quality label
fn build_quality_label(
    obj: &serde_json::Map<String, Value>,
    height: Option<u32>,
    is_audio_only: bool,
    fps: Option<f32>,
    bitrate: Option<u64>,
) -> String {
    // yt-dlp's own format_note is most accurate
    if let Some(note) = obj.get("format_note").and_then(Value::as_str) {
        if !note.is_empty() && note != "Default" {
            return note.to_string();
        }
    }

    if is_audio_only {
        if let Some(abr) = obj.get("abr").and_then(Value::as_f64) {
            return format!("Audio {:.0}kbps", abr);
        }
        if let Some(kbps) = bitrate.map(|b| b / 1000) {
            return format!("Audio {}kbps", kbps);
        }
        return "Audio".to_string();
    }

    if let Some(h) = height {
        let mut label = format!("{}p", h);
        if let Some(f) = fps {
            if f > 30.0 {
                label.push_str(&format!("{:.0}", f));
            }
        }
        return label;
    }

    obj.get("quality")
        .and_then(Value::as_f64)
        .map(|q| format!("{}p", q as u32))
        .unwrap_or_else(|| "unknown".to_string())
}

/// Human-readable video codec label
fn codec_label_video(vcodec: &str) -> Option<String> {
    let v = vcodec.to_lowercase();
    Some(if v.starts_with("av01") {
        "AV1"
    } else if v.starts_with("vp09") || v.starts_with("vp9") {
        "VP9"
    } else if v.starts_with("avc1") || v.starts_with("h264") {
        "H.264"
    } else if v.starts_with("hev1") || v.starts_with("hvc1") {
        "H.265"
    } else {
        return Some(vcodec.split('.').next().unwrap_or(vcodec).to_uppercase());
    }.to_string())
}

/// Human-readable audio codec label
fn codec_label_audio(acodec: &str) -> Option<String> {
    let a = acodec.to_lowercase();
    Some(if a.starts_with("mp4a") {
        "AAC"
    } else if a.starts_with("opus") {
        "Opus"
    } else {
        return Some(acodec.split('.').next().unwrap_or(acodec).to_uppercase());
    }.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_video_format() {
        let fmt = json!({
            "format_id": "137",
            "url": "https://googlevideo.com/test",
            "ext": "mp4",
            "vcodec": "avc1.640028",
            "acodec": "none",
            "width": 1920,
            "height": 1080,
            "fps": 30.0,
            "tbr": 3000.0,
            "filesize": 10000000,
            "format_note": "1080p"
        });
        let result = parse_format(&fmt, 0).unwrap();
        assert_eq!(result.height, Some(1080));
        assert!(!result.has_audio);
        assert!(!result.is_audio_only);
        assert_eq!(result.quality, "1080p");
        assert_eq!(result.codec_label.as_deref(), Some("H.264"));
    }

    #[test]
    fn test_parse_audio_format() {
        let fmt = json!({
            "format_id": "140",
            "url": "https://googlevideo.com/audio",
            "ext": "m4a",
            "vcodec": "none",
            "acodec": "mp4a.40.2",
            "abr": 128.0,
            "filesize": 3000000
        });
        let result = parse_format(&fmt, 1).unwrap();
        assert!(result.is_audio_only);
        assert!(result.has_audio);
        assert_eq!(result.codec_label.as_deref(), Some("AAC"));
        assert!(result.quality.starts_with("Audio"));
    }

    #[test]
    fn test_skip_m3u8_format() {
        let fmt = json!({
            "format_id": "hls-0",
            "url": "https://example.com/playlist.m3u8",
            "ext": "mp4",
            "protocol": "m3u8",
            "vcodec": "avc1",
            "acodec": "mp4a"
        });
        assert!(parse_format(&fmt, 0).is_none());
    }

    #[test]
    fn test_normalize_cache_key_from_watch_url() {
        let key = normalize_cache_key("https://www.youtube.com/watch?v=dQw4w9WgXcQ&t=42");
        assert_eq!(key, "https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    }

    #[test]
    fn test_normalize_cache_key_from_short_url() {
        let key = normalize_cache_key("https://youtu.be/dQw4w9WgXcQ?si=test");
        assert_eq!(key, "https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    }
}
