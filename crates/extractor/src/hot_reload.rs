//! Hot-reload file watcher for extractor scripts
//!
//! Watches the extractors/dist directory for changes and signals
//! the pool to reload scripts on the next extraction request.

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::watch;
use tracing::{debug, error, info, warn};

/// File watcher that monitors extractor scripts for changes
pub struct HotReloader {
    /// Channel sender for reload signals
    reload_tx: watch::Sender<bool>,
    /// Channel receiver (can be cloned for subscribers)
    reload_rx: watch::Receiver<bool>,
    /// The file system watcher
    _watcher: RecommendedWatcher,
}

impl HotReloader {
    /// Create a new hot-reload watcher for the given directory
    ///
    /// # Arguments
    /// * `watch_dir` - Directory to watch for changes
    pub fn new(watch_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let (reload_tx, reload_rx) = watch::channel(false);

        let tx = reload_tx.clone();
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                match res {
                    Ok(event) => {
                        // Only react to modify/create events on .js files
                        if event
                            .paths
                            .iter()
                            .any(|p| p.extension().map(|e| e == "js").unwrap_or(false))
                        {
                            match event.kind {
                                notify::EventKind::Modify(_) | notify::EventKind::Create(_) => {
                                    info!(
                                        "Detected change in extractor scripts: {:?}",
                                        event.paths
                                    );
                                    if let Err(e) = tx.send(true) {
                                        warn!("Failed to send reload signal: {}", e);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    Err(e) => {
                        error!("File watcher error: {}", e);
                    }
                }
            },
            Config::default(),
        )?;

        // Watch the directory recursively
        watcher.watch(watch_dir, RecursiveMode::Recursive)?;

        info!("Hot-reload watcher started for: {}", watch_dir.display());

        Ok(Self {
            reload_tx,
            reload_rx,
            _watcher: watcher,
        })
    }

    /// Get a receiver for reload signals
    pub fn subscribe(&self) -> watch::Receiver<bool> {
        self.reload_rx.clone()
    }

    /// Check if a reload is needed and reset the signal
    pub fn check_reload(&self) -> bool {
        let current = *self.reload_rx.borrow();
        if current {
            // Reset the signal
            let _ = self.reload_tx.send(false);
        }
        current
    }

    /// Manually trigger a reload
    pub fn trigger_reload(&self) {
        info!("Manual reload triggered");
        let _ = self.reload_tx.send(true);
    }
}

/// Reloadable bundle manager that watches for changes and reloads JS
pub struct ReloadableBundle {
    /// Current JS bundle content
    bundle: Arc<std::sync::RwLock<String>>,
    /// Path to the bundle file or directory
    bundle_path: std::path::PathBuf,
    /// Hot-reload watcher
    reloader: HotReloader,
}

impl ReloadableBundle {
    /// Create a new reloadable bundle
    ///
    /// # Arguments
    /// * `bundle_path` - Path to the bundled JS file or directory
    /// * `watch_dir` - Directory to watch for changes
    pub fn new(
        bundle_path: &Path,
        watch_dir: &Path,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let bundle_content = std::fs::read_to_string(bundle_path)?;
        let bundle = Arc::new(std::sync::RwLock::new(bundle_content));

        let reloader = HotReloader::new(watch_dir)?;

        Ok(Self {
            bundle,
            bundle_path: bundle_path.to_path_buf(),
            reloader,
        })
    }

    /// Get the current bundle content
    pub fn get_bundle(&self) -> Arc<std::sync::RwLock<String>> {
        Arc::clone(&self.bundle)
    }

    /// Check for reload and update bundle if needed
    pub fn check_and_reload(&self) -> Result<bool, Box<dyn std::error::Error>> {
        if self.reloader.check_reload() {
            info!("Reloading extractor bundle from: {}", self.bundle_path.display());

            match std::fs::read_to_string(&self.bundle_path) {
                Ok(new_content) => {
                    let mut bundle = self.bundle.write().map_err(|e| {
                        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
                    })?;
                    *bundle = new_content;
                    info!("Bundle reloaded successfully");
                    Ok(true)
                }
                Err(e) => {
                    error!("Failed to reload bundle: {}", e);
                    Err(Box::new(e))
                }
            }
        } else {
            Ok(false)
        }
    }

    /// Subscribe to reload signals
    pub fn subscribe(&self) -> watch::Receiver<bool> {
        self.reloader.subscribe()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::Duration;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_hot_reloader_creation() {
        let temp_dir = TempDir::new().unwrap();
        let reloader = HotReloader::new(temp_dir.path());
        assert!(reloader.is_ok());
    }

    #[tokio::test]
    async fn test_reload_signal() {
        let temp_dir = TempDir::new().unwrap();
        let reloader = HotReloader::new(temp_dir.path()).unwrap();

        // Initially no reload needed
        assert!(!reloader.check_reload());

        // Trigger reload
        reloader.trigger_reload();

        // Now reload should be needed
        assert!(reloader.check_reload());

        // After checking, should be reset
        assert!(!reloader.check_reload());
    }
}
