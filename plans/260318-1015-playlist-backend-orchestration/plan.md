---
title: "Playlist Backend Orchestration"
description: "Move playlist download orchestration from browser-managed queue to durable backend jobs while keeping just-in-time extract per item."
status: pending
priority: P1
effort: 16h
branch: main
tags: [playlist, backend, jobs, sse, worker, retry]
created: 2026-03-18
---

# Playlist Backend Orchestration

## Goal

Move playlist download from client-orchestrated queue to durable backend orchestration.
Keep `single` download flow unchanged.
Keep `just-in-time extract` per item so signed stream URLs do not expire before use.

## Current anchors

- [crates/api/src/routes/batch.rs](../../crates/api/src/routes/batch.rs)
- [crates/api/src/routes/jobs.rs](../../crates/api/src/routes/jobs.rs)
- [crates/api/src/services/job_control_plane.rs](../../crates/api/src/services/job_control_plane.rs)
- [frontend/src/routes/+page.svelte](../../frontend/src/routes/+page.svelte)
- [frontend/src/lib/api.ts](../../frontend/src/lib/api.ts)
- [frontend/src/lib/playlist-download-worker.ts](../../frontend/src/lib/playlist-download-worker.ts)
- [frontend/src/stores/batch.ts](../../frontend/src/stores/batch.ts)

## Core decision

Do not pre-extract stream URLs for the whole playlist.
Server only stores playlist item identity + requested output options.
Worker extracts each item only when item is about to run.

## Phases

| # | Phase | File | Status | Est |
|---|-------|------|--------|-----|
| 1 | Target architecture + contracts | `phase-01-target-architecture.md` | pending | 3h |
| 2 | Durable playlist data model + API | `phase-02-data-model-and-api-contract.md` | pending | 4h |
| 3 | Worker orchestration + progress fan-out | `phase-03-worker-orchestration-and-progress.md` | pending | 5h |
| 4 | Frontend migration + compatibility layer | `phase-04-frontend-migration-and-compat.md` | pending | 3h |
| 5 | Rollout, migration, validation | `phase-05-rollout-and-validation.md` | pending | 1h |

## Must keep

- Single video flow keeps using existing durable mux job path.
- Playlist item extract remains just-in-time.
- Reuse existing job/artifact primitives where possible.
- Progress stays realtime via SSE.
- Retry is per item, not whole playlist only.
- Cancel and resume are server-side concepts, not browser-only state.

## Must remove

- Browser as source of truth for playlist queue state.
- Frontend-owned retry sequencing for playlist items.
- Need to keep tab alive for playlist to continue.

## Definition of done

- User starts one playlist job with one API request.
- Browser can refresh and still recover playlist state/progress.
- Worker continues even if tab closes.
- Each item can be retried/cancelled independently.
- Completed items are reusable without rebuilding.
- Single download flow is unchanged and passes regression checks.

## Links

- [Phase 1](./phase-01-target-architecture.md)
- [Phase 2](./phase-02-data-model-and-api-contract.md)
- [Phase 3](./phase-03-worker-orchestration-and-progress.md)
- [Phase 4](./phase-04-frontend-migration-and-compat.md)
- [Phase 5](./phase-05-rollout-and-validation.md)

## Unresolved questions

- Playlist final UX: save individually only, or zip/archive later.
- Resume semantics: auto-resume all pending items or require explicit resume action.
