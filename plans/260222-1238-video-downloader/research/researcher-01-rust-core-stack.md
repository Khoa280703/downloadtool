# Rust Core Stack Research: High-Performance Video Downloader

**Date:** 2026-02-22 | **Scope:** Production readiness & integration analysis

---

## 1. tokio-uring

**Crate:** `tokio-uring` | **Status:** NOT production-ready for axum
**Latest:** 0.4.x (first published 2021) | **Maintenance:** Active but limited scope

### Key Findings

- **Production Readiness:** Single-threaded only, designed for Linux io-uring API
- **Axum Incompatible:** !Send primitives conflict with axum's work-stealing tokio runtime
- **Issue:** Axum requires Send tasks; tokio-uring doesn't provide this guarantee
- **Use Case:** File I/O optimization only, NOT network stack

### Verdict

**NOT RECOMMENDED** for video downloader. Use standard tokio + axum instead.

---

## 2. deno_core

**Crate:** `deno_core` | **Latest:** 0.295+
**Backend:** Rusty V8 (stable bindings, 2024+)

### Key Capabilities

- **V8 Embedding:** Direct C++ API bindings via rusty_v8
- **Runtime:** Event loop + ops system mapping JS Promises → Rust Futures
- **Bindings:** Fast API for minimal-overhead Rust↔JS calls
- **Hot Reload:** NOT built-in; requires custom implementation

### API Pattern (2025)

```rust
JsRuntime::new(RuntimeOptions::default())
ops: {
  http_fetch: op_http_fetch,  // Custom op definition
}
```

### Known Issues

- Very bare-bones (no Node stdlib, CommonJS, module resolution)
- Hot reload requires external watcher + runtime restart
- Steep learning curve vs. alternatives

### Production Status

Stable Rusty V8 bindings since 2024. deno_core API stable but minimal.

---

## 3. ffmpeg-sys-next

**Crate:** `ffmpeg-sys-next` | **Status:** Wrapper around FFmpeg C library
**GPU Support:** NVDEC/NVENC via FFmpeg compile flags

### GPU Transcoding Pipeline

**Build Requirements:**
```bash
--enable-nvdec --enable-nvenc --enable-cuda
```

**Rust Integration Pattern:**
```rust
// ffmpeg-sys bindings available, but low-level C API
// No high-level abstractions for GPU memory management
```

### Production Issues

- **Memory Management:** Manual; FFmpeg GPU buffers require explicit cleanup
- **API Stability:** Depends on FFmpeg version; breaking changes possible
- **Bindings:** Low-level C wrappers; high cognitive load
- **No Built-in Support:** Hot-swap codecs or on-the-fly pipeline reconfiguration

### Alternatives

- `ffmpeg` crate (higher-level Rust wrapper) — simpler but less GPU control
- Direct FFmpeg CLI (subprocess) — easiest for MVPs, overhead acceptable for low concurrency

### Verdict

GPU transcoding viable with ffmpeg-sys-next if FFmpeg compiled with NVDEC/NVENC, but manual pipeline management required.

---

## 4. axum

**Crate:** `axum` | **Latest:** 0.8.0 (Jan 2025)
**Status:** Fully production-ready

### Streaming Responses

**Pattern:**
```rust
Body::from_stream(stream: Stream<Item = Result<Bytes, E>>)
// Handles chunked transfer encoding automatically
```

**Compatibility:**
- Works with standard tokio (multi-threaded runtime)
- AsyncRead + AsyncReadBody for streaming
- Automatic chunked encoding (no content-length needed)

### tokio-uring Integration

**NOT COMPATIBLE.** Axum assumes multi-threaded Send+Sync; tokio-uring is single-threaded.

### Production Considerations

- Mature, widely adopted ecosystem
- Excellent for REST/streaming APIs
- No performance issues at scale

---

## Summary Table

| Crate | Version | Production | Issues | Recommendation |
|-------|---------|------------|--------|-----------------|
| **tokio-uring** | 0.4.x | ❌ Limited | !Send + axum incompatible | Skip for downloader |
| **deno_core** | 0.295+ | ✅ Stable | No built-in hot reload | Optional (if JS scripting needed) |
| **ffmpeg-sys-next** | Latest | ⚠️ Partial | Manual GPU memory mgmt | Use if GPU transcoding required |
| **axum** | 0.8.0 | ✅ Stable | None for streaming | **RECOMMENDED** |

---

## Recommended Stack

```
axum (HTTP server + streaming)
  ↓
tokio (standard async runtime)
  ↓
ffmpeg-sys-next (GPU-accelerated transcoding, if needed)
  ↓
deno_core (OPTIONAL: JS plugin engine for extensibility)
```

**Rationale:** Maximal production stability. Skip tokio-uring (adds complexity, breaks axum). GPU support via ffmpeg compile flags + manual memory management.

---

## Unresolved Questions

1. **GPU Crash Handling:** How to recover from NVENC failures mid-transcode?
2. **Memory Pooling:** Should GPU buffers be pooled or allocated per-request?
3. **JS Hot Reload:** If deno_core used, implement file watcher + runtime reload strategy?
4. **FFmpeg CLI vs. Bindings:** For MVP, is subprocess overhead acceptable vs. sys-next complexity?

---

## Sources

- [Tokio-uring & Axum Compatibility](https://github.com/tokio-rs/axum/discussions/2989)
- [Axum 0.8.0 Release](https://tokio.rs/blog/2025-01-01-announcing-axum-0-8-0)
- [deno_core Documentation](https://docs.rs/deno_core/latest/deno_core/)
- [Rusty V8 Stable Announcement](https://deno.com/blog/rusty-v8-stabilized)
- [NVIDIA FFmpeg GPU Transcoding](https://docs.nvidia.com/video-technologies/video-codec-sdk/13.0/ffmpeg-with-nvidia-gpu/index.html)
- [Axum Streaming Responses](https://github.com/tokio-rs/axum/issues/2107)
