# Phase 03 — In-memory URL Cache (moka, TTL 5 min)

## Overview

- **Priority:** P2
- **Status:** pending
- **Effort:** ~45 min
- **Optimization:** #3

## Key Insights

- YouTube stream URLs are valid for ~6 hours → 5-min TTL is safe, no stale-URL risk
- `moka` (async, Tokio-aware) vs `dashmap` + manual TTL: moka handles expiry natively, no custom cleanup task needed
- Cache key = normalized URL: extract `v=VIDEO_ID` → canonical form `https://www.youtube.com/watch?v=ID` → strips tracking params (`&t=`, `&si=`, `&app=`, etc.) → tăng cache hit rate đáng kể
- `Arc<VideoInfo>` avoids cloning the full struct on every cache hit
- `OnceLock<Cache<...>>` pattern mirrors phase 02 semaphore; consistent with codebase style
- Use `try_get_with()` instead of `get() + insert()` — prevents cache stampede (thundering herd): concurrent requests for same URL share one in-flight future, only first spawns yt-dlp subprocess; others wait and receive same result
- Failed extractions are NOT cached: `try_get_with()` only caches on `Ok(...)`, propagates `Err` to all waiting callers without storing

## Requirements

- Cache type: `moka::future::Cache<String, Arc<VideoInfo>>`
- TTL: 5 minutes (`Duration::from_secs(300)`)
- Max capacity: 500 entries (each ~2KB JSON parsed → ~1MB ceiling)
- Cache populated only on successful extraction
- On cache hit: return immediately, skip semaphore + subprocess

## Files to Modify

- `crates/extractor/src/ytdlp.rs`
- `crates/extractor/Cargo.toml`

## New Dependency

```toml
# crates/extractor/Cargo.toml [dependencies]
moka = { version = "0.12", features = ["future"] }
```

`features = ["future"]` enables `moka::future::Cache` which integrates with Tokio async.

## Implementation Steps

### 1. Add `moka` to `crates/extractor/Cargo.toml`

```toml
moka = { version = "0.12", features = ["future"] }
```

### 2. Add URL normalization helper

YouTube video IDs are always exactly 11 chars. Extract `v=` param to strip tracking noise:

```rust
/// Normalize YouTube URL to canonical cache key.
/// Strips tracking params (&t=, &si=, &app= etc.) to maximize cache hit rate.
/// Falls back to raw URL for non-standard or non-YouTube URLs.
fn normalize_cache_key(url: &str) -> String {
    if let Some(v_pos) = url.find("v=") {
        let id_start = v_pos + 2;
        let id_end = url[id_start..].find('&')
            .map(|i| id_start + i)
            .unwrap_or(url.len());
        let video_id = &url[id_start..id_end];
        if video_id.len() == 11 {
            return format!("https://www.youtube.com/watch?v={}", video_id);
        }
    }
    url.to_string()
}
```

No new dependencies — pure string slicing. Call this in `extract_via_ytdlp()` before cache lookup.

### 3. Add cache static + init in `ytdlp.rs`

```rust
use moka::future::Cache;
use std::time::Duration;

static EXTRACT_CACHE: OnceLock<Cache<String, Arc<VideoInfo>>> = OnceLock::new();

fn get_cache() -> &'static Cache<String, Arc<VideoInfo>> {
    EXTRACT_CACHE.get_or_init(|| {
        Cache::builder()
            .max_capacity(500)
            .time_to_live(Duration::from_secs(300)) // 5 minutes
            .build()
    })
}
```

Note: `Arc` and `OnceLock` imports already added in phase 02.

### 4. Update `extract_via_ytdlp` — use `try_get_with()` for stampede prevention

`try_get_with()` guarantees only **one** yt-dlp subprocess runs per URL at a time. Concurrent callers for the same URL block until the first completes, then receive the cached result. On error, all waiters receive the same `Err` without caching it.

```rust
pub async fn extract_via_ytdlp(url: &str) -> Result<VideoInfo, ExtractionError> {
    debug!("yt-dlp extracting: {}", url);

    let key = normalize_cache_key(url);

    // try_get_with: only first caller spawns subprocess; others wait and reuse result.
    // On Err: not cached, all waiters receive the error.
    // map_err unwraps Arc<ExtractionError> → preserves original variant for telemetry/debug.
    // Requires ExtractionError: Clone (mandatory: derive Clone on enum).
    get_cache()
        .try_get_with(key, extract_subprocess(url))
        .await
        .map(|arc| (*arc).clone())
        .map_err(|e: Arc<ExtractionError>| (*e).clone())
}

/// Inner function: acquires semaphore, runs yt-dlp, parses JSON.
/// Separated so try_get_with() can wrap it cleanly.
async fn extract_subprocess(url: &str) -> Result<Arc<VideoInfo>, ExtractionError> {
    // Semaphore — cap concurrent subprocesses (phase 02)
    let _permit = get_semaphore()
        .acquire()
        .await
        .map_err(|e| ExtractionError::ScriptExecutionFailed(
            format!("semaphore acquire failed: {}", e)
        ))?;

    let mut cmd = build_command(url);
    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| ExtractionError::ScriptExecutionFailed(
            format!("yt-dlp launch failed: {}", e)
        ))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!("yt-dlp failed for {}: {}", url, stderr.trim());
        return Err(ExtractionError::ScriptExecutionFailed(
            format!("yt-dlp error: {}", stderr.trim())
        ));
    }

    let json: Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| ExtractionError::ScriptExecutionFailed(
            format!("yt-dlp JSON parse error: {}", e)
        ))?;

    let video_info = parse_ytdlp_json(json, url)?;
    debug!("cache populated for: {}", url);
    Ok(Arc::new(video_info))
}
```

**Note:** `try_get_with()` requires the error type to implement `Send + Sync + 'static`. `ExtractionError` must derive/implement these — verify in types file.

### 5. ~~Verify `VideoInfo` derives `Clone`~~ — Already confirmed ✅

`crates/extractor/src/types.rs` already has `#[derive(Debug, Clone, Serialize, Deserialize)]` on both `VideoInfo` and `VideoFormat`. No change needed.

**Memory safety note:** `VideoInfo` contains only `String` fields and `Vec<VideoFormat>` (which also only has `String`/primitive fields). No large buffers, no stream handles. 500 entries × ~3KB average = ~1.5MB max — negligible.

## Todo List

- [ ] Add `moka = { version = "0.12", features = ["future"] }` to `crates/extractor/Cargo.toml`
- [ ] Add `use moka::future::Cache;` and `use std::time::Duration;` imports
- [ ] Add `EXTRACT_CACHE` static + `get_cache()` fn
- [x] `VideoInfo` + `VideoFormat` derive `Clone` — confirmed, no change needed
- [ ] Verify `ExtractionError` implements `Send + Sync + 'static` (required by `try_get_with`)
- [ ] Thêm `#[derive(Clone)]` vào `ExtractionError` — bắt buộc cho `map_err(|e: Arc<ExtractionError>| (*e).clone())`
- [ ] Add `normalize_cache_key()` function
- [ ] Refactor: split `extract_subprocess()` as inner fn, wrap with `try_get_with(key, ...)` in `extract_via_ytdlp()`
- [ ] Run `cargo check -p extractor`
- [ ] Run `cargo test -p extractor`

## Success Criteria

- `cargo check -p extractor` clean
- Second call with same URL within 5 min returns instantly (no subprocess spawn)
- After 5 min, next call triggers fresh subprocess
- Failed extractions are NOT cached

## Risk Assessment

- `moka` adds ~2MB to binary — acceptable
- Stampede prevention: `try_get_with()` serializes concurrent requests for same URL — only 1 subprocess spawns, others wait
- `Arc<VideoInfo>.clone()` on cache hit is cheap (pointer copy, not deep clone)
- `ExtractionError` phải `Send + Sync + 'static` — verify, add derives nếu thiếu

## Security Considerations

- Cache key là normalized URL (canonical `watch?v=ID`) — URL đã được validate ở `extract_handler` trước khi vào đây, không có injection risk
- Max capacity 500 prevents unbounded memory growth
