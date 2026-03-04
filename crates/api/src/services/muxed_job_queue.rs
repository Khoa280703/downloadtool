use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Context};
use futures::StreamExt;
use muxer::stream_fetcher::{FetchBothRefreshOptions, StreamFetcher, StreamUrlRefreshContext};
use muxer::{remux_streams, MuxerError};
use proxy::anti_bot::AntiBotError;
use proxy::Platform;
use tokio::io::AsyncWriteExt;
use tokio::sync::{mpsc, Mutex, RwLock};
use tracing::{error, info, warn};

use crate::limit_profiles::backend_limit_profile;

const READY_WITHIN_5_MIN_MS: u64 = 5 * 60 * 1000;
const UNBOUNDED_QUEUE_CHANNEL_CAPACITY: usize = 1_000_000;

#[derive(Clone, Debug)]
pub struct MuxJobRequest {
    pub video_url: String,
    pub audio_url: String,
    pub source_url: Option<String>,
    pub video_format_id: Option<String>,
    pub audio_format_id: Option<String>,
    pub title: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MuxJobStatus {
    Queued,
    Processing,
    Ready,
    Failed,
}

impl MuxJobStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Processing => "processing",
            Self::Ready => "ready",
            Self::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug)]
pub struct MuxJobSnapshot {
    pub job_id: String,
    pub status: MuxJobStatus,
    pub error: Option<String>,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
    pub file_size_bytes: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct ReadyMuxedFile {
    pub file_path: PathBuf,
    pub file_size_bytes: u64,
    pub title: Option<String>,
}

#[derive(Debug)]
pub enum MuxJobQueueError {
    QueueFull,
    QueueOverloaded {
        retry_after_secs: u64,
        estimated_wait_secs: u64,
    },
    QueueUnavailable,
    NotFound,
    NotReady(MuxJobStatus),
}

pub struct MuxedJobQueue {
    inner: Arc<MuxedJobQueueInner>,
}

struct MuxedJobQueueInner {
    jobs: RwLock<HashMap<String, MuxJobEntry>>,
    sender: mpsc::Sender<String>,
    worker_count: usize,
    estimated_job_secs: u64,
    max_estimated_wait_secs: Option<u64>,
    output_dir: PathBuf,
    queue_capacity: Option<usize>,
    queue_channel_capacity: usize,
    job_timeout: Option<Duration>,
    temp_file_ttl: Option<Duration>,
    cleanup_interval: Option<Duration>,
    ttl: Option<Duration>,
    max_refresh_attempts: Option<usize>,
    id_counter: AtomicU64,
    last_cleanup_ms: AtomicU64,
}

#[derive(Clone)]
struct MuxJobEntry {
    request: MuxJobRequest,
    status: MuxJobStatus,
    error: Option<String>,
    created_at_ms: u64,
    updated_at_ms: u64,
    file_path: Option<PathBuf>,
    file_size_bytes: Option<u64>,
}

struct MuxJobResult {
    file_path: PathBuf,
    file_size_bytes: u64,
}

#[derive(Debug, Clone, Copy)]
struct QueueSnapshot {
    queued: usize,
    processing: usize,
    inflight: usize,
    estimated_wait_secs: u64,
}

impl MuxedJobQueue {
    pub fn from_env() -> anyhow::Result<Arc<Self>> {
        let limits = backend_limit_profile();
        let workers = limits.mux_job_max_workers_value().max(1);
        let queue_capacity = limits.mux_job_queue_capacity_value();
        let queue_channel_capacity = queue_capacity.unwrap_or(UNBOUNDED_QUEUE_CHANNEL_CAPACITY);
        let estimated_job_secs = limits.mux_job_estimated_runtime_secs_value().max(1);
        let max_estimated_wait_secs = limits.mux_job_max_estimated_wait_secs_value();
        let ttl = limits.mux_job_ttl_value();
        let job_timeout = limits.mux_job_timeout_value();
        let temp_file_ttl = limits.mux_job_temp_file_ttl_value();
        let cleanup_interval = limits.mux_job_cleanup_interval_value();
        let max_refresh_attempts = limits.mux_url_refresh_max_attempts_value();
        let output_dir = PathBuf::from(limits.mux_job_output_dir_value());

        std::fs::create_dir_all(&output_dir).with_context(|| {
            format!(
                "Failed to create mux job output directory at {}",
                output_dir.display()
            )
        })?;

        let (sender, receiver) = mpsc::channel::<String>(queue_channel_capacity);
        let inner = Arc::new(MuxedJobQueueInner {
            jobs: RwLock::new(HashMap::new()),
            sender,
            worker_count: workers,
            estimated_job_secs,
            max_estimated_wait_secs,
            output_dir,
            queue_capacity,
            queue_channel_capacity,
            job_timeout,
            temp_file_ttl,
            cleanup_interval,
            ttl,
            max_refresh_attempts,
            id_counter: AtomicU64::new(1),
            last_cleanup_ms: AtomicU64::new(0),
        });

        let queue = Arc::new(Self {
            inner: inner.clone(),
        });
        Self::spawn_workers(inner.clone(), receiver, workers);
        Self::spawn_cleanup(inner);

        info!(
            workers,
            queue_capacity = ?queue.inner.queue_capacity,
            queue_channel_capacity = queue.inner.queue_channel_capacity,
            estimated_job_secs,
            max_estimated_wait_secs = ?queue.inner.max_estimated_wait_secs,
            job_timeout_secs = ?queue.inner.job_timeout.map(|value| value.as_secs()),
            temp_file_ttl_secs = ?queue.inner.temp_file_ttl.map(|value| value.as_secs()),
            ttl_secs = ?queue.inner.ttl.map(|value| value.as_secs()),
            cleanup_interval_secs = ?queue.inner.cleanup_interval.map(|value| value.as_secs()),
            max_refresh_attempts = ?queue.inner.max_refresh_attempts,
            output_dir = %queue.inner.output_dir.display(),
            "Muxed async job queue initialized"
        );

        Ok(queue)
    }

    pub async fn enqueue(&self, request: MuxJobRequest) -> Result<String, MuxJobQueueError> {
        self.maybe_cleanup().await;

        let job_id = self.next_job_id();
        let now = unix_ms_now();
        let mut jobs = self.inner.jobs.write().await;
        let snapshot = self.queue_snapshot_from_jobs(&jobs);
        if let Some(queue_capacity) = self.inner.queue_capacity {
            if snapshot.inflight >= queue_capacity {
                return Err(MuxJobQueueError::QueueFull);
            }
        }
        if let Some(max_estimated_wait_secs) = self.inner.max_estimated_wait_secs {
            if snapshot.estimated_wait_secs > max_estimated_wait_secs {
                let retry_after_secs = snapshot.estimated_wait_secs.clamp(2, 120);
                warn!(
                    queued = snapshot.queued,
                    processing = snapshot.processing,
                    inflight = snapshot.inflight,
                    estimated_wait_secs = snapshot.estimated_wait_secs,
                    max_estimated_wait_secs,
                    retry_after_secs,
                    "Rejecting mux job due to estimated wait"
                );
                return Err(MuxJobQueueError::QueueOverloaded {
                    retry_after_secs,
                    estimated_wait_secs: snapshot.estimated_wait_secs,
                });
            }
        }
        let entry = MuxJobEntry {
            request,
            status: MuxJobStatus::Queued,
            error: None,
            created_at_ms: now,
            updated_at_ms: now,
            file_path: None,
            file_size_bytes: None,
        };
        jobs.insert(job_id.clone(), entry);
        drop(jobs);

        match self.inner.sender.try_send(job_id.clone()) {
            Ok(()) => {
                info!(
                    job_id,
                    queue_depth = snapshot.inflight + 1,
                    queued = snapshot.queued + 1,
                    processing = snapshot.processing,
                    estimated_wait_secs = snapshot.estimated_wait_secs,
                    "Enqueued mux job"
                );
                Ok(job_id)
            }
            Err(_) => {
                let mut jobs = self.inner.jobs.write().await;
                jobs.remove(&job_id);
                Err(MuxJobQueueError::QueueUnavailable)
            }
        }
    }

    pub async fn get_snapshot(&self, job_id: &str) -> Result<MuxJobSnapshot, MuxJobQueueError> {
        self.maybe_cleanup().await;
        let jobs = self.inner.jobs.read().await;
        let Some(entry) = jobs.get(job_id) else {
            return Err(MuxJobQueueError::NotFound);
        };

        Ok(MuxJobSnapshot {
            job_id: job_id.to_string(),
            status: entry.status.clone(),
            error: entry.error.clone(),
            created_at_ms: entry.created_at_ms,
            updated_at_ms: entry.updated_at_ms,
            file_size_bytes: entry.file_size_bytes,
        })
    }

    pub async fn get_ready_file(&self, job_id: &str) -> Result<ReadyMuxedFile, MuxJobQueueError> {
        self.maybe_cleanup().await;
        let jobs = self.inner.jobs.read().await;
        let Some(entry) = jobs.get(job_id) else {
            return Err(MuxJobQueueError::NotFound);
        };
        if entry.status != MuxJobStatus::Ready {
            return Err(MuxJobQueueError::NotReady(entry.status.clone()));
        }
        let Some(path) = entry.file_path.clone() else {
            return Err(MuxJobQueueError::NotReady(MuxJobStatus::Failed));
        };
        Ok(ReadyMuxedFile {
            file_path: path,
            file_size_bytes: entry.file_size_bytes.unwrap_or_default(),
            title: entry.request.title.clone(),
        })
    }

    fn next_job_id(&self) -> String {
        let seq = self.inner.id_counter.fetch_add(1, Ordering::Relaxed);
        format!("mux-{}-{}", unix_ms_now(), seq)
    }

    fn queue_snapshot_from_jobs(&self, jobs: &HashMap<String, MuxJobEntry>) -> QueueSnapshot {
        let mut queued = 0usize;
        let mut processing = 0usize;
        for entry in jobs.values() {
            match entry.status {
                MuxJobStatus::Queued => queued += 1,
                MuxJobStatus::Processing => processing += 1,
                _ => {}
            }
        }
        let inflight = queued + processing;
        let workers = self.inner.worker_count.max(1) as u64;
        let currently_busy_round = if processing >= self.inner.worker_count {
            1
        } else {
            0
        };
        let queued_rounds = (queued as u64).div_ceil(workers);
        let estimated_wait_secs =
            (currently_busy_round + queued_rounds).saturating_mul(self.inner.estimated_job_secs);

        QueueSnapshot {
            queued,
            processing,
            inflight,
            estimated_wait_secs,
        }
    }

    fn spawn_workers(
        inner: Arc<MuxedJobQueueInner>,
        receiver: mpsc::Receiver<String>,
        workers: usize,
    ) {
        let shared_receiver = Arc::new(Mutex::new(receiver));
        for worker_index in 0..workers {
            let worker_inner = inner.clone();
            let worker_receiver = shared_receiver.clone();
            tokio::spawn(async move {
                loop {
                    let job_id = {
                        let mut rx = worker_receiver.lock().await;
                        rx.recv().await
                    };
                    let Some(job_id) = job_id else {
                        info!(worker = worker_index, "Mux worker stopped (channel closed)");
                        break;
                    };
                    process_single_job(worker_inner.clone(), &job_id, worker_index).await;
                }
            });
        }
    }

    fn spawn_cleanup(inner: Arc<MuxedJobQueueInner>) {
        let Some(cleanup_interval) = inner.cleanup_interval else {
            return;
        };
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(cleanup_interval).await;
                cleanup_expired_jobs(inner.clone()).await;
            }
        });
    }

    async fn maybe_cleanup(&self) {
        let Some(cleanup_interval) = self.inner.cleanup_interval else {
            return;
        };
        let now = unix_ms_now();
        let interval_ms = cleanup_interval.as_millis() as u64;
        let should_run = self
            .inner
            .last_cleanup_ms
            .fetch_update(Ordering::AcqRel, Ordering::Relaxed, |last| {
                if now.saturating_sub(last) >= interval_ms {
                    Some(now)
                } else {
                    None
                }
            })
            .is_ok();
        if should_run {
            cleanup_expired_jobs(self.inner.clone()).await;
        }
    }
}

async fn process_single_job(inner: Arc<MuxedJobQueueInner>, job_id: &str, worker_index: usize) {
    let (request, created_at_ms) = {
        let mut jobs = inner.jobs.write().await;
        let Some(entry) = jobs.get_mut(job_id) else {
            return;
        };
        entry.status = MuxJobStatus::Processing;
        entry.updated_at_ms = unix_ms_now();
        (entry.request.clone(), entry.created_at_ms)
    };

    info!(worker = worker_index, job_id, "Started mux job processing");

    let result = if let Some(job_timeout) = inner.job_timeout {
        match tokio::time::timeout(
            job_timeout,
            execute_mux_job(
                job_id,
                &request,
                inner.output_dir.clone(),
                inner.max_refresh_attempts,
            ),
        )
        .await
        {
            Ok(result) => Ok(result),
            Err(_) => Err(job_timeout),
        }
    } else {
        Ok(execute_mux_job(
            job_id,
            &request,
            inner.output_dir.clone(),
            inner.max_refresh_attempts,
        )
        .await)
    };

    match result {
        Ok(Ok(job_result)) => {
            let finished_at_ms = unix_ms_now();
            let job_age_ms = finished_at_ms.saturating_sub(created_at_ms);
            let ready_within_5m = job_age_ms <= READY_WITHIN_5_MIN_MS;
            let mut jobs = inner.jobs.write().await;
            if let Some(entry) = jobs.get_mut(job_id) {
                entry.status = MuxJobStatus::Ready;
                entry.error = None;
                entry.updated_at_ms = finished_at_ms;
                entry.file_path = Some(job_result.file_path);
                entry.file_size_bytes = Some(job_result.file_size_bytes);
            }
            info!(
                worker = worker_index,
                job_id,
                job_age_ms,
                ready_within_5m,
                eventual_ready = true,
                "Mux job completed"
            );
        }
        Ok(Err(error)) => {
            let finished_at_ms = unix_ms_now();
            let job_age_ms = finished_at_ms.saturating_sub(created_at_ms);
            let cleanup_deleted_bytes = cleanup_incomplete_job_files(inner.clone(), job_id).await;
            let mut jobs = inner.jobs.write().await;
            if let Some(entry) = jobs.get_mut(job_id) {
                entry.status = MuxJobStatus::Failed;
                entry.error = Some(error.to_string());
                entry.updated_at_ms = finished_at_ms;
                entry.file_path = None;
                entry.file_size_bytes = None;
            }
            error!(
                worker = worker_index,
                job_id,
                job_age_ms,
                cleanup_deleted_bytes,
                err = %error,
                "Mux job failed"
            );
        }
        Err(job_timeout) => {
            let finished_at_ms = unix_ms_now();
            let job_age_ms = finished_at_ms.saturating_sub(created_at_ms);
            let cleanup_deleted_bytes = cleanup_incomplete_job_files(inner.clone(), job_id).await;
            let mut jobs = inner.jobs.write().await;
            if let Some(entry) = jobs.get_mut(job_id) {
                entry.status = MuxJobStatus::Failed;
                entry.error = Some(format!(
                    "Mux job timed out after {} seconds",
                    job_timeout.as_secs()
                ));
                entry.updated_at_ms = finished_at_ms;
                entry.file_path = None;
                entry.file_size_bytes = None;
            }
            error!(
                worker = worker_index,
                job_id,
                job_age_ms,
                cleanup_deleted_bytes,
                timeout_secs = job_timeout.as_secs(),
                "Mux job timed out"
            );
        }
    }
}

async fn cleanup_expired_jobs(inner: Arc<MuxedJobQueueInner>) {
    let now = unix_ms_now();
    let ttl_ms = inner.ttl.map(|ttl| ttl.as_millis() as u64);
    let mut expired = Vec::new();
    let mut referenced_files = HashSet::new();

    {
        let mut jobs = inner.jobs.write().await;
        jobs.retain(|job_id, entry| {
            let terminal =
                entry.status == MuxJobStatus::Ready || entry.status == MuxJobStatus::Failed;
            let stale = ttl_ms
                .map(|ttl_ms| now.saturating_sub(entry.updated_at_ms) >= ttl_ms)
                .unwrap_or(false);
            if terminal && stale {
                if let Some(path) = entry.file_path.clone() {
                    expired.push((job_id.clone(), path));
                }
                return false;
            }
            if entry.status == MuxJobStatus::Queued || entry.status == MuxJobStatus::Processing {
                referenced_files.insert(format!("{job_id}.part"));
            }
            if let Some(path) = entry.file_path.as_ref() {
                if let Some(name) = path.file_name().and_then(|v| v.to_str()) {
                    referenced_files.insert(name.to_string());
                }
            }
            true
        });
    }

    for (job_id, file_path) in expired {
        if let Err(err) = tokio::fs::remove_file(&file_path).await {
            if err.kind() != std::io::ErrorKind::NotFound {
                warn!(
                    job_id,
                    path = %file_path.display(),
                    err = %err,
                    "Failed to remove expired muxed output file"
                );
            }
        } else {
            info!(job_id, path = %file_path.display(), "Removed expired muxed output file");
        }
    }

    if let Some(temp_file_ttl) = inner.temp_file_ttl {
        let stale_deleted_bytes =
            cleanup_stale_mux_output_files(inner.clone(), referenced_files, temp_file_ttl).await;
        if stale_deleted_bytes > 0 {
            info!(
                deleted_bytes = stale_deleted_bytes,
                "Removed stale mux temporary files"
            );
        }
    }
}

async fn execute_mux_job(
    job_id: &str,
    request: &MuxJobRequest,
    output_dir: PathBuf,
    max_refresh_attempts: Option<usize>,
) -> anyhow::Result<MuxJobResult> {
    let platform = Platform::YouTube;
    let mut video_url = request.video_url.clone();
    let mut audio_url = request.audio_url.clone();
    let mut refresh_attempts = 0usize;
    let fetch_refresh_options = build_fetch_refresh_options(request, max_refresh_attempts);

    let part_path = output_dir.join(format!("{job_id}.part"));
    let output_path = output_dir.join(format!("{job_id}.mp4"));
    let _ = tokio::fs::remove_file(&part_path).await;
    let _ = tokio::fs::remove_file(&output_path).await;

    loop {
        let (video_stream, audio_stream) = match StreamFetcher::fetch_both_with_refresh(
            &video_url,
            &audio_url,
            platform,
            fetch_refresh_options.clone(),
        )
        .await
        {
            Ok(streams) => streams,
            Err(error) => {
                if is_auth_like_antibot_error(&error) {
                    warn!(
                        job_id,
                        refresh_attempts,
                        max_refresh_attempts = ?max_refresh_attempts,
                        "Upstream auth-like error (401/403) while fetching mux streams"
                    );
                }
                if within_retry_limit(refresh_attempts, max_refresh_attempts)
                    && is_auth_like_antibot_error(&error)
                    && request.source_url.is_some()
                {
                    if let Some((next_video, next_audio)) =
                        refresh_both_urls(request, &video_url, &audio_url).await
                    {
                        refresh_attempts += 1;
                        info!(
                            job_id,
                            attempt = refresh_attempts,
                            max_refresh_attempts = ?max_refresh_attempts,
                            "Refreshed mux job URLs after fetch auth failure"
                        );
                        video_url = next_video;
                        audio_url = next_audio;
                        continue;
                    }
                }
                return Err(anyhow!("Failed to fetch streams: {error}"));
            }
        };

        let mut output = tokio::fs::File::create(&part_path)
            .await
            .with_context(|| format!("Failed to create mux temp file {}", part_path.display()))?;
        let mut written = 0u64;
        let mut should_refresh = false;
        let mut muxed = remux_streams(video_stream, audio_stream);

        while let Some(item) = muxed.next().await {
            match item {
                Ok(chunk) => {
                    output
                        .write_all(&chunk)
                        .await
                        .context("Failed to write muxed bytes to temp file")?;
                    written += chunk.len() as u64;
                }
                Err(error) => {
                    if is_auth_like_muxer_error(&error) {
                        warn!(
                            job_id,
                            refresh_attempts,
                            max_refresh_attempts = ?max_refresh_attempts,
                            "Upstream auth-like error (401/403) during muxing"
                        );
                    }
                    if within_retry_limit(refresh_attempts, max_refresh_attempts)
                        && is_auth_like_muxer_error(&error)
                        && request.source_url.is_some()
                    {
                        should_refresh = true;
                        break;
                    }
                    return Err(anyhow!("Muxing error: {error}"));
                }
            }
        }

        if should_refresh {
            drop(output);
            let _ = tokio::fs::remove_file(&part_path).await;
            if let Some((next_video, next_audio)) =
                refresh_both_urls(request, &video_url, &audio_url).await
            {
                refresh_attempts += 1;
                info!(
                    job_id,
                    attempt = refresh_attempts,
                    max_refresh_attempts = ?max_refresh_attempts,
                    "Refreshed mux job URLs after mux auth failure"
                );
                video_url = next_video;
                audio_url = next_audio;
                continue;
            }
            return Err(anyhow!("Muxing auth error and URL refresh failed"));
        }

        output
            .flush()
            .await
            .context("Failed to flush mux output file")?;
        drop(output);

        if written == 0 {
            let _ = tokio::fs::remove_file(&part_path).await;
            return Err(anyhow!("Muxed output is empty"));
        }

        tokio::fs::rename(&part_path, &output_path)
            .await
            .with_context(|| {
                format!(
                    "Failed to finalize muxed output file {}",
                    output_path.display()
                )
            })?;

        return Ok(MuxJobResult {
            file_path: output_path,
            file_size_bytes: written,
        });
    }
}

fn build_fetch_refresh_options(
    request: &MuxJobRequest,
    max_refresh_attempts: Option<usize>,
) -> FetchBothRefreshOptions {
    let Some(source_url) = request.source_url.clone() else {
        return FetchBothRefreshOptions::default();
    };
    let max_refresh_attempts = max_refresh_attempts_for_context(max_refresh_attempts);

    let video = StreamUrlRefreshContext {
        source_url: source_url.clone(),
        format_id: request.video_format_id.clone(),
        expected_audio_only: Some(false),
        expected_has_audio: Some(false),
        expected_ext: None,
        max_refresh_attempts,
    };
    let audio = StreamUrlRefreshContext {
        source_url,
        format_id: request.audio_format_id.clone(),
        expected_audio_only: Some(true),
        expected_has_audio: Some(true),
        expected_ext: None,
        max_refresh_attempts,
    };
    FetchBothRefreshOptions {
        video: Some(video),
        audio: Some(audio),
    }
}

fn within_retry_limit(attempts: usize, max_attempts: Option<usize>) -> bool {
    max_attempts
        .map(|max_attempts| attempts < max_attempts)
        .unwrap_or(true)
}

fn max_refresh_attempts_for_context(max_attempts: Option<usize>) -> usize {
    max_attempts.unwrap_or(usize::MAX)
}

async fn refresh_both_urls(
    request: &MuxJobRequest,
    fallback_video_url: &str,
    fallback_audio_url: &str,
) -> Option<(String, String)> {
    let source_url = request.source_url.as_deref()?;
    let refreshed = extractor::extract(source_url).await.ok()?;
    let next_video = find_refreshed_format_url(
        &refreshed.formats,
        request.video_format_id.as_deref(),
        fallback_video_url,
        Some(false),
        Some(false),
    )?;
    let next_audio = find_refreshed_format_url(
        &refreshed.formats,
        request.audio_format_id.as_deref(),
        fallback_audio_url,
        Some(true),
        Some(true),
    )?;
    Some((next_video, next_audio))
}

fn find_refreshed_format_url(
    formats: &[extractor::VideoFormat],
    format_id: Option<&str>,
    fallback_url: &str,
    expected_audio_only: Option<bool>,
    expected_has_audio: Option<bool>,
) -> Option<String> {
    if let Some(id) = format_id {
        if let Some(found) = formats.iter().find(|f| f.format_id == id) {
            return Some(found.url.clone());
        }
    }

    let fallback_ext = reqwest::Url::parse(fallback_url)
        .ok()
        .and_then(|url| {
            url.query_pairs()
                .find(|(k, _)| k == "mime")
                .map(|(_, v)| v.to_string())
        })
        .and_then(|mime| mime.split('/').nth(1).map(|v| v.to_lowercase()));

    formats.iter().find_map(|format| {
        if let Some(audio_only) = expected_audio_only {
            if format.is_audio_only != audio_only {
                return None;
            }
        }
        if let Some(has_audio) = expected_has_audio {
            if format.has_audio != has_audio {
                return None;
            }
        }
        if let Some(ext) = fallback_ext.as_deref() {
            if !format.ext.eq_ignore_ascii_case(ext) {
                return None;
            }
        }
        Some(format.url.clone())
    })
}

fn is_auth_like_muxer_error(error: &MuxerError) -> bool {
    is_auth_like_error_message(&error.to_string())
}

fn is_auth_like_antibot_error(error: &AntiBotError) -> bool {
    match error {
        AntiBotError::RequestFailed(request_error) => {
            request_error
                .status()
                .map(is_upstream_auth_status)
                .unwrap_or(false)
                || is_auth_like_error_message(&request_error.to_string())
        }
        _ => is_auth_like_error_message(&error.to_string()),
    }
}

fn is_upstream_auth_status(status: reqwest::StatusCode) -> bool {
    status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN
}

fn is_auth_like_error_message(message: &str) -> bool {
    let normalized = message.to_ascii_lowercase();
    normalized.contains("401 unauthorized")
        || normalized.contains("403 forbidden")
        || normalized.contains("status client error (401")
        || normalized.contains("status client error (403")
        || normalized.contains("http status client error (401")
        || normalized.contains("http status client error (403")
}

fn unix_ms_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

async fn cleanup_incomplete_job_files(inner: Arc<MuxedJobQueueInner>, job_id: &str) -> u64 {
    let part_path = inner.output_dir.join(format!("{job_id}.part"));
    let output_path = inner.output_dir.join(format!("{job_id}.mp4"));
    remove_files_and_count_bytes([part_path, output_path]).await
}

async fn cleanup_stale_mux_output_files(
    inner: Arc<MuxedJobQueueInner>,
    referenced_files: HashSet<String>,
    max_age: Duration,
) -> u64 {
    let mut deleted_bytes = 0u64;
    let now = SystemTime::now();
    let mut dir = match tokio::fs::read_dir(&inner.output_dir).await {
        Ok(dir) => dir,
        Err(error) => {
            warn!(
                path = %inner.output_dir.display(),
                err = %error,
                "Failed to scan mux output directory for stale files"
            );
            return 0;
        }
    };

    while let Ok(Some(entry)) = dir.next_entry().await {
        let path = entry.path();
        let Some(file_name) = path.file_name().and_then(|v| v.to_str()) else {
            continue;
        };
        let Some(ext) = path.extension().and_then(|v| v.to_str()) else {
            continue;
        };
        if ext != "part" && ext != "mp4" {
            continue;
        }
        if referenced_files.contains(file_name) {
            continue;
        }
        let metadata = match entry.metadata().await {
            Ok(meta) => meta,
            Err(_) => continue,
        };
        let modified = metadata.modified().unwrap_or(UNIX_EPOCH);
        let age = now.duration_since(modified).unwrap_or_default();
        if age < max_age {
            continue;
        }
        let bytes = metadata.len();
        match tokio::fs::remove_file(&path).await {
            Ok(()) => {
                deleted_bytes = deleted_bytes.saturating_add(bytes);
                info!(
                    path = %path.display(),
                    bytes,
                    file_age_secs = age.as_secs(),
                    "Removed stale mux artifact"
                );
            }
            Err(error) => {
                if error.kind() != std::io::ErrorKind::NotFound {
                    warn!(
                        path = %path.display(),
                        err = %error,
                        "Failed to remove stale mux artifact"
                    );
                }
            }
        }
    }

    deleted_bytes
}

async fn remove_files_and_count_bytes<const N: usize>(paths: [PathBuf; N]) -> u64 {
    let mut deleted_bytes = 0u64;
    for path in paths {
        let bytes = match tokio::fs::metadata(&path).await {
            Ok(meta) => meta.len(),
            Err(_) => 0,
        };
        match tokio::fs::remove_file(&path).await {
            Ok(()) => {
                deleted_bytes = deleted_bytes.saturating_add(bytes);
            }
            Err(error) => {
                if error.kind() != std::io::ErrorKind::NotFound {
                    warn!(path = %path.display(), err = %error, "Failed to remove mux artifact");
                }
            }
        }
    }
    deleted_bytes
}
