---
title: "Phase 1 - Playlist Job Data Model"
status: pending
---

# Phase 1: Playlist Job Data Model

## Context Links
- Current discovery endpoint: [batch.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/batch.rs)
- Current durable item jobs: [jobs.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/jobs.rs)
- Current control plane: [job_control_plane.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/services/job_control_plane.rs)

## Overview
Priority: P1
Status: pending
Goal: add playlist-level persistence so backend can own discovery, selection, queueing, item status, partial failure, and resume.

## Key Insights
- Current `mux_jobs` are item-level only. Good for artifact work, wrong place to store playlist shape.
- `/api/batch` emits ephemeral SSE only. Refresh/close tab loses orchestration state.
- Minimal clean split: `playlist_jobs` own playlist lifecycle, `playlist_job_items` own per-video lifecycle, `mux_jobs` stay artifact jobs.

## Requirements
- Persist playlist URL, owner, selected mode/quality snapshot, status, counts.
- Persist playlist items with stable `video_id`, title, thumbnail, order, selected flag, state, failure detail, linked artifact job id.
- Support resume after refresh and partial reruns.

## Architecture
- Add `playlist_jobs` table:
  `id`, `owner fields`, `playlist_url`, `playlist_id`, `title?`, `status`, `discovery_status`, `download_mode`, `quality`, `total_items`, `selected_items`, `completed_items`, `failed_items`, timestamps.
- Add `playlist_job_items` table:
  `id`, `playlist_job_id`, `video_id`, `title`, `thumbnail`, `position`, `selected`, `status`, `last_error`, `artifact_job_id`, `attempt_count`, timestamps.
- Optional `playlist_job_events` table only if admin/audit needs persistent playlist event history. If not needed now, skip and use Redis pubsub + aggregate columns.

## Related Code Files
- Modify:
  `crates/api/migrations/*`
  `crates/api/app-migrations/*`
  `crates/job-system/src/*`
- Create:
  playlist repository modules in `crates/job-system/src/`

## Implementation Steps
1. Add SQL migrations for `playlist_jobs` and `playlist_job_items`.
2. Add repository methods: create job, append items, update aggregate counters, list job/items, claim next pending item, mark item terminal.
3. Add owner scoping consistent with current `JobOwner`.
4. Add lightweight model enums for playlist/item statuses.

## Todo List
- [ ] Define playlist job status enum
- [ ] Define playlist item status enum
- [ ] Add migrations
- [ ] Add repository CRUD + claim methods
- [ ] Add aggregate counter helpers

## Success Criteria
- Backend can create a playlist job and persist items before any download starts.
- Backend can resume playlist state after process restart.
- Queries needed for frontend/admin are indexed.

## Risk Assessment
- Risk: duplicate modeling with current `mux_jobs`.
  Mitigation: playlist tables hold orchestration only, item artifact work stays in existing jobs.
- Risk: over-designing event tables.
  Mitigation: defer persistent playlist events unless admin explicitly needs them.

## Security Considerations
- Scope every read/write by current owner/session.
- Never trust item IDs from client without verifying playlist ownership.

## Next Steps
- Feed repository into playlist control plane and API layer.

## Unresolved Questions
- Whether playlist title/cover should be stored now or derived later from first item.
