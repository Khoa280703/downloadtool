---
title: "Phase 03 — Rust JWT Middleware"
priority: P1
status: pending
effort: 2h
---

# Phase 03 — Rust JWT Middleware (Verify JWT, Inject UserTier)

**Context:** `plans/reports/brainstorm-260228-1159-user-auth-subscription-system.md`
**Plan overview:** `plan.md`
**Depends on:** Phase 01 (DB pool in AppState), Phase 02 (JWT secret + payload contract defined)

## Overview

Add Axum middleware that reads the `Authorization: Bearer <jwt>` header on every request, verifies the HS256 signature using `BETTER_AUTH_SECRET`, extracts `tier` from claims, and injects a `UserTier` enum into request extensions. Handlers receive `UserTier` from extensions — no DB call needed for most routes.

**Về staleness:** `definePayload` là async và query DB khi JWT được issue → tier trong JWT là fresh tại thời điểm issue. JWT sống 15min → tối đa 15min stale sau khi subscription thay đổi. DB lookup thêm ở `/api/extract` là optional safety net, không bắt buộc — quyết định khi implement phase 04.

## Key Insights

- `jsonwebtoken` crate: `decode::<Claims>(&token, &DecodingKey::from_secret(secret), &validation)` — ~0.1ms
- `UserTier` is an Axum extension: handlers access it via `Extension<UserTier>` extractor
- Missing/invalid JWT → `UserTier::Anonymous` (not an error; anonymous users still work)
- Expired JWT → `UserTier::Anonymous` (client must refresh via SvelteKit before calling API)
- Middleware must NOT block requests — always calls `next.run(request)`
- `/api/extract` handler does its own DB lookup using `user_id` from JWT claims when `UserTier::Premium` for authoritative check

## Requirements

### Functional
- Parse `Authorization: Bearer <token>` header (or cookie fallback — see risk section)
- Verify HS256 signature with `BETTER_AUTH_SECRET`
- Verify expiry (`exp` claim)
- Map `tier` claim → `UserTier` enum
- Inject `UserTier` into request extensions for all routes
- `/api/extract` handler: DB lookup subscription là **optional** — JWT tier đã fresh từ `definePayload`. Implement nếu cần strict accuracy.

### Non-functional
- Middleware adds <1ms overhead per request
- No panic on malformed/missing tokens — log at `debug!` level and fall back to `Anonymous`
- `BETTER_AUTH_SECRET` loaded once at startup, stored in `AppState`

## Architecture

```
Request
  ↓
auth_middleware (tower::middleware::from_fn_with_state)
  ├── parse Authorization header
  ├── verify JWT (jsonwebtoken::decode)
  ├── map tier string → UserTier enum
  ├── insert UserTier into request.extensions_mut()
  └── call next.run(request)
         ↓
    Handler receives Extension<UserTier>
```

## UserTier Enum

```rust
// crates/api/src/auth/user_tier.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserTier {
    Anonymous,   // No JWT or invalid JWT
    Free,        // Valid JWT, tier = "free"
    Premium,     // Valid JWT, tier = "premium"
}

impl Default for UserTier {
    fn default() -> Self { UserTier::Anonymous }
}
```

## JWT Claims Struct

```rust
// crates/api/src/auth/jwt_claims.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JwtClaims {
    pub sub: String,   // user_id (Better Auth user.id)
    pub tier: String,  // "free" | "premium"
    pub exp: u64,
    pub iat: u64,
}
```

## Middleware Signature

```rust
// crates/api/src/auth/jwt_middleware.rs
pub async fn jwt_auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let tier = extract_user_tier(&request, &state.jwt_secret);
    request.extensions_mut().insert(tier);
    next.run(request).await
}

fn extract_user_tier(request: &Request, secret: &str) -> UserTier {
    let token = extract_bearer_token(request)?; // returns Option<&str>
    let claims = decode_jwt(token, secret).ok()?; // returns Option<JwtClaims>
    match claims.tier.as_str() {
        "premium" => UserTier::Premium,
        "free" => UserTier::Free,
        _ => UserTier::Anonymous,
    }
}
```

## AppState Update

```rust
// crates/api/src/main.rs
#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub jwt_secret: String,
    // existing extractor state...
}
```

## Handler Usage Pattern

```rust
// Example: how handlers receive UserTier
pub async fn extract_handler(
    State(state): State<AppState>,
    Extension(user_tier): Extension<UserTier>,
    // ... other extractors
) -> impl IntoResponse {
    // UserTier available here — no DB call needed for most logic
    // For premium accuracy on /api/extract:
    if user_tier == UserTier::Premium {
        // Optionally verify live subscription from DB
    }
    // ... handler logic
}
```

## Related Code Files

### Files to create
- `crates/api/src/auth/mod.rs` — module exports
- `crates/api/src/auth/user_tier.rs` — `UserTier` enum
- `crates/api/src/auth/jwt_claims.rs` — `JwtClaims` struct
- `crates/api/src/auth/jwt_middleware.rs` — middleware function

### Files to modify
- `Cargo.toml` (workspace) — add `jsonwebtoken = "9"`
- `crates/api/Cargo.toml` — add `jsonwebtoken`
- `crates/api/src/main.rs` — add `auth` module, update `AppState`, register middleware
- `crates/api/src/config.rs` — add `jwt_secret: String` field (reads `BETTER_AUTH_SECRET`)
- `crates/api/src/routes/mod.rs` — export auth module
- `docker-compose.server.yml` (api service) — add `BETTER_AUTH_SECRET` env var

## Implementation Steps

1. **Add `jsonwebtoken = "9"` to workspace `Cargo.toml`** and `crates/api/Cargo.toml`

2. **Add `BETTER_AUTH_SECRET` to `config.rs`**
   ```rust
   pub jwt_secret: String,
   // In from_env():
   jwt_secret: env::var("BETTER_AUTH_SECRET")
       .map_err(|_| anyhow::anyhow!("BETTER_AUTH_SECRET env var is required"))?,
   ```

3. **Create `crates/api/src/auth/` directory** with:
   - `mod.rs`: re-exports
   - `user_tier.rs`: `UserTier` enum with `Default`
   - `jwt_claims.rs`: `JwtClaims` with serde `Deserialize`
   - `jwt_middleware.rs`: middleware + `extract_bearer_token()` + `decode_jwt()`

4. **Implement `jwt_middleware.rs`**:
   - `extract_bearer_token`: parse `Authorization` header, strip `"Bearer "` prefix
   - `decode_jwt`: call `jsonwebtoken::decode::<JwtClaims>()` with `HS256` algorithm validation
   - Log decode failure at `tracing::debug!` (not `warn` — anonymous users are normal)
   - Return `UserTier::Anonymous` on any error

5. **Update `AppState` in `main.rs`**:
   - Add `db_pool: sqlx::PgPool`
   - Add `jwt_secret: String`

6. **Register middleware in router** in `main.rs`:
   ```rust
   let app = Router::new()
       // ... existing routes ...
       .layer(middleware::from_fn_with_state(
           app_state.clone(),
           auth::jwt_middleware::jwt_auth_middleware,
       ))
       .with_state(app_state);
   ```
   Note: JWT middleware applies to ALL routes. Rate limiter stays on `/api/extract` only.

7. **Update `extract_handler`** in `crates/api/src/routes/extract.rs`:
   - Add `Extension(user_tier): Extension<UserTier>` parameter
   - Add optional live DB check for premium tier (via `state.db_pool`)

8. **Add `BETTER_AUTH_SECRET` env var** to `api` service in `docker-compose.server.yml`

9. **Unit tests** in `jwt_middleware.rs`:
   - Test valid JWT → correct UserTier
   - Test expired JWT → Anonymous
   - Test missing header → Anonymous
   - Test invalid signature → Anonymous

## Todo

- [ ] Add `jsonwebtoken` to workspace + api Cargo.toml
- [ ] Add `jwt_secret` to `config.rs`
- [ ] Create `crates/api/src/auth/mod.rs`
- [ ] Create `crates/api/src/auth/user_tier.rs`
- [ ] Create `crates/api/src/auth/jwt_claims.rs`
- [ ] Create `crates/api/src/auth/jwt_middleware.rs`
- [ ] Update `AppState` in `main.rs`
- [ ] Register JWT middleware in router
- [ ] Update `extract_handler` to accept `Extension<UserTier>`
- [ ] Add `BETTER_AUTH_SECRET` to docker-compose api service
- [ ] Write unit tests for middleware
- [ ] `cargo build` — verify no compile errors

## Success Criteria

- `cargo test -p api` passes all JWT middleware tests
- Request with valid JWT → correct `UserTier` in handler
- Request with no/invalid JWT → `UserTier::Anonymous` (no 401 returned)
- `cargo clippy -p api` clean

## Risk Assessment

- **JWT secret sync:** `BETTER_AUTH_SECRET` in SvelteKit and `BETTER_AUTH_SECRET` in Rust MUST be the same string. If they differ, all JWT verifications fail silently (fall back to Anonymous). Document in deployment guide.
- **Token delivery:** Frontend must attach JWT in `Authorization: Bearer` header for every Rust API call. Currently `frontend/src/lib/api.ts` calls Rust directly without auth header — needs update in Phase 05.
- **`jsonwebtoken` v9 API:** Uses `DecodingKey::from_secret(secret.as_bytes())`. Confirm algorithm is `HS256` (matches Better Auth JWT plugin default).
- **Clock skew:** `exp` validation uses system clock. If Rust container clock drifts from SvelteKit container, tokens may appear expired. Use `leeway` in validation: `validation.leeway = 10;` (10 seconds).

## Security Considerations

- JWT secret loaded from env var, not hardcoded
- Verification uses constant-time comparison (handled by `jsonwebtoken` internally)
- Expired tokens always rejected (no `exp` validation bypass)
- Log decode failure at `debug!` not `error!` — anonymous requests are normal, not errors

## Next Steps

→ Phase 04: Stripe webhook (shares `AppState.db_pool`)
→ Phase 05: Frontend must attach JWT to API calls (update `frontend/src/lib/api.ts`)
