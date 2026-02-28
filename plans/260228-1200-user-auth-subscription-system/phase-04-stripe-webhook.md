---
title: "Phase 04 — Whop Payment Webhook Endpoint (Rust)"
priority: P1
status: pending
effort: 3h
---

# Phase 04 — Whop Payment Webhook Endpoint in Rust

**Context:** `plans/reports/brainstorm-260228-1159-user-auth-subscription-system.md`
**Plan overview:** `plan.md`
**Depends on:** Phase 01 (DB pool + `subscriptions` table)

## Overview

Add `POST /api/webhooks/whop` endpoint to the Rust API. Verifies Whop webhook signature (HMAC-SHA256 via `x-whop-signature` header), processes relevant membership lifecycle events, and updates the `subscriptions` table. No Whop SDK needed — use manual HMAC verification + `serde_json` for event parsing.

**Why Rust, not SvelteKit:** Rust process is more stable and won't miss webhooks during frontend deploys or SvelteKit restarts.

**Note:** Whop is a Merchant of Record (MoR) — handles global tax collection automatically. No company registration required to use Whop; individuals can sign up. 6% transaction fee. Payouts via crypto (USDT) or bank wire.

## Key Insights

- Whop webhook verification: HMAC-SHA256 of raw body using `WHOP_WEBHOOK_SECRET`
- Whop sends `x-whop-signature: <hex_sig>` header
- **Raw body must be captured before JSON deserialization** — body bytes needed for HMAC
- Relevant events:
  - `membership.went_valid` → upsert subscription row, status=active
  - `membership.went_invalid` → set status=expired/cancelled
- `whop_user_id` in event payload links Whop member to `user_id` in our DB
- Whop member ID must be stored when user initiates checkout (Phase 05)
- Whop provides its own member management portal at `https://whop.com/hub/` — no need to build a custom billing portal

## Requirements

### Functional
- Verify `x-whop-signature` header; return 400 if invalid
- Parse event type from JSON body
- Handle 2 membership lifecycle events
- Upsert `subscriptions` table row via `user_id` (looked up from `whop_user_id`)
- Return 200 for unhandled events (Whop retries on non-200)
- Return 200 for all successfully processed events

### Non-functional
- Idempotent: re-processing the same event must not corrupt data
- Timeout: Whop expects response within 30s — handler must be fast
- Raw body captured before parsing (Axum `Bytes` extractor required)

## Architecture

```
POST /api/webhooks/whop
  ↓
whop_webhook_handler (no JWT middleware — exempt)
  ├── read raw body bytes (Bytes extractor)
  ├── verify_whop_signature(raw_body, whop_sig_header, webhook_secret)
  │     ├── compute HMAC-SHA256(raw_body, webhook_secret)
  │     └── constant-time compare with header value
  ├── parse body as serde_json::Value
  ├── match event["action"] string
  │     ├── "membership.went_valid" → upsert_subscription() → status=active
  │     ├── "membership.went_invalid" → expire_subscription() → status=expired
  │     └── _ → return 200 (ignored)
  └── return 200 OK
```

## Whop Event Data Shape

```rust
// Only fields we need — no full SDK types required
#[derive(Deserialize)]
struct WhopEvent {
    action: String,          // "membership.went_valid" | "membership.went_invalid"
    data: WhopEventData,
}

#[derive(Deserialize)]
struct WhopEventData {
    object: serde_json::Value,  // The membership object
}
```

Membership object relevant fields:
```json
{
  "id": "mem_xxxxxxxx",
  "user_id": "user_xxxxxxxx",
  "product_id": "prod_xxxxxxxx",
  "plan_id": "plan_xxxxxxxx",
  "status": "active",
  "renewal_period_end": 1700000000,
  "valid": true
}
```

## Signature Verification

```rust
// crates/api/src/routes/whop_webhook.rs
use hmac::{Hmac, Mac};
use sha2::Sha256;

fn verify_whop_signature(
    payload: &[u8],
    sig_header: &str,
    secret: &str,
) -> Result<(), WebhookError> {
    // Compute HMAC-SHA256(payload, secret)
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .map_err(|_| WebhookError::InvalidSecret)?;
    mac.update(payload);
    let computed = hex::encode(mac.finalize().into_bytes());

    // Constant-time compare
    if computed != sig_header {
        return Err(WebhookError::InvalidSignature);
    }
    Ok(())
}
```

## DB Operations

```rust
// Upsert subscription (membership.went_valid)
async fn upsert_subscription(
    pool: &PgPool,
    membership: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    let whop_user_id = membership["user_id"].as_str()...;
    let whop_membership_id = membership["id"].as_str()...;
    let renewal_period_end = membership["renewal_period_end"].as_i64()...;

    // Lookup user_id from whop_user_id
    let user_id = sqlx::query_scalar!(
        "SELECT id FROM \"user\" WHERE whop_user_id = $1",
        whop_user_id
    ).fetch_optional(pool).await?;

    let Some(user_id) = user_id else {
        tracing::warn!("No user found for whop_user_id {}", whop_user_id);
        return Ok(());  // Return OK so Whop doesn't retry
    };

    // Upsert subscriptions row
    sqlx::query!(
        r#"
        INSERT INTO subscriptions
            (user_id, plan, status, current_period_end, whop_membership_id, updated_at)
        VALUES ($1, 'premium', 'active', to_timestamp($2), $3, NOW())
        ON CONFLICT (user_id) DO UPDATE SET
            plan = 'premium',
            status = 'active',
            current_period_end = EXCLUDED.current_period_end,
            whop_membership_id = EXCLUDED.whop_membership_id,
            updated_at = NOW()
        "#,
        user_id, renewal_period_end as f64, whop_membership_id
    ).execute(pool).await?;
    Ok(())
}

// Expire subscription (membership.went_invalid)
async fn expire_subscription(
    pool: &PgPool,
    membership: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    let whop_user_id = membership["user_id"].as_str()...;

    sqlx::query!(
        r#"
        UPDATE subscriptions
        SET status = 'expired', updated_at = NOW()
        WHERE user_id = (SELECT id FROM "user" WHERE whop_user_id = $1)
        "#,
        whop_user_id
    ).execute(pool).await?;
    Ok(())
}
```

Note: Add `UNIQUE(user_id)` constraint to `subscriptions` table migration to support `ON CONFLICT (user_id)`.

## Whop Product Setup (Manual — Pre-Implementation)

Before implementing, set up in Whop Dashboard:
1. Create a company/product: "FetchTube Premium"
2. Create a plan: recurring monthly
3. Webhook endpoint: `https://api-download.khoadangbui.online/api/webhooks/whop`
   - Events: `membership.went_valid`, `membership.went_invalid`
4. Record `WHOP_WEBHOOK_SECRET` from webhook endpoint settings
5. Record `WHOP_PLAN_ID` for the premium plan (used in Phase 05 checkout URL)

## Related Code Files

### Files to create
- `crates/api/src/routes/whop_webhook.rs` — handler + signature verification + DB ops

### Files to modify
- `Cargo.toml` (workspace) — add `hmac`, `sha2`, `hex` (if not already added)
- `crates/api/Cargo.toml` — add same deps
- `crates/api/src/routes/mod.rs` — export `whop_webhook_handler`
- `crates/api/src/main.rs` — register `POST /api/webhooks/whop` route (EXEMPT from JWT middleware and rate limiter)
- `crates/api/src/config.rs` — add `whop_webhook_secret: String`
- `crates/api/migrations/0001_create_subscriptions.sql` — add `UNIQUE(user_id)` constraint, rename `stripe_*` columns to `whop_*`
- `docker-compose.server.yml` (api service) — add `WHOP_WEBHOOK_SECRET` env var

## Implementation Steps

1. **Add crypto dependencies** to workspace `Cargo.toml`:
   ```toml
   hmac = "0.12"
   sha2 = "0.10"
   hex = "0.4"
   ```

2. **Add `whop_webhook_secret` to `config.rs`**

3. **Update migration** `0001_create_subscriptions.sql`:
   - Add `UNIQUE(user_id)` to support upsert
   - Use `whop_membership_id TEXT` instead of `stripe_subscription_id`
   - Add `whop_user_id` to `user` table (stored in Better Auth user custom field)

4. **Create `crates/api/src/routes/whop_webhook.rs`**:
   - `whop_webhook_handler`: uses `Bytes` extractor (not `Json`) to get raw body
   - `verify_whop_signature()` as described above
   - `handle_whop_event()`: match on `action` field, dispatch to helpers
   - `upsert_subscription()` for `membership.went_valid`
   - `expire_subscription()` for `membership.went_invalid`

5. **Register route in `main.rs`**:
   ```rust
   .route("/api/webhooks/whop", post(routes::whop_webhook_handler))
   // NOTE: This route must be added BEFORE the JWT middleware layer
   // OR excluded from JWT middleware via a nested router
   ```
   Strategy: put webhook route in a separate sub-router without JWT middleware.

6. **Add `WHOP_WEBHOOK_SECRET` to docker-compose api service**

7. **Test with curl**:
   ```bash
   # Compute expected HMAC manually for test payload
   curl -X POST https://api-download.khoadangbui.online/api/webhooks/whop \
     -H "x-whop-signature: <computed_sig>" \
     -H "Content-Type: application/json" \
     -d '{"action":"membership.went_valid","data":{"object":{...}}}'
   ```

## Todo

- [ ] Create Whop product + plan in Whop Dashboard
- [ ] Register webhook endpoint in Whop Dashboard, record `WHOP_WEBHOOK_SECRET`
- [ ] Add `hmac`, `sha2`, `hex` to workspace Cargo.toml
- [ ] Add `whop_webhook_secret` to `config.rs`
- [ ] Update migration: `UNIQUE(user_id)` on subscriptions, `whop_membership_id`, `whop_user_id` on user
- [ ] Create `crates/api/src/routes/whop_webhook.rs`
- [ ] Register route in `main.rs` (exempt from JWT middleware)
- [ ] Add `WHOP_WEBHOOK_SECRET` env var to docker-compose api service
- [ ] Test with curl (valid + invalid signature)
- [ ] Verify subscription row updated in DB after test
- [ ] `cargo build` — verify compile

## Success Criteria

- `POST /api/webhooks/whop` returns 400 on invalid signature
- `POST /api/webhooks/whop` returns 200 on valid event
- `membership.went_valid` event upserts correct row in `subscriptions` with `status='active'`
- `membership.went_invalid` sets `status='expired'`
- Unknown action types return 200 without error
- `cargo test -p api` passes webhook tests (using mock HMAC)

## Risk Assessment

- **`whop_user_id` lookup:** Depends on SvelteKit storing `whop_user_id` in the Better Auth user record when user completes Whop checkout. This handshake must be documented — it happens when user returns from Whop checkout (Phase 05).
- **`user` table custom field:** Better Auth stores `whopUserId` as a custom field. Verify actual column name after BA migration runs (`whop_user_id` snake_case).
- **Unhandled event volume:** Whop may send various event types. Unhandled events must return 200, not 500 — or Whop will retry.

## Security Considerations

- HMAC verification is the sole authentication for this endpoint — no JWT, no session
- `WHOP_WEBHOOK_SECRET` is endpoint-specific
- Route excluded from rate limiter (Whop IPs can trigger limit on bursts)
- Raw body captured as `Bytes` before any parsing — critical for signature validity

## Next Steps

→ Phase 05: Frontend billing UI (Whop Checkout redirect lives in SvelteKit — user redirected to `https://whop.com/checkout/{plan_id}/`, on return SvelteKit stores `whop_user_id`)
