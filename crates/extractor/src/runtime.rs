//! deno_core JsRuntime setup for JavaScript extractors
//!
//! This module initializes a V8 isolate with custom ops for HTTP fetching
//! and logging, then loads the bundled TypeScript extractor scripts.

use crate::types::{ExtractionError, VideoFormat, VideoInfo};
use deno_core::{op2, JsRuntime, PollEventLoopOptions, RuntimeOptions};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
        let code = format!(
            r#"
            (async () => {{
                const extractor = extractors["{}"];
                if (!extractor) {{
                    throw new Error(`Extractor for '{}' not found`);
                }}
                return await extractor.extract("{}", {});
            }})()
            "#,
            platform,
            platform,
            url,
            cookies
                .map(|c| format!("\"{}\"", c.replace('"', "\\\"")))
                .unwrap_or_else(|| "undefined".to_string())
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
            filesize: None,
        });
    }

    Ok(VideoInfo {
        title,
        description,
        duration,
        thumbnail,
        formats,
        original_url: original_url.to_string(),
    })
}

/// Allowed domains for HTTP fetch (security whitelist)
const ALLOWED_DOMAINS: &[&str] = &[
    "youtube.com",
    "www.youtube.com",
    "youtu.be",
    "tiktok.com",
    "www.tiktok.com",
    "vm.tiktok.com",
    "googlevideo.com",     // YouTube CDN
    "tiktokcdn.com",       // TikTok CDN
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

    // Create client with timeout
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(90))
        .build()?;

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
