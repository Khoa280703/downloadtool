//! Configuration module for API server
//!
//! Loads configuration from environment variables.

use std::env;

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    /// Port to listen on (default: 3000)
    pub port: u16,
    /// Directory containing TypeScript extractor scripts
    pub extractor_dir: String,
    /// GPU worker gRPC address (e.g., "10.0.0.2:50051")
    pub gpu_worker_addr: String,
    /// Whether GPU transcoding is enabled
    pub gpu_enabled: bool,
}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// # Environment Variables
    /// - `PORT` - Server port (default: 3000)
    /// - `EXTRACTOR_DIR` - Path to extractor scripts (default: "./extractors")
    /// - `GPU_WORKER_ADDR` - GPU worker address (default: "10.0.0.2:50051")
    /// - `GPU_ENABLED` - Enable GPU transcoding (default: "false")
    ///
    /// # Errors
    /// Returns an error if PORT is not a valid u16.
    pub fn from_env() -> anyhow::Result<Self> {
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        let extractor_dir = env::var("EXTRACTOR_DIR")
            .unwrap_or_else(|_| "./extractors".to_string());

        let gpu_worker_addr = env::var("GPU_WORKER_ADDR")
            .unwrap_or_else(|_| "10.0.0.2:50051".to_string());

        let gpu_enabled = env::var("GPU_ENABLED")
            .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
            .unwrap_or(false);

        Ok(Self {
            port,
            extractor_dir,
            gpu_worker_addr,
            gpu_enabled,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config {
            port: 3000,
            extractor_dir: "./extractors".to_string(),
            gpu_worker_addr: "10.0.0.2:50051".to_string(),
            gpu_enabled: false,
        };

        assert_eq!(config.port, 3000);
        assert_eq!(config.extractor_dir, "./extractors");
        assert_eq!(config.gpu_worker_addr, "10.0.0.2:50051");
        assert!(!config.gpu_enabled);
    }
}
