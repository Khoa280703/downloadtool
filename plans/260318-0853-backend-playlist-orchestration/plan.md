---
title: "Backend-Orchestrated Playlist Downloads"
description: "Move playlist downloading from browser-managed queueing to durable backend orchestration while keeping just-in-time per-item extract."
status: pending
priority: P1
effort: 14h
branch: main
tags: [playlist, backend, jobs, worker, sse, rate-limit]
created: 2026-03-18
---

# Backend-Orchestrated Playlist Downloads

## Goal

- Keep playlist discovery lightweight via `/api/batch`.
- Move playlist execution, retries, sequencing, and progress to backend.
- Keep `just-in-time extract per item` so stream URLs do not expire before use.
- Reuse existing durable mux job system instead of building a second pipeline.

## Current Truth

- `/api/batch` only returns playlist item metadata from Rust SSE.
- Browser owns queueing via `frontend/src/lib/playlist-download-worker.ts`.
- Browser re-calls `/api/proxy/extract` for every selected item.
- Browser creates mux jobs item-by-item when needed.
- This keeps URLs fresh, but rate limit and orchestration still sit in client.

## Target Shape

- Browser does 2 things only:
  - preview playlist items
  - create playlist job + subscribe progress + trigger browser save when item ready
- Backend owns:
  - playlist job state
  - item sequencing
  - retries / cooldowns
  - just-in-time extract and stream selection
  - mux delegation to existing `/api/jobs/*`

## Phases

| Phase | Status | Description |
|-------|--------|-------------|
| [Phase 1](phase-01-data-model-and-api-contract.md) | pending | Add playlist job data model and API contract |
| [Phase 2](phase-02-playlist-worker-and-item-state-machine.md) | pending | Add backend worker/orchestrator for playlist items |
| [Phase 3](phase-03-frontend-cutover-to-playlist-jobs.md) | pending | Replace client worker with playlist job UI/progress flow |
| [Phase 4](phase-04-hardening-rate-limit-and-cleanup.md) | pending | Cut over fully, tighten rate limiting, remove obsolete client orchestration |

## Non-Goals

- Do not rewrite single-video flow.
- Do not replace existing mux job system.
- Do not make backend pre-extract all stream URLs for whole playlist.
- Do not add premium/business logic in same change.

## Key Dependencies

- Existing mux job control plane in `crates/api/src/routes/jobs.rs`
- Existing durable repository in `crates/job-system/*`
- Existing SSE progress model and frontend batch UI

## Rollout Strategy

1. Add playlist job backend behind new endpoints, keep old client worker alive.
2. Switch frontend playlist download to new backend path behind a feature flag or branch-local cutover.
3. Verify real playlist downloads on local + production.
4. Remove old client orchestration only after parity is proven.

## Done Criteria

- Starting a playlist download creates exactly one backend playlist job.
- Browser no longer calls `/api/proxy/extract` once per playlist item.
- Each item still extracts just before preparation/download, not upfront for whole playlist.
- Mux-required items still reuse existing durable mux job pipeline.
- Playlist progress survives tab reload and API reconnect.
- Cloudflare rate limiting can focus on playlist create endpoints instead of per-item extract spam.

## Residual Risks

1. Direct-stream items need a durable “ready without artifact” model; this is the trickiest contract.
2. Progress UX can regress if item-level and playlist-level events are not normalized early.
3. Cutover will touch Rust API, worker, DB schema, and Svelte UI at once.

## Unresolved Questions

- None blocking for implementation plan.
