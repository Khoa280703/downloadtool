---
title: "Remove Sync Mux Route"
description: "Delete `/api/stream/muxed` and keep `/api/jobs/*` as the only mux architecture."
status: pending
priority: P1
effort: 2h
branch: main
tags: [api, frontend, mux, jobs, cleanup]
created: 2026-03-09
---

# Remove sync mux, keep jobs-only

## Goal
- Keep `GET /api/stream` only for direct single-stream proxying.
- Remove sync mux surface `/api/stream/muxed` and any caller still constructing that URL.
- Keep all mux flows on durable `/api/jobs/*`.
- External channels continue via frontend launcher `/download/mux-job`, not direct `/api/jobs/*`.

## Main runtime files
- `crates/api/src/main.rs`
  - remove route wiring for `/api/stream/muxed`; keep `/api/stream` and `/api/jobs/*`.
- `crates/api/src/routes/stream.rs`
  - delete sync mux handler/types/helpers; leave direct stream proxy only.
- `frontend/src/components/DownloadBtn.svelte`
  - force mux-required downloads through job create + poll flow.
- `frontend/src/routes/download/mux-job/+page.svelte`
  - keep as app-domain launcher for bookmarklet/userscript/extension.
- `frontend/src/lib/api.ts`
  - keep `/api/jobs/*` helpers as single mux client contract.
- `apps/injector/src/shared/stream-utils.ts`, `apps/injector/src/bookmarklet.ts`, `apps/injector/src/userscript.ts`
  - replace any sync mux URL builder with launcher URL builder.
- `apps/extension/src/shared/stream-utils.ts`, `apps/extension/src/popup/popup.ts`, `apps/extension/src/background.ts`
  - remove direct mux download path; open launcher/job flow instead.

## Docs / scripts / config cleanup
- `frontend/vite.config.ts`
  - remove stale dev proxy for `/api/stream/muxed`.
- `config/runtime-limit-profiles.json`
  - keep direct-stream knobs and `frontend.mux_job_*`; remove any old mux-route split knobs if still present.
- `docs/system-architecture.md`
  - document one mux architecture: app -> `/api/proxy/jobs/*` -> Rust `/api/jobs/*` -> worker/storage.
- `docs/codebase-summary.md`, `docs/project-roadmap.md`, `docs/project-overview-pdr.md`, `docs/code-standards.md`
  - remove stale references to sync mux route or dual-path mux behavior.
- `scripts/stress-test-*.sh`, `scripts/stress-test-muxed-jobs.mjs`
  - keep only direct-stream or jobs-based probes; drop `/api/stream/muxed` coverage.

## Safest rollout order
1. Backend first: remove route wiring and sync handler internals in Rust API while preserving `/api/stream` and `/api/jobs/*`.
2. Caller migration second: ensure frontend, injector, extension all use job flow or launcher path before final grep cleanup.
3. Dev/runtime config third: delete stale proxy/config entries only after callers no longer depend on them.
4. Docs/scripts last: update architecture docs and stress scripts to match runtime truth.
5. Verify before deploy:
   - `rg` for `/api/stream/muxed`, old mux builders, and removed limit keys
   - `cargo check --workspace`
   - `cargo test --workspace`
   - `pnpm --filter frontend check`
   - extension/injector build if those packages changed

## Risks to fold into execution
- Queue publish failure on reused `queued` job can strand a job forever; when removing legacy mux, treat republish/recovery as part of hardening the jobs-only path.
- Auth-aware mux testing must be fixed: jobs stress script cannot hit `/api/jobs/*` anonymously anymore.
- Main web download button should handle `401` cleanly, ideally matching launcher redirect/login behavior.
- `POST /api/jobs/{id}/release` semantics should be clarified; today it is only a soft access update, not real cleanup.

## Done criteria
- No active runtime route or caller can hit `/api/stream/muxed`.
- All mux-capable clients converge on `/api/jobs/*` or `/download/mux-job`.
- Direct stream path remains only `GET /api/stream`.
