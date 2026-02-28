//! Isolate pool for concurrent extractions
//!
//! Each extraction runs in its own V8 isolate (JsRuntime) because isolates
//! are not Send. The pool uses a semaphore to bound concurrent operations.

use crate::runtime::ExtractorRuntime;
use crate::types::{ExtractionError, VideoInfo};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::{debug, info};

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
    /// JsRuntime is !Send, so work runs in spawn_blocking with its own runtime.
    pub async fn extract(
        &self,
        platform: &str,
        url: &str,
        cookies: Option<&str>,
    ) -> Result<VideoInfo, ExtractionError> {
        // OwnedSemaphorePermit is Send + 'static, safe to move into spawn_blocking
        let permit = Arc::clone(&self.semaphore)
            .acquire_owned()
            .await
            .map_err(|e| ExtractionError::ScriptExecutionFailed(e.to_string()))?;

        debug!("Acquired pool permit for {} extraction", platform);

        let bundle = Arc::clone(&self.js_bundle);
        let platform = platform.to_string();
        let url = url.to_string();
        let cookies = cookies.map(String::from);

        // JsRuntime is !Send — must run on a dedicated std::thread (not spawn_blocking).
        // Using spawn_blocking + rt.block_on() creates nested tokio contexts which
        // prevents deno_core's async ops from polling correctly (they hang indefinitely).
        // std::thread::spawn starts fresh with no existing runtime context, so
        // LocalSet::block_on works properly and async ops (op_fetch) can resolve.
        let (tx, rx) = tokio::sync::oneshot::channel();

        std::thread::spawn(move || {
            let _permit = permit; // released when thread exits

            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(rt) => rt,
                Err(e) => {
                    let _ = tx.send(Err(ExtractionError::ScriptExecutionFailed(e.to_string())));
                    return;
                }
            };

            let local = tokio::task::LocalSet::new();
            let result = local.block_on(&rt, async move {
                let mut runtime = ExtractorRuntime::new(&bundle)?;
                runtime.extract(&platform, &url, cookies.as_deref()).await
            });

            let _ = tx.send(result);
        });

        let result = rx
            .await
            .map_err(|e| ExtractionError::ScriptExecutionFailed(e.to_string()))??;

        debug!("Completed extraction");
        Ok(result)
    }

    /// Extract playlist items from a URL.
    pub async fn extract_playlist(
        &self,
        platform: &str,
        url: &str,
        cookies: Option<&str>,
    ) -> Result<serde_json::Value, ExtractionError> {
        let permit = Arc::clone(&self.semaphore)
            .acquire_owned()
            .await
            .map_err(|e| ExtractionError::ScriptExecutionFailed(e.to_string()))?;

        debug!("Acquired pool permit for {} playlist extraction", platform);

        let bundle = Arc::clone(&self.js_bundle);
        let platform = platform.to_string();
        let url = url.to_string();
        let cookies = cookies.map(String::from);
        let (tx, rx) = tokio::sync::oneshot::channel();

        std::thread::spawn(move || {
            let _permit = permit;

            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(rt) => rt,
                Err(e) => {
                    let _ = tx.send(Err(ExtractionError::ScriptExecutionFailed(e.to_string())));
                    return;
                }
            };

            let local = tokio::task::LocalSet::new();
            let result = local.block_on(&rt, async move {
                let mut runtime = ExtractorRuntime::new(&bundle)?;
                runtime
                    .extract_playlist(&platform, &url, cookies.as_deref())
                    .await
            });

            let _ = tx.send(result);
        });

        let result = rx
            .await
            .map_err(|e| ExtractionError::ScriptExecutionFailed(e.to_string()))??;

        debug!("Completed playlist extraction");
        Ok(result)
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

    /// Extract playlist items.
    pub async fn extract_playlist(
        &self,
        platform: &str,
        url: &str,
        cookies: Option<&str>,
    ) -> Result<serde_json::Value, ExtractionError> {
        self.inner.extract_playlist(platform, url, cookies).await
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
