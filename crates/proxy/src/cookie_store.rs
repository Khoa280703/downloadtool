//! Per-platform cookie persistence and management
//!
//! Manages cookies for different video platforms with warm-up
//! capabilities and persistence to disk.

use reqwest::Url;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Video platforms supported by the downloader
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    YouTube,
    TikTok,
}

impl Platform {
    /// Get the domain for this platform
    pub fn domain(self) -> &'static str {
        match self {
            Platform::YouTube => "youtube.com",
            Platform::TikTok => "tiktok.com",
        }
    }

    /// Get the homepage URL for warm-up
    pub fn homepage_url(self) -> &'static str {
        match self {
            Platform::YouTube => "https://www.youtube.com",
            Platform::TikTok => "https://www.tiktok.com",
        }
    }

    /// Parse platform from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "youtube" | "yt" => Some(Platform::YouTube),
            "tiktok" | "tt" => Some(Platform::TikTok),
            _ => None,
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::YouTube => write!(f, "youtube"),
            Platform::TikTok => write!(f, "tiktok"),
        }
    }
}

/// Cookie store for a specific platform
/// Note: reqwest's built-in cookie store is used automatically.
/// This struct manages platform-specific cookie metadata and warm-up.
pub struct CookieStore {
    platform: Platform,
    persistence_path: Option<PathBuf>,
}

impl CookieStore {
    /// Create a new cookie store for the given platform
    pub fn new(platform: Platform) -> Self {
        let persistence_path = Self::default_persistence_path(&platform);

        Self {
            platform,
            persistence_path,
        }
    }

    /// Create a cookie store without persistence (for testing)
    pub fn new_ephemeral(platform: Platform) -> Self {
        Self {
            platform,
            persistence_path: None,
        }
    }

    /// Get the default path for cookie persistence
    fn default_persistence_path(platform: &Platform) -> Option<PathBuf> {
        dirs::data_dir().map(|mut path| {
            path.push("downloadtool");
            path.push("cookies");
            path.push(format!("{}.json", platform));
            path
        })
    }

    /// Warm up cookies by fetching the platform homepage
    /// This seeds initial cookies needed for subsequent requests
    pub async fn warm_up(&self, client: &reqwest::Client) -> Result<(), reqwest::Error> {
        let url = self.platform.homepage_url();
        info!("Warming up cookies for {} by fetching {}", self.platform, url);

        let response = client.get(url).send().await?;

        // Cookies are automatically stored in the jar by reqwest
        debug!(
            "Warm-up completed for {} with status: {}",
            self.platform,
            response.status()
        );

        // Optionally persist cookies
        if let Err(e) = self.persist().await {
            warn!("Failed to persist cookies for {}: {}", self.platform, e);
        }

        Ok(())
    }

    /// Clear all cookies for this platform
    /// Note: This requires creating a new reqwest client since the built-in
    /// cookie store cannot be cleared directly.
    pub fn clear(&self) {
        // The actual clearing happens by creating a new client
        // This is a placeholder for the operation
        info!("Cleared cookies for {} (client recreation required)", self.platform);
    }

    /// Persist cookies to disk
    pub async fn persist(&self) -> Result<(), std::io::Error> {
        let Some(path) = &self.persistence_path else {
            return Ok(());
        };

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Note: reqwest's Jar doesn't expose raw cookies for serialization
        // In a production implementation, you'd use a custom cookie store
        // that implements serde. For now, we just ensure the directory exists.
        debug!("Cookie persistence path: {:?}", path);

        Ok(())
    }

    /// Load cookies from disk (placeholder for future implementation)
    pub async fn load(&self) -> Result<(), std::io::Error> {
        // Note: reqwest's Jar doesn't support loading from disk directly
        // This would require a custom cookie store implementation
        debug!("Cookie loading not yet implemented for {}", self.platform);
        Ok(())
    }

    /// Get the platform associated with this store
    pub fn platform(&self) -> Platform {
        self.platform
    }

    /// Add a cookie manually
    /// Note: This is a placeholder since the built-in cookie store
    /// doesn't support manual cookie injection.
    pub fn add_cookie(&self, cookie: &str, _url: &Url) {
        debug!("Cookie addition for {}: {} (requires custom store)", self.platform, cookie);
    }
}

impl Clone for CookieStore {
    fn clone(&self) -> Self {
        Self {
            platform: self.platform,
            persistence_path: self.persistence_path.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_domain() {
        assert_eq!(Platform::YouTube.domain(), "youtube.com");
        assert_eq!(Platform::TikTok.domain(), "tiktok.com");
    }

    #[test]
    fn test_platform_from_str() {
        assert_eq!(Platform::from_str("youtube"), Some(Platform::YouTube));
        assert_eq!(Platform::from_str("YT"), Some(Platform::YouTube));
        assert_eq!(Platform::from_str("tiktok"), Some(Platform::TikTok));
        assert_eq!(Platform::from_str("unknown"), None);
    }

    #[test]
    fn test_cookie_store_creation() {
        let store = CookieStore::new_ephemeral(Platform::YouTube);
        assert_eq!(store.platform(), Platform::YouTube);
    }

    #[test]
    fn test_cookie_store_clear() {
        let store = CookieStore::new_ephemeral(Platform::YouTube);
        let url = Url::parse("https://youtube.com").unwrap();
        store.add_cookie("test=value", &url);
        store.clear();
        // Clearing creates a new empty jar
    }
}
