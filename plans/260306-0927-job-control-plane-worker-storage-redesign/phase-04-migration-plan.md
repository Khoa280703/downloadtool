# Phase 04 - Migration Plan

## Context Links
- [crates/api/src/services/job_control_plane.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/services/job_control_plane.rs)
- [crates/api/src/routes/jobs.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/jobs.rs)
- [frontend/src/lib/api.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/api.ts)
- [frontend/src/components/DownloadBtn.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/components/DownloadBtn.svelte)

## Overview
- Priority: P0
- Current status: approved
- Goal: migrate without breaking current download flow or forcing a big-bang cutover.

## Key Insights
- The safest migration is "durable state first, worker split second, storage shift third".
- Dedup and direct-to-storage should come after stable job durability, not before.
- Short-lived compatibility is acceptable during rollout, but the final architecture removes sync mux and keeps only the durable jobs pipeline.

## Requirements
- No forced downtime.
- Existing users can still download while new pipeline is dark-launched.
- Every phase is reversible by config flag.

## Architecture
- Feature flags:
  durable `/api/jobs/*` worker pipeline
  `MUX_ARTIFACT_BACKEND=localfs|minio|r2`
  `MUX_DIRECT_DOWNLOAD=false|true`

## Final Decisions
- Legacy in-process mux-job execution is not part of target architecture and is removed after migration.
- Transitional compatibility may exist only behind feature flags during rollout and rollback windows.
- Phase B may use `LocalFs` behind a `StorageBackend` trait to isolate worker/queue risk before introducing object storage.
- Phase C switches the same interface to S3-compatible backend without reworking worker business logic.

## Related Code Files
- Modify:
  [crates/api/src/main.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/main.rs)
  [crates/api/src/routes/jobs.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/jobs.rs)
  [frontend/src/lib/api.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/api.ts)
  [frontend/src/components/DownloadBtn.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/components/DownloadBtn.svelte)

## Implementation Steps
1. Phase A: Durable state
   Add Postgres-backed job repository while API still executes work locally.
2. Phase B: Worker extraction
   Introduce worker binary and Redis Streams, keep `LocalFs` artifact backend first through `StorageBackend` abstraction.
3. Phase C: Object storage
   Replace local artifact output with MinIO multipart upload; API now returns pre-signed URL.
4. Phase D: Frontend cutover
   Frontend and external launchers use `/api/jobs/*` for every mux download and download from storage directly.
5. Phase E: Dedup and proxy sharing
   Enable artifact reuse and shared proxy cooldown state.
6. Phase F: Removal
   Delete in-memory queue service and legacy endpoints after bake period.

## Rollback Strategy
- If Redis path fails:
  no in-process rollback path remains after cutover; durable jobs must be fixed in-place or temporarily avoided at the caller layer
- If object storage path fails:
  switch `MUX_ARTIFACT_BACKEND=localfs`
- If new frontend path fails:
  keep caller traffic on the app-domain launcher, but repair durable jobs in-place because no sync mux fallback remains

## Todo List
- [ ] Add flags and config plumbing
- [ ] Add observability for queue depth, lease reclaim, upload failures
- [ ] Add admin tooling for stuck jobs
- [ ] Add cleanup cron/runner
- [ ] Add artifact TTL policy

## Success Criteria
- Each migration phase can be enabled per environment.
- Rollback is config-based, not code-revert-based.
- Only one mux path remains active across API, frontend, and external clients.

## Risk Assessment
- Big-bang migration would couple DB, broker, worker, storage, and frontend changes too tightly.
- Shared dedup introduced too early can mask correctness bugs.
- Cleanup introduced too late can create uncontrolled storage growth.

## Security Considerations
- Signed URL domain must be CORS-safe for browser download flow.
- Cleanup endpoint/callback must require auth and server-side ownership checks.
- Admin requeue/delete actions must be privileged.

## Next Steps
- Convert this approved plan into implementation tasks:
  DB migrations and repository layer
  Redis Streams publisher/consumer contract
  `StorageBackend` trait with `LocalFs` then `S3`
  frontend cutover from legacy mux job endpoints to `/api/jobs/*`

## Open Questions
- None for this phase.
