# Phase 01 — yt-dlp Command Args + Binary Path Auto-detection

## Overview

- **Priority:** P2
- **Status:** pending
- **Effort:** ~30 min
- **Optimizations:** #1 (faster extractor args) + #5 (portable binary path)

## Key Insights

- `android_embedded` player client skips PO Token overhead — lighter than default `web` client
- `skip=hls,dash` was considered but **removed** — YouTube giấu format 4K/1080p trong DASH manifest; bỏ DASH = mất format cao nhất
- Two `--extractor-args` flags for same extractor override each other → must combine with `;` separator
- `--socket-timeout 30→15` is safe for residential broadband; reduces worst-case hang time
- Hardcoded `/home/khoa2807/.local/bin/yt-dlp` breaks on any other system; env var + `which` fallback is standard practice

## Requirements

- `build_command()` emits the new args in correct order
- Binary resolution: check `YTDLP_PATH` env var first → fallback to `PATH` (`yt-dlp`)
- No change to function signatures or return types
- Existing tests must still pass

## Files to Modify

- `crates/extractor/src/ytdlp.rs`

## Implementation Steps

### 1. Replace `YTDLP_BINARY` constant and `build_command()` binary resolution

**Current code (lines 13, 45-49):**
```rust
const YTDLP_BINARY: &str = "/home/khoa2807/.local/bin/yt-dlp";

// inside build_command():
let binary = if std::path::Path::new(YTDLP_BINARY).exists() {
    YTDLP_BINARY
} else {
    "yt-dlp"
};
```

**Replace with:**
```rust
/// Resolve yt-dlp binary: YTDLP_PATH env var → system PATH fallback
fn resolve_ytdlp_binary() -> String {
    if let Ok(path) = std::env::var("YTDLP_PATH") {
        if !path.is_empty() && std::path::Path::new(&path).exists() {
            return path;
        }
    }
    // Fallback: rely on system PATH
    "yt-dlp".to_string()
}
```

Remove the `YTDLP_BINARY` constant entirely. Call `resolve_ytdlp_binary()` inside `build_command()`.

### 2. Update `build_command()` args

**Current:**
```rust
cmd.args([
    "-J", "--no-playlist", "--no-warnings",
    "--socket-timeout", "30",
    "--no-check-certificates",
]);
```

**Replace with:**
```rust
cmd.args([
    "-J", "--no-playlist", "--no-warnings",
    "--extractor-args", "youtube:player_client=android_embedded,web",
    "--socket-timeout", "15",
    "--no-check-certificates",
]);
```

### 3. Updated `build_command()` full body

```rust
fn build_command(url: &str) -> Command {
    let binary = resolve_ytdlp_binary();
    let mut cmd = Command::new(&binary);
    cmd.args([
        "-J",
        "--no-playlist",
        "--no-warnings",
        "--extractor-args", "youtube:player_client=android_embedded,web",
        "--socket-timeout", "15",
        "--no-check-certificates",
    ]);

    if let Ok(proxy) = std::env::var("SOCKS5_PROXY_URL") {
        if !proxy.is_empty() {
            cmd.args(["--proxy", &proxy]);
            debug!("yt-dlp routing through proxy: {}", proxy);
        }
    }

    cmd.arg(url);
    cmd
}
```

## Todo List

- [ ] Remove `YTDLP_BINARY` constant
- [ ] Add `resolve_ytdlp_binary()` function
- [ ] Update `build_command()` with new args and binary resolution
- [ ] Run `cargo check -p extractor` — no errors
- [ ] Run `cargo test -p extractor` — existing tests pass

## Success Criteria

- `cargo check -p extractor` clean
- All existing `#[cfg(test)]` tests in `ytdlp.rs` pass (they don't invoke the subprocess)
- Log shows correct binary path at runtime

## Risk Assessment

- `android_embedded` may not support all video types — mitigated by `,web` fallback in the same `--extractor-args`
- Timeout reduction 30→15: if a video source is slow, extraction may fail → acceptable tradeoff for snappier UX
- `skip=hls,dash` **NOT used** — would strip DASH formats containing 4K/1080p HDR; JSON parse cost (~1-2ms in Rust) không đáng để hi sinh format quality

## Security Considerations

- `YTDLP_PATH` env var is operator-controlled; no user input involved
- No shell expansion (`Command::new()` does not use shell)
