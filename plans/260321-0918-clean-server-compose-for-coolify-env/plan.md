---
title: "Clean Server Compose For Coolify Env"
description: "Trim docker-compose.server.yml to service wiring only and move deploy env ownership to Coolify"
status: pending
priority: P2
effort: 1h
branch: main
tags: [docker, compose, coolify, env, cleanup]
created: 2026-03-21
---

# Clean Server Compose For Coolify Env

## Context

- Current `docker/docker-compose.server.yml` still mixes internal wiring with secrets, public app config, and runtime tuning.
- Goal: keep compose as topology only; Coolify owns deploy-time env values.

## Workstreams

1. **Classify env surface**
- Keep only topology/wiring in compose: `build`, `depends_on`, `expose`, volumes, networks, labels, healthchecks.
- Move env ownership to Coolify: secrets, public URLs, DB/Redis URLs, auth/provider vars, and runtime knobs.

2. **Clean compose**
- Remove inline/defaulted env values from `api`, `worker`, `frontend` that are not pure container wiring.
- Keep internal hostnames only in the Coolify env contract, not embedded in compose value templates.
- Recheck whether localhost `ports` on internal data services are still needed in server deploy; drop if not required.

3. **Lock env contract**
- Coolify must provide: `DATABASE_URL`, `PROXY_DATABASE_URL`, `REDIS_URL`, `PROXY_REDIS_URL`, `RUST_API_URL`, `VITE_API_URL`, `ORIGIN`, auth/provider vars, `S3_*`, `MUX_*`, `PROXY_QUARANTINE_TTL_SECS`, `RUST_LOG`, admin/public config.
- Values must still target compose service names for internal calls: `postgres`, `redis`, `shared-proxy-postgres`, `shared-proxy-redis`, `api`.

## Done

- `docker/docker-compose.server.yml` no longer stores secrets, public defaults, or runtime tuning.
- Coolify env fully owns deploy config without losing internal service connectivity.
- Frontend build-time and runtime envs are explicitly accounted for.

## Non-goals

- No code/runtime behavior changes beyond config ownership cleanup.
- No docs/markdown updates outside this plan.

## Unresolved Questions

- Should server-only localhost `ports` for Postgres/Redis/shared-proxy remain for ops access, or be removed entirely in Coolify?
