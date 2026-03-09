use job_system::{compute_dedupe_key, compute_request_hash, JobOwner, MuxJobRequest};

#[derive(Debug, Clone)]
pub struct JobIdentity {
    pub request_hash: String,
    pub dedupe_key: String,
}

pub fn derive_job_identity(owner: &JobOwner, request: &MuxJobRequest) -> JobIdentity {
    JobIdentity {
        request_hash: compute_request_hash(&owner.scope_key, request),
        dedupe_key: compute_dedupe_key(request),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_identity_prefers_source_video_id() {
        let request = MuxJobRequest {
            video_url: "https://cdn.example/video".to_string(),
            audio_url: "https://cdn.example/audio".to_string(),
            source_url: Some("https://www.youtube.com/watch?v=abc123".to_string()),
            video_format_id: Some("137".to_string()),
            audio_format_id: Some("140".to_string()),
            title: Some("Hello".to_string()),
        };

        let identity = derive_job_identity(
            &JobOwner {
                user_id: Some("user-1".to_string()),
                session_id: None,
                scope_key: "user:user-1".to_string(),
            },
            &request,
        );

        assert!(identity
            .dedupe_key
            .starts_with("yt:abc123:137:140:mux-v1:mp4"));
        assert_eq!(identity.request_hash.len(), 64);
    }
}
