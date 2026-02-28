---
title: "Phase 05 — Frontend Auth UI"
priority: P2
status: pending
effort: 4h
---

# Phase 05 — Frontend Auth UI (Login Page + Account Page)

**Context:** `plans/reports/brainstorm-260228-1159-user-auth-subscription-system.md`
**Plan overview:** `plan.md`
**Depends on:** Phase 02 (Better Auth configured), Phase 03 (Rust accepts JWT)

## Overview

Build the minimum viable auth UI in SvelteKit: a login/register page (email+password + Google OAuth button), an account page showing subscription status, and wire up the existing header "Login" button. Also update `frontend/src/lib/api.ts` to attach JWT to Rust API calls.

## Key Insights

- Current header has a static "Login" button — wire it to navigate to `/login`
- Design system: Nunito/Fredoka fonts, pink/purple palette (`#ff4d8c` primary, `#2d1b36` plum), rounded cards, `shadow-candy` — match existing page style
- `authClient` from Phase 02 exposes `signIn.email()`, `signUp.email()`, `signIn.social()`, `useSession()` (Svelte store)
- Account page shows: email, plan badge (Free/Premium), subscription expiry date, "Upgrade" CTA (links to Whop Checkout), "Manage" button (links to Whop member portal)
- **Whop Checkout:** Redirect user to `https://whop.com/checkout/{WHOP_PLAN_ID}/` — no server-side session creation needed (simpler than Stripe). After payment, Whop fires webhook to Rust API.
- **Whop member portal:** Link to `https://whop.com/hub/` — Whop's own portal for subscription management (cancel, invoices, payment method). No custom billing portal needed.
- On return from Whop checkout, SvelteKit reads `whop_user_id` from Whop callback and stores it on the Better Auth user record — this is what links the Whop member to our DB user for webhook lookups.
- JWT must be attached to every Rust API call — update `frontend/src/lib/api.ts` to fetch token from layout server load

## Requirements

### Functional
- `/login` page: email+password form + "Continue with Google" button
- `/login` page: toggle between sign-in and register modes
- `/account` page: protected route (redirect to `/login` if not authenticated)
- `/account` page: show email, subscription plan, `current_period_end`, "Manage" / "Upgrade" buttons
- Header "Login" button → `/login`; when logged in → avatar/email + link to `/account`
- JWT attached as `Authorization: Bearer` header in `api.ts` for all Rust API calls
- `GET /api/checkout` server route: redirect to `https://whop.com/checkout/{WHOP_PLAN_ID}/`
- `GET /api/checkout/callback` server route: receive Whop return, store `whop_user_id` on user

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
        +server.ts               ← GET: redirect to Whop checkout URL
        callback/
          +server.ts             ← GET: handle Whop return, store whop_user_id
      jwt/
        +server.ts               ← GET: return current JWT for client-side API calls
  components/
    UserMenu.svelte              ← header user avatar + dropdown
  lib/
    api.ts                       ← UPDATED: attach JWT to Rust API calls
    stores/
      auth.ts                    ← Svelte store wrapping authClient.useSession()
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

## JWT Attachment Strategy

Client-side API calls in `api.ts` currently call Rust directly via `VITE_API_URL`. To attach JWT:

**Strategy: Server-side load passes JWT token to page, stored in Svelte store**

```typescript
// frontend/src/routes/+layout.server.ts
export async function load({ locals }) {
  let jwt: string | null = null;
  if (locals.session) {
    const tokenResp = await auth.api.getToken({ ... });
    jwt = tokenResp?.token ?? null;
  }
  return { jwt };
}

// frontend/src/routes/+layout.svelte
<script>
  let { data } = $props();
  import { apiToken } from '$lib/stores/api-token';
  $effect(() => { apiToken.set(data.jwt); });
</script>
```

```typescript
// frontend/src/lib/api.ts  (updated)
import { get } from 'svelte/store';
import { apiToken } from '$stores/api-token';

function getAuthHeaders(): Record<string, string> {
  const token = get(apiToken);
  return token ? { Authorization: `Bearer ${token}` } : {};
}

export async function extract(url: string): Promise<ExtractResult> {
  const res = await fetch(`${API_URL}/api/extract`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
    body: JSON.stringify({ url }),
  });
  // ...
}
```

## Whop Checkout Integration

```typescript
// frontend/src/routes/api/checkout/+server.ts
import { redirect } from '@sveltejs/kit';

const WHOP_PLAN_ID = process.env.WHOP_PLAN_ID!;

export async function GET({ locals }) {
  if (!locals.user) throw error(401);

  // Redirect to Whop hosted checkout — no server-side session creation needed
  // Whop appends ?user_id=<whop_user_id> on successful payment redirect back
  const checkoutUrl = `https://whop.com/checkout/${WHOP_PLAN_ID}/`;
  throw redirect(302, checkoutUrl);
}
```

```typescript
// frontend/src/routes/api/checkout/callback/+server.ts
import { redirect } from '@sveltejs/kit';
import { auth } from '$lib/server/auth';

export async function GET({ url, locals }) {
  if (!locals.user) throw redirect(302, '/login');

  // Whop passes whop_user_id in the return URL after payment
  const whopUserId = url.searchParams.get('whop_user_id');
  if (whopUserId) {
    // Store whop_user_id on the Better Auth user record
    // This enables Rust webhook handler to link Whop events to our user
    await auth.api.updateUser({
      body: { whopUserId },
      headers: { cookie: locals.session?.sessionToken ?? '' },
    });
  }

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
- `frontend/src/routes/api/checkout/+server.ts`
- `frontend/src/routes/api/checkout/callback/+server.ts`
- `frontend/src/routes/+layout.server.ts` — JWT load
- `frontend/src/components/UserMenu.svelte`
- `frontend/src/stores/api-token.ts`

### Files to modify
- `frontend/src/routes/+page.svelte` — wire header "Login" button + `UserMenu`
- `frontend/src/lib/api.ts` — add auth headers to all Rust API calls
- `docker-compose.server.yml` (frontend service) — add `WHOP_PLAN_ID` env var

## Implementation Steps

1. **Create `frontend/src/stores/api-token.ts`**
   ```typescript
   import { writable } from 'svelte/store';
   export const apiToken = writable<string | null>(null);
   ```

2. **Create `frontend/src/routes/+layout.server.ts`** — load JWT from Better Auth and pass to client

3. **Update `frontend/src/routes/+layout.svelte`** — set `apiToken` store from layout data

4. **Update `frontend/src/lib/api.ts`** — add `getAuthHeaders()` + attach to all fetch calls

5. **Create login page** `(auth)/login/+page.svelte`:
   - Email + password fields with loading state
   - `authClient.signIn.email()` / `authClient.signUp.email()` handlers
   - Google button: `authClient.signIn.social({ provider: 'google' })`
   - Error display
   - Redirect to `/` on success (or `?redirectTo` param)

6. **Create `(auth)/login/+page.server.ts`**:
   - If `locals.session` exists, redirect to `/account`

7. **Create account page** `(auth)/account/+page.svelte`:
   - Protected: server load redirects to `/login` if no session
   - Load subscription data from `subscriptions` table in server load
   - Show plan badge, expiry date
   - "Upgrade" button → navigates to `/api/checkout` → redirects to Whop
   - "Manage Subscription" link → `https://whop.com/hub/`
   - "Logout" button → `authClient.signOut()`

8. **Create `(auth)/account/+page.server.ts`**:
   - Check `locals.session`, redirect to `/login` if null
   - Query `subscriptions` table for user's subscription
   - Return `{ user, subscription }` to page

9. **Create `UserMenu.svelte`** component — avatar + email + dropdown with Account/Logout links

10. **Update header in `+page.svelte`** — replace static Login button with session-aware component

11. **Create `frontend/src/routes/api/checkout/+server.ts`** — redirect to Whop checkout URL

12. **Create `frontend/src/routes/api/checkout/callback/+server.ts`** — handle Whop return, store `whop_user_id`

13. **Add `WHOP_PLAN_ID` env var** to frontend docker-compose service

## Todo

- [ ] Create `frontend/src/stores/api-token.ts`
- [ ] Create `frontend/src/routes/+layout.server.ts`
- [ ] Update `frontend/src/routes/+layout.svelte` to set apiToken
- [ ] Update `frontend/src/lib/api.ts` with auth headers
- [ ] Create `(auth)/login/+page.svelte`
- [ ] Create `(auth)/login/+page.server.ts`
- [ ] Create `(auth)/account/+page.svelte`
- [ ] Create `(auth)/account/+page.server.ts`
- [ ] Create `UserMenu.svelte`
- [ ] Update header in `+page.svelte`
- [ ] Create `frontend/src/routes/api/checkout/+server.ts`
- [ ] Create `frontend/src/routes/api/checkout/callback/+server.ts`
- [ ] Add `WHOP_PLAN_ID` env var to docker-compose
- [ ] Test login flow (email + Google)
- [ ] Test account page shows correct plan
- [ ] Test Whop Checkout redirect
- [ ] Test callback stores `whop_user_id` on user record
- [ ] Verify JWT attached in Rust API calls (check Rust logs for `UserTier`)

## Success Criteria

- Unauthenticated user: "Login" button visible in header, `/account` redirects to `/login`
- After login: `UserMenu` shows email in header, `/account` loads correctly
- Account page shows correct plan (Free/Premium) from DB
- "Upgrade" button redirects to `https://whop.com/checkout/{WHOP_PLAN_ID}/`
- "Manage Subscription" links to `https://whop.com/hub/`
- Checkout callback stores `whop_user_id` on user record
- Rust API receives `Authorization: Bearer <jwt>` header from frontend calls
- Rust logs show `UserTier::Free` or `UserTier::Premium` (not Anonymous) for logged-in users

## Risk Assessment

- **JWT token freshness:** Layout server load fetches JWT on every SSR render. For client-side navigation, the token in the store may be up to 15min stale (acceptable per design).
- **`+layout.server.ts` timing:** If Better Auth session doesn't exist yet during load (race with cookie), JWT will be null. Client falls back to Anonymous — acceptable.
- **Whop callback `whop_user_id`:** Must verify Whop actually appends this to the redirect URL. Check Whop dashboard webhook/redirect settings. If not available via redirect, fallback: query Whop API with user email to get `whop_user_id`.
- **`whopUserId` custom field column name:** Better Auth may store as `whop_user_id` (snake_case) in DB. Verify after BA migration and use consistent name in queries.
- **`(auth)` route group:** SvelteKit route groups `(auth)` don't affect URL — `/login` and `/account` paths remain clean.

## Security Considerations

- Account page server load validates session server-side (not just client-side check)
- `WHOP_PLAN_ID` is public (it's in the checkout URL) — safe in env var but not secret
- Checkout callback validates session before storing `whop_user_id`
- `whopUserId` stored in DB, not exposed in JWT
- Logout: `authClient.signOut()` clears session cookie + invalidates server session

## Next Steps

(Future scope — not in this plan)
- Premium feature gates (download limits, quality caps) — add checks using `UserTier` in Rust handlers
- SePay / VietQR payment option (add when VN user traction validated)
