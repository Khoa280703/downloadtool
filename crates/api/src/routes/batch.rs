//! Batch extraction handler with SSE streaming
//!
//! GET /api/batch - Server-Sent Events endpoint for playlist extraction.

use axum::extract::Query;
use axum::http::{header, HeaderValue};
use axum::response::{sse::Event, IntoResponse, Response, Sse};
use futures::stream::{self, BoxStream, Stream};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::time::Duration;
use tracing::{info, warn};

const NO_STORE_CACHE_CONTROL: &str = "no-store, no-cache, must-revalidate";

/// Query parameters for batch extraction.
#[derive(Debug, Deserialize)]
pub struct BatchParams {
    /// URL of the playlist to extract.
    pub url: String,
}

/// SSE event types for batch extraction.
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum BatchEvent {
    /// A video link extracted from the playlist.
    #[serde(rename = "link")]
    Link {
        /// Stable YouTube video ID (11 chars).
        #[serde(rename = "videoId")]
        video_id: String,
        /// Video URL.
        url: String,
        /// Video title.
        title: String,
        /// Thumbnail URL if available.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        /// Index in the batch.
        index: usize,
        /// Total expected videos.
        total: Option<usize>,
    },
    /// Batch extraction completed.
    #[serde(rename = "done")]
    Done {
        /// Total videos extracted.
        total: usize,
    },
    /// Error during extraction.
    #[serde(rename = "error")]
    Error {
        /// Error message.
        message: String,
    },
    /// Progress update.
    #[serde(rename = "progress")]
    Progress {
        /// Current progress.
        current: usize,
        /// Total expected.
        total: Option<usize>,
    },
}

#[derive(Debug, Deserialize)]
struct PlaylistVideoEntry {
    #[serde(rename = "videoId")]
    video_id: String,
    title: String,
    thumbnail: Option<String>,
    index: usize,
}

/// Batch extraction SSE endpoint.
///
/// GET /api/batch?url=<playlist_url>
pub async fn batch_handler(Query(params): Query<BatchParams>) -> Response {
    info!("Batch extraction request for URL: {}", params.url);

    let stream: BoxStream<'static, Result<Event, Infallible>> = if !is_valid_batch_url(&params.url)
    {
        warn!("Invalid batch URL: {}", params.url);
        Box::pin(stream::once(async {
            Ok(error_event(
                "Invalid URL. Only YouTube playlist URLs (?list=...) are supported.",
            ))
        }))
    } else {
        Box::pin(create_batch_stream(&params.url))
    };

    let mut response = Sse::new(stream)
        .keep_alive(axum::response::sse::KeepAlive::new().interval(Duration::from_secs(15)))
        .into_response();
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static(NO_STORE_CACHE_CONTROL),
    );
    response
}

fn create_batch_stream(url: &str) -> impl Stream<Item = Result<Event, Infallible>> {
    let requested_url = url.to_string();

    async_stream::stream! {
        info!("Starting batch extraction for: {}", requested_url);

        let playlist_id = match extract_playlist_id(&requested_url) {
            Some(id) => id,
            None => {
                yield Ok(error_event("Could not parse playlist ID from URL"));
                return;
            }
        };

        // Normalize URL for the JS extractor.
        let playlist_url = format!("https://www.youtube.com/playlist?list={}", playlist_id);

        let raw_entries = match extractor::extract_playlist("youtube", &playlist_url, None).await {
            Ok(value) => value,
            Err(err) => {
                yield Ok(error_event(format!("Batch extraction failed: {}", err)));
                return;
            }
        };

        let mut entries: Vec<PlaylistVideoEntry> = match serde_json::from_value(raw_entries) {
            Ok(value) => value,
            Err(err) => {
                yield Ok(error_event(format!("Invalid playlist payload: {}", err)));
                return;
            }
        };

        entries.sort_by_key(|entry| entry.index);
        let total = entries.len();
        yield Ok(progress_event(0, Some(total)));

        for (offset, entry) in entries.into_iter().enumerate() {
            let current = offset + 1;
            let index = if entry.index == 0 { current } else { entry.index };
            let event = BatchEvent::Link {
                video_id: entry.video_id.clone(),
                url: format!("https://www.youtube.com/watch?v={}", entry.video_id),
                title: entry.title,
                thumbnail: entry.thumbnail,
                index,
                total: Some(total),
            };
            yield Ok(data_event(&event));
            yield Ok(progress_event(current, Some(total)));
        }

        yield Ok(done_event(total));
        info!("Batch extraction completed: {} videos", total);
    }
}

fn data_event(event: &BatchEvent) -> Event {
    match serde_json::to_string(event) {
        Ok(data) => Event::default().data(data),
        Err(err) => Event::default().data(
            serde_json::json!({
                "type": "error",
                "message": format!("Failed to serialize SSE event: {}", err)
            })
            .to_string(),
        ),
    }
}

fn error_event(message: impl Into<String>) -> Event {
    data_event(&BatchEvent::Error {
        message: message.into(),
    })
}

fn progress_event(current: usize, total: Option<usize>) -> Event {
    data_event(&BatchEvent::Progress { current, total })
}

fn done_event(total: usize) -> Event {
    data_event(&BatchEvent::Done { total })
}

/// Check if URL is a valid YouTube playlist URL.
fn is_valid_batch_url(url: &str) -> bool {
    let lower = url.to_lowercase();
    (lower.contains("youtube.com") || lower.contains("youtu.be")) && lower.contains("list=")
}

fn extract_playlist_id(url: &str) -> Option<String> {
    if let Ok(parsed) = reqwest::Url::parse(url) {
        if let Some(value) = parsed.query_pairs().find(|(key, _)| key == "list") {
            if !value.1.is_empty() {
                return Some(value.1.into_owned());
            }
        }
    }

    let marker = "list=";
    let start = url.find(marker)? + marker.len();
    let rest = &url[start..];
    let end = rest.find('&').unwrap_or(rest.len());
    let id = &rest[..end];
    if id.is_empty() {
        None
    } else {
        Some(id.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_batch_url() {
        assert!(is_valid_batch_url("https://youtube.com/playlist?list=PLxxx"));
        assert!(is_valid_batch_url("https://www.youtube.com/watch?v=abc&list=PLxxx"));

        assert!(!is_valid_batch_url("https://youtube.com/watch?v=abc"));
        assert!(!is_valid_batch_url("https://example.com/playlist?list=PLxxx"));
    }

    #[test]
    fn test_extract_playlist_id() {
        assert_eq!(
            extract_playlist_id("https://youtube.com/playlist?list=PL12345"),
            Some("PL12345".to_string())
        );
        assert_eq!(
            extract_playlist_id("https://youtube.com/watch?v=abc&list=PLXYZ99&t=30"),
            Some("PLXYZ99".to_string())
        );
        assert_eq!(extract_playlist_id("https://youtube.com/watch?v=abc"), None);
    }

    #[test]
    fn test_batch_event_serialization_contains_video_id() {
        let event = BatchEvent::Link {
            video_id: "dQw4w9WgXcQ".to_string(),
            url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string(),
            title: "Test Video".to_string(),
            thumbnail: Some("https://i.ytimg.com/test.jpg".to_string()),
            index: 1,
            total: Some(10),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"type\":\"link\""));
        assert!(json.contains("\"videoId\":\"dQw4w9WgXcQ\""));
    }
}
