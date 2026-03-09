use super::user_tier::UserTier;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AuthenticatedUser {
    pub user_id: Option<String>,
    pub tier: UserTier,
}

impl AuthenticatedUser {
    pub fn is_authenticated(&self) -> bool {
        self.user_id
            .as_deref()
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
    }
}
