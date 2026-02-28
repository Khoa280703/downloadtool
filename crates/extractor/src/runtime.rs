//! deno_core JsRuntime setup for JavaScript extractors
//!
//! This module initializes a V8 isolate with custom ops for HTTP fetching
//! and logging, then loads the bundled TypeScript extractor scripts.

use crate::types::{ExtractionError, VideoFormat, VideoInfo};
use deno_core::{op2, JsRuntime, PollEventLoopOptions, RuntimeOptions};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Duration;
use tracing::{debug, error, info, warn};

/// HTTP response structure for JS interop
#[derive(Serialize, Deserialize, Debug)]
struct FetchResponse {
    ok: bool,
    status: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: String,
    url: String,
}

/// Request options for fetch op
#[derive(Deserialize, Debug, Default)]
struct FetchOptions {
    method: Option<String>,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
}

// Register custom ops with deno_core 0.300 extension macro
deno_core::extension!(
    extractor_ops,
    ops = [op_fetch, op_log],
);

/// JavaScript runtime wrapper for running extractors
pub struct ExtractorRuntime {
    runtime: JsRuntime,
}

impl ExtractorRuntime {
    /// Create a new extractor runtime with bundled scripts loaded
    pub fn new(js_bundle: &str) -> Result<Self, ExtractionError> {
        let options = RuntimeOptions {
            extensions: vec![extractor_ops::init_ops()],
            ..Default::default()
        };

        let mut runtime = JsRuntime::new(options);

        // Inject fetch polyfill that wraps op_fetch to provide standard fetch API
        // (op_fetch returns plain object; extractor code expects Response with .text()/.json())
        let fetch_polyfill = r#"
(function() {
    var _rawOpFetch = Deno.core.ops.op_fetch;
    function makeResponse(raw) {
        return {
            ok: raw.ok,
            status: raw.status,
            statusText: raw.status_text || '',
            headers: {
                get: function(h) { return (raw.headers && raw.headers[h.toLowerCase()]) || null; },
                has: function(h) { return !!(raw.headers && raw.headers[h.toLowerCase()]); }
            },
            url: raw.url || '',
            text: function() { return Promise.resolve(raw.body || ''); },
            json: function() {
                return new Promise(function(resolve, reject) {
                    try { resolve(JSON.parse(raw.body || 'null')); }
                    catch(e) { reject(e); }
                });
            }
        };
    }
    var wrappedFetch = async function fetch(url, init) {
        var opts = init ? { method: init.method, headers: init.headers, body: init.body } : null;
        var raw = await _rawOpFetch(url, opts);
        return makeResponse(raw);
    };
    // Override Deno.core.ops.op_fetch so extractor code gets Response-like object
    if (typeof Deno !== 'undefined' && Deno.core && Deno.core.ops) {
        Deno.core.ops.op_fetch = wrappedFetch;
    }
    // Also provide global fetch fallback
    globalThis.fetch = wrappedFetch;
})();
"#;
        runtime
            .execute_script("fetch_polyfill.js", fetch_polyfill.to_string())
            .map_err(|e| {
                ExtractionError::JavaScriptError(format!("Failed to load fetch polyfill: {}", e))
            })?;

        // Load the bundled extractor JavaScript
        runtime
            .execute_script("extractors.js", js_bundle.to_string())
            .map_err(|e| {
                ExtractionError::JavaScriptError(format!(
                    "Failed to load extractor bundle: {}",
                    e
                ))
            })?;

        info!("Extractor runtime initialized successfully");

        Ok(Self { runtime })
    }

    /// Extract video information by calling the JS extract function
    pub async fn extract(
        &mut self,
        platform: &str,
        url: &str,
        cookies: Option<&str>,
    ) -> Result<VideoInfo, ExtractionError> {
        let platform_json = serde_json::to_string(platform).map_err(|e| {
            ExtractionError::JavaScriptError(format!(
                "Failed to serialize platform argument: {}",
                e
            ))
        })?;
        let url_json = serde_json::to_string(url).map_err(|e| {
            ExtractionError::JavaScriptError(format!("Failed to serialize URL argument: {}", e))
        })?;
        let cookies_json = cookies
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| {
                ExtractionError::JavaScriptError(format!(
                    "Failed to serialize cookies argument: {}",
                    e
                ))
            })?
            .unwrap_or_else(|| "undefined".to_string());

        let code = format!(
            r#"
            (async () => {{
                const extractor = extractors[{}];
                if (!extractor) {{
                    throw new Error("Extractor not found");
                }}
                if (typeof extractor.extract !== "function") {{
                    throw new Error("extract function not found on extractor");
                }}
                return await extractor.extract({}, {});
            }})()
            "#,
            platform_json, url_json, cookies_json
        );

        let result = self
            .runtime
            .execute_script("extract_call.js", code)
            .map_err(|e| {
                ExtractionError::JavaScriptError(format!("Script execution failed: {}", e))
            })?;

        // Resolve the promise while driving the event loop (required for async ops like op_fetch)
        let resolve_fut = self.runtime.resolve(result);
        let resolved = self
            .runtime
            .with_event_loop_promise(resolve_fut, PollEventLoopOptions::default())
            .await
            .map_err(|e| {
                ExtractionError::JavaScriptError(format!("Promise resolution failed: {}", e))
            })?;

        // Convert to serde_json::Value
        let scope = &mut self.runtime.handle_scope();
        let local = deno_core::v8::Local::new(scope, &resolved);
        let value = deno_core::serde_v8::from_v8::<serde_json::Value>(scope, local).map_err(|e| {
            ExtractionError::JavaScriptError(format!("Failed to deserialize result: {}", e))
        })?;

        parse_extraction_result(value, url)
    }

    /// Extract playlist videos by calling the JS extractPlaylist function.
    pub async fn extract_playlist(
        &mut self,
        platform: &str,
        url: &str,
        cookies: Option<&str>,
    ) -> Result<serde_json::Value, ExtractionError> {
        let platform_json = serde_json::to_string(platform).map_err(|e| {
            ExtractionError::JavaScriptError(format!(
                "Failed to serialize platform argument: {}",
                e
            ))
        })?;
        let url_json = serde_json::to_string(url).map_err(|e| {
            ExtractionError::JavaScriptError(format!("Failed to serialize URL argument: {}", e))
        })?;
        let cookies_json = cookies
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| {
                ExtractionError::JavaScriptError(format!(
                    "Failed to serialize cookies argument: {}",
                    e
                ))
            })?
            .unwrap_or_else(|| "undefined".to_string());

        let code = format!(
            r#"
            (async () => {{
                const extractor = extractors[{}];
                if (!extractor) {{
                    throw new Error("Extractor not found");
                }}
                if (typeof extractor.extractPlaylist !== "function") {{
                    throw new Error("extractPlaylist function not found on extractor");
                }}
                return await extractor.extractPlaylist({}, {});
            }})()
            "#,
            platform_json, url_json, cookies_json
        );

        let result = self
            .runtime
            .execute_script("extract_playlist_call.js", code)
            .map_err(|e| {
                ExtractionError::JavaScriptError(format!("Script execution failed: {}", e))
            })?;

        let resolve_fut = self.runtime.resolve(result);
        let resolved = self
            .runtime
            .with_event_loop_promise(resolve_fut, PollEventLoopOptions::default())
            .await
            .map_err(|e| {
                ExtractionError::JavaScriptError(format!("Promise resolution failed: {}", e))
            })?;

        let scope = &mut self.runtime.handle_scope();
        let local = deno_core::v8::Local::new(scope, &resolved);
        let value = deno_core::serde_v8::from_v8::<serde_json::Value>(scope, local).map_err(|e| {
            ExtractionError::JavaScriptError(format!("Failed to deserialize result: {}", e))
        })?;

        Ok(value)
    }
}

/// Parse the extraction result from JS into VideoInfo
fn parse_extraction_result(
    value: serde_json::Value,
    original_url: &str,
) -> Result<VideoInfo, ExtractionError> {
    let result = value.as_object().ok_or_else(|| {
        ExtractionError::JavaScriptError("Expected object result from extractor".to_string())
    })?;

    let title = result
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown")
        .to_string();

    let channel = result
        .get("channel")
        .and_then(|v| v.as_str())
        .map(String::from);

    let view_count = result.get("viewCount").and_then(parse_u64_value);

    let description = result.get("description").and_then(|v| v.as_str()).map(String::from);

    let duration = result.get("duration").and_then(|v| v.as_u64());

    let thumbnail = result.get("thumbnail").and_then(|v| v.as_str()).map(String::from);

    let streams = result
        .get("streams")
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            ExtractionError::JavaScriptError("Missing streams array in result".to_string())
        })?;

    let mut formats = Vec::new();

    for (idx, stream) in streams.iter().enumerate() {
        let stream_obj = stream.as_object().ok_or_else(|| {
            ExtractionError::JavaScriptError(format!("Stream {} is not an object", idx))
        })?;

        let url = stream_obj
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                ExtractionError::JavaScriptError(format!("Stream {} missing URL", idx))
            })?;

        let quality = stream_obj
            .get("quality")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let format = stream_obj
            .get("format")
            .and_then(|v| v.as_str())
            .unwrap_or("mp4");

        let mime = stream_obj
            .get("mime")
            .and_then(|v| v.as_str())
            .unwrap_or("video/mp4");

        let height = stream_obj.get("height").and_then(|v| v.as_u64()).map(|h| h as u32);

        let width = stream_obj.get("width").and_then(|v| v.as_u64()).map(|w| w as u32);

        let bitrate = stream_obj.get("bitrate").and_then(|v| v.as_u64());
        let filesize = stream_obj
            .get("filesize")
            .and_then(parse_u64_value)
            .or_else(|| stream_obj.get("contentLength").and_then(parse_u64_value))
            .or_else(|| extract_clen_from_url(url));

        let codec = stream_obj.get("codec").and_then(|v| v.as_str()).map(String::from);
        let codec_label = stream_obj.get("codecLabel").and_then(|v| v.as_str()).map(String::from);
        let has_audio = stream_obj.get("hasAudio").and_then(|v| v.as_bool()).unwrap_or(false);
        let is_audio_only = stream_obj.get("isAudioOnly").and_then(|v| v.as_bool()).unwrap_or(false);

        formats.push(VideoFormat {
            format_id: format!("{}-{}", format, idx),
            quality: quality.to_string(),
            vcodec: if mime.contains("video") { codec.clone() } else { None },
            acodec: if mime.contains("audio") { codec } else { None },
            codec_label,
            has_audio,
            is_audio_only,
            width,
            height,
            fps: None,
            bitrate,
            ext: format.to_string(),
            url: url.to_string(),
            filesize,
        });
    }

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

/// Parse an integer-like JSON value to u64.
///
/// Accepts either number (`123`) or string (`"123"`).
fn parse_u64_value(value: &serde_json::Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|s| s.parse::<u64>().ok()))
}

/// Extract `clen` (content length) from CDN URL query params.
fn extract_clen_from_url(url: &str) -> Option<u64> {
    let parsed = reqwest::Url::parse(url).ok()?;
    parsed
        .query_pairs()
        .find(|(k, _)| k == "clen")
        .and_then(|(_, v)| v.parse::<u64>().ok())
}

/// Allowed domains for HTTP fetch (security whitelist)
const ALLOWED_DOMAINS: &[&str] = &[
    "youtube.com",
    "www.youtube.com",
    "youtu.be",
    "googlevideo.com",     // YouTube CDN
];

/// Validate URL against allowed domains
fn validate_url(url: &str) -> Result<(), anyhow::Error> {
    let parsed = reqwest::Url::parse(url)
        .map_err(|e| anyhow::anyhow!("Invalid URL: {}", e))?;

    let host = parsed.host_str().unwrap_or("");

    let is_allowed = ALLOWED_DOMAINS.iter().any(|domain| {
        host == *domain || host.ends_with(&format!(".{}", domain))
    });

    if !is_allowed {
        return Err(anyhow::anyhow!(
            "Domain not allowed: {}. Allowed domains: {:?}",
            host,
            ALLOWED_DOMAINS
        ));
    }

    Ok(())
}

/// Cached direct client (no proxy).
static DIRECT_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
/// Cached SOCKS5 client (built lazily from `SOCKS5_PROXY_URL` env var).
static SOCKS5_CLIENT: OnceLock<Option<reqwest::Client>> = OnceLock::new();

/// Get the appropriate cached reqwest client for the given URL.
/// Routes youtube.com through SOCKS5 if `SOCKS5_PROXY_URL` is set.
/// All other domains use the direct client.
fn get_fetch_client(url: &str) -> Result<&'static reqwest::Client, anyhow::Error> {
    if should_use_socks5(url) {
        let socks5 = SOCKS5_CLIENT.get_or_init(|| {
            std::env::var("SOCKS5_PROXY_URL").ok().and_then(|socks5_url| {
                let proxy = reqwest::Proxy::all(&socks5_url).ok()?;
                reqwest::Client::builder()
                    .timeout(Duration::from_secs(90))
                    .proxy(proxy)
                    .build()
                    .ok()
            })
        });
        if let Some(client) = socks5 {
            debug!("Routing {} through SOCKS5 proxy", url);
            return Ok(client);
        }
        // Fallback to direct if SOCKS5 not configured or build failed
    }

    Ok(DIRECT_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(90))
            .build()
            .expect("Failed to build direct HTTP client")
    }))
}

/// Returns true if the URL's host matches youtube.com or subdomains.
/// These domains are routed through SOCKS5 proxy to avoid rate-limiting
/// on home server IPs.
fn should_use_socks5(url: &str) -> bool {
    let Ok(parsed) = reqwest::Url::parse(url) else { return false };
    let host = parsed.host_str().unwrap_or("");
    host == "youtube.com" || host.ends_with(".youtube.com") || host == "youtu.be"
}

/// HTTP fetch operation exposed to JavaScript
#[op2(async)]
#[serde]
async fn op_fetch(
    #[string] url: String,
    #[serde] options: Option<FetchOptions>,
) -> Result<FetchResponse, anyhow::Error> {
    // Security: validate domain whitelist
    if let Err(e) = validate_url(&url) {
        warn!("Blocked fetch to non-allowed domain: {}", e);
        return Ok(FetchResponse {
            ok: false,
            status: 403,
            status_text: format!("Forbidden: {}", e),
            headers: HashMap::new(),
            body: String::new(),
            url: url.clone(),
        });
    }

    let opts = options.unwrap_or_default();
    let method = opts.method.unwrap_or_else(|| "GET".to_string());

    // Build headers
    let mut headers = HeaderMap::new();
    if let Some(hdrs) = opts.headers {
        for (key, value) in hdrs {
            if let (Ok(name), Ok(val)) = (
                HeaderName::from_bytes(key.as_bytes()),
                HeaderValue::from_str(&value),
            ) {
                headers.insert(name, val);
            }
        }
    }

    // Get cached client; routes youtube.com through SOCKS5 if configured
    let client = get_fetch_client(&url)?;

    let mut request = client.request(
        reqwest::Method::from_bytes(method.as_bytes())?,
        &url,
    );

    request = request.headers(headers);

    if let Some(body) = opts.body {
        request = request.body(body);
    }

    let response = request.send().await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let url = resp.url().to_string();

            // Convert headers
            let mut header_map = HashMap::new();
            for (key, value) in resp.headers() {
                if let Ok(val) = value.to_str() {
                    header_map.insert(key.to_string(), val.to_string());
                }
            }

            // Get body as text (reqwest auto-decompresses gzip/brotli/deflate)
            let body = resp.text().await.unwrap_or_default();

            debug!("op_fetch {} -> status={}, body_len={}, body_preview={:?}",
                url, status.as_u16(), body.len(),
                &body.chars().take(200).collect::<String>());

            Ok(FetchResponse {
                ok: status.is_success(),
                status: status.as_u16(),
                status_text: status.canonical_reason().unwrap_or("Unknown").to_string(),
                headers: header_map,
                body,
                url,
            })
        }
        Err(e) => {
            warn!("Fetch failed for {}: {}", url, e);
            Ok(FetchResponse {
                ok: false,
                status: 0,
                status_text: e.to_string(),
                headers: HashMap::new(),
                body: String::new(),
                url,
            })
        }
    }
}

/// Logging operation exposed to JavaScript
#[op2(fast)]
fn op_log(#[string] level: String, #[string] message: String) {
    match level.as_str() {
        "debug" => debug!("[JS] {}", message),
        "info" => info!("[JS] {}", message),
        "warn" => warn!("[JS] {}", message),
        "error" => error!("[JS] {}", message),
        _ => info!("[JS] {}", message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_should_use_socks5_youtube() {
        assert!(should_use_socks5("https://youtube.com/watch?v=abc123"));
    }

    #[test]
    fn test_should_use_socks5_www_youtube() {
        assert!(should_use_socks5("https://www.youtube.com/watch?v=abc123"));
    }

    #[test]
    fn test_should_use_socks5_youtu_be() {
        assert!(should_use_socks5("https://youtu.be/abc123"));
    }

    #[test]
    fn test_should_use_socks5_googlevideo_false() {
        assert!(!should_use_socks5("https://rr1---sn-abc.googlevideo.com/videoplayback?..."));
    }

    #[test]
    fn test_should_use_socks5_unknown_false() {
        assert!(!should_use_socks5("https://example.com/page"));
    }

    #[test]
    fn test_should_use_socks5_invalid_url_false() {
        assert!(!should_use_socks5("not-a-valid-url"));
    }

    #[test]
    fn test_parse_u64_value_number_and_string() {
        assert_eq!(parse_u64_value(&json!(12345)), Some(12_345));
        assert_eq!(parse_u64_value(&json!("67890")), Some(67_890));
        assert_eq!(parse_u64_value(&json!("not-a-number")), None);
    }

    #[test]
    fn test_extract_clen_from_url() {
        let url = "https://rr1.googlevideo.com/videoplayback?clen=20971520&mime=video%2Fmp4";
        assert_eq!(extract_clen_from_url(url), Some(20_971_520));
        assert_eq!(extract_clen_from_url("https://example.com/video?foo=bar"), None);
        assert_eq!(extract_clen_from_url("not-a-url"), None);
    }
}
