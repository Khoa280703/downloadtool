---
title: "Phase 04 — Whop Payment Webhook Endpoint (Rust)"
priority: P1
status: completed
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
- **Constant-time comparison via `mac.verify_slice()`** — NOT `computed != sig_header` (timing attack)
- Relevant events:
  - `membership.went_valid` → upsert subscription row, status=active
  - `membership.went_invalid` → set status=expired
- **`custom_data` is the join key**: SvelteKit embeds `user.id` in the Whop checkout URL as `custom_data`. Webhook payload carries it back → Rust reads `custom_data` and updates `subscriptions` directly. No email lookup, no `whop_user_id` column on `user` table.
- **Idempotency guard**: `whop_updated_at` column — if incoming event timestamp ≤ stored value, ignore (prevents out-of-order replay overwriting newer state)
- Whop provides its own member management portal at `https://whop.com/hub/` — no need to build a custom billing portal

## Requirements

### Functional
- Verify `x-whop-signature` header; return 400 if invalid
- Parse event type from JSON body
- Handle 2 membership lifecycle events
- Upsert `subscriptions` table row using `custom_data` (= our `user.id`) from webhook payload
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
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .map_err(|_| WebhookError::InvalidSecret)?;
    mac.update(payload);
    // True constant-time comparison via verify_slice — prevents timing attacks.
    // sig_header is hex-encoded; decode first.
    let sig_bytes = hex::decode(sig_header).map_err(|_| WebhookError::InvalidSignature)?;
    mac.verify_slice(&sig_bytes).map_err(|_| WebhookError::InvalidSignature)
}
```

## DB Operations

```rust
// Upsert subscription (membership.went_valid)
async fn upsert_subscription(
    pool: &PgPool,
    event: &serde_json::Value,
    membership: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    // custom_data = our user.id, embedded by SvelteKit when generating checkout URL
    let user_id = event["data"]["custom_data"].as_str().unwrap_or_default();
    let whop_membership_id = membership["id"].as_str().unwrap_or_default();
    let renewal_period_end = membership["renewal_period_end"].as_i64().unwrap_or(0);
    // Unix timestamp of this event — used as idempotency guard
    let event_ts = event["created_at"].as_i64().unwrap_or(0);

    if user_id.is_empty() {
        tracing::warn!("Whop webhook missing custom_data (user_id)");
        return Ok(()); // Return 200 so Whop doesn't retry
    }

    // Idempotency guard: ignore event if it's older than the last processed event
    // Prevents out-of-order replay from overwriting newer subscription state
    sqlx::query!(
        r#"
        INSERT INTO subscriptions
            (user_id, plan, status, current_period_end, whop_membership_id, whop_updated_at, updated_at)
        VALUES ($1, 'premium', 'active', to_timestamp($2), $3, to_timestamp($4), NOW())
        ON CONFLICT (user_id) DO UPDATE SET
            plan = 'premium',
            status = 'active',
            current_period_end = EXCLUDED.current_period_end,
            whop_membership_id = EXCLUDED.whop_membership_id,
            whop_updated_at = EXCLUDED.whop_updated_at,
            updated_at = NOW()
        WHERE subscriptions.whop_updated_at IS NULL
           OR subscriptions.whop_updated_at < EXCLUDED.whop_updated_at
        "#,
        user_id, renewal_period_end as f64, whop_membership_id, event_ts as f64
    ).execute(pool).await?;
    Ok(())
}

// Expire subscription (membership.went_invalid)
async fn expire_subscription(
    pool: &PgPool,
    event: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    let user_id = event["data"]["custom_data"].as_str().unwrap_or_default();
    let event_ts = event["created_at"].as_i64().unwrap_or(0);

    if user_id.is_empty() {
        tracing::warn!("Whop webhook missing custom_data (user_id)");
        return Ok(());
    }

    // Only update if this event is newer than last processed
    sqlx::query!(
        r#"
        UPDATE subscriptions
        SET status = 'expired', whop_updated_at = to_timestamp($2), updated_at = NOW()
        WHERE user_id = $1
          AND (whop_updated_at IS NULL OR whop_updated_at < to_timestamp($2))
        "#,
        user_id, event_ts as f64
    ).execute(pool).await?;
    Ok(())
}
```

Note: `ON CONFLICT (user_id)` requires `UNIQUE(user_id)` in `subscriptions`; this is already present in the Phase 01 migration.

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
- `crates/api/migrations/0001_create_subscriptions.sql` — already clean in Phase 01 (no changes needed)
- `docker-compose.server.yml` (api service) — add `WHOP_WEBHOOK_SECRET` env var

## Implementation Steps

1. **Add crypto dependencies** to workspace `Cargo.toml`:
   ```toml
   hmac = "0.12"
   sha2 = "0.10"
   hex = "0.4"
   ```

2. **Add `whop_webhook_secret` to `config.rs`**

3. **Migration `0001_create_subscriptions.sql`** — already clean in Phase 01:
   - Has `UNIQUE(user_id)`, `whop_membership_id`, `whop_updated_at`
   - No stripe columns, no `whop_user_id` on `user` table
   - No changes needed in this phase

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
- [ ] Verify migration `0001_create_subscriptions.sql` has: `UNIQUE(user_id)`, `whop_membership_id`, `whop_updated_at` columns (set up in Phase 01, no changes needed here)
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

- **`custom_data` path in payload:** Verify actual JSON path for `custom_data` in Whop webhook. Check Whop Dashboard "Test webhook" output. If path differs, update field accessor.
- **`created_at` timestamp path:** Verify field name for event timestamp in Whop payload (may be `created_at`, `timestamp`, or similar). Used for idempotency guard.
- **`custom_data` missing:** If user initiated checkout via old link (without custom_data), webhook has empty value → log warning + 200, no DB update. Operator must link manually.
- **Unhandled event volume:** Whop may send various event types. Unhandled events must return 200, not 500 — or Whop will retry.

## Security Considerations

- HMAC verification via `mac.verify_slice()` — true constant-time, no timing attack risk
- `WHOP_WEBHOOK_SECRET` is endpoint-specific
- Route excluded from rate limiter (Whop IPs can trigger limit on bursts)
- Raw body captured as `Bytes` before any parsing — critical for signature validity

## Next Steps

→ Phase 05: Frontend sends user to `https://whop.com/checkout/{plan_id}/?custom_data={user.id}` — `custom_data` flows back via webhook to Rust for direct DB update
