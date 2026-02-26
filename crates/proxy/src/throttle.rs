//! Per-domain request throttling to avoid rate limiting
//!
//! Ensures minimum delay between requests to the same domain.

use dashmap::DashMap;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, trace};

/// Default minimum delay between requests to the same domain (100ms)
const DEFAULT_MIN_DELAY: Duration = Duration::from_millis(100);

/// Tracks last request time per domain and enforces rate limits
pub struct DomainThrottle {
    last_request: DashMap<String, Instant>,
    min_delay: Duration,
}

impl DomainThrottle {
    /// Create a new domain throttle with default 100ms delay
    pub fn new() -> Self {
        Self {
            last_request: DashMap::new(),
            min_delay: DEFAULT_MIN_DELAY,
        }
    }

    /// Create a new domain throttle with custom delay
    pub fn with_delay(min_delay: Duration) -> Self {
        Self {
            last_request: DashMap::new(),
            min_delay,
        }
    }

    /// Wait if necessary to respect the minimum delay between requests
    ///
    /// This method checks when the last request was made to the given domain
    /// and sleeps if the minimum delay hasn't passed yet.
    pub async fn wait(&self, domain: &str) {
        let now = Instant::now();

        // Check when the last request was made to this domain
        let should_wait = if let Some(last) = self.last_request.get(domain) {
            let elapsed = last.elapsed();
            if elapsed < self.min_delay {
                let wait_time = self.min_delay - elapsed;
                trace!(
                    "Throttling request to {}: waiting {:?}",
                    domain,
                    wait_time
                );
                Some(wait_time)
            } else {
                None
            }
        } else {
            None
        };

        // Wait if needed
        if let Some(wait_time) = should_wait {
            sleep(wait_time).await;
        }

        // Update last request time
        self.last_request.insert(domain.to_string(), Instant::now());

        debug!("Request allowed for domain: {} (min_delay: {:?})", domain, self.min_delay);
    }

    /// Get the minimum delay configured for this throttle
    pub fn min_delay(&self) -> Duration {
        self.min_delay
    }

    /// Clear all tracking data (useful for testing)
    pub fn clear(&self) {
        self.last_request.clear();
    }

    /// Get the number of domains being tracked
    pub fn tracked_domains(&self) -> usize {
        self.last_request.len()
    }
}

impl Default for DomainThrottle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_throttle_enforces_delay() {
        let throttle = DomainThrottle::with_delay(Duration::from_millis(50));
        let start = Instant::now();

        // First request should not wait
        throttle.wait("example.com").await;
        let first_elapsed = start.elapsed();
        assert!(first_elapsed < Duration::from_millis(10));

        // Second request should wait ~50ms
        throttle.wait("example.com").await;
        let second_elapsed = start.elapsed();
        assert!(second_elapsed >= Duration::from_millis(50));
    }

    #[tokio::test]
    async fn test_throttle_different_domains() {
        let throttle = DomainThrottle::with_delay(Duration::from_millis(100));
        let start = Instant::now();

        // Requests to different domains should not interfere
        throttle.wait("example.com").await;
        throttle.wait("other.com").await;

        let elapsed = start.elapsed();
        // Both should complete quickly since they're different domains
        assert!(elapsed < Duration::from_millis(50));
    }

    #[test]
    fn test_throttle_default_delay() {
        let throttle = DomainThrottle::new();
        assert_eq!(throttle.min_delay(), Duration::from_millis(100));
    }

    #[test]
    fn test_throttle_clear() {
        let throttle = DomainThrottle::new();
        throttle.last_request.insert("example.com".to_string(), Instant::now());
        assert_eq!(throttle.tracked_domains(), 1);

        throttle.clear();
        assert_eq!(throttle.tracked_domains(), 0);
    }
}
