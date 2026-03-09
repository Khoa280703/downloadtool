# Phase 03 - Worker and Storage Pipeline

## Context Links
- [crates/muxer/src/stream_fetcher.rs](/home/khoa2807/working-sources/downloadtool/crates/muxer/src/stream_fetcher.rs)
- [crates/muxer/src/fmp4_remuxer.rs](/home/khoa2807/working-sources/downloadtool/crates/muxer/src/fmp4_remuxer.rs)
- [crates/proxy/src/proxy_pool.rs](/home/khoa2807/working-sources/downloadtool/crates/proxy/src/proxy_pool.rs)
- [crates/extractor/src/ytdlp.rs](/home/khoa2807/working-sources/downloadtool/crates/extractor/src/ytdlp.rs)

## Overview
- Priority: P0
- Current status: approved
- Goal: define worker execution loop, proxy coordination, and zero-temp-file upload path.

## Key Insights
- The hardest technical constraint is whether mux output is fully streamable into multipart upload without seek/rewrite.
- Worker-local proxy health is insufficient once multiple workers run in parallel.
- MinIO phase 1 and R2 phase 2 should use one shared object storage trait.

## Requirements
- Functional:
1. Worker claims jobs with lease.
2. Worker can refresh URLs on auth-like failures.
3. Worker uploads final artifact through multipart upload.
4. Worker updates DB and broker atomically enough to avoid orphan work.
- Non-functional:
1. No local artifact file writes in happy path.
2. Retry bounded and observable.
3. Shared proxy cooldown semantics.
4. Crash recovery stays simple and restart-safe in V1.

## Architecture
- Worker loop:
1. Read message from Redis group.
2. Load job row from Postgres.
3. Acquire DB lease if job still runnable.
4. Check dedup artifact. If ready, attach and finish.
5. Resolve proxy and stream source URLs.
6. Start multipart upload.
7. Pipe muxed bytes into multipart uploader.
8. Complete upload, update artifact + job rows, ack stream.
- Storage abstraction:
  `begin_multipart`, `upload_part`, `complete_multipart`, `abort_multipart`, `presign_get`
- Proxy coordination:
  worker keeps fast local cache, but writes cooldown/failure back to shared store.

## Final Decisions
- V1 crash recovery for multipart upload is restart-from-zero only.
- On worker death or lease expiry, replacement worker aborts stale multipart upload and rebuilds artifact from scratch.
- Shared proxy health authority is Redis because proxy liveness/cooldown is ephemeral and high-churn.
- Worker may keep a short-lived local proxy cache, but Redis is authoritative for cooldown enforcement.

## Worker Lease Model
- `lease_owner = worker_id`
- `lease_expires_at = now + lease_window`
- Heartbeat extends lease every N seconds
- Reclaimer can requeue jobs whose lease expired and status still non-terminal
- `attempt_count` increments only when worker actually starts execution, not on enqueue

## Retry Rules
- Retry on transient upstream/network/proxy/object-storage errors
- Do not retry on deterministic validation errors:
  unsupported WebM-in-fMP4, invalid stream url, missing selected format ids
- Backoff source of truth stays in DB metadata or worker config

## Multipart Upload Strategy
- Buffer target part size in memory, e.g. 8-16 MB
- Each filled buffer becomes one multipart part
- Last partial buffer uploads as final part
- On failure: abort multipart upload and keep artifact status `failed` or `incomplete`
- On worker crash: new worker aborts prior multipart state and restarts cleanly

## Dedupe Rules
- Before execution:
  if `mux_artifacts.status = ready` for `dedupe_key`, attach and finish immediately
- During execution:
  first worker sets artifact row to `building`
  later workers seeing same `dedupe_key` join existing artifact row instead of rebuilding
- After execution:
  many jobs can point to same `artifact_id`

## Related Code Files
- Modify:
  [crates/proxy/src/proxy_pool.rs](/home/khoa2807/working-sources/downloadtool/crates/proxy/src/proxy_pool.rs)
  [crates/muxer/src/stream_fetcher.rs](/home/khoa2807/working-sources/downloadtool/crates/muxer/src/stream_fetcher.rs)
- Create:
  `crates/worker/src/main.rs`
  `crates/worker/src/job-runner.rs`
  `crates/worker/src/lease-heartbeat.rs`
  `crates/object-store/src/lib.rs`
  `crates/object-store/src/minio.rs`
  `crates/object-store/src/r2.rs`
  `crates/queue/src/redis_streams.rs`

## Implementation Steps
1. Validate remuxer output can be consumed as forward-only byte stream.
2. Build object storage abstraction.
3. Build worker binary.
4. Add shared proxy health writeback.
5. Add artifact dedup lock/build semantics.

## Todo List
- [ ] Confirm remuxer seek requirements
- [ ] Pick multipart part size
- [ ] Define worker heartbeat interval
- [ ] Define max attempts by error class
- [ ] Define Redis key schema and TTL policy for shared proxy cooldown

## Success Criteria
- Worker can finish job without writing artifact file locally.
- Two workers cannot both build the same dedupe artifact successfully at the same time.
- Crash during upload does not leave job permanently stuck in `processing`.

## Risk Assessment
- Forward-only streaming may expose hidden muxer assumptions.
- Weak Redis keying/TTL rules for proxy cooldown could cause workers to hammer a blocked proxy together.
- Multipart uploads need explicit abort or storage costs leak.

## Security Considerations
- Storage credentials stay server-side only.
- Worker logs must not print full signed URLs or proxy secrets.
- Proxy URLs in DB should be redacted or encrypted at rest if persisted.

## Next Steps
- Write migration/cutover phases in phase 04.

## Open Questions
- None for this phase.
