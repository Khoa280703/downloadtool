use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use tracing::debug;

use crate::{auth::jwt_claims::JwtClaims, auth::user_tier::UserTier, AppState};

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
    let Some(token) = extract_bearer_token(request) else {
        return UserTier::Anonymous;
    };

    match decode_jwt(token, secret) {
        Ok(claims) => match claims.tier.as_str() {
            "premium" => UserTier::Premium,
            "free" => UserTier::Free,
            _ => UserTier::Anonymous,
        },
        Err(err) => {
            debug!("JWT decode failed: {err}");
            UserTier::Anonymous
        }
    }
}

fn extract_bearer_token(request: &Request) -> Option<&str> {
    let auth = request
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())?;
    auth.strip_prefix("Bearer ").map(str::trim).filter(|v| !v.is_empty())
}

fn decode_jwt(token: &str, secret: &str) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 10;
    validation.validate_exp = true;

    decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use axum::body::Body;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde::Serialize;

    use super::*;

    #[derive(Debug, Serialize)]
    struct TestClaims {
        sub: String,
        tier: String,
        exp: u64,
        iat: u64,
    }

    fn now_unix() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    fn build_token(tier: &str, exp_offset_secs: i64, secret: &str) -> String {
        let now = now_unix() as i64;
        let exp = (now + exp_offset_secs).max(0) as u64;
        let claims = TestClaims {
            sub: "user_1".to_string(),
            tier: tier.to_string(),
            iat: now as u64,
            exp,
        };
        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .expect("token encode should succeed")
    }

    fn request_with_auth_header(value: &str) -> Request {
        Request::builder()
            .uri("/api/extract")
            .header("Authorization", value)
            .body(Body::empty())
            .expect("request should build")
    }

    #[test]
    fn valid_jwt_maps_to_premium_tier() {
        let secret = "secret123";
        let token = build_token("premium", 300, secret);
        let req = request_with_auth_header(&format!("Bearer {token}"));
        assert_eq!(extract_user_tier(&req, secret), UserTier::Premium);
    }

    #[test]
    fn expired_jwt_maps_to_anonymous() {
        let secret = "secret123";
        let token = build_token("premium", -3600, secret);
        let req = request_with_auth_header(&format!("Bearer {token}"));
        assert_eq!(extract_user_tier(&req, secret), UserTier::Anonymous);
    }

    #[test]
    fn missing_auth_header_maps_to_anonymous() {
        let req = Request::builder()
            .uri("/api/extract")
            .body(Body::empty())
            .expect("request should build");
        assert_eq!(extract_user_tier(&req, "secret123"), UserTier::Anonymous);
    }

    #[test]
    fn invalid_signature_maps_to_anonymous() {
        let token = build_token("free", 300, "secret_a");
        let req = request_with_auth_header(&format!("Bearer {token}"));
        assert_eq!(extract_user_tier(&req, "secret_b"), UserTier::Anonymous);
    }
}
