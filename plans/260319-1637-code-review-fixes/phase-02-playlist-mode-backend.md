# Phase 02 — Playlist Mode Backend Fix

## Context Links
- Frontend modes: `frontend/src/lib/playlist-download-stream-selection.ts:14-18`
- Mode saved: `crates/api/src/routes/playlist_jobs.rs:317-322`
- Mode ignored: `crates/api/src/services/playlist_processor.rs:270-308`
- Stream picker: `crates/api/src/services/playlist_processor.rs:425-471`
- Models: `crates/job-system/src/playlist_job_models.rs`

## Overview
- **Priority:** P1 High (functional bug)
- **Status:** completed
- **Effort:** 2h
- Frontend sends `requested_mode` (`video`, `video-only`, `audio`) but backend `process_single_item()` ignores it. `pick_best_streams()` only uses `quality` param. Result: user selects "audio only" but gets video+audio.

## Key Insights
- `PlaylistJobRecord.requested_mode` is persisted correctly (DB has the value)
- `process_single_item()` accesses `job.requested_quality` but never `job.requested_mode`
- `pick_best_streams()` signature: `fn pick_best_streams(formats, quality) -> (Option<video>, Option<audio>)` — no mode param
- **WebM filter bug:** current code filters `f.ext != "webm"` for BOTH video (line 443) AND audio (line 466). This is correct for mux mode (fMP4 rejects WebM), but **wrong for audio-only and video-only** which are direct downloads. WebM audio (opus) is often highest quality on YouTube.
- Frontend also filters WebM video-only at `playlist-download-stream-selection.ts:96,149` before mode check — so frontend does NOT "allow all formats" for video-only either. Backend fix should be the source of truth for format filtering per mode
- Three modes to handle:
  - `video` (default): pick best video + best audio, mux if needed — **keep WebM filter** (mux compatibility)
  - `video-only`: pick best video only, direct download — **allow all formats including WebM**
  - `audio`: pick best audio only, direct download — **allow all formats including WebM**

## Requirements
- `pick_best_streams()` must accept `mode` parameter
- When mode=`audio`: return `(None, Some(best_audio))`, skip video entirely
- When mode=`video-only`: return `(Some(best_video), None)`, skip audio entirely
- When mode=`video` (default): current behavior (both streams)
- `process_single_item()` must handle audio-only and video-only as direct downloads (no mux needed)

## Architecture
No structural change. Add `mode` param to `pick_best_streams()`, adjust routing logic in `process_single_item()`.

## Related Code Files
- **Modify:** `crates/api/src/services/playlist_processor.rs`
  - `pick_best_streams()` — add `mode` param
  - `process_single_item()` — read `job.requested_mode`, handle single-stream cases
- **No change:** `crates/job-system/src/playlist_job_models.rs` (mode already stored)
- **No change:** `crates/api/src/routes/playlist_jobs.rs` (mode already saved to DB)

## Implementation Steps

1. **Update `pick_best_streams()` signature:**
   ```rust
   fn pick_best_streams(
       formats: &[extractor::VideoFormat],
       quality: &str,
       mode: &str,
   ) -> (Option<extractor::VideoFormat>, Option<extractor::VideoFormat>) {
   ```

2. **Add mode filtering in `pick_best_streams()`:**
   - At top of function, early return for **audio-only** mode — **NO WebM filter** (direct download, not mux):
     ```rust
     if mode == "audio" {
         // Direct download — all formats allowed (WebM opus is often best quality)
         let best_audio = formats.iter()
             .filter(|f| f.is_audio_only)
             .max_by_key(|f| f.bitrate.unwrap_or(0))
             .cloned();
         return (None, best_audio);
     }
     ```
   - For **video-only** mode — **NO WebM filter** (direct download):
     ```rust
     if mode == "video-only" {
         // Direct download — all formats allowed
         let all_videos: Vec<_> = formats.iter()
             .filter(|f| !f.is_audio_only)
             .collect();
         // Apply quality ceiling logic (reuse target_height matching)
         // Return (best_video, None)
         return (best_video, None);
     }
     ```
   - Default `"video"` mode: **keep existing behavior with WebM filter** (mux compatibility required)

3. **Update call site in `process_single_item()`:**
   ```rust
   let (video, audio) = pick_best_streams(&info.formats, &job.requested_quality, &job.requested_mode);
   ```

4. **Fix `process_single_item()` routing logic (lines 290-308):**
   Current logic assumes video stream always present. Must handle:
   - **Audio-only mode:** `video=None, audio=Some(a)` → direct download with audio URL
     ```rust
     if video.is_none() {
         if let Some(ref a) = audio {
             let download_url = build_stream_download_url(&a.url, &info.title, &a.ext);
             repo.update_item_status(item_id, PlaylistItemStatus::Ready, None, None, Some(&download_url)).await?;
             return Ok(());
         }
         anyhow::bail!("No suitable audio stream found for {video_id}");
     }
     ```
   - **Video-only mode:** `video=Some(v), audio=None` → direct download (no mux)
     Already handled by existing `v.has_audio || audio.is_none()` check at line 291, but verify it produces correct result for video-only streams
   - **Default mode:** existing behavior

5. **Run `cargo build --workspace` and `cargo test --workspace`**

## Todo List
- [x] Add `mode` param to `pick_best_streams()`
- [x] Implement audio-only early return in `pick_best_streams()`
- [x] Implement video-only early return in `pick_best_streams()`
- [x] Update call site in `process_single_item()`
- [x] Handle audio-only routing in `process_single_item()`
- [x] Verify video-only routing works with existing logic
- [x] Add unit tests for `pick_best_streams()` with all 3 modes
- [x] Compile and test

## Success Criteria
- Playlist with mode=`audio` downloads audio-only files for each item
- Playlist with mode=`video-only` downloads video-only files
- Playlist with mode=`video` (default) works as before
- No mux job created for audio-only or video-only single-stream downloads
- `cargo test` passes

## Risk Assessment
- **Medium risk**: changes core playlist processing logic
- Test with real playlist after merge to verify all 3 modes
- Existing default mode must remain unchanged — regression test critical

## Security Considerations
- No new security surface. Mode values are already validated/defaulted at API layer.

## Next Steps
- Consider adding mode info to SSE status events so frontend shows correct download type
- Phase 04 touches same file — coordinate if running in parallel
