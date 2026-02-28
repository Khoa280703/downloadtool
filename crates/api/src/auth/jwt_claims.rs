use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct JwtClaims {
    /// Better Auth user.id
    pub sub: String,
    /// "free" | "premium"
    pub tier: String,
    pub exp: u64,
    pub iat: u64,
}
