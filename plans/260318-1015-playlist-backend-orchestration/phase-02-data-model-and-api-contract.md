# Phase 2: Data Model And API Contract

## Context Links

- [Job control plane](../../crates/api/src/services/job_control_plane.rs)
- [Jobs route](../../crates/api/src/routes/jobs.rs)
- [Batch types in frontend](../../frontend/src/lib/types.ts)
- [Overview plan](./plan.md)

## Overview

- Priority: P1
- Status: pending
- Goal: add minimal durable state and HTTP/SSE contracts.

## Key Insights

- Existing durable jobs already solve owner scoping, queue publish, artifact reuse.
- Playlist should not create 50 stream URLs upfront.
- Playlist creation endpoint should only store item identity + requested output preference.

## Requirements

- Create playlist job from URL + requested mode + quality.
- Read playlist job status.
- Stream realtime playlist events.
- Cancel playlist job.
- Resume playlist job.
- Retry one failed item.

## Architecture

- Tables:
  - `playlist_jobs`
  - `playlist_job_items`
- Fields:
  - playlist status, requested mode/quality, totals, completed/failed counts
  - per-item video_id, title, thumbnail, status, attempts, last_error
  - optional `mux_job_id`, optional `artifact_id`
- Routes:
  - `POST /api/proxy/playlists`
  - `GET /api/proxy/playlists/:id`
  - `GET /api/proxy/playlists/:id/events`
  - `POST /api/proxy/playlists/:id/cancel`
  - `POST /api/proxy/playlists/:id/resume`
  - `POST /api/proxy/playlists/:id/items/:item_id/retry`

## Related Code Files

- Modify:
  - `crates/api/src/main.rs`
  - `crates/api/src/routes/mod.rs`
  - `crates/api/src/routes/jobs.rs`
  - `frontend/src/lib/api.ts`
  - `frontend/src/lib/types.ts`
- Create:
  - playlist repository/service/route modules
  - DB migrations in existing migration location

## Implementation Steps

1. Add schema for playlist job and item records.
2. Add repository methods for create/update/list/query.
3. Add create/status/events/cancel/resume/retry endpoints.
4. Reuse existing owner model and audit logging.

## Todo List

- [ ] Schema drafted
- [ ] Repository contract drafted
- [ ] API payloads drafted
- [ ] Access control rules aligned with current jobs API

## Success Criteria

- Playlist state survives API restart and browser refresh.
- API contract is enough for frontend to render aggregate + item progress.

## Risk Assessment

- Too much duplication with durable mux job schema.
- Resume logic becomes messy if status naming is not clean.

## Security Considerations

- Item retry/cancel must check playlist ownership.
- Audit log should capture playlist create/cancel/resume/retry.

## Next Steps

- Wire worker execution and SSE fan-out in Phase 3.

## Unresolved questions

- Whether playlist create should be synchronous-until-items-listed or queued asynchronously.
