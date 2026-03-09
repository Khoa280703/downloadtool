use super::user_tier::UserTier;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AuthenticatedUser {
    pub user_id: Option<String>,
    pub tier: UserTier,
}
