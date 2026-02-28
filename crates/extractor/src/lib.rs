//! Extractor crate - video extraction via yt-dlp subprocess
//!
//! Primary extraction uses `yt-dlp -J` subprocess which handles PO Token,
//! signature decryption, and throttle bypass automatically.
//! The JS pool (deno_core) is kept for playlist extraction only.
//!
//! # Example
//! ```rust,no_run
//! use extractor::extract;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let info = extract("https://youtube.com/watch?v=...", None).await?;
//!     println!("Title: {}", info.title);
//!     println!("Formats: {}", info.formats.len());
//!     Ok(())
//! }
//! ```

use std::path::Path;
use std::sync::OnceLock;
use tracing::{debug, info};

pub mod hot_reload;
pub mod pool;
pub mod runtime;
pub mod types;
pub mod ytdlp;

pub use hot_reload::{HotReloader, ReloadableBundle};
pub use pool::{ExtractorPool, PoolHandle};
pub use runtime::ExtractorRuntime;
pub use types::{ExtractionError, VideoFormat, VideoInfo};

// Global pool instance for the simple API
static GLOBAL_POOL: OnceLock<PoolHandle> = OnceLock::new();

/// Default bundle path - bundled at compile time
const DEFAULT_BUNDLE: &str = include_str!(concat!(env!("OUT_DIR"), "/extractors_bundle.js"));

/// Initialize the global extractor pool
///
/// This should be called once at application startup.
///
/// # Arguments
/// * `bundle_path` - Optional path to a custom JS bundle file
pub async fn init(bundle_path: Option<&Path>) -> Result<(), ExtractionError> {
    let bundle = if let Some(path) = bundle_path {
        tokio::fs::read_to_string(path)
            .await
            .map_err(|e| ExtractionError::ScriptExecutionFailed(e.to_string()))?
    } else {
        DEFAULT_BUNDLE.to_string()
    };

    let pool = ExtractorPool::new(bundle, None);
    let handle = PoolHandle::new(pool);

    GLOBAL_POOL
        .set(handle)
        .map_err(|_| ExtractionError::ScriptExecutionFailed("Already initialized".to_string()))?;

    info!("Extractor crate initialized successfully");
    Ok(())
}

/// Extract video information from a URL via yt-dlp subprocess.
///
/// Uses `yt-dlp -J --no-playlist` which handles PO Token, signature decryption,
/// and throttle bypass automatically. The `cookies` parameter is accepted for
/// API compatibility but yt-dlp manages cookies internally.
pub async fn extract(url: &str, _cookies: Option<&str>) -> Result<VideoInfo, ExtractionError> {
    debug!("Extracting via yt-dlp: {}", url);
    ytdlp::extract_via_ytdlp(url).await
}

/// Extract video information with a specific platform
///
/// Use this when you already know the platform and want to skip auto-detection.
///
/// # Arguments
/// * `platform` - The platform identifier (e.g., "youtube")
/// * `url` - The video URL to extract
/// * `cookies` - Optional cookies string
pub async fn extract_with_platform(
    platform: &str,
    url: &str,
    cookies: Option<&str>,
) -> Result<VideoInfo, ExtractionError> {
    let pool = GLOBAL_POOL.get().ok_or_else(|| {
        ExtractionError::ScriptExecutionFailed(
            "Extractor not initialized. Call extractor::init() first.".to_string(),
        )
    })?;

    pool.extract(platform, url, cookies).await
}

/// Extract playlist items with a specific platform extractor.
///
/// Returns raw JSON value from JavaScript extractor (expected array of entries).
pub async fn extract_playlist(
    platform: &str,
    url: &str,
    cookies: Option<&str>,
) -> Result<serde_json::Value, ExtractionError> {
    let pool = GLOBAL_POOL.get().ok_or_else(|| {
        ExtractionError::ScriptExecutionFailed(
            "Extractor not initialized. Call extractor::init() first.".to_string(),
        )
    })?;

    pool.extract_playlist(platform, url, cookies).await
}

/// Create a new extractor pool with custom settings
///
/// Use this for advanced use cases where you need multiple pools
/// or custom configuration.
///
/// # Arguments
/// * `js_bundle` - The bundled JavaScript containing extractors
/// * `pool_size` - Number of concurrent workers (defaults to num_cpus)
pub fn create_pool(js_bundle: String, pool_size: Option<usize>) -> ExtractorPool {
    ExtractorPool::new(js_bundle, pool_size)
}

/// Get the default bundled JavaScript
pub fn default_bundle() -> &'static str {
    DEFAULT_BUNDLE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify public API exports compile correctly
        let _: Option<VideoInfo> = None;
        let _: Option<VideoFormat> = None;
        let _: Option<ExtractionError> = None;
    }
}
