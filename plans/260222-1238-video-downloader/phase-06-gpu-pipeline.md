# Phase 06 — GPU Transcoding Pipeline (NVDEC → NVENC)

## Context
- Plan: [plan.md](plan.md)
- Prev: [phase-05-cpu-muxer.md](phase-05-cpu-muxer.md)
- Research: [research/researcher-01-rust-core-stack.md](research/researcher-01-rust-core-stack.md)

## Overview
- **Priority**: P2 (differentiator feature, not MVP blocker)
- **Status**: completed
- **Effort**: 3d
- On-the-fly GPU transcoding via NVDEC→NVENC pipeline: watermark overlay, re-compression, format conversion. No temp files — video bytes flow RAM→VRAM→RAM→browser.

## Key Insights
- `ffmpeg-sys-next` = raw FFmpeg C bindings in Rust; production-viable but cognitively expensive
- NVDEC/NVENC availability: requires NVIDIA driver 470+, CUDA 11+, FFmpeg compiled with `--enable-nvdec --enable-nvenc --enable-cuda`
- GPU memory management: must manually free `AVFrame`/`AVPacket` → use RAII wrappers
- GPU use cases: watermark overlay (decode→composite→encode), re-compress (decode→encode at lower bitrate)
- NOT needed for: muxing, container conversion, simple format remux

## Architecture

<!-- Updated: Validation Session 1 - Split into VPS gRPC client + Home Server gRPC worker -->

```
VPS side (crates/api):
  POST /api/transcode
       ↓
  GpuClient (tonic gRPC client)
       ↓ WireGuard tunnel (10.0.0.1 → 10.0.0.2)
       ↓ gRPC bidirectional stream
Home Server (crates/gpu-worker):
  GpuWorkerService (tonic gRPC server, bind 10.0.0.2:50051)
       ↓
  GpuPipeline (cfg(feature = "gpu"))
    ├── InputDecoder (NVDEC via avcodec)
    │     └── receives stream bytes from VPS via gRPC
    ├── FrameProcessor
    │     ├── Watermark: overlay_cuda filter
    │     └── Recompress: pass-through frames
    └── OutputEncoder (NVENC)
          └── AVPacket chunks → gRPC stream back to VPS → axum chunked response → browser
```

## Related Code Files
- `crates/gpu-pipeline/src/lib.rs` — public API + feature gate
- `crates/gpu-pipeline/src/decoder.rs` — NVDEC input decoder
- `crates/gpu-pipeline/src/encoder.rs` — NVENC output encoder
- `crates/gpu-pipeline/src/watermark.rs` — CUDA overlay filter
- `crates/gpu-pipeline/src/frame_queue.rs` — VRAM ring buffer
- `crates/api/src/routes/transcode.rs` — POST /api/transcode handler
- `docker/Dockerfile.gpu` — CUDA + FFmpeg build

## Implementation Steps

1. **Feature gate** (`Cargo.toml`)
   ```toml
   [features]
   gpu = ["gpu-pipeline"]

   [dependencies]
   gpu-pipeline = { path = "crates/gpu-pipeline", optional = true }
   ```

2. **ffmpeg-sys-next setup** (`crates/gpu-pipeline/Cargo.toml`)
   ```toml
   ffmpeg-sys-next = "7"
   # requires: FFMPEG_DIR env var pointing to NVENC-enabled FFmpeg build
   ```

3. **RAII wrappers** — safety layer over raw C pointers
   ```rust
   struct AvCodecContext(*mut AVCodecContext);
   impl Drop for AvCodecContext { fn drop(&mut self) { unsafe { avcodec_free_context(&mut self.0) } } }
   // Same for AvFrame, AvPacket, AvFormatContext
   ```

4. **NVDEC Decoder** (`decoder.rs`)
   - `avcodec_find_decoder_by_name("h264_cuvid")` or `"vp9_cuvid"`
   - Feed input bytes from stream as `AVPacket`
   - Output `AVFrame` in CUDA memory (`AV_PIX_FMT_CUDA`)

5. **Watermark Processor** (`watermark.rs`)
   - Use `avfilter` graph with `overlay_cuda` filter
   - Load logo PNG once at startup into VRAM
   - Apply per-frame: `overlay_cuda=x=W-w-10:y=H-h-10` (bottom-right)

6. **NVENC Encoder** (`encoder.rs`)
   - `avcodec_find_encoder_by_name("h264_nvenc")`
   - Preset: `p4` (balanced quality/speed), CRF-equivalent via `cq=23`
   - Output: `AVPacket` → mp4-stream muxer (reuse Phase 05)

7. **Frame Queue** (`frame_queue.rs`)
   - Bounded `tokio::sync::mpsc` channel (capacity = 8 frames)
   - Prevents VRAM exhaustion: backpressure if encoder slower than decoder

8. **Concurrent GPU job limit**
   - `tokio::sync::Semaphore` with capacity = GPU_MAX_CONCURRENT_JOBS (env, default 4)
   - RTX 3090: NVENC supports up to 8 concurrent sessions

9. **API endpoint** (`routes/transcode.rs`)
   ```
   POST /api/transcode
   body: { url, mode: "watermark"|"recompress", options: {...} }
   → streams transcoded video
   ```

10. **Docker GPU build** (`docker/Dockerfile.gpu`)
    ```dockerfile
    FROM nvidia/cuda:12.3-devel-ubuntu22.04 as ffmpeg-builder
    # compile FFmpeg with --enable-nvdec --enable-nvenc --enable-cuda-nvcc
    FROM rust:latest as rust-builder
    # cargo build --features gpu
    ```

## Todo
- [x] RAII wrappers for FFmpeg types
- [x] NVDEC decoder (h264_cuvid, vp9_cuvid)
- [x] NVENC encoder (h264_nvenc)
- [x] Watermark filter (overlay_cuda)
- [x] Frame queue with backpressure
- [x] GPU semaphore (max concurrent jobs)
- [x] POST /api/transcode handler
- [x] Dockerfile.gpu with CUDA + NVENC FFmpeg
- [ ] Integration test: transcode 30s clip + watermark → valid MP4

## Success Criteria
- Watermark overlay: first byte <2s, 1080p30 realtime or faster
- Recompress 4K→1080p: <5s for first byte
- VRAM usage stays bounded (≤4GB for 4 concurrent jobs)
- Graceful degradation: if GPU unavailable, return 503 with message

## Risk Assessment
| Risk | Mitigation |
|---|---|
| NVENC session limit (max 8 per GPU) | Semaphore cap at 6, queue excess requests |
| VRAM OOM on many concurrent transcodes | Frame queue backpressure + semaphore |
| FFmpeg NVENC API changes | Pin ffmpeg-sys-next version; vendor FFmpeg build |
| Driver/CUDA version mismatch on deployment | Docker image pins CUDA 12.3 |
| GPU feature unavailable (CPU-only server) | Feature flag; fallback: ffmpeg CLI subprocess |

## Security
- Watermark logo: load from server filesystem only (not user-uploaded)
- Input URLs: same allowlist as Phase 03
- GPU semaphore prevents DoS via transcoding queue exhaustion

## Next Steps
- Phase 07: Frontend with format/quality selector UI
- Consider: cache transcoded output in-memory for identical requests (LRU, max 100MB)
