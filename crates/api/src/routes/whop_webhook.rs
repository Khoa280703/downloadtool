use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use hmac::{Hmac, Mac};
use serde_json::Value;
use sha2::Sha256;
use sqlx::PgPool;
use tracing::{debug, error, warn};

use crate::AppState;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug)]
pub enum WebhookError {
    InvalidSignature,
    InvalidPayload(String),
    Database(sqlx::Error),
}

impl IntoResponse for WebhookError {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidSignature => {
                (StatusCode::BAD_REQUEST, "invalid webhook signature").into_response()
            }
            Self::InvalidPayload(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            Self::Database(err) => {
                error!("whop webhook db error: {err}");
                (StatusCode::INTERNAL_SERVER_ERROR, "database error").into_response()
            }
        }
    }
}

pub async fn whop_webhook_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<StatusCode, WebhookError> {
    let signature = headers
        .get("x-whop-signature")
        .and_then(|value| value.to_str().ok())
        .ok_or(WebhookError::InvalidSignature)?;

    verify_whop_signature(&body, signature, &state.whop_webhook_secret)?;

    let event: Value = serde_json::from_slice(&body)
        .map_err(|err| WebhookError::InvalidPayload(format!("invalid json payload: {err}")))?;
    let action = event
        .get("action")
        .and_then(Value::as_str)
        .unwrap_or_default();

    match action {
        "membership.went_valid" => {
            upsert_subscription(&state.db_pool, &event).await?;
        }
        "membership.went_invalid" => {
            expire_subscription(&state.db_pool, &event).await?;
        }
        _ => {
            debug!("ignored whop action: {action}");
        }
    }

    Ok(StatusCode::OK)
}

fn verify_whop_signature(payload: &[u8], sig_header: &str, secret: &str) -> Result<(), WebhookError> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| WebhookError::InvalidSignature)?;
    mac.update(payload);

    let sig_bytes = hex::decode(sig_header).map_err(|_| WebhookError::InvalidSignature)?;
    mac.verify_slice(&sig_bytes)
        .map_err(|_| WebhookError::InvalidSignature)
}

async fn upsert_subscription(pool: &PgPool, event: &Value) -> Result<(), WebhookError> {
    let user_id = extract_custom_data(event);
    if user_id.is_empty() {
        warn!("whop webhook missing custom_data (user_id)");
        return Ok(());
    }

    let membership = event
        .get("data")
        .and_then(|data| data.get("object"))
        .unwrap_or(&Value::Null);
    let whop_membership_id = membership
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let renewal_period_end = extract_i64(membership.get("renewal_period_end")).map(|v| v as f64);
    let event_ts = extract_i64(event.get("created_at")).unwrap_or_else(current_unix_ts) as f64;

    sqlx::query(
        r#"
        INSERT INTO subscriptions
            (user_id, plan, status, current_period_end, whop_membership_id, whop_updated_at, updated_at)
        VALUES (
            $1,
            'premium',
            'active',
            CASE
                WHEN $2::double precision IS NULL THEN NULL
                ELSE to_timestamp($2::double precision)
            END,
            $3,
            to_timestamp($4::double precision),
            NOW()
        )
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
    )
    .bind(user_id)
    .bind(renewal_period_end)
    .bind(whop_membership_id)
    .bind(event_ts)
    .execute(pool)
    .await
    .map_err(WebhookError::Database)?;

    Ok(())
}

async fn expire_subscription(pool: &PgPool, event: &Value) -> Result<(), WebhookError> {
    let user_id = extract_custom_data(event);
    if user_id.is_empty() {
        warn!("whop webhook missing custom_data (user_id)");
        return Ok(());
    }

    let event_ts = extract_i64(event.get("created_at")).unwrap_or_else(current_unix_ts) as f64;
    sqlx::query(
        r#"
        UPDATE subscriptions
        SET status = 'expired', whop_updated_at = to_timestamp($2::double precision), updated_at = NOW()
        WHERE user_id = $1
          AND (whop_updated_at IS NULL OR whop_updated_at < to_timestamp($2::double precision))
        "#,
    )
    .bind(user_id)
    .bind(event_ts)
    .execute(pool)
    .await
    .map_err(WebhookError::Database)?;

    Ok(())
}

fn extract_custom_data(event: &Value) -> &str {
    event
        .get("data")
        .and_then(|data| data.get("custom_data"))
        .and_then(Value::as_str)
        .unwrap_or_default()
}

fn extract_i64(value: Option<&Value>) -> Option<i64> {
    let value = value?;
    if let Some(v) = value.as_i64() {
        return Some(v);
    }
    value.as_str().and_then(|v| v.parse::<i64>().ok())
}

fn current_unix_ts() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or_default()
}
