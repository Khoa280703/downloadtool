//! Proxy pool with round-robin rotation and failure tracking
//!
//! Manages a pool of proxy URLs with automatic health checking
//! and failover support.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, warn};

/// Maximum consecutive failures before marking proxy as unhealthy
const MAX_FAILURES: usize = 3;
/// Cooldown period for failed proxies (60 seconds)
const FAILURE_COOLDOWN: Duration = Duration::from_secs(60);

/// Entry in the proxy pool with health tracking
struct ProxyEntry {
    url: String,
    failed_count: AtomicUsize,
    last_failed: RwLock<Option<Instant>>,
}

impl ProxyEntry {
    fn new(url: String) -> Self {
        Self {
            url,
            failed_count: AtomicUsize::new(0),
            last_failed: RwLock::new(None),
        }
    }

    /// Check if proxy is currently healthy
    fn is_healthy(&self) -> bool {
        let failures = self.failed_count.load(Ordering::Relaxed);
        if failures < MAX_FAILURES {
            return true;
        }

        // Check if cooldown has passed
        if let Ok(last_failed) = self.last_failed.read() {
            if let Some(instant) = *last_failed {
                return instant.elapsed() > FAILURE_COOLDOWN;
            }
        }
        false
    }

    /// Mark proxy as failed
    fn mark_failed(&self) {
        let count = self.failed_count.fetch_add(1, Ordering::Relaxed) + 1;
        if let Ok(mut last_failed) = self.last_failed.write() {
            *last_failed = Some(Instant::now());
        }
        warn!(
            "Proxy {} marked as failed (count: {}, cooldown: {}s)",
            self.url,
            count,
            FAILURE_COOLDOWN.as_secs()
        );
    }

    /// Reset failure count on success
    fn mark_success(&self) {
        let previous = self.failed_count.swap(0, Ordering::Relaxed);
        if previous > 0 {
            if let Ok(mut last_failed) = self.last_failed.write() {
                *last_failed = None;
            }
            debug!("Proxy {} recovered from failure state", self.url);
        }
    }
}

/// Pool of proxies with round-robin selection
pub struct ProxyPool {
    proxies: Vec<Arc<ProxyEntry>>,
    current: AtomicUsize,
}

impl ProxyPool {
    /// Create a new proxy pool from a list of proxy URLs
    pub fn new(urls: Vec<String>) -> Self {
        let proxies = urls.into_iter().map(|url| Arc::new(ProxyEntry::new(url))).collect();
        Self {
            proxies,
            current: AtomicUsize::new(0),
        }
    }

    /// Create a proxy pool from environment variable PROXY_LIST
    /// Format: "http://user:pass@host1:port,http://user:pass@host2:port"
    pub fn from_env() -> Option<Self> {
        std::env::var("PROXY_LIST").ok().map(|env_str| {
            let urls: Vec<String> = env_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            debug!("Loaded {} proxies from PROXY_LIST environment variable", urls.len());
            Self::new(urls)
        })
    }

    /// Get the next healthy proxy using round-robin
    /// Returns None if no proxies are configured or all are unhealthy
    pub fn next(&self) -> Option<&str> {
        if self.proxies.is_empty() {
            return None;
        }

        let start_idx = self.current.fetch_add(1, Ordering::Relaxed);
        let proxy_count = self.proxies.len();

        // Try to find a healthy proxy, checking at most proxy_count entries
        for i in 0..proxy_count {
            let idx = (start_idx + i) % proxy_count;
            let entry = &self.proxies[idx];

            if entry.is_healthy() {
                return Some(&entry.url);
            }
        }

        // No healthy proxies found, return the first one anyway (it will retry)
        warn!("No healthy proxies available, falling back to first proxy");
        self.proxies.first().map(|e| e.url.as_str())
    }

    /// Mark a specific proxy as failed
    pub fn mark_failed(&self, proxy_url: &str) {
        for entry in &self.proxies {
            if entry.url == proxy_url {
                entry.mark_failed();
                return;
            }
        }
    }

    /// Mark a specific proxy as successful (reset failure count)
    pub fn mark_success(&self, proxy_url: &str) {
        for entry in &self.proxies {
            if entry.url == proxy_url {
                entry.mark_success();
                return;
            }
        }
    }

    /// Get the number of configured proxies
    pub fn len(&self) -> usize {
        self.proxies.len()
    }

    /// Check if pool has any proxies
    pub fn is_empty(&self) -> bool {
        self.proxies.is_empty()
    }
}

impl Default for ProxyPool {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_pool_round_robin() {
        let pool = ProxyPool::new(vec![
            "http://proxy1:8080".to_string(),
            "http://proxy2:8080".to_string(),
            "http://proxy3:8080".to_string(),
        ]);

        let p1 = pool.next().unwrap();
        let p2 = pool.next().unwrap();
        let p3 = pool.next().unwrap();
        let p4 = pool.next().unwrap(); // Should wrap around

        assert_eq!(p1, "http://proxy1:8080");
        assert_eq!(p2, "http://proxy2:8080");
        assert_eq!(p3, "http://proxy3:8080");
        assert_eq!(p4, "http://proxy1:8080");
    }

    #[test]
    fn test_proxy_pool_empty() {
        let pool = ProxyPool::new(vec![]);
        assert!(pool.next().is_none());
        assert!(pool.is_empty());
    }

    #[test]
    fn test_proxy_failure_tracking() {
        let pool = ProxyPool::new(vec![
            "http://proxy1:8080".to_string(),
            "http://proxy2:8080".to_string(),
        ]);

        // Mark first proxy as failed
        pool.mark_failed("http://proxy1:8080");

        // Should skip failed proxy and return second one
        let proxy = pool.next().unwrap();
        assert_eq!(proxy, "http://proxy2:8080");
    }

    #[test]
    fn test_proxy_success_reset() {
        let pool = ProxyPool::new(vec!["http://proxy1:8080".to_string()]);

        pool.mark_failed("http://proxy1:8080");
        pool.mark_success("http://proxy1:8080");

        // Should be healthy again
        let proxy = pool.next().unwrap();
        assert_eq!(proxy, "http://proxy1:8080");
    }
}
