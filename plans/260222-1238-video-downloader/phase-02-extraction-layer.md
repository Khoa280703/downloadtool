# Phase 02 — Extraction Layer (deno_core V8)

## Context
- Plan: [plan.md](plan.md)
- Prev: [phase-01-project-scaffold.md](phase-01-project-scaffold.md)
- Next: [phase-03-stream-proxy.md](phase-03-stream-proxy.md)
- Research: [research/researcher-01-rust-core-stack.md](research/researcher-01-rust-core-stack.md)

## Overview
- **Priority**: P0
- **Status**: completed
- **Effort**: 2d
- Embed V8 via deno_core into Rust; load TypeScript extractors at startup; hot-reload on file change; extract direct video URLs from YouTube/TikTok links.

## Key Insights
- `deno_core` 0.295+ maps JS Promises → Rust Futures natively
- No built-in hot-reload → use `notify` crate to watch `extractors/` dir
- V8 Isolates are single-threaded; use one isolate per worker thread in a pool
- Real bottleneck: HTTP fetch to platform (500ms-2s), not JS execution

## Architecture

```
ExtractorPool (N workers = num_cpus)
  └── each worker owns: JsRuntime (deno_core Isolate)
            ↓ loaded once at startup
      TypeScript extractors (bundled JS via esbuild)
            ↓ hot-reload via notify watcher
      extract(url) → ExtractionResult { streams, title, format }
```

## Related Code Files
- `crates/extractor/src/lib.rs` — public API
- `crates/extractor/src/pool.rs` — isolate pool
- `crates/extractor/src/runtime.rs` — deno_core JsRuntime setup
- `crates/extractor/src/hot_reload.rs` — file watcher
- `extractors/youtube.ts` — YouTube extractor logic
- `extractors/tiktok.ts` — TikTok extractor logic
- `extractors/types.ts` — shared TypeScript types
- `Makefile` — esbuild bundling targets
- `crates/extractor/build.rs` — compile-time bundling

## Implementation Steps

1. **Add deno_core to `crates/extractor/Cargo.toml`**
   ```toml
   deno_core = "0.295"
   notify = "6"        # file watcher
   tokio = { features = ["full"] }
   serde_v8 = "0.195"  # v8 serialization
   num_cpus = "1"
   ```

2. **TypeScript extractor interface** (`extractors/types.ts`)
   - Stream, ExtractionResult interfaces
   - ExtractFn type
   - ExtractionError class

3. **Bundle extractors** — use `esbuild` (CLI) at build time to bundle TS → single JS file per platform. Output to `extractors/dist/`.

4. **JsRuntime setup** (`crates/extractor/src/runtime.rs`)
   - Create `deno_core::JsRuntime` with `RuntimeOptions`
   - Register Rust ops: `op_fetch` (HTTP fetch via reqwest), `op_log`
   - Load bundled extractor JS via `runtime.execute_script()`
   - Parse extraction results into VideoInfo

5. **Isolate pool** (`crates/extractor/src/pool.rs`)
   - `tokio::sync::Semaphore` to bound concurrent extractions
   - Each task runs in its own `JsRuntime` (not shared — isolates aren't Send)
   - Pool size = `num_cpus::get()`
   - PoolHandle for sharing across tasks

6. **Hot-reload watcher** (`crates/extractor/src/hot_reload.rs`)
   - `notify::RecommendedWatcher` on `extractors/dist/`
   - On change: re-read JS files, signal pool workers to reload on next use
   - Use `tokio::sync::watch` channel to broadcast reload signal
   - ReloadableBundle for automatic reloading

7. **Public API** (`crates/extractor/src/lib.rs`)
   ```rust
   pub async fn extract(url: &str, cookies: Option<&str>) -> Result<VideoInfo, ExtractionError>
   pub async fn extract_with_platform(platform: &str, url: &str, cookies: Option<&str>) -> Result<VideoInfo, ExtractionError>
   ```

8. **YouTube extractor** (`extractors/youtube.ts`)
   - Fetch `https://www.youtube.com/watch?v={id}` with real UA
   - Parse `ytInitialPlayerResponse` from page HTML
   - Extract `streamingData.adaptiveFormats` (separate audio+video for 1080p+)
   - Return streams sorted by quality

9. **TikTok extractor** (`extractors/tiktok.ts`)
   - Resolve shortened URLs to canonical
   - Fetch TikTok page with auth cookies
   - Parse `__INITIAL_STATE__` or `__UNIVERSAL_DATA_FOR_REHYDRATION__`
   - Extract `video.playAddr` and `video.downloadAddr`
   - Handle both watermarked and non-watermarked URLs

## Todo
- [x] esbuild bundler setup (Makefile target)
- [x] TypeScript types + extractor interface
- [x] deno_core JsRuntime with op_fetch
- [x] Isolate pool (Semaphore-based)
- [x] Hot-reload watcher
- [x] YouTube extractor TS
- [x] TikTok extractor TS
- [ ] Integration test: extract YouTube URL → assert streams non-empty

## Success Criteria
- Extract YouTube 1080p+audio URLs in <2s
- Extract TikTok no-watermark URL in <1s
- Hot-reload: update `youtube.ts` → running server picks up within 2s
- Pool handles 100 concurrent extract() calls without panic

## Risk Assessment
| Risk | Mitigation |
|---|---|
| deno_core API breaks between versions | Pin exact version in Cargo.lock |
| YouTube page structure changes | Fallback to `ytdl-core` JS lib bundled via esbuild |
| TikTok auth cookie requirement | Accept cookies param from user; document setup |
| V8 Isolate memory leaks | Drop JsRuntime after use; use pool with bounded size |

## Security
- `op_fetch` in JS runtime: whitelist allowed domains (youtube.com, tiktok.com only)
- Never expose raw extraction URLs to frontend without validation
