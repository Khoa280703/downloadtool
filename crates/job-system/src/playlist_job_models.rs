use serde::{Deserialize, Serialize};

/// Status of a playlist job (the overall orchestration).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaylistJobStatus {
    Queued,
    Discovering,
    Ready,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

impl PlaylistJobStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Discovering => "discovering",
            Self::Ready => "ready",
            Self::Processing => "processing",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "discovering" => Self::Discovering,
            "ready" => Self::Ready,
            "processing" => Self::Processing,
            "completed" => Self::Completed,
            "failed" => Self::Failed,
            "cancelled" => Self::Cancelled,
            _ => Self::Queued,
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

/// Status of a single item within a playlist job.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaylistItemStatus {
    Pending,
    Extracting,
    QueuedMux,
    Muxing,
    Ready,
    Completed,
    Failed,
    Cancelled,
}

impl PlaylistItemStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Extracting => "extracting",
            Self::QueuedMux => "queued_mux",
            Self::Muxing => "muxing",
            Self::Ready => "ready",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "extracting" => Self::Extracting,
            "queued_mux" => Self::QueuedMux,
            "muxing" => Self::Muxing,
            "ready" => Self::Ready,
            "completed" => Self::Completed,
            "failed" => Self::Failed,
            "cancelled" => Self::Cancelled,
            _ => Self::Pending,
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

/// Persisted playlist job record.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistJobRecord {
    pub id: String,
    pub source_url: String,
    pub title: Option<String>,
    pub status: PlaylistJobStatus,
    pub total_items: i32,
    pub completed_items: i32,
    pub failed_items: i32,
    pub requested_quality: String,
    pub requested_mode: String,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub request_ip: Option<String>,
    pub created_at_ms: i64,
    pub updated_at_ms: i64,
}

/// Persisted playlist job item record.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistJobItemRecord {
    pub id: String,
    pub playlist_job_id: String,
    pub video_id: String,
    pub title: Option<String>,
    pub thumbnail: Option<String>,
    pub ordinal: i32,
    pub status: PlaylistItemStatus,
    pub attempt_count: i32,
    pub last_error: Option<String>,
    pub selected_stream_meta: Option<serde_json::Value>,
    pub mux_job_id: Option<String>,
    pub artifact_key: Option<String>,
    pub download_url: Option<String>,
    pub created_at_ms: i64,
    pub updated_at_ms: i64,
}

/// Request payload to create a playlist job.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePlaylistJobRequest {
    pub source_url: String,
    pub requested_quality: Option<String>,
    pub requested_mode: Option<String>,
}
