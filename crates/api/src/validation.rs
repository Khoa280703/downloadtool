//! URL validation utilities.
//!
//! Centralises host-based YouTube URL validation to prevent SSRF.
//! Replaces the former substring-match approach (`contains("youtube.com")`),
//! which allowed crafted URLs like `https://evil.tld/?next=youtube.com` to
//! bypass validation and reach yt-dlp against arbitrary hosts.

/// Allowed YouTube hostnames (exact match, case-insensitive).
const ALLOWED_HOSTS: &[&str] = &[
    "youtube.com",
    "www.youtube.com",
    "m.youtube.com",
    "music.youtube.com",
    "youtu.be",
];

/// Validate that a URL points to a legitimate YouTube domain.
///
/// Uses `reqwest::Url` for proper RFC-compliant parsing — host is extracted
/// structurally, never via substring search. Returns `false` for any URL
/// whose host is not in the explicit allow-list.
pub fn is_valid_youtube_url(input: &str) -> bool {
    let Ok(parsed) = reqwest::Url::parse(input) else {
        return false;
    };
    let Some(host) = parsed.host_str() else {
        return false;
    };
    let host_lower = host.to_lowercase();
    ALLOWED_HOSTS.contains(&host_lower.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Valid URLs ---

    #[test]
    fn test_standard_youtube_watch_url() {
        assert!(is_valid_youtube_url(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
        ));
    }

    #[test]
    fn test_bare_youtube_com() {
        assert!(is_valid_youtube_url(
            "https://youtube.com/watch?v=dQw4w9WgXcQ"
        ));
    }

    #[test]
    fn test_youtu_be_short_link() {
        assert!(is_valid_youtube_url("https://youtu.be/dQw4w9WgXcQ"));
    }

    #[test]
    fn test_music_youtube_com() {
        assert!(is_valid_youtube_url(
            "https://music.youtube.com/watch?v=dQw4w9WgXcQ"
        ));
    }

    #[test]
    fn test_m_youtube_com() {
        assert!(is_valid_youtube_url(
            "https://m.youtube.com/watch?v=dQw4w9WgXcQ"
        ));
    }

    // --- SSRF attack vectors that must be rejected ---

    #[test]
    fn test_rejects_youtube_in_query_param() {
        // Classic SSRF: legitimate-looking host but youtube.com is only in path/query
        assert!(!is_valid_youtube_url(
            "https://evil.tld/?next=youtube.com/watch?v=abc"
        ));
    }

    #[test]
    fn test_rejects_youtube_as_subdomain_of_evil() {
        // youtube.com.evil.tld — host ends with youtube.com but is a different domain
        assert!(!is_valid_youtube_url(
            "https://youtube.com.evil.tld/watch?v=abc"
        ));
    }

    #[test]
    fn test_rejects_not_youtube_com() {
        assert!(!is_valid_youtube_url("https://notyoutube.com/watch?v=abc"));
    }

    #[test]
    fn test_rejects_vimeo() {
        assert!(!is_valid_youtube_url("https://vimeo.com/123456"));
    }

    // --- Edge cases ---

    #[test]
    fn test_rejects_empty_string() {
        assert!(!is_valid_youtube_url(""));
    }

    #[test]
    fn test_rejects_no_scheme() {
        // Relative URLs without scheme fail reqwest::Url::parse
        assert!(!is_valid_youtube_url("youtube.com/watch?v=abc"));
    }

    #[test]
    fn test_rejects_javascript_scheme() {
        assert!(!is_valid_youtube_url("javascript:alert(1)"));
    }

    #[test]
    fn test_rejects_data_uri() {
        assert!(!is_valid_youtube_url("data:text/html,<h1>hi</h1>"));
    }

    #[test]
    fn test_rejects_file_scheme() {
        assert!(!is_valid_youtube_url("file:///etc/passwd"));
    }
}
