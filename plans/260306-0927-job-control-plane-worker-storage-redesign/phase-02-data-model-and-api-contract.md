# Phase 02 - Data Model and API Contract

## Context Links
- [crates/api/src/routes/jobs.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/jobs.rs)
- [crates/api/migrations/0001_create_subscriptions.sql](/home/khoa2807/working-sources/downloadtool/crates/api/migrations/0001_create_subscriptions.sql)
- [frontend/src/lib/api.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/api.ts)

## Overview
- Priority: P0
- Current status: approved
- Goal: define durable schema and contracts that replace in-memory queue semantics cleanly.

## Key Insights
- PostgreSQL should be the only authority for status shown to frontend.
- Redis Streams should carry only execution signals, not canonical state.
- Dedup must be artifact-centric, not request-centric.

## Requirements
- Functional:
1. Represent queued, leased, processing, ready, failed, expired states.
2. Track retries and lease ownership.
3. Link multiple jobs to one artifact when dedup hits.
4. Track object key, size, checksum, expiry.
- Non-functional:
1. Idempotent `POST /jobs`.
2. Safe worker reclaim after crash.
3. Cheap polling and future SSE support.
4. Avoid high-frequency progress writes to PostgreSQL.

## Architecture
- PostgreSQL tables:
  `mux_jobs`
  `mux_artifacts`
  `mux_job_events`
- Redis Streams:
  stream name `mux_jobs`
  message body minimal: `job_id`, `dedupe_key`, `requested_at_ms`
- Redis keys:
  live proxy cooldown/health state with TTL-backed keys, separate from canonical job state

## Final Decisions
- `mux_jobs` records user requests and lifecycle history.
- `mux_artifacts` records dedupable physical outputs and may be referenced by many jobs.
- Cross-user artifact reuse is allowed when the artifact is derived from the same canonical public source and same mux profile.
- PostgreSQL stores phase-level status only: `queued`, `leased`, `processing`, `ready`, `failed`, `expired`.
- If a smooth percent bar is later needed, worker publishes ephemeral progress snapshots into Redis, not PostgreSQL.

## Proposed Schema
- `mux_artifacts`
  `id`, `artifact_key`, `dedupe_key`, `storage_bucket`, `object_key`, `status`, `content_type`, `size_bytes`, `etag`, `sha256`, `created_at`, `ready_at`, `expires_at`, `last_accessed_at`
- `mux_jobs`
  `id`, `user_id`, `request_hash`, `dedupe_key`, `source_url`, `video_format_id`, `audio_format_id`, `status`, `artifact_id`, `attempt_count`, `max_attempts`, `lease_owner`, `lease_expires_at`, `last_error`, `created_at`, `updated_at`, `completed_at`, `delete_after_at`
- `mux_job_events`
  `id`, `job_id`, `event_type`, `payload_json`, `created_at`
- Redis proxy keys
  `proxy:cooldown:{proxy_hash}`, `proxy:fail-count:{proxy_hash}`, `proxy:last-error:{proxy_hash}` with TTL-driven expiry

## Dedupe Key
- Canonical inputs:
1. normalized source video id
2. normalized `video_format_id`
3. normalized `audio_format_id`
4. mux profile version
5. output container `mp4`
- Example:
  `yt:{video_id}:{video_format_id}:{audio_format_id}:mux-v1:mp4`

## HTTP Contract
- `POST /api/jobs`
  request: source URL, video/audio stream ids, title hints
  response: `job_id`, `status`, `poll_url`, optional `existing_artifact`
- `GET /api/jobs/{job_id}`
  response: status, phase, artifact metadata, retry hints, error
- `GET /api/jobs/{job_id}/file-ticket`
  response: short-lived pre-signed URL when ready
- `POST /api/jobs/{job_id}/release`
  optional hint from frontend after successful download; never authoritative
- `GET /api/jobs/events?ids=...`
  optional later for SSE/websocket fanout

## Queue Contract
- Redis stream: `mux_jobs`
- Consumer group: `mux-workers`
- Publish only after DB commit
- Worker creates/extends DB lease before execution starts
- Worker ack only after DB updated to a terminal state (`ready`, `failed`, `expired`)
- Stale pending entries are reclaimed through Redis consumer-group reclaim plus DB lease-expiry checks

## Related Code Files
- Modify:
  [crates/api/src/routes/jobs.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/jobs.rs)
  [frontend/src/lib/api.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/api.ts)
- Create:
  `crates/api/migrations/0002_create_mux_jobs.sql`
  `crates/api/migrations/0003_create_mux_artifacts.sql`
  `crates/api/src/routes/jobs.rs`
  `crates/api/src/services/job-repository.rs`

## Implementation Steps
1. Add new DB migrations.
2. Add repository/service layer over `sqlx`.
3. Implement idempotent create job path.
4. Implement signed URL ticket path.
5. Switch frontend job polling to new contract.

## Todo List
- [ ] Finalize status enum
- [ ] Finalize job id format
- [ ] Finalize artifact expiry policy
- [ ] Finalize idempotency key strategy

## Success Criteria
- A job can be reconstructed entirely from Postgres.
- Broker loss does not lose canonical status.
- Same dedupe key does not produce duplicate artifact builds.

## Risk Assessment
- If `POST /jobs` is not idempotent, frontend retries will duplicate jobs.
- If `dedupe_key` is too broad, wrong artifact may be reused.
- If lease timeout is too short, same job may be processed twice.

## Security Considerations
- Pre-signed URL TTL default should be short, e.g. 5-15 minutes.
- `GET /file-ticket` must verify job ownership before issuing ticket.
- Artifact keys should not expose raw user ids or secrets.

## Next Steps
- Define worker and upload mechanics in phase 03.

## Open Questions
- None for this phase.
