---
title: "Phase 2 - Playlist API And Control Plane"
status: pending
---

# Phase 2: Playlist API And Control Plane

## Context Links
- Existing API entry: [main.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/main.rs)
- Existing batch discovery: [batch.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/batch.rs)
- Existing item job API: [jobs.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/jobs.rs)

## Overview
Priority: P1
Status: pending
Goal: replace `/api/batch` as the main playlist runtime path with persistent playlist APIs and SSE progress.

## Key Insights
- Current UX is 2-step: fetch playlist first, then choose items and start.
- Keep that UX. Do not force “download immediately on create”.
- Backend should own discovery and run state; frontend should only render and send explicit mutations.

## Requirements
- Create playlist discovery job.
- Stream discovery progress and run progress.
- Allow item selection changes before start.
- Start, cancel, rerun failed items.

## Architecture
- New routes:
  - `POST /api/playlist-jobs` create discovery job from playlist URL
  - `GET /api/playlist-jobs/{id}` fetch snapshot + items
  - `GET /api/playlist-jobs/{id}/events` SSE for discovery + run progress
  - `PATCH /api/playlist-jobs/{id}/selection` update selected items / mode / quality before run
  - `POST /api/playlist-jobs/{id}/start` start processing selected items
  - `POST /api/playlist-jobs/{id}/cancel` cancel active run
  - optional `POST /api/playlist-jobs/{id}/retry-failed`
- New service:
  `playlist_control_plane.rs` in API layer orchestrates discovery, enqueue, state transitions, SSE event publication.

## Related Code Files
- Modify:
  `crates/api/src/main.rs`
  `crates/api/src/routes/mod.rs`
  `crates/api/src/routes/openapi.rs`
- Create:
  `crates/api/src/routes/playlist_jobs.rs`
  `crates/api/src/services/playlist_control_plane.rs`
  `crates/api/src/services/playlist_progress_service.rs` or equivalent

## Implementation Steps
1. Add playlist routes and DTOs.
2. Move current playlist extraction logic from ephemeral SSE handler into a reusable service.
3. On create: persist playlist job, kick off discovery task, publish discovery SSE events.
4. On start: mark selected items queued, publish run state, kick dispatcher loop.
5. On cancel: stop claiming new items and signal active item job creation to stop.

## Todo List
- [ ] Define playlist API contracts
- [ ] Reuse extraction service for discovery
- [ ] Add SSE event payload model
- [ ] Add start/cancel/retry handlers
- [ ] Add auth/owner enforcement

## Success Criteria
- Refreshing browser does not lose playlist state.
- Frontend can reconnect to SSE and reconstruct exact playlist state from backend snapshot.
- Playlist can be started after selection, not only at create time.

## Risk Assessment
- Risk: route sprawl.
  Mitigation: keep only create/get/events/selection/start/cancel in first ship.
- Risk: duplicate progress systems with mux job SSE.
  Mitigation: playlist SSE aggregates item progress, item SSE remains internal.

## Security Considerations
- Rate-limit create/start/cancel separately from item extract routes.
- Validate selected item IDs belong to playlist job owner.

## Next Steps
- Extend durable item job model so every playlist item can finish as a durable artifact.

## Unresolved Questions
- Whether cancel should abort in-flight item immediately or stop after current item.
