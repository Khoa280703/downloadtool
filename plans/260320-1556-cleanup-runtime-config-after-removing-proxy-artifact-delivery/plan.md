---
title: "Cleanup Runtime Config After Removing Proxy Artifact Delivery"
description: "Short plan to remove leftover runtime code/config tied to proxy-based artifact delivery."
status: pending
priority: P2
effort: 1h
branch: main
tags: [cleanup, runtime, config, delivery]
created: 2026-03-20
---

# Cleanup Runtime Config After Removing Proxy Artifact Delivery

## Goal

Remove leftover runtime code/config/env wiring that only existed for artifact delivery through frontend proxy or hybrid direct/proxy ticket handling. Do not touch docs/md unless build depends on it.

## Scope Guard

- Keep proxy inventory/health runtime (`PROXY_DATABASE_URL`, `PROXY_REDIS_URL`, admin proxy screens) if still used outside artifact delivery.
- Focus only on mux artifact delivery path: ticket issuance, client download branch, env/build knobs, compose wiring.

## Steps

1. Check backend ticket/runtime path
   - Verify Rust `file-ticket` is now authoritative direct delivery path.
   - Remove delivery-specific leftover toggles/branches that only supported proxy or hybrid artifact delivery, especially `MUX_ARTIFACT_BACKEND` assumptions if runtime is now fixed.
   - Re-check `crates/api/src/config.rs`, `crates/worker/src/worker_config.rs`, and any call sites still branching for proxy artifact delivery.

2. Check frontend/BFF delivery path
   - Audit `frontend/src/routes/api/proxy/jobs/[jobId]/file-ticket/+server.ts`, `frontend/src/lib/api.ts`, and `frontend/src/lib/playlist-download-file-saver.ts`.
   - Remove proxy-delivery fallback logic, `ticket_delivery` handling, and cache-buster/same-origin branches that only existed for direct-vs-proxy artifact delivery.
   - Keep non-delivery `/api/proxy/*` routes intact.

3. Check env/build/runtime config
   - Remove stale env/config keys and compose wiring that only existed for hybrid/proxy artifact delivery, such as `DOWNLOAD_DELIVERY_MODE` and any unused delivery-only build args/env passthrough.
   - Keep storage env required for direct artifact retrieval alive.
   - Validate `.env`, `.env.example`, `.env.production`, and `docker/docker-compose.server.yml` only for runtime/build impact.

4. Verify no runtime regressions
   - Run repo search for delivery leftovers (`file-ticket`, `ticket_delivery`, `backend-relative`, `DOWNLOAD_DELIVERY_MODE`, proxy artifact file routes).
   - Run the smallest compile/build checks covering touched areas so removed env/code paths do not break typecheck or build.

## Success Criteria

- No runtime code still pretends artifact download may go through frontend proxy if that path is removed.
- No dead delivery-only env/config/build wiring remains.
- Proxy health/inventory runtime still works.
- Touched apps compile.

## Unresolved Questions

- Is `MUX_ARTIFACT_BACKEND` still needed for real runtime backend selection, or is artifact storage now effectively fixed to one backend?
