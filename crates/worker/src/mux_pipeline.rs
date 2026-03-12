use std::sync::Arc;
use std::time::Instant;

use anyhow::{anyhow, Context, Result};
use futures::TryStreamExt;
use job_system::{JobProgressPhase, MuxJobRequest};
use muxer::stream_fetcher::{FetchBothRefreshOptions, StreamFetcher, StreamUrlRefreshContext};
use muxer::{remux_streams, MuxerError};
use object_store::{StorageBackend, StoredArtifact, UploadStream};
use proxy::anti_bot::AntiBotError;
use proxy::Platform;
use tracing::{info, warn};

use crate::job_progress_publisher::JobProgressPublisher;

#[derive(Debug, Clone, Default)]
pub struct MuxProxyBinding {
    pub video_proxy: Option<String>,
    pub audio_proxy: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ResolvedMuxJobSources {
    pub video_url: String,
    pub audio_url: String,
    pub proxy_binding: MuxProxyBinding,
    pub resolution_strategy: &'static str,
    pub fallback_reason: Option<&'static str>,
}

pub async fn resolve_mux_job_sources(request: &MuxJobRequest) -> ResolvedMuxJobSources {
    let (video_url, audio_url, resolution_strategy, fallback_reason) =
        match resolve_fresh_mux_urls(request).await {
            Ok((video_url, audio_url)) => (video_url, audio_url, "late_extract", None),
            Err(reason) => (
                request.video_url.clone(),
                request.audio_url.clone(),
                "stored_urls_fallback",
                Some(reason),
            ),
        };
    let proxy_binding = resolve_mux_proxy_binding_for_urls(&video_url, &audio_url).await;
    ResolvedMuxJobSources {
        video_url,
        audio_url,
        proxy_binding,
        resolution_strategy,
        fallback_reason,
    }
}

pub async fn upload_muxed_artifact(
    job_id: &str,
    request: &MuxJobRequest,
    storage: Arc<dyn StorageBackend>,
    artifact_key: &str,
    resolved: &ResolvedMuxJobSources,
    progress: &JobProgressPublisher,
) -> Result<StoredArtifact> {
    let mut video_url = resolved.video_url.clone();
    let mut audio_url = resolved.audio_url.clone();
    let mut video_proxy = resolved.proxy_binding.video_proxy.clone();
    let mut audio_proxy = resolved.proxy_binding.audio_proxy.clone();
    let refresh_options = build_refresh_options(request);
    let started_at = Instant::now();

    info!(
        job_id,
        artifact_key,
        source_url = request.source_url.as_deref().unwrap_or(""),
        video_format_id = request.video_format_id.as_deref().unwrap_or(""),
        audio_format_id = request.audio_format_id.as_deref().unwrap_or(""),
        video_proxy = sanitize_proxy_for_log(video_proxy.as_deref()),
        audio_proxy = sanitize_proxy_for_log(audio_proxy.as_deref()),
        refresh_attempts = refresh_options
            .video
            .as_ref()
            .map(|value| value.max_refresh_attempts)
            .unwrap_or(0),
        "Starting mux upload pipeline"
    );

    for attempt in 0..=refresh_options
        .video
        .as_ref()
        .map(|value| value.max_refresh_attempts)
        .unwrap_or(0)
    {
        if let Err(error) = progress
            .publish_phase(JobProgressPhase::FetchingStreams, None)
            .await
        {
            warn!(job_id, err = %error, "Failed to publish fetching phase progress");
        }
        let fetch_started_at = Instant::now();
        let (video_stream, audio_stream) = StreamFetcher::fetch_both_with_refresh_and_proxy(
            &video_url,
            &audio_url,
            Platform::YouTube,
            refresh_options.clone(),
            video_proxy.clone(),
            audio_proxy.clone(),
        )
        .await
        .with_context(|| format!("failed to fetch streams for job {job_id}"))?;
        info!(
            job_id,
            artifact_key,
            attempt,
            video_proxy = sanitize_proxy_for_log(video_proxy.as_deref()),
            audio_proxy = sanitize_proxy_for_log(audio_proxy.as_deref()),
            elapsed_ms = fetch_started_at.elapsed().as_millis() as u64,
            total_elapsed_ms = started_at.elapsed().as_millis() as u64,
            "Fetched source streams for mux pipeline"
        );
        if let Err(error) = progress
            .publish_phase(JobProgressPhase::MuxingUploading, None)
            .await
        {
            warn!(job_id, err = %error, "Failed to publish upload phase progress");
        }

        let upload_progress = progress.clone();
        let muxed = remux_streams(video_stream, audio_stream).inspect_ok(move |bytes| {
            upload_progress.record_uploaded_bytes(bytes.len() as u64);
        });
        let upload_stream: UploadStream = Box::pin(muxed.map_err(anyhow::Error::new));
        let store_started_at = Instant::now();
        info!(
            job_id,
            artifact_key, attempt, "Storing muxed artifact to storage backend"
        );

        match storage
            .store_stream(artifact_key, "video/mp4", upload_stream)
            .await
        {
            Ok(stored) => {
                if let Err(error) = progress
                    .publish_phase(JobProgressPhase::CompletingUpload, Some(99.0))
                    .await
                {
                    warn!(job_id, err = %error, "Failed to publish completion phase progress");
                }
                info!(
                    job_id,
                    artifact_key,
                    attempt,
                    backend = stored.backend,
                    size_bytes = stored.size_bytes,
                    elapsed_ms = store_started_at.elapsed().as_millis() as u64,
                    total_elapsed_ms = started_at.elapsed().as_millis() as u64,
                    "Stored muxed artifact successfully"
                );
                return Ok(stored);
            }
            Err(error) if is_auth_like_error(&error) && request.source_url.is_some() => {
                if let Some((next_video, next_audio)) =
                    refresh_both_urls(request, &video_url, &audio_url).await
                {
                    warn!(
                        job_id,
                        artifact_key,
                        attempt,
                        video_proxy = sanitize_proxy_for_log(video_proxy.as_deref()),
                        audio_proxy = sanitize_proxy_for_log(audio_proxy.as_deref()),
                        err = %error,
                        err_chain = %format!("{error:#}"),
                        elapsed_ms = store_started_at.elapsed().as_millis() as u64,
                        "Refreshing mux URLs after auth-like storage pipeline error"
                    );
                    video_url = next_video;
                    audio_url = next_audio;
                    let refreshed =
                        resolve_mux_proxy_binding_for_urls(&video_url, &audio_url).await;
                    video_proxy = refreshed.video_proxy;
                    audio_proxy = refreshed.audio_proxy;
                    info!(
                        job_id,
                        artifact_key,
                        attempt,
                        video_proxy = sanitize_proxy_for_log(video_proxy.as_deref()),
                        audio_proxy = sanitize_proxy_for_log(audio_proxy.as_deref()),
                        "Resolved refreshed mux proxy binding"
                    );
                    continue;
                }
                return Err(error).context("mux upload auth refresh failed");
            }
            Err(error) => {
                warn!(
                    job_id,
                    artifact_key,
                    attempt,
                    video_proxy = sanitize_proxy_for_log(video_proxy.as_deref()),
                    audio_proxy = sanitize_proxy_for_log(audio_proxy.as_deref()),
                    err = %error,
                    err_chain = %format!("{error:#}"),
                    elapsed_ms = store_started_at.elapsed().as_millis() as u64,
                    total_elapsed_ms = started_at.elapsed().as_millis() as u64,
                    "Failed to store muxed artifact in storage backend"
                );
                return Err(error).context("failed to store muxed artifact");
            }
        }
    }

    Err(anyhow!("mux upload exhausted refresh attempts"))
}

fn build_refresh_options(request: &MuxJobRequest) -> FetchBothRefreshOptions {
    let Some(source_url) = request.source_url.clone() else {
        return FetchBothRefreshOptions::default();
    };

    FetchBothRefreshOptions {
        video: Some(StreamUrlRefreshContext {
            source_url: source_url.clone(),
            format_id: request.video_format_id.clone(),
            expected_audio_only: Some(false),
            expected_has_audio: Some(false),
            expected_ext: Some("mp4".to_string()),
            max_refresh_attempts: 2,
        }),
        audio: Some(StreamUrlRefreshContext {
            source_url,
            format_id: request.audio_format_id.clone(),
            expected_audio_only: Some(true),
            expected_has_audio: Some(true),
            expected_ext: Some("m4a".to_string()),
            max_refresh_attempts: 2,
        }),
    }
}

async fn refresh_both_urls(
    request: &MuxJobRequest,
    current_video_url: &str,
    current_audio_url: &str,
) -> Option<(String, String)> {
    let source_url = request.source_url.as_deref()?;
    let info = extractor::extract(source_url).await.ok()?;
    let next_video = find_format_url(
        &info.formats,
        request.video_format_id.as_deref(),
        current_video_url,
        false,
    )?;
    let next_audio = find_format_url(
        &info.formats,
        request.audio_format_id.as_deref(),
        current_audio_url,
        true,
    )?;
    info!(job_source = source_url, "Refreshed mux URLs from extractor");
    Some((next_video, next_audio))
}

async fn resolve_fresh_mux_urls(request: &MuxJobRequest) -> Result<(String, String), &'static str> {
    if request.source_url.is_none() {
        return Err("missing_source_url");
    }
    if request.video_format_id.is_none() {
        return Err("missing_video_format_id");
    }
    if request.audio_format_id.is_none() {
        return Err("missing_audio_format_id");
    }

    let video_url = request.video_url.clone();
    let audio_url = request.audio_url.clone();
    refresh_both_urls(request, &video_url, &audio_url)
        .await
        .ok_or("late_extract_failed")
}

async fn resolve_mux_proxy_binding_for_urls(video_url: &str, audio_url: &str) -> MuxProxyBinding {
    MuxProxyBinding {
        video_proxy: extractor::resolve_stream_proxy(video_url).await,
        audio_proxy: extractor::resolve_stream_proxy(audio_url).await,
    }
}

fn find_format_url(
    formats: &[extractor::VideoFormat],
    format_id: Option<&str>,
    fallback_url: &str,
    audio_only: bool,
) -> Option<String> {
    if let Some(format_id) = format_id {
        if let Some(format) = formats.iter().find(|format| format.format_id == format_id) {
            return Some(format.url.clone());
        }
    }

    let fallback_ext = reqwest::Url::parse(fallback_url)
        .ok()
        .and_then(|url| {
            url.query_pairs()
                .find(|(key, _)| key == "mime")
                .map(|(_, value)| value.to_string())
        })
        .and_then(|mime| mime.split('/').nth(1).map(ToString::to_string));

    formats.iter().find_map(|format| {
        if format.is_audio_only != audio_only {
            return None;
        }
        if let Some(ext) = fallback_ext.as_deref() {
            if !format.ext.eq_ignore_ascii_case(ext) {
                return None;
            }
        }
        Some(format.url.clone())
    })
}

fn is_auth_like_error(error: &anyhow::Error) -> bool {
    let text = error.to_string().to_ascii_lowercase();
    text.contains("401")
        || text.contains("403")
        || text.contains("forbidden")
        || text.contains("unauthorized")
}

fn sanitize_proxy_for_log(proxy: Option<&str>) -> String {
    let Some(raw) = proxy else {
        return String::new();
    };

    let Ok(mut parsed) = reqwest::Url::parse(raw) else {
        return mask_proxy_credential_segment(raw);
    };

    let has_credentials = !parsed.username().is_empty() || parsed.password().is_some();
    if has_credentials {
        let _ = parsed.set_username("***");
        let _ = parsed.set_password(Some("***"));
    }

    parsed.to_string()
}

fn mask_proxy_credential_segment(raw: &str) -> String {
    let Some((prefix, suffix)) = raw.rsplit_once('@') else {
        return raw.to_string();
    };

    let scheme = prefix
        .split_once("://")
        .map(|(value, _)| value)
        .unwrap_or("proxy");
    format!("{scheme}://***:***@{suffix}")
}

#[allow(dead_code)]
fn _is_auth_like_muxer_error(error: &MuxerError) -> bool {
    is_auth_like_error(&anyhow!(error.to_string()))
}

#[allow(dead_code)]
fn _is_auth_like_antibot_error(error: &AntiBotError) -> bool {
    is_auth_like_error(&anyhow!(error.to_string()))
}
