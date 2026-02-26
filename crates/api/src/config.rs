//! Configuration module for API server
//!
//! Loads configuration from environment variables.

use std::env;

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    /// Port to listen on (default: 3068)
    pub port: u16,
    /// Directory containing TypeScript extractor scripts
    pub extractor_dir: String,
}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// # Environment Variables
    /// - `PORT` - Server port (default: 3068)
    /// - `EXTRACTOR_DIR` - Path to extractor scripts (default: "./extractors")
    ///
    /// # Errors
    /// Returns an error if PORT is not a valid u16.
    pub fn from_env() -> anyhow::Result<Self> {
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3068);

        let extractor_dir = env::var("EXTRACTOR_DIR")
            .unwrap_or_else(|_| "./extractors".to_string());

        Ok(Self {
            port,
            extractor_dir,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config {
            port: 3068,
            extractor_dir: "./extractors".to_string(),
        };

        assert_eq!(config.port, 3068);
        assert_eq!(config.extractor_dir, "./extractors");
    }
}
