---
title: "Cleanup Proxy Artifact Delivery Runtime"
description: "Remove leftover hybrid/proxy artifact-delivery runtime, env, and build wiring after direct signed-ticket delivery"
status: pending
priority: P2
effort: 3h
branch: main
tags: [cleanup, runtime, config, delivery]
created: 2026-03-20
---

# Cleanup Proxy Artifact Delivery Runtime

## Context

- Follow-up to `plans/260320-1030-hybrid-r2-direct-delivery/plan.md`
- Goal: remove leftover runtime/config/build surfaces that still imply proxy artifact delivery or hybrid direct/proxy ticket handling.
- Keep proxy inventory/health runtime (`PROXY_DATABASE_URL`, `PROXY_REDIS_URL`, `PROXY_QUARANTINE_TTL_SECS`) unless a reference is proven artifact-delivery-specific.

## Workstreams

1. **Reality-check sweep**
- Re-scan current source for `file-ticket`, `ticket_delivery`, `backend-relative`, `DOWNLOAD_DELIVERY_MODE`, and `MUX_ARTIFACT_BACKEND`.
- Confirm which reported leftovers still exist in HEAD before deleting anything. Current quick grep only confirms root `.env`; plan assumes some prior findings may already be stale.

2. **Frontend/BFF delivery contract cleanup**
- Simplify `frontend/src/routes/api/proxy/jobs/[jobId]/file-ticket/+server.ts` and nearby client callers so mux artifact delivery has one supported contract: backend returns a direct signed URL.
- Remove legacy hybrid/proxy-only naming, decision branches, and telemetry fields from `frontend/src/lib/api.ts`, `frontend/src/lib/playlist-download-file-saver.ts`, and adjacent helpers/tests if still present.

3. **Runtime env + build wiring cleanup**
- Remove obsolete artifact-delivery knobs such as `DOWNLOAD_DELIVERY_MODE` and `MUX_ARTIFACT_BACKEND` from active env files, Docker/build args, generated env typings, and deploy/local scripts if they are no longer read.
- Keep unrelated proxy pool/inventory envs untouched.

4. **Build-safety verification**
- Verify no active codepath still expects hybrid artifact delivery or backend-relative tickets.
- Run `pnpm --filter frontend check`, `pnpm --filter frontend build`, and `cargo check -p api -p worker`.

## Done

- No active runtime/build surface still advertises hybrid direct/proxy artifact ticket handling.
- Obsolete artifact-delivery envs are removed only where they are truly unused.
- Frontend and Rust checks pass without reintroducing proxy-inventory regressions.

## Non-goals

- No cleanup of extractor proxy usage, proxy pool management, admin proxy tooling, or proxy DB/Redis wiring used for inventory/health.
- No docs/markdown updates unless required to keep build or generated artifacts green.

## Unresolved Questions

- Is any non-R2/local-storage artifact backend still intentionally supported in live codepaths, or can artifact ticket generation now be treated as permanently single-mode?
