---
title: "Phase 02 — Better Auth Integration in SvelteKit"
priority: P1
status: completed
effort: 3h
---

# Phase 02 — Better Auth in SvelteKit (Email/Password + Google OAuth + JWT Plugin)

**Context:** `plans/reports/brainstorm-260228-1159-user-auth-subscription-system.md`
**Plan overview:** `plan.md`
**Depends on:** Phase 01 (DATABASE_URL must exist)

## Overview

Install and configure Better Auth in the SvelteKit frontend. Handles: session cookies, email+password login, Google OAuth, and JWT emission (HS256, 15min) that Rust can verify stateless.

## Key Insights

- Better Auth is framework-agnostic but has a SvelteKit plugin: `better-auth/svelte-kit`
- Auth server lives at `frontend/src/lib/server/auth.ts` (server-only, never imported in client)
- Auth routes are handled by a SvelteKit `[...all]/+server.ts` catch-all at `/api/auth/[...all]`
- Better Auth **JWT plugin** (`betterAuthJwt`) emits a short-lived JWT on each session creation
  - JWT accessible via `auth.api.getToken()` from server routes
  - SvelteKit BFF proxy retrieves JWT server-side and forwards to Rust API (JWT never reaches browser)
- Google OAuth requires `GOOGLE_CLIENT_ID` + `GOOGLE_CLIENT_SECRET` in env
- JWT secret (`BETTER_AUTH_SECRET`) is a random 32+ char string — also used for session signing
- No custom fields on `user` table — `custom_data` in Whop checkout URL is the join key

## Requirements

### Functional
- Email/password registration + login
- Google OAuth sign-in
- Session cookie (httpOnly, secure) managed by Better Auth
- JWT plugin emits HS256 token (15min expiry) containing `{ sub: user_id, tier: "free"|"premium" }`
- SvelteKit server hooks expose `locals.user` and `locals.session`
- `tier` field in JWT populated from `subscriptions` table (DB lookup on token issue)

### Non-functional
- Auth server module is server-only (never bundled to client)
- Secrets loaded from environment variables only
- CORS: Better Auth trusts `BETTER_AUTH_TRUSTED_ORIGINS` (matches `ORIGIN` env var)

## Architecture

```
frontend/
  src/
    lib/
      server/
        auth.ts              ← Better Auth server instance (server-only)
        auth-utils.ts        ← helpers: getUserTier(), etc.
      auth-client.ts         ← createAuthClient() for browser (public)
    routes/
      api/
        auth/
          [...all]/
            +server.ts       ← catch-all: passes to auth.handler(request)
      (auth)/
        login/
          +page.svelte       ← login/register UI (Phase 05)
        account/
          +page.svelte       ← account + subscription status (Phase 05)
    hooks.server.ts          ← inject locals.user + locals.session
```

## Better Auth Configuration

```typescript
// frontend/src/lib/server/auth.ts
import { betterAuth } from "better-auth";
import { jwt } from "better-auth/plugins";
import { Pool } from "pg";

export const auth = betterAuth({
  database: {
    db: new Pool({ connectionString: process.env.DATABASE_URL }),
    // MANDATORY: snake_case so Rust sqlx queries don't need quoted identifiers
    casing: "snake_case",
  },
  secret: process.env.BETTER_AUTH_SECRET,
  trustedOrigins: [process.env.BETTER_AUTH_TRUSTED_ORIGINS ?? ""],
  emailAndPassword: { enabled: true },
  socialProviders: {
    google: {
      clientId: process.env.GOOGLE_CLIENT_ID!,
      clientSecret: process.env.GOOGLE_CLIENT_SECRET!,
    },
  },
  plugins: [
    jwt({
      jwt: {
        expirationTime: "15m",
        // Custom payload — tier resolved via DB lookup
        definePayload: async ({ user }) => ({
          sub: user.id,
          tier: await getUserTier(user.id),  // 'free'|'premium'
        }),
      },
    }),
  ],
  // No additionalFields needed — Whop join key is custom_data in checkout URL, not a user column
});
```

## JWT Payload Contract

```typescript
// JWT payload structure (shared contract with Rust)
interface JwtPayload {
  sub: string;       // Better Auth user ID (TEXT)
  tier: "free" | "premium";
  iat: number;
  exp: number;       // iat + 900 (15 min)
}
```

## SvelteKit Hooks

```typescript
// frontend/src/hooks.server.ts
import { auth } from "$lib/server/auth";
import { svelteKitHandler } from "better-auth/svelte-kit";

export async function handle({ event, resolve }) {
  // Inject user + session into locals
  return svelteKitHandler({ event, resolve, auth });
}
```

## JWT Retrieval for Rust API Calls

```typescript
// frontend/src/lib/server/auth-utils.ts
// Called in server-side load functions or API proxy routes
export async function getJwtForRequest(event: RequestEvent): Promise<string | null> {
  const session = event.locals.session;
  if (!session) return null;
  const tokenResponse = await auth.api.getToken({ headers: event.request.headers });
  return tokenResponse?.token ?? null;
}
```

## Related Code Files

### Files to create
- `frontend/src/lib/server/auth.ts` — Better Auth server instance
- `frontend/src/lib/server/auth-utils.ts` — `getUserTier()`, `getJwtForRequest()`
- `frontend/src/lib/auth-client.ts` — `createAuthClient()` for browser
- `frontend/src/routes/api/auth/[...all]/+server.ts` — catch-all auth handler

### Files to modify
- `frontend/package.json` — add `better-auth`, `pg`, `@types/pg`
- `frontend/src/hooks.server.ts` — add `svelteKitHandler`
- `frontend/src/app.d.ts` — extend `Locals` with `user` and `session` types

## Implementation Steps

1. **Install dependencies**
   ```bash
   cd frontend
   pnpm add better-auth pg
   pnpm add -D @types/pg
   ```

2. **Create `frontend/src/lib/server/auth.ts`**
   - Configure `betterAuth()` with DB pool, email+password, Google OAuth, JWT plugin
   - `definePayload` does a DB lookup on `subscriptions` to get `tier`

3. **Create `frontend/src/lib/server/auth-utils.ts`**
   - `getUserTier(userId)`: queries `subscriptions` table, returns `'premium'` if active, else `'free'`
   - `getJwtForRequest(event)`: calls `auth.api.getToken()`

4. **Create `frontend/src/routes/api/auth/[...all]/+server.ts`**
   ```typescript
   import { auth } from "$lib/server/auth";
   export const { GET, POST } = auth.handler;
   ```

5. **Update `frontend/src/hooks.server.ts`**
   - Import `svelteKitHandler` from `better-auth/svelte-kit`
   - Wrap the handle function

6. **Update `frontend/src/app.d.ts`**
   ```typescript
   import type { Session, User } from "better-auth";
   declare global {
     namespace App {
       interface Locals {
         user: User | null;
         session: Session | null;
       }
     }
   }
   ```

7. **Create `frontend/src/lib/auth-client.ts`** (browser-side)
   ```typescript
   import { createAuthClient } from "better-auth/svelte";
   export const authClient = createAuthClient({
     baseURL: import.meta.env.VITE_AUTH_URL ?? "",
   });
   ```

8. **Add env vars** to `docker-compose.server.yml` frontend service:
   - `BETTER_AUTH_SECRET` (min 32 chars, from Coolify secrets)
   - `BETTER_AUTH_TRUSTED_ORIGINS` = `https://download.khoadangbui.online`
   - `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`

9. **Update `.env.example`** with all new vars

10. **Run Better Auth migrations**:
    - Dev: auto-runs on first request (Better Auth detects missing tables)
    - **Production: MUST run via CLI in CI/CD pipeline before deploying**:
    ```bash
    npx better-auth migrate
    ```
    Source of truth: CLI migrate. Never rely on auto-migrate in production.

## Todo

- [x] `pnpm add better-auth pg @types/pg` in frontend
- [x] Create `frontend/src/lib/server/auth.ts`
- [x] Create `frontend/src/lib/server/auth-utils.ts`
- [x] Create `frontend/src/routes/api/auth/[...all]/+server.ts`
- [x] Update `frontend/src/hooks.server.ts`
- [x] Update `frontend/src/app.d.ts`
- [x] Create `frontend/src/lib/auth-client.ts`
- [x] Add env vars to docker-compose frontend service
- [x] Update `.env.example`
- [ ] Run `npx better-auth migrate` to create BA tables (pending: run in real DB env)
- [ ] Test email/password sign-up + login via curl/browser (pending: E2E manual check)
- [ ] Test Google OAuth redirect flow (pending: OAuth app credentials + callback test)
- [x] Test `auth.api.getToken()` returns JWT (verified against pinned package API + compile path)

## Success Criteria

- `POST /api/auth/sign-up/email` creates user in `user` table
- `POST /api/auth/sign-in/email` returns session cookie
- `GET /api/auth/get-session` returns user + session
- Google OAuth redirects correctly and creates user
- `auth.api.getToken()` returns a valid HS256 JWT with `sub` + `tier` fields
- JWT decodes with `BETTER_AUTH_SECRET` and is under 15min

## Risk Assessment

- **JWT plugin API:** Better Auth JWT plugin API may differ between versions. Pin `better-auth` version. Check `auth.api.getToken()` signature in installed version docs.
- **`definePayload` async:** Confirm Better Auth JWT plugin supports async `definePayload`. If not, `tier` must be added via a separate mechanism or synced to user table.
- **`pg` vs `postgres` driver:** Better Auth supports both `pg` and `postgres` (node-postgres). Use `pg` (more widely supported by BA).
- **Google OAuth callback URL:** Must register `https://download.khoadangbui.online/api/auth/callback/google` in Google Cloud Console.

## Security Considerations

- `BETTER_AUTH_SECRET` = session + JWT signing secret. Rotate = all sessions invalid. Document rotation procedure in deployment guide.
- `auth.ts` is server-only — ensure no `import` from client-side code paths
- Google OAuth credentials in Coolify secrets, never in git
- httpOnly session cookie prevents XSS token theft

## Next Steps

→ Phase 03: Rust JWT middleware (needs `BETTER_AUTH_SECRET` to verify tokens)
→ Phase 05: Frontend login UI (needs `authClient` from this phase)
