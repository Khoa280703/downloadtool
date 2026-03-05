//! yt-dlp subprocess extractor
//!
//! Calls `yt-dlp -J --no-playlist` to get video metadata + stream URLs.
//! yt-dlp handles PO Token, signature decryption, and throttle bypass automatically.

use crate::runtime_limit_profiles::extractor_limit_profile;
use crate::types::{ExtractionError, VideoFormat, VideoInfo};
use moka::future::Cache;
use proxy::ProxyPool;
use serde_json::Value;
use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::process::Command;
use tokio::sync::Semaphore;
use tracing::{debug, warn};

const MAX_CONCURRENT_YTDLP: usize = 10;
const EXTRACT_CACHE_MAX_CAPACITY: u64 = 500;
const DEFAULT_EXTRACT_CACHE_TTL_SECONDS: u64 = 300;
const MAX_PROXY_ROTATION_ATTEMPTS: usize = 3;
const STREAM_PROXY_CACHE_MAX_CAPACITY: u64 = 10_000;
const DEFAULT_STREAM_PROXY_CACHE_TTL_SECONDS: u64 = 1800;

static YTDLP_SEMAPHORE: OnceLock<Arc<Semaphore>> = OnceLock::new();
static EXTRACT_CACHE: OnceLock<Cache<String, Arc<VideoInfo>>> = OnceLock::new();
static EXTRACT_CACHE_HITS: AtomicU64 = AtomicU64::new(0);
static EXTRACT_CACHE_MISSES: AtomicU64 = AtomicU64::new(0);
static YTDLP_PROXY_POOL: OnceLock<Option<ProxyPool>> = OnceLock::new();
static STREAM_PROXY_CACHE: OnceLock<Cache<String, Arc<String>>> = OnceLock::new();

fn extract_cache_ttl_secs() -> u64 {
    extractor_limit_profile().extract_cache_ttl_secs_value(DEFAULT_EXTRACT_CACHE_TTL_SECONDS)
}

fn stream_proxy_cache_ttl_secs() -> u64 {
    extractor_limit_profile()
        .stream_proxy_cache_ttl_secs_value(DEFAULT_STREAM_PROXY_CACHE_TTL_SECONDS)
}

fn get_semaphore() -> &'static Arc<Semaphore> {
    YTDLP_SEMAPHORE.get_or_init(|| Arc::new(Semaphore::new(MAX_CONCURRENT_YTDLP)))
}

fn get_cache() -> &'static Cache<String, Arc<VideoInfo>> {
    EXTRACT_CACHE.get_or_init(|| {
        Cache::builder()
            .max_capacity(EXTRACT_CACHE_MAX_CAPACITY)
            .time_to_live(Duration::from_secs(extract_cache_ttl_secs()))
            .build()
    })
}

fn get_stream_proxy_cache() -> &'static Cache<String, Arc<String>> {
    STREAM_PROXY_CACHE.get_or_init(|| {
        Cache::builder()
            .max_capacity(STREAM_PROXY_CACHE_MAX_CAPACITY)
            .time_to_live(Duration::from_secs(stream_proxy_cache_ttl_secs()))
            .build()
    })
}

fn resolve_ytdlp_binary() -> String {
    std::env::var("YTDLP_PATH")
        .ok()
        .filter(|path| !path.trim().is_empty())
        .unwrap_or_else(|| "yt-dlp".to_string())
}

fn get_proxy_pool() -> Option<&'static ProxyPool> {
    YTDLP_PROXY_POOL.get_or_init(ProxyPool::from_env).as_ref()
}

fn fixed_proxy_from_env() -> Option<String> {
    std::env::var("SOCKS5_PROXY_URL")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

#[derive(Debug, Clone)]
struct SelectedProxy {
    url: String,
    from_pool: bool,
}

fn select_proxy() -> Option<SelectedProxy> {
    if let Some(url) = fixed_proxy_from_env() {
        return Some(SelectedProxy {
            url,
            from_pool: false,
        });
    }

    get_proxy_pool()
        .and_then(ProxyPool::next_owned)
        .map(|url| SelectedProxy {
            url,
            from_pool: true,
        })
}

fn should_mark_proxy_failed(stderr: &str) -> bool {
    let normalized = stderr.to_ascii_lowercase();
    normalized.contains("sign in to confirm")
        || normalized.contains("not a bot")
        || normalized.contains("429")
        || normalized.contains("too many requests")
        || normalized.contains("403")
        || normalized.contains("timed out")
        || normalized.contains("connection reset")
        || normalized.contains("unable to download")
}

/// Resolve pinned proxy URL for a previously extracted stream URL.
pub async fn resolve_stream_proxy(url: &str) -> Option<String> {
    get_stream_proxy_cache()
        .get(url)
        .await
        .map(|proxy| (*proxy).clone())
}

async fn remember_stream_proxy(formats: &[VideoFormat], proxy_url: &str) {
    let cache = get_stream_proxy_cache();
    let proxy = Arc::new(proxy_url.to_string());
    for format in formats {
        cache.insert(format.url.clone(), Arc::clone(&proxy)).await;
    }
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
    let cache = get_cache();

    if let Some(cached_video_info) = cache.get(&cache_key).await {
        let hits = EXTRACT_CACHE_HITS.fetch_add(1, Ordering::Relaxed) + 1;
        debug!(
            cache_key = %cache_key,
            cache_hits = hits,
            "extract cache hit"
        );
        return Ok((*cached_video_info).clone());
    }

    let misses = EXTRACT_CACHE_MISSES.fetch_add(1, Ordering::Relaxed) + 1;
    debug!(
        cache_key = %cache_key,
        cache_misses = misses,
        "extract cache miss"
    );

    cache
        .try_get_with(cache_key, extract_subprocess(url.to_string()))
        .await
        .map(|video_info| (*video_info).clone())
        .map_err(|error: Arc<ExtractionError>| (*error).clone())
}

async fn extract_subprocess(url: String) -> Result<Arc<VideoInfo>, ExtractionError> {
    let _permit = get_semaphore().acquire().await.map_err(|error| {
        ExtractionError::ScriptExecutionFailed(format!("semaphore acquire failed: {error}"))
    })?;

    let pool_rotation_enabled = fixed_proxy_from_env().is_none() && get_proxy_pool().is_some();
    let max_attempts = if pool_rotation_enabled {
        MAX_PROXY_ROTATION_ATTEMPTS
    } else {
        1
    };

    let mut last_error: Option<String> = None;

    for attempt in 0..max_attempts {
        let selected_proxy = select_proxy();
        let proxy_log = selected_proxy
            .as_ref()
            .map(|proxy| proxy.url.as_str())
            .unwrap_or("direct");
        debug!(
            "yt-dlp attempt {}/{} for {} using {}",
            attempt + 1,
            max_attempts,
            url,
            proxy_log
        );

        let output = run_ytdlp_command(&url, selected_proxy.as_ref()).await?;
        if output.status.success() {
            if let Some(proxy) = selected_proxy.as_ref().filter(|proxy| proxy.from_pool) {
                if let Some(pool) = get_proxy_pool() {
                    pool.mark_success(&proxy.url);
                }
            }

            let info = parse_ytdlp_success(&output.stdout, &url)?;
            if let Some(proxy) = selected_proxy.as_ref() {
                remember_stream_proxy(&info.formats, &proxy.url).await;
            }
            return Ok(Arc::new(info));
        }

        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if let Some(proxy) = selected_proxy.filter(|proxy| proxy.from_pool) {
            if should_mark_proxy_failed(&stderr) {
                if let Some(pool) = get_proxy_pool() {
                    pool.mark_failed(&proxy.url);
                }
            }
        }

        last_error = Some(stderr.clone());
        warn!(
            "yt-dlp failed for {} (attempt {}/{}): {}",
            url,
            attempt + 1,
            max_attempts,
            stderr
        );
    }

    let final_error = last_error.unwrap_or_else(|| "unknown yt-dlp error".to_string());
    Err(ExtractionError::ScriptExecutionFailed(format!(
        "yt-dlp error: {}",
        final_error
    )))
}

struct YtdlpAttemptOutput {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    status: std::process::ExitStatus,
}

async fn run_ytdlp_command(
    url: &str,
    proxy: Option<&SelectedProxy>,
) -> Result<YtdlpAttemptOutput, ExtractionError> {
    let mut command = build_command(url, proxy);
    let output = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|error| {
            ExtractionError::ScriptExecutionFailed(format!("yt-dlp launch failed: {error}"))
        })?;

    Ok(YtdlpAttemptOutput {
        stdout: output.stdout,
        stderr: output.stderr,
        status: output.status,
    })
}

fn parse_ytdlp_success(stdout: &[u8], url: &str) -> Result<VideoInfo, ExtractionError> {
    let json: Value = serde_json::from_slice(stdout).map_err(|error| {
        ExtractionError::ScriptExecutionFailed(format!("yt-dlp JSON parse error: {error}"))
    })?;
    parse_ytdlp_json(json, url)
}

/// Build the yt-dlp Command with appropriate flags
fn build_command(url: &str, proxy: Option<&SelectedProxy>) -> Command {
    let mut cmd = Command::new(resolve_ytdlp_binary());
    cmd.args([
        "-J",            // Dump JSON metadata to stdout
        "--no-playlist", // Single video only (ignore playlist)
        "--no-warnings", // Suppress non-fatal warnings
        "--socket-timeout",
        "15",
        "--no-check-certificates",
    ]);

    if let Some(proxy) = proxy {
        cmd.args(["--proxy", &proxy.url]);
        debug!("yt-dlp routing through proxy: {}", proxy.url);
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

    let view_count = obj.get("view_count").and_then(Value::as_u64);

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
        .ok_or_else(|| {
            ExtractionError::ScriptExecutionFailed("yt-dlp: no formats array".to_string())
        })?;

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
    formats.sort_by(|a, b| b.height.unwrap_or(0).cmp(&a.height.unwrap_or(0)));

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
    let protocol = obj
        .get("protocol")
        .and_then(Value::as_str)
        .unwrap_or("https");
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

    let vcodec_str = if has_video {
        Some(vcodec.to_string())
    } else {
        None
    };
    let acodec_str = if has_audio_codec {
        Some(acodec.to_string())
    } else {
        None
    };

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
    Some(
        if v.starts_with("av01") {
            "AV1"
        } else if v.starts_with("vp09") || v.starts_with("vp9") {
            "VP9"
        } else if v.starts_with("avc1") || v.starts_with("h264") {
            "H.264"
        } else if v.starts_with("hev1") || v.starts_with("hvc1") {
            "H.265"
        } else {
            return Some(vcodec.split('.').next().unwrap_or(vcodec).to_uppercase());
        }
        .to_string(),
    )
}

/// Human-readable audio codec label
fn codec_label_audio(acodec: &str) -> Option<String> {
    let a = acodec.to_lowercase();
    Some(
        if a.starts_with("mp4a") {
            "AAC"
        } else if a.starts_with("opus") {
            "Opus"
        } else {
            return Some(acodec.split('.').next().unwrap_or(acodec).to_uppercase());
        }
        .to_string(),
    )
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
