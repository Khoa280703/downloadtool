# Phase 02: SOCKS5 Selective Routing

## Context Links
- Plan overview: `./plan.md`
- JS extraction runtime: `crates/extractor/src/runtime.rs` — `op_fetch` fn (line 298)
- Proxy HTTP client: `crates/proxy/src/client.rs`
- Anti-bot client: `crates/proxy/src/anti_bot.rs` — `build_client()` (line 94)
- Proxy pool: `crates/proxy/src/proxy_pool.rs`

## Overview

- **Priority:** P1 (extraction reliability on home server)
- **Status:** pending
- **Problem:** JS extractor (`op_fetch`) makes all HTTP calls directly — no proxy. On a home server, `youtube.com` requests get rate-limited/bot-detected. CDN downloads (`googlevideo.com`) must stay direct to use Oracle's 10TB bandwidth.
- **Solution:** Two independent routing rules — one in the JS runtime layer (extraction), one already correct in the Rust streaming layer.

## Key Insights

1. **Two completely separate code paths** — no shared abstraction needed (YAGNI):
   - `op_fetch` in `runtime.rs`: handles YouTube page/API requests (JS extractor)
   - `AntiBotClient` in `anti_bot.rs`: handles CDN stream fetching (Rust streaming)

2. **CDN path already correct** — `AntiBotClient` fetches `googlevideo.com` directly. No change needed there. The `ProxyPool::from_env()` in `anti_bot.rs::new()` reads proxy env vars, but we don't want SOCKS5 for CDN. Confirm `SOCKS5_PROXY_URL` is NOT the same env var as any existing proxy pool var.

3. **`op_fetch` builds a fresh `reqwest::Client` per call** (line 332-334) — no shared state. Adding SOCKS5 is simply: detect domain, set `Proxy::socks5(url)` on the builder. No refactor needed.

4. **Env var `SOCKS5_PROXY_URL` is optional** — if unset, `op_fetch` behaves exactly as today. Zero regression risk.

5. **Domain detection in `op_fetch` is already structured** — `validate_url()` already parses the URL and extracts host (line 274-292). Reuse the same `reqwest::Url::parse()` + `host_str()` pattern.

6. **`reqwest` SOCKS5 support** requires the `socks` feature flag — must add to `Cargo.toml` of the extractor crate.

## Requirements

### Functional
- `youtube.com/*` and `*.youtube.com` → route through `SOCKS5_PROXY_URL` (if set)
- `googlevideo.com/*` → always direct (no proxy) — already the case, must not regress
- `SOCKS5_PROXY_URL` not set → all `op_fetch` calls go direct (no behaviour change)
- `SOCKS5_PROXY_URL` format: `socks5://user:pass@host:port` or `socks5://host:port`

### Non-Functional
- Zero overhead on the hot CDN download path (no proxy lookup for `googlevideo.com`)
- No new structs, no new files — inline change in `op_fetch` only
- `reqwest` client creation stays per-call (existing pattern) — SOCKS5 client not cached (acceptable: extraction is infrequent, not per-byte)

## Architecture

### Routing Table

| Domain | JS extraction (`op_fetch`) | Rust streaming (`AntiBotClient`) |
|---|---|---|
| `youtube.com`, `*.youtube.com` | SOCKS5 proxy (if `SOCKS5_PROXY_URL` set) | N/A (not fetched here) |
| `googlevideo.com`, `*.googlevideo.com` | direct (already allowed, no proxy needed) | direct (already correct) |
| `youtube.com`, `youtubecdn.com`, etc. | direct (no change) | direct (already correct) |

### Change in `op_fetch` (runtime.rs)

Current flow (line 332-347):
```rust
let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(90))
    .build()?;
```

New flow:
```rust
let client = build_fetch_client(&url)?;
```

New helper (add just above `op_fetch`):
```rust
/// Build reqwest client, routing youtube.com through SOCKS5 if SOCKS5_PROXY_URL is set.
fn build_fetch_client(url: &str) -> Result<reqwest::Client, anyhow::Error> {
    let mut builder = reqwest::Client::builder()
        .timeout(Duration::from_secs(90));

    // Route youtube.com extraction requests through SOCKS5 proxy.
    // googlevideo.com CDN traffic stays direct (not fetched via op_fetch).
    if should_use_socks5(url) {
        if let Ok(socks5_url) = std::env::var("SOCKS5_PROXY_URL") {
            let proxy = reqwest::Proxy::all(&socks5_url)
                .map_err(|e| anyhow::anyhow!("Invalid SOCKS5_PROXY_URL: {}", e))?;
            builder = builder.proxy(proxy);
        }
    }

    builder.build().map_err(Into::into)
}

/// Returns true if the URL's host matches youtube.com or subdomains.
fn should_use_socks5(url: &str) -> bool {
    let Ok(parsed) = reqwest::Url::parse(url) else { return false };
    let host = parsed.host_str().unwrap_or("");
    host == "youtube.com" || host.ends_with(".youtube.com") || host == "youtu.be"
}
```

### `reqwest` Feature Flag

Add `socks` feature to extractor's `Cargo.toml`:
```toml
reqwest = { workspace = true, features = ["socks"] }
```

Check workspace `Cargo.toml` — if `reqwest` is defined there without `socks`, either:
- Add `socks` to workspace default features, or
- Override with `{ workspace = true, features = ["socks"] }` in extractor's `Cargo.toml`

## Related Code Files

**Modify:**
- `crates/extractor/src/runtime.rs` — add `build_fetch_client()` + `should_use_socks5()`, replace inline client build in `op_fetch`
- `crates/extractor/Cargo.toml` — add `socks` feature to `reqwest` dependency

**Do NOT modify:**
- `crates/proxy/src/anti_bot.rs` — CDN path already correct, no SOCKS5 needed
- `crates/proxy/src/client.rs` — unchanged
- `crates/proxy/src/proxy_pool.rs` — unchanged

## Implementation Steps

### Step 1 — Check workspace `Cargo.toml` for `reqwest` definition

```bash
grep -n "reqwest" /home/khoa2807/working-sources/downloadtool/Cargo.toml
```

Determine if `socks` feature is already present. If not, add it.

### Step 2 — Add `socks` feature to extractor `Cargo.toml`

File: `crates/extractor/Cargo.toml`

Find the `reqwest` dependency line and add/ensure `socks` feature:
```toml
reqwest = { workspace = true, features = ["socks"] }
```

If workspace definition already includes `socks`, this step is a no-op.

### Step 3 — Add helpers to `runtime.rs`

In `crates/extractor/src/runtime.rs`, insert just **above** the `op_fetch` function (before line 296):

```rust
/// Build reqwest client, routing youtube.com through SOCKS5 if SOCKS5_PROXY_URL is set.
/// googlevideo.com CDN fetches are handled separately by AntiBotClient (direct, no proxy).
fn build_fetch_client(url: &str) -> Result<reqwest::Client, anyhow::Error> {
    let mut builder = reqwest::Client::builder()
        .timeout(Duration::from_secs(90));

    if should_use_socks5(url) {
        if let Ok(socks5_url) = std::env::var("SOCKS5_PROXY_URL") {
            let proxy = reqwest::Proxy::all(&socks5_url)
                .map_err(|e| anyhow::anyhow!("Invalid SOCKS5_PROXY_URL: {}", e))?;
            builder = builder.proxy(proxy);
        }
    }

    builder.build().map_err(Into::into)
}

/// Returns true if url host is youtube.com or a subdomain (routes through SOCKS5).
fn should_use_socks5(url: &str) -> bool {
    let Ok(parsed) = reqwest::Url::parse(url) else { return false };
    let host = parsed.host_str().unwrap_or("");
    host == "youtube.com" || host.ends_with(".youtube.com") || host == "youtu.be"
}
```

### Step 4 — Replace client build in `op_fetch`

In `op_fetch` (around line 332), replace:
```rust
// Create client with timeout
let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(90))
    .build()?;
```

With:
```rust
// Create client with timeout; routes youtube.com through SOCKS5 if configured
let client = build_fetch_client(&url)?;
```

### Step 5 — Verify compile and test

```bash
cd /home/khoa2807/working-sources/downloadtool
cargo build -p extractor 2>&1
cargo test -p extractor 2>&1
cargo clippy -p extractor -- -D warnings 2>&1
```

### Step 6 — Integration smoke test (manual)

Set env var and run extraction:
```bash
SOCKS5_PROXY_URL=socks5://127.0.0.1:1080 cargo run -- extract "https://www.youtube.com/watch?v=..."
```

Verify:
- YouTube extraction succeeds (goes through SOCKS5)
- CDN download URL (`googlevideo.com`) is direct (check logs — `AntiBotClient` does not use SOCKS5)
- Without `SOCKS5_PROXY_URL` set, extraction still works (direct)

## Todo List

- [ ] Check workspace `Cargo.toml` for existing `reqwest` `socks` feature
- [ ] Add `socks` feature to `crates/extractor/Cargo.toml` reqwest dependency
- [ ] Add `should_use_socks5(url: &str) -> bool` fn to `runtime.rs`
- [ ] Add `build_fetch_client(url: &str) -> Result<reqwest::Client, anyhow::Error>` fn to `runtime.rs`
- [ ] Replace inline `reqwest::Client::builder()...build()` in `op_fetch` with `build_fetch_client(&url)?`
- [ ] Run `cargo build -p extractor` — compiles clean
- [ ] Run `cargo test -p extractor` — all tests pass
- [ ] Run `cargo clippy -p extractor -- -D warnings` — no warnings
- [ ] Add unit tests for `should_use_socks5`: youtube.com → true, www.youtube.com → true, googlevideo.com → false, youtube.com → false
- [ ] Smoke test with `SOCKS5_PROXY_URL` set and unset

## Success Criteria

- `cargo build -p extractor` compiles clean with `socks` feature
- `should_use_socks5` unit tests pass for all domain cases
- `SOCKS5_PROXY_URL` unset → `op_fetch` behaviour identical to today (no regression)
- `SOCKS5_PROXY_URL` set → `youtube.com` requests use SOCKS5, `googlevideo.com` direct
- No change to `AntiBotClient` or CDN streaming path

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|---|---|---|---|
| `reqwest socks` feature not in workspace, conflicts | Low | Build error | Check workspace `Cargo.toml` first; add feature to crate-level dep only |
| SOCKS5 proxy down → extraction fails completely | Medium | Feature unavailable | Document: `SOCKS5_PROXY_URL` should only be set when proxy is reliable; fallback is to unset the var |
| Per-call client build with SOCKS5 adds latency | Very Low | Negligible | Client build is ~1ms; extraction is not latency-critical |
| `youtu.be` redirect leads to `googlevideo.com` CDN, accidentally proxied | N/A | N/A | `youtu.be` resolves to youtube.com; CDN fetches never go through `op_fetch` |

## Security Considerations

- `SOCKS5_PROXY_URL` is read from env — never from user input or request parameters
- SOCKS5 credentials in env var are a standard pattern; document to use a secrets manager in production
- `should_use_socks5` uses strict host matching (no substring match on full URL) — prevents bypass via crafted URLs

## Next Steps

- After both phases: update `docs/system-architecture.md` to document streaming pipeline + SOCKS5 routing table
- Consider: add `SOCKS5_PROXY_URL` to `.env.example` with documentation comment
- Consider: structured logging of which proxy path was used per extraction request (for observability)
