//! Platform identifiers used across proxy and anti-bot layers.

/// Video platforms supported by the downloader.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    YouTube,
}

impl Platform {
    /// Get the domain for this platform.
    pub fn domain(self) -> &'static str {
        match self {
            Platform::YouTube => "youtube.com",
        }
    }

    /// Parse platform from string.
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "youtube" | "yt" => Some(Platform::YouTube),
            _ => None,
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::YouTube => write!(f, "youtube"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_domain() {
        assert_eq!(Platform::YouTube.domain(), "youtube.com");
    }

    #[test]
    fn test_platform_from_str() {
        assert_eq!(Platform::parse("youtube"), Some(Platform::YouTube));
        assert_eq!(Platform::parse("YT"), Some(Platform::YouTube));
        assert_eq!(Platform::parse("unknown"), None);
    }
}
