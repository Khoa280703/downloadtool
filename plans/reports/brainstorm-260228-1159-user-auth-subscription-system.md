# Brainstorm Report: User Auth + Subscription System

**Date:** 2026-02-28
**Status:** Agreed, ready to plan

## Problem Statement

Build user authentication + subscription infrastructure for the download tool. Tool is public, monetized with ads, planning to add premium subscription tier. Need to implement the foundation now; premium feature gates (download limits, quality caps, playlist limits) defined later.

## Requirements

- Self-hosted auth (no vendor lock-in)
- Recurring subscription (monthly/yearly) via Whop (primary)
- Global market; SePay/VietQR deferred until VN traction validated
- Anonymous users continue to work (no forced login)
- JWT-based auth between SvelteKit IdP and Rust API (stateless Rust)
- Infrastructure ready for feature gates; actual limits = future scope

## Evaluated Approaches

### Option A: Managed auth (Supabase/Clerk)
- Pros: Zero auth implementation, generous free tier
- Cons: Vendor lock-in, data outside homeserver → rejected (user wants self-hosted)

### Option B: Rust-native auth
- Pros: Single process, no Node.js
- Cons: Must implement session management, OAuth, password hashing from scratch in Rust — no battle-tested library equivalent to Better Auth
- Rejected: too much undifferentiated work

### Option C: Better Auth in SvelteKit + Rust JWT verification ✅ SELECTED
- Better Auth handles: login/OAuth/session/DB
- SvelteKit emits JWT (via Better Auth JWT plugin)
- Rust verifies JWT (stateless, ~0.1ms, no DB call for most requests)
- Whop webhook handled by Rust API (not SvelteKit — more stable process)

## Final Architecture

```
[Browser]
  ↓ (cookie session)
[SvelteKit + Better Auth]  →  PostgreSQL (users, sessions)
  ↓ JWT (signed, 15min expiry, contains user_id + tier)
[Rust Axum API]
  ├── Middleware: verify JWT signature (stateless)
  ├── On /api/extract: DB lookup subscription status (accuracy for billing)
  └── Inject UserTier(anonymous|free|premium) into request extensions

[Whop]  →  POST /api/webhooks/whop  →  Rust  →  PostgreSQL (subscriptions)
```

## Technology Stack

| Layer | Choice | Reason |
|-------|--------|--------|
| Auth | Better Auth (TypeScript) | Battle-tested, SvelteKit native, supports OAuth + JWT plugin |
| JWT emit | Better Auth JWT plugin | Standard RS256/HS256, Rust can verify with `jsonwebtoken` crate |
| Database | PostgreSQL | Shared between Better Auth and Rust |
| Payment (primary) | Whop | MoR (handles global tax), no company needed, 6% fee, HMAC webhook same pattern as Stripe |
| Payment (VN) | SePay | Bank transfer for VN users — deferred |
| JWT verify | `jsonwebtoken` crate in Rust | ~0.1ms, stateless |

## Database Schema

```sql
-- Managed by Better Auth
users (id, email, whop_user_id TEXT, created_at, ...)
sessions (id, user_id, expires_at, ...)

-- Managed by Rust API
subscriptions (
  id, user_id, plan TEXT,      -- 'free' | 'premium'
  status TEXT,                 -- 'active' | 'cancelled' | 'expired'
  current_period_end TIMESTAMPTZ,
  whop_membership_id TEXT,
  created_at, updated_at,
  UNIQUE(user_id)
)
-- usage_daily: deferred — add when implementing feature limits
```

## Key Design Decisions

1. **Webhook on Rust, not SvelteKit** — Rust process more stable, won't miss webhooks during frontend deploys
2. **DB check on /api/extract only** — verify live subscription status for expensive operation; /api/stream trusts JWT
3. **JWT 15min expiry + refresh** — Better Auth handles refresh automatically; subscription changes reflected within 15min
4. **UserTier enum in Rust** — handlers receive `UserTier` from middleware extension; actual feature gates added later without middleware changes
5. **Whop as primary payment** — MoR model means no tax/company overhead; 6% fee acceptable at early stage; HMAC webhook is same pattern as Stripe so migration cost is low
6. **SePay deferred** — YAGNI; add only when VN user analytics or crypto demand justify the complexity

## Scope of This Plan (Infrastructure Only)

**IN scope:**
- Better Auth setup in SvelteKit (email/password + Google OAuth)
- JWT emission via Better Auth JWT plugin
- PostgreSQL `subscriptions` table migration
- Rust: `jsonwebtoken` JWT middleware + `UserTier` injection
- Rust: `POST /api/webhooks/whop` endpoint
- Whop product/plan setup (1 recurring plan)
- Frontend: Whop Checkout redirect + callback to store `whop_user_id`

**OUT of scope (future):**
- Premium feature gates (download limits, quality caps, playlist limits)
- Usage tracking (`usage_daily` table)
- Per-user rate limiting (currently IP-based from phase 04 plan)
- SePay VietQR payment
- NowPayments crypto payment

## Risks

- Better Auth JWT plugin compatibility with SvelteKit server routes: verify in research phase
- Whop webhook signature verification: manual HMAC-SHA256 (no official Rust SDK)
- JWT secret rotation: need env var strategy; document in deployment guide
- Whop checkout callback may not reliably pass `whop_user_id` in redirect URL — fallback: query Whop API by email
