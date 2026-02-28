import type { Stream } from '$lib/types';

export const PLAYLIST_QUALITY_OPTIONS = [
	{ value: 'best', label: 'Best available' },
	{ value: '2160', label: '2160p (4K)' },
	{ value: '1440', label: '1440p (2K)' },
	{ value: '1080', label: '1080p (Full HD)' },
	{ value: '720', label: '720p (HD)' },
	{ value: '480', label: '480p' },
	{ value: '360', label: '360p' }
] as const;

export type PlaylistQuality = (typeof PLAYLIST_QUALITY_OPTIONS)[number]['value'];

const PLAYLIST_QUALITY_KEY = 'fetchtube.playlist-quality.v1';
const PREFERRED_FORMAT_KEY = 'fetchtube.preferred-format.v1';

interface StoredPreferredFormat {
	mode?: 'video' | 'audio';
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

export function pickBestStreams(
	streams: Stream[],
	quality: PlaylistQuality,
	options: { preferMuxed?: boolean } = {}
): { video: Stream | null; audio: Stream | null } {
	const mp4Video = streams.filter((stream) => !stream.isAudioOnly && stream.format === 'mp4');

	// In anchor-download mode (no FSAA), prefer muxed streams (video with embedded audio).
	// This lets us use the direct CDN URL instead of routing through the Vite proxy,
	// avoiding HTTP/1.1 connection pool exhaustion on non-localhost HTTP origins.
	if (options.preferMuxed) {
		const muxed = mp4Video.filter((s) => s.hasAudio);
		if (muxed.length > 0) {
			const sorted = [...muxed].sort(
				(a, b) => resolutionScore(b.quality) - resolutionScore(a.quality)
			);
			return { video: selectByTargetCeiling(sorted, quality), audio: null };
		}
		// No muxed stream available — fall through to normal selection
	}

	const sortedVideo = [...mp4Video].sort((a, b) => {
		const resolutionDiff = resolutionScore(b.quality) - resolutionScore(a.quality);
		if (resolutionDiff !== 0) return resolutionDiff;
		if (a.hasAudio !== b.hasAudio) return a.hasAudio ? 1 : -1;
		return (b.bitrate ?? 0) - (a.bitrate ?? 0);
	});

	const videoOnly = sortedVideo.filter((stream) => !stream.hasAudio);
	const videoPool = videoOnly.length > 0 ? videoOnly : sortedVideo;
	const video = selectByTargetCeiling(videoPool, quality);

	const audio = [...streams]
		.filter((stream) => stream.isAudioOnly)
		.sort((a, b) => {
			if (a.format !== b.format) return a.format === 'mp4' ? -1 : 1;
			return (b.bitrate ?? 0) - (a.bitrate ?? 0);
		})[0] ?? null;

	return { video, audio };
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

function resolutionScore(label: string): number {
	const lower = label.toLowerCase();
	if (lower.includes('8k')) return 4320;
	if (lower.includes('4k')) return 2160;
	if (lower.includes('2k')) return 1440;
	const match = lower.match(/(\d{3,4})p/);
	if (!match) return 0;
	const parsed = Number.parseInt(match[1], 10);
	return Number.isFinite(parsed) ? parsed : 0;
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
