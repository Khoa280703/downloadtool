---
title: "Phase 3 - Durable Item Execution And Worker Reuse"
status: pending
---

# Phase 3: Durable Item Execution And Worker Reuse

## Context Links
- Current item creation: [jobs.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/jobs.rs)
- Current worker: [job_runner.rs](/home/khoa2807/working-sources/downloadtool/crates/worker/src/job_runner.rs)
- Current mux pipeline: [mux_pipeline.rs](/home/khoa2807/working-sources/downloadtool/crates/worker/src/mux_pipeline.rs)

## Overview
Priority: P1
Status: pending
Goal: let backend playlist orchestration reuse current durable artifact pipeline for both mux-required and direct-stream-capable items.

## Key Insights
- If backend owns playlist fully, direct-stream items also need durable downloadable outputs.
- Reusing only current `mux_jobs` is almost enough, but direct-stream items have no artifact job today.
- Minimal path: widen the item job model, do not build a second queue system.

## Requirements
- Keep just-in-time extract per item, right before actual processing.
- Support two item job kinds:
  `mux` = video+audio merge
  `passthrough` = single stream copied to artifact storage without mux
- Preserve artifact reuse by dedupe key.

## Architecture
- Extend current durable job request with:
  `job_kind`, optional `stream_url`, optional `audio_url`, optional `source_url`, format metadata.
- Worker flow:
  1. Playlist dispatcher claims next selected pending item.
  2. Performs extract on server for that one video.
  3. Chooses best stream based on saved mode/quality.
  4. Creates durable item job:
     - `mux` if separate video/audio needed
     - `passthrough` if single downloadable stream is enough
  5. Item subscribes to existing job progress and mirrors aggregate progress to playlist job.

## Related Code Files
- Modify:
  `crates/api/src/routes/jobs.rs`
  `crates/api/src/services/job_control_plane.rs`
  `crates/worker/src/job_runner.rs`
  `crates/worker/src/mux_pipeline.rs`
  `crates/job-system/src/*`
  `crates/api/migrations/*`
- Create:
  maybe `crates/worker/src/passthrough_pipeline.rs`

## Implementation Steps
1. Add `job_kind` and nullable fields needed for passthrough jobs.
2. Update dedupe identity to include kind + relevant formats.
3. Add worker branch for passthrough artifact upload.
4. Mirror durable item job terminal state back into `playlist_job_items`.
5. Record item-to-artifact-job link for audit/admin.

## Todo List
- [ ] Extend durable job schema
- [ ] Add passthrough worker path
- [ ] Keep existing mux path untouched for single-video flow
- [ ] Mirror item progress to playlist aggregates
- [ ] Add tests for mux and passthrough creation

## Success Criteria
- Playlist backend can finish both “video only/direct stream” and “needs mux” items durably.
- Existing single-video download flow remains backward compatible.
- Artifact reuse still works for repeated items.

## Risk Assessment
- Risk: widening `mux_jobs` semantics under old naming.
  Mitigation: keep table/route names for now, rename later only if needed.
- Risk: passthrough path accidentally slower than current browser direct stream.
  Mitigation: ship behind flag; playlist flow values durability over peak speed.

## Security Considerations
- Server-side extract must preserve current proxy selection and anti-bot protections.
- Validate all upstream URLs before job creation.

## Next Steps
- Switch frontend playlist UI to backend job snapshots and SSE.

## Unresolved Questions
- Whether `passthrough` should upload full artifact to R2 always or allow localfs in dev only.
