import type { Stream } from '$lib/types';
import * as m from '$lib/paraglide/messages';

export const PLAYLIST_QUALITY_OPTIONS = [
	{ value: 'best', label: m.home_playlist_quality_best() },
	{ value: '2160', label: '2160p (4K)' },
	{ value: '1440', label: '1440p (2K)' },
	{ value: '1080', label: '1080p (Full HD)' },
	{ value: '720', label: '720p (HD)' },
	{ value: '480', label: '480p' },
	{ value: '360', label: '360p' }
] as const;

export const PLAYLIST_DOWNLOAD_MODE_OPTIONS = [
	{ value: 'video', label: m.home_playlist_download_mode_video_audio() },
	{ value: 'video-only', label: m.format_picker_delivery_video_only() },
	{ value: 'audio', label: m.home_playlist_download_mode_audio_only() }
] as const;

export type PlaylistQuality = (typeof PLAYLIST_QUALITY_OPTIONS)[number]['value'];
export type PlaylistDownloadMode = (typeof PLAYLIST_DOWNLOAD_MODE_OPTIONS)[number]['value'];

const PLAYLIST_QUALITY_KEY = 'fetchtube.playlist-quality.v1';
const PLAYLIST_DOWNLOAD_MODE_KEY = 'fetchtube.playlist-download-mode.v1';
const PREFERRED_FORMAT_KEY = 'fetchtube.preferred-format.v1';

interface StoredPreferredFormat {
	mode?: 'video' | 'videoOnly' | 'audio';
	qualityValue?: number;
}

export function toWatchUrl(videoId: string): string {
	return `https://www.youtube.com/watch?v=${videoId}`;
}

export function safeFilename(title: string, extension = 'mp4'): string {
	const base = title
		.trim()
		.replace(/[\\/:*?"<>|]+/g, '_')
		.replace(/\s+/g, ' ')
		.slice(0, 120);
	const normalizedBase = base || 'video';
	const normalizedExt = extension.replace(/[^a-z0-9]/gi, '').toLowerCase() || 'mp4';
	return `${normalizedBase}.${normalizedExt}`;
}

export function getStoredPlaylistQuality(): PlaylistQuality {
	if (typeof window === 'undefined') return 'best';

	try {
		const explicit = window.localStorage.getItem(PLAYLIST_QUALITY_KEY);
		if (isPlaylistQuality(explicit)) return explicit;

		const raw = window.localStorage.getItem(PREFERRED_FORMAT_KEY);
		if (!raw) return 'best';

		const parsed = JSON.parse(raw) as StoredPreferredFormat;
		if (parsed.mode !== 'video' || typeof parsed.qualityValue !== 'number') return 'best';
		return qualityFromScore(parsed.qualityValue);
	} catch {
		return 'best';
	}
}

export function getStoredPlaylistDownloadMode(): PlaylistDownloadMode {
	if (typeof window === 'undefined') return 'video';

	try {
		const explicit = window.localStorage.getItem(PLAYLIST_DOWNLOAD_MODE_KEY);
		if (isPlaylistDownloadMode(explicit)) return explicit;

		const raw = window.localStorage.getItem(PREFERRED_FORMAT_KEY);
		if (!raw) return 'video';

		const parsed = JSON.parse(raw) as StoredPreferredFormat;
		if (parsed.mode === 'audio') return 'audio';
		if (parsed.mode === 'videoOnly') return 'video-only';
		return 'video';
	} catch {
		return 'video';
	}
}

export function pickBestStreams(
	streams: Stream[],
	quality: PlaylistQuality,
	options: { preferCombinedStream?: boolean; mode?: PlaylistDownloadMode } = {}
): { video: Stream | null; audio: Stream | null } {
	const mode = options.mode ?? 'video';
	const audio = selectBestAudioStream(streams);

	if (mode === 'audio') {
		return { video: null, audio };
	}

	// Keep all video streams except WebM video-only because fMP4 mux jobs only accept MP4/M4A.
	// This keeps "Best available" free to choose the highest compatible stream pair.
	const compatibleVideo = streams.filter((stream) => isCompatibleVideoStream(stream));

	const sortedVideo = [...compatibleVideo].sort((a, b) => {
		const resolutionDiff = resolutionScore(b.quality) - resolutionScore(a.quality);
		if (resolutionDiff !== 0) return resolutionDiff;

		// Prefer a stream that already includes audio before falling back to a mux job.
		if (options.preferCombinedStream && a.hasAudio !== b.hasAudio) return a.hasAudio ? -1 : 1;

		// Prefer MP4 over others when resolution is equal for better compatibility.
		const formatDiff = formatRank(a.format) - formatRank(b.format);
		if (formatDiff !== 0) return formatDiff;

		return (b.bitrate ?? 0) - (a.bitrate ?? 0);
	});

	const video = selectByTargetCeiling(sortedVideo, quality);

	if (mode === 'video-only') {
		return { video, audio: null };
	}

	return { video, audio };
}

function selectBestAudioStream(streams: Stream[]): Stream | null {
	return (
		[...streams]
			.filter((stream) => stream.isAudioOnly)
			.sort((a, b) => {
				if (a.format !== b.format) return a.format === 'mp4' ? -1 : 1;
				return (b.bitrate ?? 0) - (a.bitrate ?? 0);
			})[0] ?? null
	);
}

function selectByTargetCeiling(
	videoStreams: Stream[],
	quality: PlaylistQuality
): Stream | null {
	if (videoStreams.length === 0) return null;
	if (quality === 'best') return videoStreams[0];

	const target = Number.parseInt(quality, 10);
	if (!Number.isFinite(target) || target <= 0) return videoStreams[0];

	const eligible = videoStreams.filter((stream) => resolutionScore(stream.quality) >= target);
	if (eligible.length > 0) return eligible[eligible.length - 1];
	return videoStreams[0];
}

function isCompatibleVideoStream(stream: Stream): boolean {
	if (stream.isAudioOnly) return false;
	// Job-based fMP4 muxing rejects WebM video-only streams.
	if (!stream.hasAudio && stream.format.toLowerCase() === 'webm') return false;
	return true;
}

function formatRank(format: string): number {
	return format.toLowerCase() === 'mp4' ? 0 : 1;
}

export function resolutionScore(label: string): number {
	const lower = label.toLowerCase();
	if (lower.includes('8k')) return 4320;
	if (lower.includes('4k')) return 2160;
	if (lower.includes('2k')) return 1440;
	const match = lower.match(/(\d{3,4})p/);
	if (!match) return 0;
	const parsed = Number.parseInt(match[1], 10);
	return Number.isFinite(parsed) ? parsed : 0;
}

export function isSingleCombined360pMp4Fallback(streams: Stream[]): boolean {
	if (streams.length !== 1) return false;

	const [stream] = streams;
	return (
		!stream.isAudioOnly &&
		stream.hasAudio &&
		stream.format.toLowerCase() === 'mp4' &&
		resolutionScore(stream.quality) === 360
	);
}

function qualityFromScore(score: number): PlaylistQuality {
	if (score >= 2160) return '2160';
	if (score >= 1440) return '1440';
	if (score >= 1080) return '1080';
	if (score >= 720) return '720';
	if (score >= 480) return '480';
	if (score >= 360) return '360';
	return 'best';
}

function isPlaylistQuality(value: string | null): value is PlaylistQuality {
	return PLAYLIST_QUALITY_OPTIONS.some((option) => option.value === value);
}

function isPlaylistDownloadMode(value: string | null): value is PlaylistDownloadMode {
	return PLAYLIST_DOWNLOAD_MODE_OPTIONS.some((option) => option.value === value);
}
