# Phase 2: Playlist Worker And Item State Machine

## Context Links

- `crates/worker/src/job_runner.rs`
- `crates/queue/src/*`
- `crates/api/src/services/job_control_plane.rs`
- `frontend/src/lib/playlist-download-worker.ts`
- `frontend/src/lib/playlist-download-stream-selection.ts`

## Overview

- Priority: P1
- Status: pending
- Brief: Move per-item sequencing and retry logic from browser worker into backend worker.

## Key Insights

- Current browser worker already encodes the useful decision logic: extract item, select stream, decide direct vs mux, reject degraded 360p fallback.
- That logic should move server-side nearly as-is, not be redesigned.
- Existing mux worker already handles mux artifact build. Playlist worker should delegate to it, not merge with it.

## Requirements

- Process playlist items sequentially per playlist job by default.
- Keep just-in-time extract per item when item reaches head of queue.
- If direct-stream item is selected, mark item ready with persisted stream selection metadata.
- If mux is required, create underlying mux job and track it until ready/failed.
- Retry item-level transient failures without rewinding whole playlist.

## Architecture

- Add a dedicated playlist queue message, one message per playlist job wake-up.
- Playlist worker loop:
  1. lease playlist job
  2. pick next pending item
  3. run extract + stream selection
  4. if direct stream: persist selected stream metadata, mark ready
  5. if mux required: create mux job, persist `mux_job_id`, observe status until terminal
  6. move to next item
- Keep concurrency policy simple:
  - sequential within one playlist job
  - multiple playlist jobs can run globally with small worker concurrency

## Related Code Files

- Modify:
  - `crates/worker/src/main.rs`
  - `crates/worker/src/job_runner.rs`
  - `crates/extractor/src/lib.rs`
  - `crates/api/src/services/job_control_plane.rs`
- Create:
  - `crates/worker/src/playlist_job_runner.rs`
  - `crates/worker/src/playlist_progress_publisher.rs`
  - `crates/queue/src/playlist_streams.rs` or extend current queue module

## Implementation Steps

1. Extract reusable stream-selection rules from frontend into a backend-compatible module.
2. Add playlist queue publisher/consumer.
3. Implement item state transitions:
   - `pending -> preparing -> ready`
   - `pending -> preparing -> queued_mux -> mux_processing -> ready`
   - `pending -> preparing -> failed`
4. Reuse existing mux job create/reuse path for mux-required items.
5. Publish playlist-level and item-level progress snapshots to Redis pubsub/SSE feed.

## Todo List

- [ ] Port stream selection logic
- [ ] Add playlist queue message type
- [ ] Add playlist worker runner
- [ ] Delegate mux-required items to existing job system
- [ ] Publish progress snapshots

## Success Criteria

- Browser no longer needs to decide per-item orchestration.
- Playlist job continues even if tab disconnects.
- Mux job reuse and artifact reuse still work.

## Risk Assessment

- Observing child mux jobs can become noisy. Prefer polling/status subscription in one place, not duplicated across API and worker.

## Security Considerations

- Do not let playlist worker operate on items outside owner scope.
- Sanitize and store only necessary upstream metadata.

## Next Steps

- Frontend can switch from local queues to server-driven playlist state.
