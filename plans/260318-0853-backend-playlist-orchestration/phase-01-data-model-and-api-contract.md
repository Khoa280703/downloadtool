# Phase 1: Data Model And API Contract

## Context Links

- `crates/api/src/routes/batch.rs`
- `crates/api/src/routes/jobs.rs`
- `crates/api/src/services/job_control_plane.rs`
- `crates/job-system/src/job_models.rs`
- `frontend/src/lib/api.ts`
- `frontend/src/lib/types.ts`

## Overview

- Priority: P1
- Status: pending
- Brief: Add a thin playlist job layer on top of the existing mux job system.

## Key Insights

- Current `/api/batch` is good enough for preview and selection. Do not overload it.
- Existing `mux_jobs` model is specialized for one muxed output. Playlist needs parent/child orchestration, not a generic rewrite of mux tables.
- Minimal path: add `playlist_jobs` + `playlist_job_items`, keep `mux_jobs` untouched.

## Requirements

- Create durable playlist job after user selects playlist items.
- Persist requested quality/mode once at playlist job creation.
- Track item status independently from mux job status.
- Support direct-stream and mux-required items in the same playlist job.
- Support SSE progress stream for the whole playlist.

## Architecture

- Keep `/api/batch?url=` as preview/discovery only.
- Add new endpoints:
  1. `POST /api/playlist-jobs`
  2. `GET /api/playlist-jobs/{job_id}`
  3. `GET /api/playlist-jobs/{job_id}/events`
  4. `GET /api/playlist-jobs/{job_id}/items/{item_id}/file`
  5. Optional: `POST /api/playlist-jobs/{job_id}/cancel`
- Persist item delivery kind:
  - `direct_stream`
  - `mux_job`
- Persist item selection result:
  - `source_url`
  - `video_format_id`
  - `audio_format_id`
  - `selected_container`
  - `patch_init_metadata`
  - `mux_job_id` nullable

## Related Code Files

- Modify:
  - `crates/api/src/main.rs`
  - `crates/api/src/routes/batch.rs`
  - `frontend/src/lib/api.ts`
  - `frontend/src/lib/types.ts`
- Create:
  - `crates/api/src/routes/playlist_jobs.rs`
  - `crates/api/src/services/playlist_job_control_plane.rs`
  - `crates/job-system/src/playlist_job_models.rs`
  - `crates/job-system/src/playlist_repository.rs`
  - DB migrations for `playlist_jobs` and `playlist_job_items`

## Implementation Steps

1. Add DB schema for playlist parent and item rows.
2. Add repository read/write methods for create, list items, update item status, attach mux job reference.
3. Add API contracts and response payloads.
4. Keep `/api/batch` response stable for backward compatibility.
5. Add SSE event payload shape early and freeze it before frontend migration.

## Todo List

- [ ] Define playlist job statuses
- [ ] Define playlist item statuses
- [ ] Write migrations
- [ ] Add repository methods
- [ ] Add API DTOs and routes

## Success Criteria

- Backend can create a playlist job with selected items in one request.
- Backend can return durable status after reload/reconnect.
- Frontend has enough typed payloads to migrate without hidden fields.

## Risk Assessment

- Biggest risk is over-generalizing item schema. Keep it narrow and tied to current playlist use case.

## Security Considerations

- Reuse existing owner/session scoping pattern from mux jobs.
- Never expose raw foreign job IDs across owners.

## Next Steps

- Feed the new schema into a playlist worker/orchestrator in Phase 2.
