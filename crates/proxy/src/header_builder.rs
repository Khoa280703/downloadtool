//! Browser-realistic header generation with user-agent rotation
//!
//! Generates headers that mimic real browsers to avoid bot detection.

use crate::cookie_store::Platform;
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, REFERER, USER_AGENT,
};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Current Chrome/Firefox user agents for rotation
const USER_AGENTS: &[(&str, &str)] = &[
    (
        "Chrome/120.0.0.0",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.0",
    ),
    (
        "Chrome/120.0.0.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.0",
    ),
    (
        "Chrome/120.0.0.0",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.0",
    ),
    (
        "Firefox/121.0",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
    ),
    (
        "Firefox/121.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:121.0) Gecko/20100101 Firefox/121.0",
    ),
    (
        "Chrome/119.0.0.0",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.0 Edg/119.0.0.0",
    ),
];

/// Builds realistic browser headers for requests
pub struct HeaderBuilder {
    current_index: AtomicUsize,
}

impl HeaderBuilder {
    /// Create a new header builder
    pub fn new() -> Self {
        Self {
            current_index: AtomicUsize::new(0),
        }
    }

    /// Get the next user agent in rotation
    pub fn next_user_agent(&self) -> &'static str {
        let index = self.current_index.fetch_add(1, Ordering::Relaxed) % USER_AGENTS.len();
        USER_AGENTS[index].1
    }

    /// Build headers for a specific platform
    pub fn build_headers(&self,
        platform: Platform,
        referer: Option<&str>,
    ) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let user_agent = self.next_user_agent();

        // User-Agent
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(user_agent),
        );

        // Accept headers
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"),
        );

        headers.insert(
            ACCEPT_LANGUAGE,
            HeaderValue::from_static("en-US,en;q=0.9"),
        );

        headers.insert(
            ACCEPT_ENCODING,
            HeaderValue::from_static("gzip, deflate, br"),
        );

        // Platform-specific headers
        match platform {
            Platform::YouTube => {
                headers.insert(
                    "sec-ch-ua",
                    HeaderValue::from_static("\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\""),
                );
                headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
                headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Windows\""));
                headers.insert("sec-fetch-dest", HeaderValue::from_static("document"));
                headers.insert("sec-fetch-mode", HeaderValue::from_static("navigate"));
                headers.insert("sec-fetch-site", HeaderValue::from_static("none"));
                headers.insert("sec-fetch-user", HeaderValue::from_static("?1"));
                headers.insert("upgrade-insecure-requests", HeaderValue::from_static("1"));

                if referer.is_none() {
                    headers.insert(REFERER, HeaderValue::from_static("https://www.youtube.com/"));
                }
            }
        }

        // Custom referer if provided
        if let Some(ref_url) = referer {
            if let Ok(value) = HeaderValue::from_str(ref_url) {
                headers.insert(REFERER, value);
            }
        }

        headers
    }

    /// Build headers for a generic request (no platform-specific headers)
    pub fn build_generic_headers(&self,
        referer: Option<&str>,
    ) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let user_agent = self.next_user_agent();

        headers.insert(USER_AGENT, HeaderValue::from_static(user_agent));
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));

        if let Some(ref_url) = referer {
            if let Ok(value) = HeaderValue::from_str(ref_url) {
                headers.insert(REFERER, value);
            }
        }

        headers
    }
}

impl Default for HeaderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Get a random user agent (for one-off use)
pub fn random_user_agent() -> &'static str {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let index = (now as usize) % USER_AGENTS.len();
    USER_AGENTS[index].1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_builder_rotation() {
        let builder = HeaderBuilder::new();
        let ua1 = builder.next_user_agent();
        let ua2 = builder.next_user_agent();
        let ua3 = builder.next_user_agent();

        // Should rotate through different UAs
        assert!(!ua1.is_empty());
        assert!(!ua2.is_empty());
        assert!(!ua3.is_empty());
    }

    #[test]
    fn test_build_youtube_headers() {
        let builder = HeaderBuilder::new();
        let headers = builder.build_headers(Platform::YouTube, None);

        assert!(headers.contains_key(USER_AGENT));
        assert!(headers.contains_key(REFERER));
        assert!(headers.contains_key("sec-ch-ua"));
    }

    #[test]
    fn test_custom_referer() {
        let builder = HeaderBuilder::new();
        let headers = builder.build_headers(Platform::YouTube, Some("https://example.com"));

        let referer = headers.get(REFERER).unwrap().to_str().unwrap();
        assert_eq!(referer, "https://example.com");
    }

    #[test]
    fn test_random_user_agent() {
        let ua = random_user_agent();
        assert!(!ua.is_empty());
        assert!(ua.starts_with("Mozilla/5.0"));
    }
}
