//! yt-dlp subprocess extractor
//!
//! Calls `yt-dlp -J --no-playlist` to get video metadata + stream URLs.
//! yt-dlp handles PO Token, signature decryption, and throttle bypass automatically.

use crate::runtime_limit_profiles::extractor_limit_profile;
use crate::types::{ExtractionError, VideoFormat, VideoInfo};
use moka::future::Cache;
use proxy::{ProxyExtractEvent, ProxyLease, ProxyPool, BOT_CHECK_QUARANTINE_THRESHOLD};
use serde_json::Value;
use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};
use tokio::process::Command;
use tokio::sync::Semaphore;
use tracing::{debug, info, warn};

const MAX_CONCURRENT_YTDLP: usize = 10;
const EXTRACT_CACHE_MAX_CAPACITY: u64 = 500;
const DEFAULT_EXTRACT_CACHE_TTL_SECONDS: u64 = 300;
const MAX_PROXY_ROTATION_ATTEMPTS: usize = 5;
const STREAM_PROXY_CACHE_MAX_CAPACITY: u64 = 10_000;
const DEFAULT_STREAM_PROXY_CACHE_TTL_SECONDS: u64 = 1800;

static YTDLP_SEMAPHORE: OnceLock<Arc<Semaphore>> = OnceLock::new();
static EXTRACT_CACHE: OnceLock<Cache<String, Arc<VideoInfo>>> = OnceLock::new();
static EXTRACT_CACHE_HITS: AtomicU64 = AtomicU64::new(0);
static EXTRACT_CACHE_MISSES: AtomicU64 = AtomicU64::new(0);
static YTDLP_PROXY_POOL: OnceLock<Option<Arc<ProxyPool>>> = OnceLock::new();
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

fn get_proxy_pool() -> Option<&'static Arc<ProxyPool>> {
    YTDLP_PROXY_POOL
        .get_or_init(proxy::proxy_runtime::global_proxy_pool)
        .as_ref()
}

#[derive(Debug)]
struct SelectedProxy {
    url: String,
    from_pool: bool,
    _lease: Option<ProxyLease>,
}

#[derive(Debug, Clone, Copy)]
struct ExtractCatalogStats {
    usable_format_count: u32,
    combined_stream_count: u32,
    video_only_count: u32,
    audio_only_count: u32,
    combined_360_only: bool,
    full_format_available: bool,
}

fn select_proxy() -> Option<SelectedProxy> {
    get_proxy_pool()
        .and_then(|pool| pool.try_acquire_next_owned())
        .map(|lease| SelectedProxy {
            url: lease.proxy_url().to_string(),
            from_pool: true,
            _lease: Some(lease),
        })
}

fn select_proxy_with_preference(preferred_proxy: Option<&str>) -> Option<SelectedProxy> {
    if let Some(preferred_proxy) = preferred_proxy {
        if let Some(pool) = get_proxy_pool() {
            if let Some(lease) = pool.try_acquire_preferred_owned(preferred_proxy) {
                return Some(SelectedProxy {
                    url: lease.proxy_url().to_string(),
                    from_pool: true,
                    _lease: Some(lease),
                });
            }
        } else {
            return Some(SelectedProxy {
                url: preferred_proxy.to_string(),
                from_pool: false,
                _lease: None,
            });
        }
    }

    select_proxy()
}

async fn require_proxy(preferred_proxy: Option<&str>) -> Result<SelectedProxy, ExtractionError> {
    const PROXY_ACQUIRE_RETRY_DELAY_MS: u64 = 50;
    const PROXY_ACQUIRE_TIMEOUT: Duration = Duration::from_secs(5);

    let deadline = Instant::now() + PROXY_ACQUIRE_TIMEOUT;
    while Instant::now() < deadline {
        if let Some(proxy) = select_proxy_with_preference(preferred_proxy) {
            return Ok(proxy);
        }
        tokio::time::sleep(Duration::from_millis(PROXY_ACQUIRE_RETRY_DELAY_MS)).await;
    }

    Err(ExtractionError::ScriptExecutionFailed(
        "proxy-only mode requires at least one free healthy proxy".to_string(),
    ))
}

/// How a proxy failure should be handled.
#[derive(Debug, Clone, Copy)]
enum ProxyFailureKind {
    /// Hard transport failure: timeout, connection reset, DNS failure.
    /// Proxy should be cooled down immediately (1 fail = cooldown).
    TransportDead,
    /// Bot check detected: "sign in to confirm", "not a bot".
    /// Proxy should only be quarantined after repeated consecutive hits.
    BotCheck,
    /// Rate limit or soft block: 429, 403, too many requests.
    /// Use normal mark_failed() (needs MAX_FAILURES to cooldown).
    RateLimit,
    /// Error not related to proxy health: video private/deleted/geo-blocked,
    /// parse errors, upstream errors. Proxy should NOT be penalized.
    NotProxyRelated,
}

impl ProxyFailureKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::TransportDead => "transport_dead",
            Self::BotCheck => "bot_check",
            Self::RateLimit => "rate_limit",
            Self::NotProxyRelated => "not_proxy_related",
        }
    }
}

fn classify_failure_kind(stderr: &str) -> ProxyFailureKind {
    let normalized = stderr.to_ascii_lowercase();

    // Bot check / sign-in wall — quarantine proxy
    if normalized.contains("sign in to confirm") || normalized.contains("not a bot") {
        return ProxyFailureKind::BotCheck;
    }

    // Hard transport failures — cooldown proxy immediately
    if normalized.contains("timed out")
        || normalized.contains("connection reset")
        || normalized.contains("connection refused")
        || normalized.contains("network is unreachable")
        || normalized.contains("name or service not known")
        || normalized.contains("no route to host")
    {
        return ProxyFailureKind::TransportDead;
    }

    // Rate limit / soft block — mark_failed (needs MAX_FAILURES)
    if normalized.contains("429")
        || normalized.contains("too many requests")
        || normalized.contains("403")
    {
        return ProxyFailureKind::RateLimit;
    }

    // Everything else: video private, deleted, geo-blocked, parse error, etc.
    // Do NOT penalize proxy for these.
    ProxyFailureKind::NotProxyRelated
}

/// Resolve pinned proxy URL for a previously extracted stream URL.
pub async fn resolve_stream_proxy(url: &str) -> Option<String> {
    let proxy = get_stream_proxy_cache()
        .get(url)
        .await
        .map(|proxy| (*proxy).clone())?;

    if let Some(pool) = get_proxy_pool() {
        if !pool.is_proxy_usable(&proxy) {
            get_stream_proxy_cache().invalidate(url).await;
            return None;
        }
    }

    Some(proxy)
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
pub async fn extract_via_ytdlp(
    url: &str,
    bypass_cache: bool,
    preferred_proxy: Option<&str>,
) -> Result<VideoInfo, ExtractionError> {
    debug!("yt-dlp extracting: {}", url);

    let cache_key = normalize_cache_key(url);
    let cache = get_cache();
    let cache_allowed = preferred_proxy.is_none();

    if cache_allowed && !bypass_cache {
        if let Some(cached_video_info) = cache.get(&cache_key).await {
            let hits = EXTRACT_CACHE_HITS.fetch_add(1, Ordering::Relaxed) + 1;
            debug!(
                cache_key = %cache_key,
                cache_hits = hits,
                "extract cache hit"
            );
            return Ok((*cached_video_info).clone());
        }
    } else {
        debug!(cache_key = %cache_key, "extract bypassing cache");
    }

    let misses = EXTRACT_CACHE_MISSES.fetch_add(1, Ordering::Relaxed) + 1;
    debug!(
        cache_key = %cache_key,
        cache_misses = misses,
        "extract cache miss"
    );

    let video_info = extract_subprocess(
        url.to_string(),
        preferred_proxy.map(std::string::ToString::to_string),
    )
    .await?;
    if cache_allowed {
        maybe_cache_extract_result(&cache_key, &video_info).await;
    }
    Ok((*video_info).clone())
}

async fn maybe_cache_extract_result(cache_key: &str, video_info: &VideoInfo) {
    if is_single_combined_360p_fallback(video_info) {
        debug!(
            cache_key = %cache_key,
            "skipping extract cache insert for degraded single 360p combined result"
        );
        get_cache().invalidate(cache_key).await;
        return;
    }

    get_cache()
        .insert(cache_key.to_string(), Arc::new(video_info.clone()))
        .await;
}

fn is_single_combined_360p_fallback(video_info: &VideoInfo) -> bool {
    if video_info.formats.len() != 1 {
        return false;
    }

    let format = &video_info.formats[0];
    !format.is_audio_only
        && format.has_audio
        && format.ext.eq_ignore_ascii_case("mp4")
        && format.height == Some(360)
}

fn summarize_catalog_stats(video_info: &VideoInfo) -> ExtractCatalogStats {
    let mut combined_stream_count = 0u32;
    let mut video_only_count = 0u32;
    let mut audio_only_count = 0u32;
    let mut has_hd_combined = false;

    for format in &video_info.formats {
        if format.is_audio_only {
            audio_only_count += 1;
            continue;
        }

        if format.has_audio {
            combined_stream_count += 1;
            if format.height.unwrap_or_default() > 360 {
                has_hd_combined = true;
            }
            continue;
        }

        video_only_count += 1;
    }

    let combined_360_only = is_single_combined_360p_fallback(video_info);
    let full_format_available =
        !combined_360_only && ((video_only_count > 0 && audio_only_count > 0) || has_hd_combined);

    ExtractCatalogStats {
        usable_format_count: video_info.formats.len() as u32,
        combined_stream_count,
        video_only_count,
        audio_only_count,
        combined_360_only,
        full_format_available,
    }
}

fn build_success_extract_event(video_info: &VideoInfo, elapsed_ms: u64) -> ProxyExtractEvent {
    let stats = summarize_catalog_stats(video_info);
    let outcome = if stats.combined_360_only {
        "success_360_only"
    } else if stats.full_format_available {
        "success_full_format"
    } else {
        "success_limited_catalog"
    };

    ProxyExtractEvent {
        outcome: outcome.to_string(),
        success: true,
        elapsed_ms,
        failure_kind: None,
        usable_format_count: stats.usable_format_count,
        combined_stream_count: stats.combined_stream_count,
        video_only_count: stats.video_only_count,
        audio_only_count: stats.audio_only_count,
        combined_360_only: stats.combined_360_only,
        full_format_available: stats.full_format_available,
    }
}

fn build_failed_extract_event(failure_kind: &str, elapsed_ms: u64) -> ProxyExtractEvent {
    ProxyExtractEvent {
        outcome: format!("failure_{failure_kind}"),
        success: false,
        elapsed_ms,
        failure_kind: Some(failure_kind.to_string()),
        usable_format_count: 0,
        combined_stream_count: 0,
        video_only_count: 0,
        audio_only_count: 0,
        combined_360_only: false,
        full_format_available: false,
    }
}

/// Timeout for the entire yt-dlp subprocess per attempt.
/// Must be greater than --socket-timeout to cover DNS, proxy handshake, etc.
const SUBPROCESS_TIMEOUT: Duration = Duration::from_secs(8);

async fn extract_subprocess(
    url: String,
    preferred_proxy: Option<String>,
) -> Result<Arc<VideoInfo>, ExtractionError> {
    let _permit = get_semaphore().acquire().await.map_err(|error| {
        ExtractionError::ScriptExecutionFailed(format!("semaphore acquire failed: {error}"))
    })?;

    let pool_rotation_enabled = get_proxy_pool().is_some();
    let max_attempts = if pool_rotation_enabled {
        MAX_PROXY_ROTATION_ATTEMPTS
    } else {
        1
    };

    let mut last_error: Option<String> = None;

    for attempt in 0..max_attempts {
        let selected_proxy = require_proxy(preferred_proxy.as_deref()).await?;
        let attempt_started_at = Instant::now();
        debug!(
            url = %url,
            attempt = attempt + 1,
            max_attempts = max_attempts,
            proxy = %selected_proxy.url,
            "yt-dlp attempt starting"
        );

        let mut child = match spawn_ytdlp_command(&url, &selected_proxy) {
            Ok(child) => child,
            Err(error) => {
                let elapsed_ms = attempt_started_at.elapsed().as_millis() as u64;
                warn!(
                    url = %url,
                    attempt = attempt + 1,
                    max_attempts = max_attempts,
                    proxy = %selected_proxy.url,
                    elapsed_ms = elapsed_ms,
                    failure_kind = "launch_failed",
                    "yt-dlp subprocess launch failed"
                );
                return Err(error);
            }
        };

        // Take pipe handles before timeout. Read them concurrently with wait()
        // using try_join! to avoid pipe buffer deadlock when yt-dlp outputs > 64KB.
        let mut stdout_pipe = child.stdout.take();
        let mut stderr_pipe = child.stderr.take();

        let wait_result = tokio::time::timeout(SUBPROCESS_TIMEOUT, async {
            tokio::try_join!(
                child.wait(),
                async {
                    let data = drain_stdout(&mut stdout_pipe).await;
                    Ok::<_, std::io::Error>(data)
                },
                async {
                    let data = drain_stderr(&mut stderr_pipe).await;
                    Ok::<_, std::io::Error>(data)
                },
            )
        })
        .await;

        let output = match wait_result {
            Ok(Ok((status, stdout, stderr))) => YtdlpAttemptOutput {
                stdout,
                stderr,
                status,
            },
            Ok(Err(error)) => {
                let elapsed_ms = attempt_started_at.elapsed().as_millis() as u64;
                warn!(
                    url = %url,
                    attempt = attempt + 1,
                    max_attempts = max_attempts,
                    proxy = %selected_proxy.url,
                    elapsed_ms = elapsed_ms,
                    failure_kind = "io_error",
                    "yt-dlp subprocess I/O error: {error}"
                );
                return Err(ExtractionError::ScriptExecutionFailed(format!(
                    "yt-dlp I/O error: {error}"
                )));
            }
            Err(_timeout) => {
                // Timeout: kill child process. Pipes were taken out, so dropping
                // the async block closed our read ends; the process may already
                // have received SIGPIPE. kill() ensures it's fully reaped.
                let _ = child.kill().await;
                let elapsed_ms = attempt_started_at.elapsed().as_millis() as u64;
                warn!(
                    url = %url,
                    attempt = attempt + 1,
                    max_attempts = max_attempts,
                    proxy = %selected_proxy.url,
                    elapsed_ms = elapsed_ms,
                    failure_kind = "subprocess_timeout",
                    timeout_secs = SUBPROCESS_TIMEOUT.as_secs(),
                    "yt-dlp subprocess timed out, child process killed"
                );
                if selected_proxy.from_pool {
                    if let Some(pool) = get_proxy_pool() {
                        pool.cooldown_now(&selected_proxy.url, "yt-dlp-subprocess-timeout");
                        pool.record_extract_result(
                            &selected_proxy.url,
                            build_failed_extract_event("subprocess_timeout", elapsed_ms),
                        );
                    }
                }
                last_error = Some(format!(
                    "yt-dlp subprocess timed out after {}s",
                    SUBPROCESS_TIMEOUT.as_secs()
                ));
                continue;
            }
        };

        let elapsed_ms = attempt_started_at.elapsed().as_millis() as u64;

        if output.status.success() {
            let info = parse_ytdlp_success(&output.stdout, &url)?;
            if selected_proxy.from_pool {
                if let Some(pool) = get_proxy_pool() {
                    pool.mark_success(&selected_proxy.url);
                    pool.record_extract_result(
                        &selected_proxy.url,
                        build_success_extract_event(&info, elapsed_ms),
                    );
                }
            }

            let catalog_stats = summarize_catalog_stats(&info);
            info!(
                url = %url,
                attempt = attempt + 1,
                proxy = %selected_proxy.url,
                elapsed_ms = elapsed_ms,
                failure_kind = "success",
                format_count = info.formats.len(),
                combined_stream_count = catalog_stats.combined_stream_count,
                video_only_count = catalog_stats.video_only_count,
                audio_only_count = catalog_stats.audio_only_count,
                combined_360_only = catalog_stats.combined_360_only,
                full_format_available = catalog_stats.full_format_available,
                "yt-dlp extract succeeded"
            );
            remember_stream_proxy(&info.formats, &selected_proxy.url).await;
            return Ok(Arc::new(info));
        }

        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let failure_kind = classify_failure_kind(&stderr);
        let stderr_snippet: String = stderr.chars().take(200).collect();

        warn!(
            url = %url,
            attempt = attempt + 1,
            max_attempts = max_attempts,
            proxy = %selected_proxy.url,
            elapsed_ms = elapsed_ms,
            failure_kind = failure_kind.as_str(),
            stderr_snippet = %stderr_snippet,
            "yt-dlp attempt failed"
        );

        if selected_proxy.from_pool {
            if let Some(pool) = get_proxy_pool() {
                pool.record_extract_result(
                    &selected_proxy.url,
                    build_failed_extract_event(failure_kind.as_str(), elapsed_ms),
                );
                match failure_kind {
                    ProxyFailureKind::BotCheck => {
                        if pool
                            .note_bot_check(&selected_proxy.url, "yt-dlp-bot-check")
                            .await
                        {
                            pool.quarantine(&selected_proxy.url, "yt-dlp-bot-check-streak-5");
                        } else {
                            warn!(
                                proxy = %selected_proxy.url,
                                threshold = BOT_CHECK_QUARANTINE_THRESHOLD,
                                "Proxy hit bot-check; waiting for more consecutive bot-checks before quarantine"
                            );
                        }
                    }
                    ProxyFailureKind::TransportDead => {
                        pool.cooldown_now(&selected_proxy.url, "yt-dlp-transport-dead");
                    }
                    ProxyFailureKind::RateLimit => {
                        pool.mark_failed(&selected_proxy.url);
                    }
                    ProxyFailureKind::NotProxyRelated => {
                        pool.clear_bot_check_streak(&selected_proxy.url);
                    }
                }
            }
        }

        last_error = Some(stderr);
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

/// Drain remaining data from a taken stdout pipe handle.
/// Safe to call after the child process has exited (pipe write end is closed).
async fn drain_stdout(pipe: &mut Option<tokio::process::ChildStdout>) -> Vec<u8> {
    use tokio::io::AsyncReadExt;
    let mut buf = Vec::new();
    if let Some(ref mut pipe) = pipe {
        let _ = pipe.read_to_end(&mut buf).await;
    }
    buf
}

/// Drain remaining data from a taken stderr pipe handle.
async fn drain_stderr(pipe: &mut Option<tokio::process::ChildStderr>) -> Vec<u8> {
    use tokio::io::AsyncReadExt;
    let mut buf = Vec::new();
    if let Some(ref mut pipe) = pipe {
        let _ = pipe.read_to_end(&mut buf).await;
    }
    buf
}

fn spawn_ytdlp_command(
    url: &str,
    proxy: &SelectedProxy,
) -> Result<tokio::process::Child, ExtractionError> {
    let mut command = build_command(url, proxy);
    command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| {
            ExtractionError::ScriptExecutionFailed(format!("yt-dlp launch failed: {error}"))
        })
}

fn parse_ytdlp_success(stdout: &[u8], url: &str) -> Result<VideoInfo, ExtractionError> {
    let raw_payload = String::from_utf8_lossy(stdout);
    info!(
        original_url = %url,
        raw_ytdlp_payload = %raw_payload,
        "yt-dlp raw upstream payload"
    );

    let json: Value = serde_json::from_slice(stdout).map_err(|error| {
        ExtractionError::ScriptExecutionFailed(format!("yt-dlp JSON parse error: {error}"))
    })?;
    parse_ytdlp_json(json, url)
}

/// Build the yt-dlp Command with appropriate flags
fn build_command(url: &str, proxy: &SelectedProxy) -> Command {
    let mut cmd = Command::new(resolve_ytdlp_binary());
    cmd.args([
        "-J",            // Dump JSON metadata to stdout
        "--no-playlist", // Single video only (ignore playlist)
        "--no-warnings", // Suppress non-fatal warnings
        "--socket-timeout",
        "5",
        "--no-check-certificates",
    ]);

    cmd.args(["--proxy", &proxy.url]);
    debug!("yt-dlp routing through proxy: {}", proxy.url);

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
