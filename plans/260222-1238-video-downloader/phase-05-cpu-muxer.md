# Phase 05 — CPU Muxer (Fragmented MP4)

## Context
- Plan: [plan.md](plan.md)
- Prev: [phase-03-stream-proxy.md](phase-03-stream-proxy.md)
- Parallel: [phase-06-gpu-pipeline.md](phase-06-gpu-pipeline.md)

## Overview
- **Priority**: P1
- **Status**: completed
- **Effort**: 1d
- Merge separate audio + video streams (YouTube DASH) into a single fMP4 container on-the-fly, without temp files. CPU-only, trivially fast.

## Key Insights
- YouTube 1080p+ always delivers separate audio (AAC/Opus) and video (VP9/AV1/H264) streams
- fMP4 (fragmented MP4) allows streaming output before knowing total file size → no need to buffer entire file
- `mp4-stream` crate supports channel-based fMP4 generation for live/streaming use
- Muxing = container operation, not codec operation → CPU trivially handles at memory bandwidth speeds
- For formats where audio+video are already merged (YouTube, YouTube ≤720p) → skip muxer, proxy directly

## Architecture

```
ExtractionResult { audio_stream_url, video_stream_url }
         ↓
MuxRouter: needs_mux? (audio_url != video_url)
  ├── NO  → Phase 03 direct proxy
  └── YES → Muxer
              ├── fetch audio bytes (AntiBotClient)
              ├── fetch video bytes (AntiBotClient)
              └── mp4-stream mux → chunked fMP4 output → axum Body
```

## Related Code Files
- `crates/muxer/src/lib.rs` — public API
- `crates/muxer/src/fmp4_muxer.rs` — core mux logic
- `crates/muxer/src/stream_fetcher.rs` — concurrent audio+video fetch
- `crates/api/src/routes/stream.rs` — updated to route through muxer

## Implementation Steps

1. **Add deps** (`crates/muxer/Cargo.toml`)
   ```toml
   mp4-stream = "0.4"    # fMP4 channel-based muxer
   tokio = { features = ["full"] }
   bytes = "1"
   futures = "0.3"
   ```

2. **MuxRouter** — decision logic
   ```rust
   pub enum StreamSource {
       Direct { url: String },
       Mux { video_url: String, audio_url: String },
   }
   pub fn route(result: &ExtractionResult) -> StreamSource
   ```

3. **StreamFetcher** (`stream_fetcher.rs`)
   - `tokio::join!` to fetch audio + video concurrently
   - Returns `(impl Stream<Bytes>, impl Stream<Bytes>)`
   - Both fetched via `AntiBotClient`

4. **fMP4 Muxer** (`fmp4_muxer.rs`)
   ```rust
   pub fn mux_streams(
       video: impl Stream<Item=Bytes>,
       audio: impl Stream<Item=Bytes>,
       video_codec: Codec,
       audio_codec: Codec,
   ) -> impl Stream<Item=Result<Bytes>>
   ```
   - Use `mp4-stream` to initialize fMP4 writer with video+audio tracks
   - Feed interleaved chunks from both streams
   - Output fMP4 segments as they're generated (no full buffer)

5. **Update stream route** — `routes/stream.rs`
   - Check `MuxRouter::route()`
   - If `Mux`: call `fmp4_muxer::mux_streams()`, return as axum Body
   - Set `Content-Type: video/mp4`, `Content-Disposition: attachment`
   - Note: no Content-Length (unknown for muxed stream) → chunked TE

## Todo
- [x] Add mp4-stream crate
- [x] MuxRouter decision logic
- [x] Concurrent audio+video StreamFetcher
- [x] fMP4 mux stream pipeline
- [x] Update stream route handler
- [ ] Integration test: mux YouTube 1080p → valid MP4 output

## Success Criteria
- First byte of fMP4 output within 500ms of request
- Zero temp files on disk
- Muxed MP4 plays in VLC/browser without errors
- Memory usage flat during 500MB+ mux (no buffering)

## Risk Assessment
| Risk | Mitigation |
|---|---|
| mp4-stream API insufficient | Fallback: `ffmpeg` CLI subprocess for muxing only (no GPU) |
| Audio/video sync drift | Use PTS/DTS from source streams; mp4-stream handles this |
| One stream 403s mid-mux | Abort both fetches; return error to client |
