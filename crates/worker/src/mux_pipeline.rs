use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use futures::TryStreamExt;
use job_system::MuxJobRequest;
use muxer::stream_fetcher::{FetchBothRefreshOptions, StreamFetcher, StreamUrlRefreshContext};
use muxer::{remux_streams, MuxerError};
use object_store::{StorageBackend, StoredArtifact, UploadStream};
use proxy::anti_bot::AntiBotError;
use proxy::Platform;
use tracing::{info, warn};

pub async fn upload_muxed_artifact(
    job_id: &str,
    request: &MuxJobRequest,
    storage: Arc<dyn StorageBackend>,
    artifact_key: &str,
) -> Result<StoredArtifact> {
    let mut video_url = request.video_url.clone();
    let mut audio_url = request.audio_url.clone();
    let refresh_options = build_refresh_options(request);

    for attempt in 0..=refresh_options
        .video
        .as_ref()
        .map(|value| value.max_refresh_attempts)
        .unwrap_or(0)
    {
        let (video_stream, audio_stream) = StreamFetcher::fetch_both_with_refresh(
            &video_url,
            &audio_url,
            Platform::YouTube,
            refresh_options.clone(),
        )
        .await
        .with_context(|| format!("failed to fetch streams for job {job_id}"))?;

        let muxed = remux_streams(video_stream, audio_stream);
        let upload_stream: UploadStream = Box::pin(muxed.map_err(anyhow::Error::new));

        match storage
            .store_stream(artifact_key, "video/mp4", upload_stream)
            .await
        {
            Ok(stored) => return Ok(stored),
            Err(error) if is_auth_like_error(&error) && request.source_url.is_some() => {
                if let Some((next_video, next_audio)) =
                    refresh_both_urls(request, &video_url, &audio_url).await
                {
                    warn!(
                        job_id,
                        attempt, "Refreshing mux URLs after auth-like storage pipeline error"
                    );
                    video_url = next_video;
                    audio_url = next_audio;
                    continue;
                }
                return Err(error).context("mux upload auth refresh failed");
            }
            Err(error) => return Err(error).context("failed to store muxed artifact"),
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

#[allow(dead_code)]
fn _is_auth_like_muxer_error(error: &MuxerError) -> bool {
    is_auth_like_error(&anyhow!(error.to_string()))
}

#[allow(dead_code)]
fn _is_auth_like_antibot_error(error: &AntiBotError) -> bool {
    is_auth_like_error(&anyhow!(error.to_string()))
}
