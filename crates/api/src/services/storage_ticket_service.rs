use std::sync::Arc;

use anyhow::{anyhow, Context};
use object_store::{StorageBackend, StoredArtifact};

use super::job_control_plane::JobStatusRecord;

#[derive(Debug, Clone)]
pub struct StorageDownloadTicket {
    pub download_url: String,
    pub backend: String,
    pub direct_download: bool,
}

pub struct StorageTicketService {
    storage_backend: Option<Arc<dyn StorageBackend>>,
    ticket_ttl_secs: u64,
}

impl StorageTicketService {
    pub fn new(
        storage_backend: Option<Arc<dyn StorageBackend>>,
        ticket_ttl_secs: u64,
    ) -> Arc<Self> {
        Arc::new(Self {
            storage_backend,
            ticket_ttl_secs: ticket_ttl_secs.max(60),
        })
    }

    pub async fn build_ticket(
        &self,
        job: &JobStatusRecord,
        prefer_direct_download: bool,
    ) -> anyhow::Result<StorageDownloadTicket> {
        let backend = job.backend.as_deref().unwrap_or("localfs");
        if backend == "localfs" || !prefer_direct_download {
            return Ok(StorageDownloadTicket {
                download_url: format!("/api/jobs/{}/file", job.job_id),
                backend: backend.to_string(),
                direct_download: false,
            });
        }

        let storage_backend = self
            .storage_backend
            .as_ref()
            .cloned()
            .context("object storage backend is not configured")?;
        let artifact = StoredArtifact {
            backend: backend.to_string(),
            local_path: job
                .local_path
                .as_ref()
                .map(|path| path.to_string_lossy().to_string()),
            storage_bucket: job.storage_bucket.clone(),
            object_key: job.object_key.clone(),
            size_bytes: job.file_size_bytes.unwrap_or_default() as i64,
            etag: job.etag.clone(),
            content_type: job
                .content_type
                .clone()
                .unwrap_or_else(|| "video/mp4".to_string()),
        };
        let content_disposition = build_download_content_disposition(job.title.as_deref(), "mp4");
        let presigned = storage_backend
            .presign_get(&artifact, self.ticket_ttl_secs, Some(&content_disposition))
            .await
            .with_context(|| format!("failed to presign download for job {}", job.job_id))?;

        Ok(StorageDownloadTicket {
            download_url: presigned.url,
            backend: backend.to_string(),
            direct_download: true,
        })
    }

    pub fn supports_proxy_file_stream(&self, job: &JobStatusRecord) -> bool {
        matches!(job.backend.as_deref(), None | Some("localfs"))
    }

    pub fn ensure_local_path(job: &JobStatusRecord) -> anyhow::Result<&std::path::Path> {
        job.local_path
            .as_deref()
            .ok_or_else(|| anyhow!("ready artifact is missing local storage path"))
    }
}

fn build_download_content_disposition(title: Option<&str>, extension: &str) -> String {
    let filename = build_download_filename(title, extension);
    let ascii_name: String = filename
        .chars()
        .map(|c| if c.is_ascii() { c } else { '_' })
        .collect();
    let encoded: String = filename
        .bytes()
        .flat_map(|b| {
            if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
                vec![b as char]
            } else {
                format!("%{:02X}", b).chars().collect::<Vec<_>>()
            }
        })
        .collect();

    format!(
        r#"attachment; filename="{}"; filename*=UTF-8''{}"#,
        ascii_name, encoded
    )
}

fn build_download_filename(title: Option<&str>, extension: &str) -> String {
    let base = sanitize_filename(title.unwrap_or("video"));
    let ext = extension.trim_start_matches('.').trim();
    if ext.is_empty() {
        base
    } else {
        format!("{base}.{ext}")
    }
}

fn sanitize_filename(name: &str) -> String {
    let sanitized = name
        .chars()
        .map(|c| match c {
            c if c.is_control() => '_',
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            c => c,
        })
        .take(180)
        .collect::<String>()
        .trim()
        .to_string();

    if sanitized.is_empty() {
        "video".to_string()
    } else {
        sanitized
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use job_system::JobStatus;

    use super::*;

    fn ready_local_job() -> JobStatusRecord {
        JobStatusRecord {
            job_id: "job-1".to_string(),
            status: JobStatus::Ready,
            error: None,
            created_at_ms: 1,
            updated_at_ms: 2,
            queue_position: None,
            file_size_bytes: Some(42),
            title: Some("demo".to_string()),
            backend: Some("localfs".to_string()),
            local_path: Some(PathBuf::from("/tmp/demo.mp4")),
            object_key: None,
            storage_bucket: None,
            content_type: Some("video/mp4".to_string()),
            etag: None,
        }
    }

    #[tokio::test]
    async fn localfs_ticket_falls_back_to_api_file_route() {
        let service = StorageTicketService::new(None, 900);
        let ticket = service
            .build_ticket(&ready_local_job(), true)
            .await
            .expect("localfs ticket should succeed");

        assert_eq!(ticket.download_url, "/api/jobs/job-1/file");
        assert!(!ticket.direct_download);
    }

    #[test]
    fn content_disposition_uses_video_title() {
        let disposition = build_download_content_disposition(Some("Rick Astley / Demo"), "mp4");
        assert!(disposition.contains("attachment; filename="));
        assert!(disposition.contains("Rick Astley _ Demo.mp4"));
        assert!(disposition.contains("filename*=UTF-8''"));
    }
}
