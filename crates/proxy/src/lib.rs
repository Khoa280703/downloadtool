//! Proxy crate - Stream proxy and anti-bot protection
//!
//! Provides HTTP proxying capabilities for video streams with
//! anti-bot detection evasion techniques including:
//! - Proxy rotation with health tracking
//! - Per-platform cookie persistence
//! - Browser-realistic header rotation
//! - Per-domain request throttling

use reqwest::Client;

pub mod anti_bot;
pub mod client;
pub mod cookie_store;
pub mod header_builder;
pub mod proxy_pool;
pub mod stream;
pub mod throttle;

pub use anti_bot::{AntiBotClient, AntiBotError, AntiBotGuard};
pub use client::ProxyClient;
pub use cookie_store::{CookieStore, Platform};
pub use header_builder::HeaderBuilder;
pub use proxy_pool::ProxyPool;
pub use stream::StreamProxy;
pub use throttle::DomainThrottle;

/// Default HTTP client configuration for proxy operations.
pub fn default_client() -> Client {
    Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .build()
        .expect("Failed to build HTTP client")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_client_creation() {
        let client = default_client();
        // Just verify it builds without panicking
        drop(client);
    }

    #[test]
    fn test_platform_enum_values() {
        assert_eq!(Platform::YouTube.to_string(), "youtube");
    }
}
