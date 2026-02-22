//! Extractor engine using deno_core V8 runtime

use crate::types::{ExtractionError, VideoInfo};
use std::path::Path;
use tracing::{debug, info};

/// JavaScript extraction engine running on deno_core.
pub struct ExtractorEngine {
    /// Path to the extractor scripts directory
    scripts_dir: std::path::PathBuf,
}

impl ExtractorEngine {
    /// Create a new extractor engine.
    ///
    /// # Arguments
    /// * `scripts_dir` - Path to the directory containing TypeScript scripts
    ///
    /// # Errors
    /// Returns an error if the engine cannot be initialized.
    pub async fn new<P: AsRef<Path>>(scripts_dir: P) -> Result<Self, ExtractionError> {
        let scripts_dir = scripts_dir.as_ref().to_path_buf();

        if !scripts_dir.exists() {
            return Err(ExtractionError::ScriptsDirectoryNotFound(
                scripts_dir.display().to_string(),
            ));
        }

        info!(
            "Initializing extractor engine with scripts from: {}",
            scripts_dir.display()
        );

        // deno_core initialization will be implemented in Phase 02
        debug!("deno_core runtime initialization pending");

        Ok(Self { scripts_dir })
    }

    /// Extract video information from a URL.
    ///
    /// # Arguments
    /// * `url` - The video URL to extract
    ///
    /// # Errors
    /// Returns an error if extraction fails.
    pub async fn extract(&self, url: &str) -> Result<VideoInfo, ExtractionError> {
        info!("Extracting video info from: {}", url);

        // Actual extraction logic will be implemented in Phase 02
        // This is a placeholder that returns an error
        Err(ExtractionError::NotImplemented)
    }

    /// Get the scripts directory path.
    pub fn scripts_dir(&self) -> &Path {
        &self.scripts_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_engine_creation_fails_for_missing_dir() {
        let result = ExtractorEngine::new("/nonexistent/path").await;
        assert!(matches!(
            result,
            Err(ExtractionError::ScriptsDirectoryNotFound(_))
        ));
    }
}
