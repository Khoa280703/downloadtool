---
title: "Hybrid R2 Direct Download Delivery"
description: "Default browser->R2 direct download, fallback browser->frontend proxy->R2 for incompatible UAs"
status: in_progress
priority: P1
effort: 6h
branch: main
tags: [download, r2, performance, delivery]
created: 2026-03-20
---

# Hybrid R2 Direct Download Delivery

## Goal

Change artifact delivery so browser downloads directly from R2 (signed URL) by default, with frontend proxy as fallback for incompatible user agents (Safari/iOS). Keep extract->mux->upload R2 pipeline unchanged.

## Current State

- Rust backend already supports presigned R2 URLs via `StorageTicketService::build_ticket()` with `content_disposition` param
- `MUX_DIRECT_DOWNLOAD=true` env already exists in Rust backend
- Frontend `file-ticket` route **overrides** backend response -- always returns proxy URL (`/api/proxy/jobs/:jobId/file`)
- Frontend `file` route fetches ticket from backend, then proxies the actual download (R2 or local)
- Client (`api.ts`) always receives proxy URL, never sees R2 directly

## Architecture Change

```
BEFORE: Browser -> file-ticket -> ALWAYS proxy URL -> file route -> R2
AFTER:  Browser -> file-ticket -> DECIDE(direct R2 URL | proxy URL) -> download
```

## Phases

| # | Phase | Status | Effort |
|---|-------|--------|--------|
| 1 | [Backend ticket + hybrid delivery config](phase-01-backend-ticket-hybrid-delivery.md) | **done** | 1.5h |
| 2 | [Client consume hybrid ticket](phase-02-client-consume-hybrid-ticket.md) | **done** | 2h |
| 3 | [R2 CORS verification + content-disposition](phase-03-cors-r2-verification.md) | pending (ops) | 0.5h |
| 4 | [Observability + audit logging](phase-04-observability-audit-logging.md) | **done** | 1h |
| 5 | [Rollout + testing](phase-05-rollout-testing.md) | pending (ops) | 1h |

## Critical Constraints (from reviewer feedback)

1. **Cache-buster breaks signed URLs**: `appendCacheBuster()` must be skipped for absolute R2 URLs. Adding `&t=123` to signed URL → 403. Only apply cache-buster to relative proxy paths.
2. **FSAA-only direct in Phase 1**: Anchor-mode (`<a>` click) provides no JS failure signal for cross-origin downloads. Direct R2 delivery only for FSAA path (fetch + stream). Anchor path always uses proxy URL.
3. **SvelteKit env pattern**: Server code MUST use `$env/dynamic/private`, not `import.meta.env`. All existing server files follow this pattern.
4. **Observability — two-level tracking**: `job_file_ticket.ticket_delivery` = server's decision. Client may downgrade direct→proxy if no FSAA. `job_file` rows = always proxy. Estimate actual browser-direct: ticket=direct minus matching job_file rows. No single authoritative source — accept gap, approximate.
5. **`DOWNLOAD_DELIVERY_MODE=direct` semantics**: Server skips UA filter and always returns R2 URL in ticket. Client still has safety override — no FSAA → downgrade to proxy. "direct" = "server offers direct", not "force browser direct".

## Key Findings from Code Review

1. **Rust presign already handles Content-Disposition**: `s3_storage_backend.rs:93-95` passes `response_content_disposition` to AWS SDK. `storage_ticket_service.rs:65-67` builds proper `attachment; filename="..."; filename*=UTF-8''...` header. No Rust changes needed.
2. **Frontend file-ticket route is the bottleneck**: Line 61-64 of `file-ticket/+server.ts` always overrides with proxy URL. This is where hybrid decision logic goes.
3. **Frontend file route already handles signed URLs**: `file/+server.ts:46-51` detects absolute URLs and fetches directly from R2 without auth headers. This route stays as-is for proxy fallback.
4. **Client `toAbsoluteDownloadUrl()` already handles absolute URLs**: `api.ts:189-196` returns external URLs as-is. Minimal client change needed.
5. **`MUX_DIRECT_DOWNLOAD` env exists but only controls Rust backend behavior**: Frontend needs its own env vars (`DOWNLOAD_DELIVERY_MODE`, `DOWNLOAD_PROXY_UA_REGEX`).

## Config

| Env Var | Default | Description |
|---------|---------|-------------|
| `DOWNLOAD_DELIVERY_MODE` | `hybrid` | `hybrid` / `direct` / `proxy` |
| `DOWNLOAD_PROXY_UA_REGEX` | `Safari\|iPhone\|iPad` | UA patterns that force proxy mode |

## Dependencies

- R2 CORS must be configured before enabling direct mode (Phase 3 before Phase 5)
- No Rust backend changes needed -- all changes in frontend SvelteKit routes + client TS
