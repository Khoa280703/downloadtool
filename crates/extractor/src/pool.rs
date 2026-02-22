//! Isolate pool for concurrent extractions
//!
//! Each extraction runs in its own V8 isolate (JsRuntime) because isolates
//! are not Send. The pool uses a semaphore to bound concurrent operations.

use crate::runtime::ExtractorRuntime;
use crate::types::{ExtractionError, VideoInfo};
use std::sync::Arc;
use tokio::sync::{Semaphore, SemaphorePermit};
use tracing::{debug, error, info, warn};

/// Pool of V8 isolates for running extractors concurrently
pub struct ExtractorPool {
    /// Semaphore controlling concurrent extraction access
    semaphore: Arc<Semaphore>,
    /// JavaScript bundle containing all extractors
    js_bundle: Arc<String>,
    /// Pool size (number of concurrent isolates)
    pool_size: usize,
}

impl ExtractorPool {
    /// Create a new extractor pool with the specified size
    ///
    /// # Arguments
    /// * `js_bundle` - The bundled JavaScript containing all extractors
    /// * `pool_size` - Maximum number of concurrent extractions (defaults to num_cpus)
    pub fn new(js_bundle: String, pool_size: Option<usize>) -> Self {
        let size = pool_size.unwrap_or_else(num_cpus::get);
        info!("Creating ExtractorPool with {} workers", size);

        Self {
            semaphore: Arc::new(Semaphore::new(size)),
            js_bundle: Arc::new(js_bundle),
            pool_size: size,
        }
    }

    /// Extract video information from a URL
    ///
    /// This method acquires a permit from the semaphore, creates a new
    /// JsRuntime isolate, and runs the extraction.
    pub async fn extract(
        &self,
        platform: &str,
        url: &str,
        cookies: Option<&str>,
    ) -> Result<VideoInfo, ExtractionError> {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|e| ExtractionError::ScriptExecutionFailed(e.to_string()))?;

        debug!("Acquired pool permit for {} extraction", platform);

        // Create a new runtime for this extraction
        // Each isolate is single-threaded and not Send, so we create it here
        let mut runtime = ExtractorRuntime::new(&self.js_bundle)?;

        // Run the extraction
        let result = runtime.extract(platform, url, cookies).await;

        // Permit is automatically released when _permit drops
        debug!("Released pool permit for {} extraction", platform);

        result
    }

    /// Get the pool size
    pub fn size(&self) -> usize {
        self.pool_size
    }

    /// Get current available permits
    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }
}

/// Pool handle that can be cloned and shared across tasks
#[derive(Clone)]
pub struct PoolHandle {
    inner: Arc<ExtractorPool>,
}

impl PoolHandle {
    /// Create a new pool handle
    pub fn new(pool: ExtractorPool) -> Self {
        Self {
            inner: Arc::new(pool),
        }
    }

    /// Extract video information
    pub async fn extract(
        &self,
        platform: &str,
        url: &str,
        cookies: Option<&str>,
    ) -> Result<VideoInfo, ExtractionError> {
        self.inner.extract(platform, url, cookies).await
    }

    /// Get pool size
    pub fn size(&self) -> usize {
        self.inner.size()
    }

    /// Get available permits
    pub fn available_permits(&self) -> usize {
        self.inner.available_permits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_creation() {
        let bundle = "// empty bundle".to_string();
        let pool = ExtractorPool::new(bundle, Some(4));
        assert_eq!(pool.size(), 4);
    }

    #[tokio::test]
    async fn test_pool_default_size() {
        let bundle = "// empty bundle".to_string();
        let pool = ExtractorPool::new(bundle, None);
        assert_eq!(pool.size(), num_cpus::get());
    }
}
