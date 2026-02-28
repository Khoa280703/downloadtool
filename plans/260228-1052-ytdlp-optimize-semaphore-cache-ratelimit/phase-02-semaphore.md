# Phase 02 — Tokio Semaphore (Concurrent yt-dlp Cap)

## Overview

- **Priority:** P1
- **Status:** pending
- **Effort:** ~30 min
- **Optimization:** #2

## Key Insights

- Each yt-dlp subprocess: ~100MB RSS + ~1 CPU core during extraction
- Without a cap, a loop caller can spawn unlimited subprocesses → OOM / server crash
- `tokio::sync::Semaphore` is already in `tokio` (no new dep) → zero cost addition
- `OnceLock<Arc<Semaphore>>` = safe global static init on first use
- `SemaphorePermit` is dropped at end of scope → automatic release

## Requirements

- Max `MAX_CONCURRENT_YTDLP = 10` concurrent yt-dlp subprocesses
- If semaphore is exhausted, request blocks (not rejected) — consistent with how expensive work queues behave
- No change to `extract_via_ytdlp` function signature

## Files to Modify

- `crates/extractor/src/ytdlp.rs`

## No New Dependencies

`tokio::sync::Semaphore` is part of `tokio = { features = ["full"] }` already in workspace.

## Implementation Steps

### 1. Add static semaphore at top of file

After the existing `use` imports, add:

```rust
use std::sync::{Arc, OnceLock};
use tokio::sync::Semaphore;

static YTDLP_SEMAPHORE: OnceLock<Arc<Semaphore>> = OnceLock::new();
const MAX_CONCURRENT_YTDLP: usize = 10;

fn get_semaphore() -> &'static Arc<Semaphore> {
    YTDLP_SEMAPHORE.get_or_init(|| Arc::new(Semaphore::new(MAX_CONCURRENT_YTDLP)))
}
```

### 2. Wrap subprocess execution in `extract_via_ytdlp`

**Current flow:**
```rust
pub async fn extract_via_ytdlp(url: &str) -> Result<VideoInfo, ExtractionError> {
    debug!("yt-dlp extracting: {}", url);
    let mut cmd = build_command(url);
    let output = cmd.stdout(...).stderr(...).output().await...;
    ...
}
```

**Updated flow — acquire permit before spawning:**
```rust
pub async fn extract_via_ytdlp(url: &str) -> Result<VideoInfo, ExtractionError> {
    debug!("yt-dlp extracting: {}", url);

    // Acquire semaphore permit — blocks if MAX_CONCURRENT_YTDLP already running
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

    // _permit dropped here → semaphore released
    ...rest unchanged...
}
```

The `_permit` binding keeps the permit alive until the function returns, regardless of success or error path.

## Todo List

- [ ] Add `use std::sync::{Arc, OnceLock};` import
- [ ] Add `use tokio::sync::Semaphore;` import (if not already present via glob)
- [ ] Add `YTDLP_SEMAPHORE`, `MAX_CONCURRENT_YTDLP`, `get_semaphore()`
- [ ] Wrap subprocess call in `extract_via_ytdlp` with permit acquisition
- [ ] Run `cargo check -p extractor`
- [ ] Run `cargo test -p extractor`

## Success Criteria

- `cargo check -p extractor` clean
- Under concurrent load, at most 10 yt-dlp processes visible via `ps aux | grep yt-dlp`
- Excess requests queue (do not error) and execute after a slot frees

## Risk Assessment

- If all 10 slots are held by slow extractions, new requests block indefinitely → acceptable; add timeout layer at HTTP level separately if needed
- `OnceLock` is `Send + Sync` — safe for multi-threaded Tokio runtime

## Security Considerations

- Semaphore prevents resource exhaustion (DoS via process flooding)
- Does not expose any new surface area
