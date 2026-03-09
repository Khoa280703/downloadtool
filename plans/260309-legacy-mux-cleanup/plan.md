---
title: "Legacy Mux Cleanup"
description: "Remove legacy mux paths and stale config/docs references so only direct stream and durable mux jobs remain."
status: completed
priority: P1
effort: 3h
branch: main
tags: [api, frontend, cleanup, mux, jobs]
created: 2026-03-09
---

# Legacy mux cleanup

## Goal
- Keep `GET /api/stream` direct streaming path.
- Remove the legacy direct mux route from the API surface.
- Remove legacy backend in-process/rollback mux job code.
- Keep durable mux job flow under `/api/jobs/*` as the only job pipeline.

## Scope cleaned
- Runtime/backend:
  - removed in-process mux job queue/store services
  - removed obsolete execution-mode flag from active config path
  - `/api/jobs/*` now always uses durable Postgres + Redis worker pipeline
- Config/docs:
  - removed dead frontend mux-routing limit references
  - updated docs that still described obsolete execution flow or deleted legacy route files
- Preserved surfaces:
  - `GET /api/stream`
  - `POST/GET /api/jobs/*`

## Cleanup completed
- Backend cleanup
  - deleted old in-process mux job services and route wiring
  - removed sync mux route and moved remaining callers to jobs-only launcher flow
- Config/doc cleanup
  - removed stale runtime-limit variable descriptions no longer read by frontend
  - updated roadmap/summary/implementation plans to reflect current state
- Verification
  - `cargo check --workspace`
  - `cargo test --workspace`
  - `pnpm --filter frontend check`

## Success criteria
- No runtime reference to legacy in-process mux job execution remains.
- No stale docs/config reference dead frontend mux-routing variables.
- Only `GET /api/stream` and durable `/api/jobs/*` remain as active download surfaces.
