//! Extractor crate - JavaScript-based video extraction using deno_core
//!
//! This crate provides video URL extraction capabilities using
//! TypeScript extractor scripts running in a V8 isolate via deno_core.
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
use tracing::{debug, error, info, warn};

pub mod hot_reload;
pub mod pool;
pub mod runtime;
pub mod types;

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

/// Extract video information from a URL
///
/// This is the main public API for extracting video information.
/// It automatically detects the platform from the URL.
///
/// # Arguments
/// * `url` - The video URL to extract
/// * `cookies` - Optional cookies string for authenticated requests
///
/// # Returns
/// Video information including available formats
///
/// # Errors
/// Returns an error if the URL is invalid, extraction fails, or
/// the extractor encounters an error.
///
/// # Example
/// ```rust,no_run
/// use extractor::extract;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let info = extract("https://youtube.com/watch?v=dQw4w9WgXcQ", None).await?;
/// println!("Found {} formats", info.formats.len());
/// # Ok(())
/// # }
/// ```
pub async fn extract(url: &str, cookies: Option<&str>) -> Result<VideoInfo, ExtractionError> {
    // Detect platform from URL
    let platform = detect_platform(url);
    debug!("Detected platform '{}' for URL: {}", platform, url);

    // Get or initialize global pool
    let pool = GLOBAL_POOL.get().ok_or_else(|| {
        ExtractionError::ScriptExecutionFailed(
            "Extractor not initialized. Call extractor::init() first.".to_string(),
        )
    })?;

    pool.extract(platform, url, cookies).await
}

/// Extract video information with a specific platform
///
/// Use this when you already know the platform and want to skip auto-detection.
///
/// # Arguments
/// * `platform` - The platform identifier (e.g., "youtube", "tiktok")
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

/// Detect the platform from a URL
///
/// Returns the platform identifier string or "unknown" if not recognized.
fn detect_platform(url: &str) -> &str {
    let url_lower = url.to_lowercase();

    if url_lower.contains("youtube.com")
        || url_lower.contains("youtu.be")
        || url_lower.contains("youtube.com/shorts")
        || url_lower.contains("youtube.com/live")
    {
        "youtube"
    } else if url_lower.contains("tiktok.com") {
        "tiktok"
    } else {
        "unknown"
    }
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
    fn test_detect_platform_youtube() {
        assert_eq!(
            detect_platform("https://youtube.com/watch?v=abc123"),
            "youtube"
        );
        assert_eq!(detect_platform("https://youtu.be/abc123"), "youtube");
        assert_eq!(
            detect_platform("https://www.youtube.com/shorts/abc123"),
            "youtube"
        );
    }

    #[test]
    fn test_detect_platform_tiktok() {
        assert_eq!(
            detect_platform("https://tiktok.com/@user/video/123456"),
            "tiktok"
        );
        assert_eq!(detect_platform("https://vm.tiktok.com/abc123"), "tiktok");
    }

    #[test]
    fn test_detect_platform_unknown() {
        assert_eq!(detect_platform("https://example.com/video"), "unknown");
    }

    #[test]
    fn test_module_exports() {
        // Verify public API exports compile correctly
        let _: Option<VideoInfo> = None;
        let _: Option<VideoFormat> = None;
        let _: Option<ExtractionError> = None;
    }
}
