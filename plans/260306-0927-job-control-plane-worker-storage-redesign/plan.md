# Job Control Plane / Worker Storage Redesign

Status: completed
Date: 2026-03-06

Phases:
- [phase-01-target-architecture.md](/home/khoa2807/working-sources/downloadtool/plans/260306-0927-job-control-plane-worker-storage-redesign/phase-01-target-architecture.md) - TO-BE architecture, responsibilities, runtime boundaries
- [phase-02-data-model-and-api-contract.md](/home/khoa2807/working-sources/downloadtool/plans/260306-0927-job-control-plane-worker-storage-redesign/phase-02-data-model-and-api-contract.md) - PostgreSQL schema, Redis Streams contract, HTTP/SSE contract
- [phase-03-worker-and-storage-pipeline.md](/home/khoa2807/working-sources/downloadtool/plans/260306-0927-job-control-plane-worker-storage-redesign/phase-03-worker-and-storage-pipeline.md) - Worker lifecycle, proxy coordination, multipart upload, dedup
- [phase-04-migration-plan.md](/home/khoa2807/working-sources/downloadtool/plans/260306-0927-job-control-plane-worker-storage-redesign/phase-04-migration-plan.md) - Safe migration from in-memory jobs to durable queue/storage

Current system anchors:
- [crates/api/src/routes/jobs.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/jobs.rs)
- [crates/api/src/services/job_control_plane.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/services/job_control_plane.rs)
- [crates/api/src/routes/stream.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/stream.rs)
- [crates/muxer/src/stream_fetcher.rs](/home/khoa2807/working-sources/downloadtool/crates/muxer/src/stream_fetcher.rs)
- [frontend/src/lib/api.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/api.ts)
- [frontend/src/components/DownloadBtn.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/components/DownloadBtn.svelte)

Primary decisions:
- PostgreSQL is source of truth for job state and artifact metadata.
- Redis Streams is the first broker choice; RabbitMQ is unnecessary for phase 1.
- Worker becomes a separate Rust process from API.
- Muxed output goes to S3-compatible storage via multipart upload; browser downloads artifacts directly from storage using pre-signed URL.
- Frontend completion callback is optional optimization only; cleanup authority remains server-side TTL/GC.
- Dedup is artifact-key based, not just video-id based.
- V1 requires JWT-authenticated job creation; anonymous jobs are out of scope.
- Artifact reuse across users is enabled when `dedupe_key` matches and access policy permits reuse.
- PostgreSQL stores phase-level status only; fine-grained progress, if later needed, lives in Redis with short TTL.
- Worker crash during multipart upload restarts the build from zero in V1; resumable multipart recovery is deferred.
- Shared proxy health authority lives in Redis, not PostgreSQL.
- Durable `/api/jobs/*` is the only mux execution path; direct sync mux was removed to keep one architecture.
- Introduce a `StorageBackend` abstraction with `LocalFs` first for worker rollout safety, then `S3`/`MinIO`/`R2`.

Dependencies:
- New Redis deployment
- New S3-compatible endpoint (`MinIO` first, `R2` later)
- DB migration set for jobs/artifacts/proxy health
- Shared config for API + worker + cleanup runner

Definition of done:
- Restarting API does not lose queued/processing jobs.
- Restarting a worker at any point does not orphan jobs permanently.
- Browser downloads ready files directly from object storage, not through API body streaming.
- Dedup prevents duplicate mux execution for identical requested artifacts.
- Cleanup removes abandoned artifacts by TTL even if frontend never calls back.

Implementation snapshot:
- `/api/jobs/*` now runs only on the durable worker pipeline.
- Durable worker mode publishes jobs into Redis Streams and reads status from PostgreSQL-backed `job-system`.
- Worker binary compiles, extends job leases with heartbeat, reclaims expired leases, and deletes expired artifacts via storage backend TTL cleanup.
- Frontend mux download flow now consumes `/api/proxy/jobs/*`, accepts direct presigned URLs, and sends release hints after successful save.
