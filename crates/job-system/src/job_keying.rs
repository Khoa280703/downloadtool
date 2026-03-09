use sha2::{Digest, Sha256};

use crate::job_models::MuxJobRequest;

const MUX_PROFILE_VERSION: &str = "mux-v1";

pub fn compute_request_hash(user_id: &str, request: &MuxJobRequest) -> String {
    let mut hasher = Sha256::new();
    hasher.update(user_id.as_bytes());
    hasher.update([0]);
    hasher.update(request.video_url.as_bytes());
    hasher.update([0]);
    hasher.update(request.audio_url.as_bytes());
    hasher.update([0]);
    hasher.update(request.source_url.as_deref().unwrap_or_default().as_bytes());
    hasher.update([0]);
    hasher.update(
        request
            .video_format_id
            .as_deref()
            .unwrap_or_default()
            .as_bytes(),
    );
    hasher.update([0]);
    hasher.update(
        request
            .audio_format_id
            .as_deref()
            .unwrap_or_default()
            .as_bytes(),
    );
    hex::encode(hasher.finalize())
}

pub fn compute_dedupe_key(request: &MuxJobRequest) -> String {
    if let Some(video_id) = request
        .source_url
        .as_deref()
        .and_then(extract_youtube_video_id)
    {
        return format!(
            "yt:{video_id}:{}:{}:{MUX_PROFILE_VERSION}:mp4",
            request
                .video_format_id
                .as_deref()
                .unwrap_or("unknown-video"),
            request
                .audio_format_id
                .as_deref()
                .unwrap_or("unknown-audio"),
        );
    }

    let mut hasher = Sha256::new();
    hasher.update(request.video_url.as_bytes());
    hasher.update([0]);
    hasher.update(request.audio_url.as_bytes());
    hasher.update([0]);
    hasher.update(
        request
            .video_format_id
            .as_deref()
            .unwrap_or_default()
            .as_bytes(),
    );
    hasher.update([0]);
    hasher.update(
        request
            .audio_format_id
            .as_deref()
            .unwrap_or_default()
            .as_bytes(),
    );
    format!(
        "raw:{}:{MUX_PROFILE_VERSION}:mp4",
        hex::encode(hasher.finalize())
    )
}

fn extract_youtube_video_id(url: &str) -> Option<String> {
    let parsed = reqwest::Url::parse(url).ok()?;
    if let Some(host) = parsed.host_str() {
        if host.contains("youtu.be") {
            let segment = parsed.path().trim_matches('/').split('/').next()?;
            if !segment.is_empty() {
                return Some(segment.to_string());
            }
        }
        if host.contains("youtube.com") {
            if let Some(video_id) = parsed
                .query_pairs()
                .find(|(key, _)| key == "v")
                .map(|(_, value)| value.to_string())
            {
                return Some(video_id);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> MuxJobRequest {
        MuxJobRequest {
            video_url: "https://cdn.example/video".to_string(),
            audio_url: "https://cdn.example/audio".to_string(),
            source_url: Some("https://www.youtube.com/watch?v=abc123".to_string()),
            video_format_id: Some("137".to_string()),
            audio_format_id: Some("140".to_string()),
            title: Some("demo".to_string()),
        }
    }

    #[test]
    fn test_compute_dedupe_key_prefers_youtube_id() {
        let key = compute_dedupe_key(&request());
        assert_eq!(key, "yt:abc123:137:140:mux-v1:mp4");
    }

    #[test]
    fn test_compute_request_hash_is_stable() {
        let value_a = compute_request_hash("user-1", &request());
        let value_b = compute_request_hash("user-1", &request());
        assert_eq!(value_a, value_b);
    }
}
