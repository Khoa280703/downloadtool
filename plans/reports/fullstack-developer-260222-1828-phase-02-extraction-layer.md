## Phase Implementation Report

### Executed Phase
- Phase: phase-02-extraction-layer
- Plan: /home/khoa2807/working-sources/downloadtool/plans/260222-1238-video-downloader
- Status: completed

### Files Modified

| File | Lines | Description |
|------|-------|-------------|
| crates/extractor/Cargo.toml | +6 | Added notify, num_cpus, serde_v8 deps |
| crates/extractor/src/lib.rs | 180 | Public API with extract() function |
| crates/extractor/src/runtime.rs | 278 | deno_core JsRuntime with op_fetch/op_log |
| crates/extractor/src/pool.rs | 146 | Isolate pool with Semaphore |
| crates/extractor/src/hot_reload.rs | 192 | File watcher with notify |
| crates/extractor/build.rs | 165 | Compile-time bundling script |
| extractors/types.ts | 56 | Shared TypeScript interfaces |
| extractors/youtube.ts | 227 | YouTube extraction logic |
| extractors/tiktok.ts | 357 | TikTok extraction logic |
| Makefile | 120 | esbuild bundling targets |

### Tasks Completed
- [x] esbuild bundler setup (Makefile target)
- [x] TypeScript types + extractor interface
- [x] deno_core JsRuntime with op_fetch
- [x] Isolate pool (Semaphore-based)
- [x] Hot-reload watcher
- [x] YouTube extractor TS
- [x] TikTok extractor TS
- [ ] Integration test (pending runtime verification)

### Tests Status
- Type check: pending (Rust not installed in environment)
- Unit tests: included in source files
- Integration tests: pending

### Implementation Details

#### Rust Modules
1. **runtime.rs**: JsRuntime wrapper with deno_core
   - op_fetch: HTTP client via reqwest with 30s timeout
   - op_log: tracing integration for JS console
   - Promise resolution to Rust Futures

2. **pool.rs**: Concurrent extraction pool
   - Semaphore-based concurrency control
   - Pool size defaults to num_cpus::get()
   - Each extraction gets fresh JsRuntime

3. **hot_reload.rs**: File watching
   - notify::RecommendedWatcher on extractors/dist/
   - tokio::sync::watch channel for signals
   - ReloadableBundle for auto-reload

4. **lib.rs**: Public API
   - extract(url, cookies) -> Result<VideoInfo>
   - Platform auto-detection from URL
   - Global pool with OnceLock

#### TypeScript Extractors
1. **youtube.ts**: ytInitialPlayerResponse parsing
   - Multiple URL format support
   - adaptiveFormats + formats extraction
   - Quality sorting (1080p first)

2. **tiktok.ts**: Multi-strategy extraction
   - URL resolution for short links
   - __INITIAL_STATE__ parsing
   - __UNIVERSAL_DATA_FOR_REHYDRATION__ fallback
   - Embed page fallback
   - Watermarked + no-watermark streams

#### Build System
- Makefile with bundle-extractors target
- build.rs for compile-time bundling
- esbuild with ESM format, neutral platform

### Issues Encountered
1. Rust environment not available for compilation check
2. serde_v8 version matched to deno_core 0.295
3. Extractor registry naming in bundled JS may need adjustment based on esbuild output

### Next Steps
1. Run cargo check to verify compilation
2. Run cargo test for unit tests
3. Create integration test with real URLs
4. Phase 03 can begin (Stream Proxy)

### Unresolved Questions
- Should op_fetch whitelist domains (youtube.com, tiktok.com)?
- Should we add signature deciphering for YouTube cipher URLs?
- Integration test strategy for external URLs?
