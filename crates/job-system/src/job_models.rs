use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MuxJobRequest {
    pub video_url: String,
    pub audio_url: String,
    pub source_url: Option<String>,
    pub video_format_id: Option<String>,
    pub audio_format_id: Option<String>,
    pub title: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobOwner {
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub scope_key: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    Queued,
    Leased,
    Processing,
    Ready,
    Failed,
    Expired,
}

impl JobStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Leased => "leased",
            Self::Processing => "processing",
            Self::Ready => "ready",
            Self::Failed => "failed",
            Self::Expired => "expired",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "leased" => Self::Leased,
            "processing" => Self::Processing,
            "ready" => Self::Ready,
            "failed" => Self::Failed,
            "expired" => Self::Expired,
            _ => Self::Queued,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactStatus {
    Building,
    Ready,
    Failed,
    Incomplete,
}

impl ArtifactStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Building => "building",
            Self::Ready => "ready",
            Self::Failed => "failed",
            Self::Incomplete => "incomplete",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobRecord {
    pub id: String,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub request_hash: String,
    pub dedupe_key: String,
    pub request: MuxJobRequest,
    pub status: JobStatus,
    pub artifact_id: Option<String>,
    pub attempt_count: i32,
    pub max_attempts: i32,
    pub lease_owner: Option<String>,
    pub lease_expires_at_ms: Option<i64>,
    pub last_error: Option<String>,
    pub created_at_ms: i64,
    pub updated_at_ms: i64,
    pub file_size_bytes: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtifactRecord {
    pub id: String,
    pub dedupe_key: String,
    pub backend: String,
    pub local_path: Option<String>,
    pub storage_bucket: Option<String>,
    pub object_key: Option<String>,
    pub status: String,
    pub size_bytes: Option<i64>,
    pub content_type: String,
    pub etag: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobArtifactDownload {
    pub job: JobRecord,
    pub artifact: ArtifactRecord,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobCreationResult {
    pub job: JobRecord,
    pub reused_existing: bool,
}
