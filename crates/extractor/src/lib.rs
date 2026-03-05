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
//!     let info = extract("https://youtube.com/watch?v=...").await?;
//!     println!("Title: {}", info.title);
//!     println!("Formats: {}", info.formats.len());
//!     Ok(())
//! }
//! ```

use std::path::Path;
use std::sync::OnceLock;
use tracing::{debug, info};

pub mod pool;
pub mod runtime;
pub mod types;
pub mod ytdlp;

pub use types::{ExtractionError, VideoFormat, VideoInfo};

// Global pool instance for the simple API
static GLOBAL_POOL: OnceLock<pool::PoolHandle> = OnceLock::new();

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

    let pool = pool::ExtractorPool::new(bundle, None);
    let handle = pool::PoolHandle::new(pool);

    GLOBAL_POOL
        .set(handle)
        .map_err(|_| ExtractionError::ScriptExecutionFailed("Already initialized".to_string()))?;

    info!("Extractor crate initialized successfully");
    Ok(())
}

/// Extract video information from a URL via yt-dlp subprocess.
///
/// Uses `yt-dlp -J --no-playlist` which handles PO Token, signature decryption
/// and throttle bypass automatically.
pub async fn extract(url: &str) -> Result<VideoInfo, ExtractionError> {
    debug!("Extracting via yt-dlp: {}", url);
    ytdlp::extract_via_ytdlp(url).await
}

/// Resolve pinned proxy URL for a direct stream URL, if previously extracted.
pub async fn resolve_stream_proxy(url: &str) -> Option<String> {
    ytdlp::resolve_stream_proxy(url).await
}

/// Extract playlist items with a specific platform extractor.
///
/// Returns raw JSON value from JavaScript extractor (expected array of entries).
pub async fn extract_playlist(
    platform: &str,
    url: &str,
) -> Result<serde_json::Value, ExtractionError> {
    let pool = GLOBAL_POOL.get().ok_or_else(|| {
        ExtractionError::ScriptExecutionFailed(
            "Extractor not initialized. Call extractor::init() first.".to_string(),
        )
    })?;

    pool.extract_playlist(platform, url).await
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
