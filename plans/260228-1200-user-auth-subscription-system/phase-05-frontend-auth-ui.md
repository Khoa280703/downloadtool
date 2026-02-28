---
title: "Phase 05 — Frontend Auth UI"
priority: P2
status: completed
effort: 4h
---

# Phase 05 — Frontend Auth UI (Login Page + Account Page)

**Context:** `plans/reports/brainstorm-260228-1159-user-auth-subscription-system.md`
**Plan overview:** `plan.md`
**Depends on:** Phase 02 (Better Auth configured), Phase 03 (Rust accepts JWT)

## Overview

Build the minimum viable auth UI in SvelteKit: a login/register page (email+password + Google OAuth button), an account page showing subscription status, and wire up the existing header "Login" button. Also update `frontend/src/lib/api.ts` to call SvelteKit BFF proxy routes for Rust API calls.

## Key Insights

- Current header has a static "Login" button — wire it to navigate to `/login`
- Design system: Nunito/Fredoka fonts, pink/purple palette (`#ff4d8c` primary, `#2d1b36` plum), rounded cards, `shadow-candy` — match existing page style
- `authClient` from Phase 02 exposes `signIn.email()`, `signUp.email()`, `signIn.social()`, `useSession()` (Svelte store)
- Account page shows: email, plan badge (Free/Premium), subscription expiry date, "Upgrade" CTA (links to Whop Checkout), "Manage" button (links to Whop member portal)
- **Whop Checkout:** Redirect user to `https://whop.com/checkout/{WHOP_PLAN_ID}/?custom_data={user.id}` — no server-side session creation needed (simpler than Stripe). After payment, Whop fires webhook to Rust API.
- **Whop member portal:** Link to `https://whop.com/hub/` — Whop's own portal for subscription management (cancel, invoices, payment method). No custom billing portal needed.
- **Webhook is the source of truth:** Rust webhook reads `custom_data` (= our `user.id`) embedded in the checkout URL. No email lookup, no `whop_user_id`. Callback only redirects — no DB writes in SvelteKit.
- **BFF proxy:** JWT never reaches browser. SvelteKit `/api/proxy/extract` fetches JWT server-side and forwards to Rust with `Authorization: Bearer`. Browser calls only SvelteKit.

## Requirements

### Functional
- `/login` page: email+password form + "Continue with Google" button
- `/login` page: toggle between sign-in and register modes
- `/account` page: protected route (redirect to `/login` if not authenticated)
- `/account` page: show email, subscription plan, `current_period_end`, "Manage" / "Upgrade" buttons
- Header "Login" button → `/login`; when logged in → avatar/email + link to `/account`
- JWT attached server-side in `/api/proxy/extract` before forwarding to Rust; `api.ts` only calls same-origin SvelteKit proxy
- `GET /api/checkout` server route: redirect to `https://whop.com/checkout/{WHOP_PLAN_ID}/?custom_data={user.id}`
- `GET /api/checkout/callback` server route: receive Whop return, redirect to `/account?checkout=success` (no DB write)
- `POST /api/proxy/extract` server route: BFF proxy — attach JWT server-side, forward to Rust

### Non-functional
- Auth pages match existing design system (no new fonts/colors)
- Forms show loading state + error messages
- Accessible: proper `<label>`, keyboard navigation
- Auth state reactive via Svelte store (`$session`)

## Architecture

```
frontend/src/
  routes/
    (auth)/
      login/
        +page.svelte             ← login + register form
        +page.server.ts          ← redirect if already logged in
      account/
        +page.svelte             ← account info + subscription status
        +page.server.ts          ← protected: redirect to /login if no session
    api/
      checkout/
        +server.ts               ← GET: redirect to Whop checkout URL (with custom_data)
        callback/
          +server.ts             ← GET: handle Whop return → redirect /account?checkout=success
      proxy/
        extract/
          +server.ts             ← POST: BFF proxy — attach JWT server-side, call Rust
  components/
    UserMenu.svelte              ← header user avatar + dropdown
  lib/
    api.ts                       ← UPDATED: calls /api/proxy/* instead of Rust directly
```

## Page Designs

### Login Page (`/login`)

```
┌─────────────────────────────────────┐
│  FetchTube                          │
│                                     │
│  ┌─────────────────────────────┐    │
│  │  Welcome back               │    │
│  │                             │    │
│  │  [Continue with Google]     │    │
│  │  ─────── or ───────────     │    │
│  │  Email: [____________]      │    │
│  │  Password: [__________]     │    │
│  │                             │    │
│  │  [Sign In]  [Register]      │    │
│  │                             │    │
│  │  Error message (if any)     │    │
│  └─────────────────────────────┘    │
└─────────────────────────────────────┘
```

### Account Page (`/account`)

```
┌─────────────────────────────────────┐
│  FetchTube                [Logout]  │
│                                     │
│  ┌── Your Account ─────────────┐    │
│  │  user@example.com           │    │
│  │                             │    │
│  │  Plan: [Free]  ← badge      │    │
│  │  or    [Premium]            │    │
│  │                             │    │
│  │  Renews: Jan 15, 2027       │    │
│  │  (only shown for premium)   │    │
│  │                             │    │
│  │  [Upgrade to Premium →]     │    │
│  │  (hidden if already premium)│    │
│  │                             │    │
│  │  [Manage Subscription]      │    │
│  │  → links to whop.com/hub/   │    │
│  └─────────────────────────────┘    │
└─────────────────────────────────────┘
```

## BFF Proxy Strategy (JWT never touches browser)

Browser calls SvelteKit server routes → SvelteKit fetches JWT server-side and proxies to Rust → JWT stays on server. Eliminates XSS risk.

```
Browser → POST /api/proxy/extract (httpOnly cookie) → SvelteKit server
                                                         ↓ get JWT from Better Auth
                                                         ↓ POST Rust /api/extract (Authorization: Bearer)
                                                         ↓ return result to browser
```

```typescript
// frontend/src/routes/api/proxy/extract/+server.ts  (NEW)
import { auth } from '$lib/server/auth';
import { RUST_API_URL } from '$env/static/private';

export async function POST({ request, locals }) {
  const body = await request.json();

  // Get JWT server-side — never sent to browser
  let authHeaders: Record<string, string> = {};
  if (locals.session) {
    const tokenResp = await auth.api.getToken({ headers: request.headers });
    if (tokenResp?.token) {
      authHeaders['Authorization'] = `Bearer ${tokenResp.token}`;
    }
  }

  const resp = await fetch(`${RUST_API_URL}/api/extract`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...authHeaders },
    body: JSON.stringify(body),
  });
  return new Response(resp.body, { status: resp.status, headers: { 'Content-Type': 'application/json' } });
}
```

```typescript
// frontend/src/lib/api.ts  (updated — calls SvelteKit proxy, not Rust directly)
export async function extract(url: string): Promise<ExtractResult> {
  const res = await fetch('/api/proxy/extract', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    // credentials: 'include' not needed — same-origin by default
    body: JSON.stringify({ url }),
  });
  // ...
}
```

No `apiToken` store, no JWT in `$page.data`, no client-side auth headers.

## Whop Checkout Integration

```typescript
// frontend/src/routes/api/checkout/+server.ts
import { redirect } from '@sveltejs/kit';

const WHOP_PLAN_ID = process.env.WHOP_PLAN_ID!;

export async function GET({ locals }) {
  if (!locals.user) throw error(401);

  // Embed our user.id as custom_data — Whop carries it in the webhook payload
  // This is the join key: Rust webhook reads custom_data to identify which user paid
  const checkoutUrl = `https://whop.com/checkout/${WHOP_PLAN_ID}/?custom_data=${locals.user.id}`;
  throw redirect(302, checkoutUrl);
}
```

```typescript
// frontend/src/routes/api/checkout/callback/+server.ts
import { redirect } from '@sveltejs/kit';

export async function GET({ locals }) {
  if (!locals.user) throw redirect(302, '/login');

  // Rust webhook reads custom_data (user.id) from Whop payload to link subscription.
  // No DB write here — callback only redirects. Subscription activates via webhook.
  throw redirect(302, '/account?checkout=success');
}
```

## Header Update

```svelte
<!-- frontend/src/routes/+page.svelte or +layout.svelte header section -->
<!-- Replace static "Login" button with: -->
{#if $session?.user}
  <UserMenu user={$session.user} />
{:else}
  <a href="/login" class="flex h-10 px-6 ...">Login</a>
{/if}
```

## Related Code Files

### Files to create
- `frontend/src/routes/(auth)/login/+page.svelte`
- `frontend/src/routes/(auth)/login/+page.server.ts`
- `frontend/src/routes/(auth)/account/+page.svelte`
- `frontend/src/routes/(auth)/account/+page.server.ts`
- `frontend/src/routes/api/checkout/+server.ts` — redirect with `custom_data`
- `frontend/src/routes/api/checkout/callback/+server.ts` — redirect only, no DB write
- `frontend/src/routes/api/proxy/extract/+server.ts` — BFF proxy to Rust
- `frontend/src/components/UserMenu.svelte`

### Files to modify
- `frontend/src/routes/+page.svelte` — wire header "Login" button + `UserMenu`
- `frontend/src/lib/api.ts` — call `/api/proxy/extract` instead of Rust directly
- `docker-compose.server.yml` (frontend service) — add `WHOP_PLAN_ID`, `RUST_API_URL` env vars

## Implementation Steps

1. **Create `frontend/src/routes/api/proxy/extract/+server.ts`** — BFF proxy:
   - Reads JWT server-side from Better Auth (`auth.api.getToken()`)
   - Forwards request to Rust with `Authorization: Bearer <jwt>`
   - JWT never reaches browser

2. **Update `frontend/src/lib/api.ts`** — change target from `VITE_API_URL/api/extract` → `/api/proxy/extract` (same-origin call, no auth headers needed client-side)

3. **Create login page** `(auth)/login/+page.svelte`:
   - Email + password fields with loading state
   - `authClient.signIn.email()` / `authClient.signUp.email()` handlers
   - Google button: `authClient.signIn.social({ provider: 'google' })`
   - Error display
   - Redirect to `/` on success (or `?redirectTo` param)

4. **Create `(auth)/login/+page.server.ts`**:
   - If `locals.session` exists, redirect to `/account`

5. **Create account page** `(auth)/account/+page.svelte`:
   - Protected: server load redirects to `/login` if no session
   - Load subscription data from `subscriptions` table in server load
   - Show plan badge, expiry date
   - "Upgrade" button → navigates to `/api/checkout` → redirects to Whop
   - "Manage Subscription" link → `https://whop.com/hub/`
   - "Logout" button → `authClient.signOut()`

6. **Create `(auth)/account/+page.server.ts`**:
   - Check `locals.session`, redirect to `/login` if null
   - Query `subscriptions` table for user's subscription
   - Return `{ user, subscription }` to page

7. **Create `UserMenu.svelte`** component — avatar + email + dropdown with Account/Logout links

8. **Update header in `+page.svelte`** — replace static Login button with session-aware component

9. **Create `frontend/src/routes/api/checkout/+server.ts`** — redirect to `https://whop.com/checkout/{WHOP_PLAN_ID}/?custom_data={user.id}`

10. **Create `frontend/src/routes/api/checkout/callback/+server.ts`** — redirect to `/account?checkout=success` (no DB write)

11. **Add env vars** to frontend docker-compose service: `WHOP_PLAN_ID`, `RUST_API_URL` (private, used by BFF proxy)

## Todo

- [x] Create `frontend/src/routes/api/proxy/extract/+server.ts` (BFF proxy)
- [x] Update `frontend/src/lib/api.ts` to call `/api/proxy/extract` instead of Rust directly
- [x] Create `(auth)/login/+page.svelte`
- [x] Create `(auth)/login/+page.server.ts`
- [x] Create `(auth)/account/+page.svelte`
- [x] Create `(auth)/account/+page.server.ts`
- [x] Create `UserMenu.svelte`
- [x] Update header in `+page.svelte`
- [x] Create `frontend/src/routes/api/checkout/+server.ts`
- [x] Create `frontend/src/routes/api/checkout/callback/+server.ts`
- [x] Add `WHOP_PLAN_ID`, `RUST_API_URL` env vars to frontend docker-compose service
- [ ] Test login flow (email + Google) in running environment with real Better Auth tables
- [ ] Test account page shows correct plan with seeded subscription data
- [ ] Test Whop Checkout redirect against real `WHOP_PLAN_ID`
- [ ] Test callback redirects to `/account?checkout=success` (no DB write)
- [ ] Verify JWT attached in Rust API calls (check Rust logs for `UserTier`)

## Success Criteria

- Unauthenticated user: "Login" button visible in header, `/account` redirects to `/login`
- After login: `UserMenu` shows email in header, `/account` loads correctly
- Account page shows correct plan (Free/Premium) from DB
- "Upgrade" button redirects to `https://whop.com/checkout/{WHOP_PLAN_ID}/?custom_data={user.id}`
- "Manage Subscription" links to `https://whop.com/hub/`
- Checkout callback redirects to `/account?checkout=success` (no DB write)
- `/api/proxy/extract` calls Rust with `Authorization: Bearer <jwt>` (verify in Rust logs)
- Rust logs show `UserTier::Free` or `UserTier::Premium` (not Anonymous) for logged-in users

## Risk Assessment

- **Webhook latency on account page:** After callback redirects to `/account?checkout=success`, webhook may not have arrived yet → plan still shows "Free". Show hint: "Subscription may take a few seconds to activate. Refresh if needed."
- **`custom_data` missing:** If user finds a raw Whop checkout link (without `custom_data`), webhook cannot link to user. Mitigate: only expose checkout via `/api/checkout` server route (never expose raw Whop URL in UI).
- **`(auth)` route group:** SvelteKit route groups `(auth)` don't affect URL — `/login` and `/account` paths remain clean.

## Security Considerations

- Account page server load validates session server-side (not just client-side check)
- `WHOP_PLAN_ID` is public (it's in the checkout URL) — safe in env var but not secret
- Checkout callback validates session before redirecting (no DB write in SvelteKit)
- Subscription linking handled server-side by Rust webhook (not client-exposed)
- Logout: `authClient.signOut()` clears session cookie + invalidates server session

## Next Steps

(Future scope — not in this plan)
- Premium feature gates (download limits, quality caps) — add checks using `UserTier` in Rust handlers
- SePay / VietQR payment option (add when VN user traction validated)
