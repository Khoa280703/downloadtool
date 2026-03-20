//! Background playlist orchestration engine.
//!
//! Spawns tokio tasks for the two playlist phases:
//! discovery (`FETCH IT`) and selected-item processing (`Start download`).

use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use futures::stream::{FuturesUnordered, StreamExt};
use job_system::{
    JobOwner, JobProgressPhase, JobProgressStore, MuxJobRequest, PlaylistItemStatus,
    PlaylistJobRecord, PlaylistJobRepository, PlaylistJobStatus,
};
use serde::Deserialize;
use tracing::{info, warn};

use super::job_control_plane::JobControlPlaneService;
use crate::limit_profiles::backend_limit_profile;

/// Recover orphaned playlist jobs after server restart.
/// Finds active (non-terminal) jobs and re-spawns processors for them.
pub async fn recover_orphaned_playlist_jobs(
    repo: Arc<PlaylistJobRepository>,
    job_control_plane: Arc<JobControlPlaneService>,
    job_progress_store: Arc<JobProgressStore>,
) {
    let jobs = match repo.find_active_jobs().await {
        Ok(jobs) => jobs,
        Err(err) => {
            warn!(err = %err, "Failed to query active playlist jobs for recovery");
            return;
        }
    };

    if jobs.is_empty() {
        info!("No orphaned playlist jobs to recover");
        return;
    }

    info!(count = jobs.len(), "Recovering orphaned playlist jobs");

    for job in jobs {
        match job.status {
            PlaylistJobStatus::Queued | PlaylistJobStatus::Discovering => {
                spawn_playlist_discovery(job, repo.clone(), job_control_plane.clone());
            }
            PlaylistJobStatus::Processing => {
                if let Err(err) = repo.reset_extracting_items_to_pending(&job.id).await {
                    warn!(job_id = %job.id, err = %err, "Failed to reset extracting items");
                }
                spawn_playlist_processor(
                    job,
                    repo.clone(),
                    job_control_plane.clone(),
                    job_progress_store.clone(),
                );
            }
            _ => {}
        }
    }
}

/// Spawn the discovery phase for a playlist job.
pub fn spawn_playlist_discovery(
    job: PlaylistJobRecord,
    repo: Arc<PlaylistJobRepository>,
    _job_control_plane: Arc<JobControlPlaneService>,
) {
    tokio::spawn(async move {
        if let Err(err) = run_playlist_discovery(&job, &repo).await {
            warn!(job_id = %job.id, err = %err, "Playlist discovery failed");
            let _ = repo
                .update_job_status(&job.id, PlaylistJobStatus::Failed)
                .await;
        }
    });
}

/// Spawn the selected-item processing phase for a discovered playlist job.
pub fn spawn_playlist_processor(
    job: PlaylistJobRecord,
    repo: Arc<PlaylistJobRepository>,
    job_control_plane: Arc<JobControlPlaneService>,
    job_progress_store: Arc<JobProgressStore>,
) {
    tokio::spawn(async move {
        if let Err(err) =
            run_playlist_processing(&job, &repo, &job_control_plane, &job_progress_store).await
        {
            warn!(job_id = %job.id, err = %err, "Playlist job failed");
            let _ = repo
                .update_job_status(&job.id, PlaylistJobStatus::Failed)
                .await;
        }
    });
}

async fn run_playlist_discovery(
    job: &PlaylistJobRecord,
    repo: &PlaylistJobRepository,
) -> anyhow::Result<()> {
    repo.update_job_status(&job.id, PlaylistJobStatus::Discovering)
        .await?;

    let items = discover_playlist_items(&job.id, &job.source_url, repo).await?;
    if items.is_empty() {
        repo.update_job_status(&job.id, PlaylistJobStatus::Ready)
            .await?;
        info!(job_id = %job.id, "Playlist discovery completed (0 items discovered)");
        return Ok(());
    }

    let discovered_total = items.len();
    info!(
        job_id = %job.id,
        discovered_total,
        "Playlist discovery finished and is waiting for selected items"
    );

    Ok(())
}

async fn run_playlist_processing(
    job: &PlaylistJobRecord,
    repo: &PlaylistJobRepository,
    job_control_plane: &JobControlPlaneService,
    job_progress_store: &JobProgressStore,
) -> anyhow::Result<()> {
    let owner = build_job_owner(job);
    let worker_count = backend_limit_profile().playlist_job_max_concurrent_items_value();
    let mut workers = FuturesUnordered::new();

    for _ in 0..worker_count {
        workers.push(process_playlist_worker_loop(
            job,
            &owner,
            repo,
            job_control_plane,
            job_progress_store,
        ));
    }

    while let Some(result) = workers.next().await {
        result?;
    }

    // Mark completed
    repo.update_job_status(&job.id, PlaylistJobStatus::Completed)
        .await?;

    let final_job = repo.get_job(&job.id, None, None).await?;
    if let Some(fj) = final_job {
        info!(
            job_id = %job.id,
            completed = fj.completed_items,
            failed = fj.failed_items,
            total = fj.total_items,
            "Playlist job finished"
        );
    }

    Ok(())
}

async fn process_playlist_worker_loop(
    job: &PlaylistJobRecord,
    owner: &JobOwner,
    repo: &PlaylistJobRepository,
    job_control_plane: &JobControlPlaneService,
    job_progress_store: &JobProgressStore,
) -> anyhow::Result<()> {
    loop {
        let current = repo.get_job(&job.id, None, None).await?;
        if let Some(ref playlist_job) = current {
            if playlist_job.status == PlaylistJobStatus::Cancelled {
                info!(job_id = %job.id, "Playlist job cancelled, stopping processor");
                return Ok(());
            }
        }

        let Some(item) = repo.claim_next_pending_item(&job.id).await? else {
            return Ok(());
        };

        let result =
            process_single_item(
                &item.id,
                &item.video_id,
                job,
                owner,
                repo,
                job_control_plane,
                job_progress_store,
            )
            .await;

        match result {
            Ok(()) => {
                let updated = repo.increment_completed(&job.id).await?;
                info!(
                    job_id = %job.id,
                    item_id = %item.id,
                    video_id = %item.video_id,
                    completed = updated.completed_items,
                    total = updated.total_items,
                    "Playlist item completed"
                );
            }
            Err(err) => {
                warn!(
                    job_id = %job.id,
                    item_id = %item.id,
                    video_id = %item.video_id,
                    err = %err,
                    "Playlist item failed"
                );
                repo.update_item_status(
                    &item.id,
                    PlaylistItemStatus::Failed,
                    Some(&err.to_string()),
                    None,
                    None,
                )
                .await?;
                let _ = repo.increment_failed(&job.id).await;
            }
        }

        tokio::time::sleep(Duration::from_millis(500 + rand_jitter(500))).await;
    }
}

/// Discover playlist items and insert into DB.
async fn discover_playlist_items(
    job_id: &str,
    source_url: &str,
    repo: &PlaylistJobRepository,
) -> anyhow::Result<Vec<(String, String, Option<String>, i32)>> {
    let playlist_id = extract_playlist_id(source_url)
        .context("Could not parse playlist ID from URL")?;
    let playlist_url = format!("https://www.youtube.com/playlist?list={playlist_id}");

    let raw = extractor::extract_playlist("youtube", &playlist_url)
        .await
        .map_err(|e| anyhow::anyhow!("Playlist extraction failed: {e}"))?;

    let mut entries: Vec<PlaylistVideoEntry> = serde_json::from_value(raw)
        .context("Invalid playlist payload")?;
    entries.sort_by_key(|e| e.index);

    let items: Vec<(String, String, Option<String>, i32)> = entries
        .into_iter()
        .enumerate()
        .map(|(i, e)| {
            let ordinal = if e.index == 0 { (i + 1) as i32 } else { e.index as i32 };
            (e.video_id, e.title, e.thumbnail, ordinal)
        })
        .collect();

    let total = items.len() as i32;
    repo.set_discovery_result(job_id, None, total).await?;
    repo.insert_items(job_id, &items).await?;

    info!(job_id, total, "Playlist discovery complete");
    Ok(items)
}

/// Process a single playlist item: extract → select stream → direct or mux.
async fn process_single_item(
    item_id: &str,
    video_id: &str,
    job: &PlaylistJobRecord,
    owner: &JobOwner,
    repo: &PlaylistJobRepository,
    job_control_plane: &JobControlPlaneService,
    job_progress_store: &JobProgressStore,
) -> anyhow::Result<()> {
    let watch_url = format!("https://www.youtube.com/watch?v={video_id}");

    // 1. Extract video info
    let info = extractor::extract(&watch_url)
        .await
        .map_err(|e| anyhow::anyhow!("Extract failed for {video_id}: {e}"))?;

    // 2. Pick best streams based on mode
    let (video, audio) =
        pick_best_streams(&info.formats, &job.requested_quality, &job.requested_mode);

    // 3. Audio-only mode: direct download, no mux
    if video.is_none() {
        if let Some(ref a) = audio {
            let download_url = build_stream_download_url(&a.url, &info.title, &a.ext);
            repo.update_item_status(
                item_id,
                PlaylistItemStatus::Ready,
                None,
                None,
                Some(&download_url),
            )
            .await?;
            return Ok(());
        }
        anyhow::bail!("No suitable audio stream found for {video_id}");
    }

    // 4. Build download URL
    if let Some(ref v) = video {
        if v.has_audio || audio.is_none() {
            // Direct combined or video-only stream
            let download_url = build_stream_download_url(&v.url, &info.title, &v.ext);
            repo.update_item_status(
                item_id,
                PlaylistItemStatus::Ready,
                None,
                None,
                Some(&download_url),
            )
            .await?;
            return Ok(());
        }
    }

    // 5. Need mux: video-only + audio
    let v = video.context("No suitable video stream found")?;
    let a = audio.context("No suitable audio stream found for mux")?;

    repo.update_item_status(item_id, PlaylistItemStatus::QueuedMux, None, None, None)
        .await?;

    let preferred_video_proxy = extractor::resolve_stream_proxy(&v.url).await;
    let preferred_audio_proxy = extractor::resolve_stream_proxy(&a.url).await;

    let mux_request = MuxJobRequest {
        video_url: v.url.clone(),
        audio_url: a.url.clone(),
        source_url: Some(info.original_url.clone()),
        video_format_id: Some(v.format_id.clone()),
        audio_format_id: Some(a.format_id.clone()),
        title: Some(info.title.clone()),
        preferred_video_proxy,
        preferred_audio_proxy,
    };

    let mux_result = job_control_plane.create_job(owner, mux_request).await?;

    repo.update_item_status(
        item_id,
        PlaylistItemStatus::Muxing,
        None,
        Some(&mux_result.job_id),
        None,
    )
    .await?;

    // Wait for mux job to finish
    let download_url =
        wait_for_mux_ready(&mux_result.job_id, owner, job_control_plane, job_progress_store)
            .await?;

    repo.update_item_status(
        item_id,
        PlaylistItemStatus::Ready,
        None,
        None,
        Some(&download_url),
    )
    .await?;

    Ok(())
}

/// Poll mux job until ready, return file-ticket download URL.
async fn wait_for_mux_ready(
    mux_job_id: &str,
    owner: &JobOwner,
    job_control_plane: &JobControlPlaneService,
    job_progress_store: &JobProgressStore,
) -> anyhow::Result<String> {
    let poll_interval = Duration::from_secs(3);
    let idle_timeout =
        Duration::from_secs(backend_limit_profile().playlist_mux_idle_timeout_secs_value());
    let mut last_activity = std::time::Instant::now();
    let mut last_progress_updated_at_ms: Option<u64> = None;
    let mut last_status_updated_at_ms: Option<u64> = None;

    loop {
        if let Ok(Some(snapshot)) = job_progress_store.read_snapshot(mux_job_id).await {
            let progress_updated_at_ms = snapshot.updated_at_ms.max(0) as u64;
            if last_progress_updated_at_ms != Some(progress_updated_at_ms) {
                last_progress_updated_at_ms = Some(progress_updated_at_ms);
                last_activity = std::time::Instant::now();
            }

            match snapshot.phase {
                JobProgressPhase::Ready => {
                    return Ok(format!("/api/jobs/{mux_job_id}/file-ticket"));
                }
                JobProgressPhase::Failed => {
                    anyhow::bail!("Mux job {mux_job_id} failed");
                }
                _ => {}
            }
        }

        let status = job_control_plane
            .get_job_for_user(mux_job_id, owner)
            .await?
            .context("Mux job disappeared")?;

        if last_status_updated_at_ms != Some(status.updated_at_ms) {
            last_status_updated_at_ms = Some(status.updated_at_ms);
            last_activity = std::time::Instant::now();
        }

        match status.status {
            job_system::JobStatus::Ready => {
                return Ok(format!("/api/jobs/{mux_job_id}/file-ticket"));
            }
            job_system::JobStatus::Failed => {
                anyhow::bail!(
                    "Mux job failed: {}",
                    status.error.unwrap_or_else(|| "unknown".into())
                );
            }
            job_system::JobStatus::Expired => {
                anyhow::bail!("Mux job expired");
            }
            _ => {
                if last_activity.elapsed() > idle_timeout {
                    anyhow::bail!(
                        "Mux job {mux_job_id} became idle for more than {} seconds",
                        idle_timeout.as_secs()
                    );
                }
                tokio::time::sleep(poll_interval).await;
            }
        }
    }
}

/// Stream selection based on quality and mode.
///
/// - `mode = "audio"`: returns `(None, best_audio)` — all formats allowed (WebM opus is often
///   highest quality; direct download, not mux).
/// - `mode = "video-only"`: returns `(best_video, None)` — all formats allowed (direct
///   download, not mux).
/// - `mode = "video"` (default): returns `(best_video, best_audio)` — WebM filtered out
///   because output goes through fMP4 muxer which rejects WebM containers.
fn pick_best_streams(
    formats: &[extractor::VideoFormat],
    quality: &str,
    mode: &str,
) -> (Option<extractor::VideoFormat>, Option<extractor::VideoFormat>) {
    // Audio-only mode: direct download — allow all formats including WebM
    if mode == "audio" {
        let best_audio = formats
            .iter()
            .filter(|f| f.is_audio_only)
            .max_by_key(|f| f.bitrate.unwrap_or(0))
            .cloned();
        return (None, best_audio);
    }

    let target_height: Option<u32> = match quality {
        "best" => None,
        "2160p" | "4k" => Some(2160),
        "1440p" => Some(1440),
        "1080p" => Some(1080),
        "720p" => Some(720),
        "480p" => Some(480),
        "360p" => Some(360),
        _ => quality.trim_end_matches('p').parse().ok(),
    };

    // Video-only mode: direct download — allow all formats including WebM
    if mode == "video-only" {
        let all_videos: Vec<&extractor::VideoFormat> =
            formats.iter().filter(|f| !f.is_audio_only).collect();

        let best_video = if let Some(target) = target_height {
            all_videos
                .iter()
                .filter(|f| f.height.unwrap_or(0) <= target)
                .max_by_key(|f| f.height.unwrap_or(0))
                .or_else(|| all_videos.iter().min_by_key(|f| f.height.unwrap_or(u32::MAX)))
                .cloned()
                .cloned()
        } else {
            all_videos
                .iter()
                .max_by_key(|f| f.height.unwrap_or(0))
                .cloned()
                .cloned()
        };
        return (best_video, None);
    }

    // Default "video" mode: filter out WebM — output goes through fMP4 muxer
    let mp4_videos: Vec<&extractor::VideoFormat> = formats
        .iter()
        .filter(|f| !f.is_audio_only && f.ext != "webm")
        .collect();

    let best_video = if let Some(target) = target_height {
        // Pick closest to target that doesn't exceed it, or closest above
        mp4_videos
            .iter()
            .filter(|f| f.height.unwrap_or(0) <= target)
            .max_by_key(|f| f.height.unwrap_or(0))
            .or_else(|| mp4_videos.iter().min_by_key(|f| f.height.unwrap_or(u32::MAX)))
            .cloned()
            .cloned()
    } else {
        // "best" — highest resolution
        mp4_videos
            .iter()
            .max_by_key(|f| f.height.unwrap_or(0))
            .cloned()
            .cloned()
    };

    let best_audio = formats
        .iter()
        .filter(|f| f.is_audio_only && f.ext != "webm")
        .max_by_key(|f| f.bitrate.unwrap_or(0))
        .cloned();

    (best_video, best_audio)
}

fn build_stream_download_url(stream_url: &str, title: &str, ext: &str) -> String {
    let encoded_url = urlencoding::encode(stream_url);
    let encoded_title = urlencoding::encode(title);
    let encoded_ext = urlencoding::encode(ext);
    format!("/api/stream?url={encoded_url}&title={encoded_title}&format={encoded_ext}")
}

fn build_job_owner(job: &PlaylistJobRecord) -> JobOwner {
    if let Some(ref uid) = job.user_id {
        JobOwner {
            user_id: Some(uid.clone()),
            session_id: None,
            scope_key: format!("user:{uid}"),
        }
    } else if let Some(ref sid) = job.session_id {
        JobOwner {
            user_id: None,
            session_id: Some(sid.clone()),
            scope_key: format!("session:{sid}"),
        }
    } else {
        JobOwner {
            user_id: None,
            session_id: None,
            scope_key: format!("ip:{}", job.request_ip.as_deref().unwrap_or("unknown")),
        }
    }
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
    if id.is_empty() { None } else { Some(id.to_string()) }
}

fn rand_jitter(max_ms: u64) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    hasher.finish() % max_ms
}

#[derive(Debug, Deserialize)]
struct PlaylistVideoEntry {
    #[serde(rename = "videoId")]
    video_id: String,
    title: String,
    thumbnail: Option<String>,
    index: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal VideoFormat for testing.
    fn make_video(ext: &str, height: u32, has_audio: bool) -> extractor::VideoFormat {
        extractor::VideoFormat {
            format_id: format!("{ext}-{height}"),
            quality: format!("{height}p"),
            vcodec: None,
            acodec: None,
            codec_label: None,
            has_audio,
            is_audio_only: false,
            width: None,
            height: Some(height),
            fps: None,
            bitrate: None,
            ext: ext.to_string(),
            url: format!("https://example.com/{ext}-{height}.{ext}"),
            filesize: None,
        }
    }

    fn make_audio(ext: &str, bitrate: u64) -> extractor::VideoFormat {
        extractor::VideoFormat {
            format_id: format!("{ext}-audio-{bitrate}"),
            quality: format!("{bitrate}kbps"),
            vcodec: None,
            acodec: None,
            codec_label: None,
            has_audio: true,
            is_audio_only: true,
            width: None,
            height: None,
            fps: None,
            bitrate: Some(bitrate),
            ext: ext.to_string(),
            url: format!("https://example.com/audio-{bitrate}.{ext}"),
            filesize: None,
        }
    }

    fn sample_formats() -> Vec<extractor::VideoFormat> {
        vec![
            make_video("mp4", 1080, false),
            make_video("mp4", 720, false),
            make_video("webm", 1080, false),
            make_audio("m4a", 128_000),
            make_audio("webm", 160_000), // WebM opus — higher bitrate
        ]
    }

    #[test]
    fn audio_mode_returns_best_audio_including_webm() {
        let formats = sample_formats();
        let (video, audio) = pick_best_streams(&formats, "best", "audio");
        assert!(video.is_none(), "audio mode must not return a video stream");
        let a = audio.expect("audio mode must return an audio stream");
        // WebM opus has higher bitrate — should be selected (no WebM filter in audio mode)
        assert_eq!(a.ext, "webm");
        assert_eq!(a.bitrate, Some(160_000));
    }

    #[test]
    fn video_only_mode_returns_best_video_no_audio_allows_webm() {
        let formats = sample_formats();
        let (video, audio) = pick_best_streams(&formats, "best", "video-only");
        assert!(audio.is_none(), "video-only mode must not return an audio stream");
        let v = video.expect("video-only mode must return a video stream");
        // Both mp4 and webm 1080p exist — pick highest resolution (webm allowed)
        assert_eq!(v.height, Some(1080));
    }

    #[test]
    fn video_mode_filters_webm_from_audio() {
        let formats = sample_formats();
        let (video, audio) = pick_best_streams(&formats, "best", "video");
        let v = video.expect("video mode must have video");
        let a = audio.expect("video mode must have audio");
        // WebM filtered out for mux compatibility
        assert_ne!(v.ext, "webm", "video stream must not be webm in video mode");
        assert_ne!(a.ext, "webm", "audio stream must not be webm in video mode");
        assert_eq!(v.ext, "mp4");
        assert_eq!(a.ext, "m4a");
    }

    #[test]
    fn video_mode_default_applies_quality_ceiling() {
        let formats = sample_formats();
        let (video, _) = pick_best_streams(&formats, "720p", "video");
        let v = video.expect("must select a video stream");
        assert_eq!(v.height, Some(720));
    }

    #[test]
    fn audio_mode_empty_formats_returns_none() {
        let (video, audio) = pick_best_streams(&[], "best", "audio");
        assert!(video.is_none());
        assert!(audio.is_none());
    }
}
