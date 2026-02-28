#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserTier {
    /// No JWT or invalid JWT
    Anonymous,
    /// Valid JWT, tier = "free"
    Free,
    /// Valid JWT, tier = "premium"
    Premium,
}

impl Default for UserTier {
    fn default() -> Self {
        Self::Anonymous
    }
}
