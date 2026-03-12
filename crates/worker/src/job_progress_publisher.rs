use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::sync::Arc;

use job_system::{repository::now_ms, JobProgressPhase, JobProgressSnapshot, JobProgressStore};
use reqwest::Url;
use tracing::warn;

const DEFAULT_PROGRESS_TTL_SECS: u64 = 2 * 60 * 60;
const MIN_PUBLISH_INTERVAL_MS: i64 = 300;
const MIN_PUBLISH_BYTES: u64 = 2 * 1024 * 1024;

#[derive(Clone)]
pub struct JobProgressPublisher {
    store: Arc<JobProgressStore>,
    job_id: String,
    total_bytes: Option<u64>,
    uploaded_bytes: Arc<AtomicU64>,
    last_published_at_ms: Arc<AtomicI64>,
    last_published_bytes: Arc<AtomicU64>,
    ttl_secs: u64,
}

impl JobProgressPublisher {
    pub fn new(
        store: Arc<JobProgressStore>,
        job_id: impl Into<String>,
        video_url: &str,
        audio_url: &str,
    ) -> Self {
        Self {
            store,
            job_id: job_id.into(),
            total_bytes: infer_total_bytes(video_url, audio_url),
            uploaded_bytes: Arc::new(AtomicU64::new(0)),
            last_published_at_ms: Arc::new(AtomicI64::new(0)),
            last_published_bytes: Arc::new(AtomicU64::new(0)),
            ttl_secs: DEFAULT_PROGRESS_TTL_SECS,
        }
    }

    pub async fn publish_phase(
        &self,
        phase: JobProgressPhase,
        percent_override: Option<f32>,
    ) -> anyhow::Result<()> {
        let uploaded_bytes = self.uploaded_bytes.load(Ordering::Relaxed);
        self.write_snapshot(phase, uploaded_bytes, percent_override)
            .await
    }

    pub fn job_id(&self) -> &str {
        &self.job_id
    }

    pub fn record_uploaded_bytes(&self, chunk_len: u64) {
        let uploaded_bytes =
            self.uploaded_bytes.fetch_add(chunk_len, Ordering::Relaxed) + chunk_len;
        if !self.should_publish(uploaded_bytes) {
            return;
        }

        self.last_published_bytes
            .store(uploaded_bytes, Ordering::Relaxed);
        self.last_published_at_ms.store(now_ms(), Ordering::Relaxed);

        let publisher = self.clone();
        tokio::spawn(async move {
            if let Err(error) = publisher
                .write_snapshot(JobProgressPhase::MuxingUploading, uploaded_bytes, None)
                .await
            {
                warn!(
                    job_id = publisher.job_id,
                    err = %error,
                    "Failed to publish mux progress snapshot"
                );
            }
        });
    }

    fn should_publish(&self, uploaded_bytes: u64) -> bool {
        let now = now_ms();
        let last_at = self.last_published_at_ms.load(Ordering::Relaxed);
        let last_bytes = self.last_published_bytes.load(Ordering::Relaxed);
        let bytes_delta = uploaded_bytes.saturating_sub(last_bytes);
        let time_delta = now.saturating_sub(last_at);

        last_at == 0 || bytes_delta >= MIN_PUBLISH_BYTES || time_delta >= MIN_PUBLISH_INTERVAL_MS
    }

    async fn write_snapshot(
        &self,
        phase: JobProgressPhase,
        uploaded_bytes: u64,
        percent_override: Option<f32>,
    ) -> anyhow::Result<()> {
        let snapshot = JobProgressSnapshot::new(
            self.job_id.clone(),
            phase,
            percent_override.or_else(|| self.estimated_percent(uploaded_bytes, phase)),
            uploaded_bytes,
            self.total_bytes,
        );
        self.store.write_snapshot(&snapshot, self.ttl_secs).await
    }

    fn estimated_percent(&self, uploaded_bytes: u64, phase: JobProgressPhase) -> Option<f32> {
        let total_bytes = self.total_bytes?;
        if total_bytes == 0 {
            return None;
        }

        let raw_percent = (uploaded_bytes as f64 / total_bytes as f64) * 100.0;
        let bounded = match phase {
            JobProgressPhase::Ready => 100.0,
            JobProgressPhase::CompletingUpload => raw_percent.clamp(0.0, 99.0).max(99.0),
            JobProgressPhase::Failed | JobProgressPhase::Retrying => raw_percent.clamp(0.0, 99.0),
            _ => raw_percent.clamp(0.0, 99.0),
        };
        Some(bounded as f32)
    }
}

fn infer_total_bytes(video_url: &str, audio_url: &str) -> Option<u64> {
    let video_bytes = parse_clen(video_url)?;
    let audio_bytes = parse_clen(audio_url)?;
    Some(video_bytes.saturating_add(audio_bytes))
}

fn parse_clen(url: &str) -> Option<u64> {
    let parsed = Url::parse(url).ok()?;
    parsed
        .query_pairs()
        .find_map(|(key, value)| (key == "clen").then(|| value.parse::<u64>().ok()).flatten())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infer_total_bytes_from_googlevideo_urls() {
        let video = "https://example.com/videoplayback?clen=2000&itag=401";
        let audio = "https://example.com/videoplayback?itag=140&clen=300";
        assert_eq!(infer_total_bytes(video, audio), Some(2300));
    }

    #[test]
    fn missing_clen_disables_total_estimate() {
        assert_eq!(
            infer_total_bytes("https://example.com/a", "https://example.com/b"),
            None
        );
    }
}
