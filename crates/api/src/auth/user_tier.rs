#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum UserTier {
    /// No JWT or invalid JWT
    #[default]
    Anonymous,
    /// Valid JWT, tier = "free"
    Free,
    /// Valid JWT, tier = "premium"
    Premium,
}
