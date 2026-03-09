# Phase 01 - Target Architecture

## Context Links
- [crates/api/src/main.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/main.rs)
- [crates/api/src/services/job_control_plane.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/services/job_control_plane.rs)
- [crates/api/src/routes/jobs.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/jobs.rs)
- [frontend/src/components/DownloadBtn.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/components/DownloadBtn.svelte)

## Overview
- Priority: P0
- Current status: approved
- Goal: replace in-memory mux jobs with durable control plane + broker + worker + object storage architecture.

## Key Insights
- AS-IS keeps queue state inside API memory, so API restarts drop local execution state.
- AS-IS also mixes control plane and data plane: API receives commands and can still proxy/stream work itself.
- The new design should keep API light and move long-running network + mux work into worker nodes.
- "Stateless API" here means no job state in RAM, not zero dependencies.

## Requirements
- Functional:
1. Create durable job records before enqueue.
2. Let workers claim jobs safely and resume after crash/restart.
3. Upload artifact to S3-compatible storage without serving final bytes through API.
4. Expose job status and ready-file ticket APIs to frontend.
5. Support dedup for equivalent mux requests.
6. Require authenticated users for job creation in V1.
- Non-functional:
1. API restart-safe.
2. Worker crash-safe with lease timeout and retry.
3. No local `.part` output files in steady state.
4. Easy switch from MinIO to Cloudflare R2 by config only.

## Architecture
- Control Plane:
  API Rust/Axum accepts `POST /jobs`, validates JWT, normalizes request, writes PostgreSQL row, publishes `job_id` to Redis Streams, exposes `GET /jobs/{id}` and `GET /jobs/{id}/file-ticket`.
- State Plane:
  PostgreSQL stores `jobs`, `artifacts`, and audit/event history. It is not authoritative for live proxy cooldown.
- Message Plane:
  Redis Streams stores pending job messages. Consumer group gives ack/reclaim mechanics and low operational friction.
- Worker Plane:
  Independent Rust process claims jobs, leases them, resolves proxy, streams video/audio through muxer, uploads multipart to object storage, updates DB, acks broker.
- Artifact Plane:
  MinIO now, R2 later. API only generates pre-signed GET URLs; browser downloads directly from storage.
- Cleanup Plane:
  Periodic cleanup task marks expired artifacts/jobs and deletes stale object keys.

## Final Decisions
- Anonymous jobs: rejected in V1.
- Artifact reuse across users: accepted for public-equivalent YouTube artifacts when `dedupe_key` matches.
- End-state UX path: every mux download goes through durable `/api/jobs/*`; no parallel sync mux surface remains.

## Related Code Files
- Modify:
  [crates/api/src/main.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/main.rs)
  [crates/api/src/routes/jobs.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/jobs.rs)
  [crates/api/src/services/mod.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/services/mod.rs)
  [frontend/src/lib/api.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/api.ts)
  [frontend/src/components/DownloadBtn.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/components/DownloadBtn.svelte)
- Create:
  `crates/api/src/routes/jobs.rs`
  `crates/api/src/services/job-control-plane.rs`
  `crates/api/src/services/storage-ticket-service.rs`
  `crates/worker/`
  `crates/object-store/`
  `crates/queue/`
- Delete later:
  legacy in-memory mux job queue after cutover

## Implementation Steps
1. Introduce durable DB schema for jobs/artifacts.
2. Add broker publisher from API.
3. Add worker binary with claim/lease/retry loop.
4. Add object storage upload abstraction.
5. Move every mux caller from legacy mux/sync semantics to `/api/jobs/*`.
6. Remove in-memory queue after stable cutover.

## Todo List
- [ ] Finalize job state machine
- [ ] Finalize storage artifact key convention
- [ ] Finalize signed URL generation strategy
- [ ] Finalize worker lease/reclaim rules
- [ ] Finalize cutover path from current endpoints

## Success Criteria
- API process restart does not lose queued jobs.
- Ready artifact is downloadable even if API restarts immediately after completion.
- Worker pool can scale horizontally without shared-memory assumptions.

## Risk Assessment
- Main risk: muxer may still assume output semantics incompatible with pure streaming upload.
- Main risk: duplicate execution if dedup and lease logic are weak.
- Main risk: proxy cooldown becomes inconsistent if worker-local only.

## Security Considerations
- Signed download URLs must be short-lived.
- `job_id` access must be user-scoped in V1.
- Worker never trusts frontend-provided artifact keys or statuses.

## Next Steps
- Write exact schema and contracts in phase 02.
- Validate muxer streaming constraints in phase 03.

## Open Questions
- None for this phase.
