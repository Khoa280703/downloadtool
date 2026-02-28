---
title: "User Auth + Subscription System"
description: "Better Auth (SvelteKit) + JWT (Rust) + Whop webhooks — infrastructure only, no feature gates"
status: pending
priority: P1
effort: 14h
branch: main
tags: [auth, subscription, whop, postgres, better-auth, jwt]
created: 2026-02-28
---

# User Auth + Subscription System

**Brainstorm:** `plans/reports/brainstorm-260228-1159-user-auth-subscription-system.md`

## Architecture Summary

```
[Browser]
  ↓ cookie session
[SvelteKit + Better Auth]  →  PostgreSQL (users, sessions, accounts)
  ↓ JWT HS256 (15min, user_id + tier)
[Rust Axum API]
  ├── middleware: verify JWT (stateless, ~0.1ms)
  ├── /api/extract: DB lookup subscription for accuracy
  └── inject UserTier(Anonymous|Free|Premium) into extensions

[Whop]  →  POST /api/webhooks/whop  →  Rust  →  PostgreSQL (subscriptions)
```

## Payment Stack

| Provider | Purpose | Status |
|----------|---------|--------|
| Whop | Primary — global recurring, MoR (handles tax), 6% fee, no company required | **active** |
| SePay | VN bank transfer | deferred |
| NowPayments | Crypto backup (USDT/BTC) | deferred |

## Phases

| # | Phase | File | Est. | Status |
|---|-------|------|------|--------|
| 1 | Database Setup (PostgreSQL + migrations) | [phase-01](./phase-01-database-setup.md) | 2h | pending |
| 2 | Better Auth in SvelteKit | [phase-02](./phase-02-better-auth-sveltekit.md) | 3h | pending |
| 3 | Rust JWT Middleware | [phase-03](./phase-03-rust-jwt-middleware.md) | 2h | pending |
| 4 | Whop Webhook Endpoint (Rust) | [phase-04](./phase-04-stripe-webhook.md) | 3h | pending |
| 5 | Frontend Auth UI | [phase-05](./phase-05-frontend-auth-ui.md) | 4h | pending |

## Key Dependencies

- Phase 1 must complete before all others (DB must exist)
- Phase 2 must complete before Phase 3 (JWT secret defined in Better Auth first)
- Phase 4 is independent of Phase 3 (both read DB but don't share code)
- Phase 5 depends on Phase 2 (needs auth client configured)

## Out of Scope

- Premium feature gates (download limits, quality caps)
- `usage_daily` table / per-user rate limiting
- SePay / VietQR payment (deferred)
- NowPayments crypto (deferred)
