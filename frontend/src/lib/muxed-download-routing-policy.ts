import type { Stream } from '$lib/types';

const MB = 1024 * 1024;
const SYNC_ACTIVE_HARD_LIMIT = 12;
const SYNC_ACTIVE_SOFT_LIMIT = 6;
const SYNC_DURATION_MAX_SECONDS = 30 * 60;
const FORCE_JOB_DURATION_SECONDS = 90 * 60;
const SYNC_SIZE_MAX_BYTES = 120 * MB;
const FORCE_JOB_SIZE_BYTES = 300 * MB;
const SYNC_RESOLUTION_MAX = 1080;
const FORCE_JOB_RESOLUTION_MIN = 1440;
const TIMEOUT_STATUS_CODES = new Set([408, 504]);
const TIMEOUT_MESSAGE_PATTERNS = [
	'timed out',
	'timeout',
	'time out',
	'network timeout',
	'operation timed out',
	'mux preflight timed out'
];

let activeSyncMuxedDownloads = 0;

export type MuxedDownloadRoute = 'sync' | 'jobs';

export interface MuxedDownloadRouteInput {
	videoStream: Stream;
	audioStream: Stream;
	durationSeconds?: number | null;
	activeSyncMuxed?: number;
}

export interface MuxedDownloadRouteDecision {
	route: MuxedDownloadRoute;
	reason:
		| 'resolution>=1440p'
		| 'duration>90m'
		| 'size>300mb'
		| 'size-unknown'
		| 'active-sync>=12'
		| 'small-safe-sync'
		| 'mid-zone-sync-low-load'
		| 'mid-zone-jobs-high-load';
	estimatedTotalBytes?: number;
	resolution?: number;
}

export function getActiveSyncMuxedDownloads(): number {
	return activeSyncMuxedDownloads;
}

export async function withSyncMuxedDownloadSlot<T>(task: () => Promise<T>): Promise<T> {
	activeSyncMuxedDownloads += 1;
	try {
		return await task();
	} finally {
		activeSyncMuxedDownloads = Math.max(0, activeSyncMuxedDownloads - 1);
	}
}

export function decideMuxedDownloadRoute(input: MuxedDownloadRouteInput): MuxedDownloadRouteDecision {
	const activeSyncMuxed = input.activeSyncMuxed ?? getActiveSyncMuxedDownloads();
	const resolution = resolutionScore(input.videoStream.quality);
	const totalSizeBytes = estimateMuxedTotalBytes(
		input.videoStream,
		input.audioStream,
		input.durationSeconds
	);
	const durationSeconds =
		typeof input.durationSeconds === 'number' && input.durationSeconds > 0
			? input.durationSeconds
			: undefined;

	if (activeSyncMuxed >= SYNC_ACTIVE_HARD_LIMIT) {
		return {
			route: 'jobs',
			reason: 'active-sync>=12',
			estimatedTotalBytes: totalSizeBytes,
			resolution
		};
	}
	if (typeof resolution === 'number' && resolution >= FORCE_JOB_RESOLUTION_MIN) {
		return {
			route: 'jobs',
			reason: 'resolution>=1440p',
			estimatedTotalBytes: totalSizeBytes,
			resolution
		};
	}
	if (typeof durationSeconds === 'number' && durationSeconds > FORCE_JOB_DURATION_SECONDS) {
		return {
			route: 'jobs',
			reason: 'duration>90m',
			estimatedTotalBytes: totalSizeBytes,
			resolution
		};
	}
	if (typeof totalSizeBytes !== 'number') {
		return { route: 'jobs', reason: 'size-unknown', resolution };
	}
	if (totalSizeBytes > FORCE_JOB_SIZE_BYTES) {
		return {
			route: 'jobs',
			reason: 'size>300mb',
			estimatedTotalBytes: totalSizeBytes,
			resolution
		};
	}

	const smallSafeDuration =
		typeof durationSeconds !== 'number' || durationSeconds <= SYNC_DURATION_MAX_SECONDS;
	const smallSafeResolution = typeof resolution !== 'number' || resolution <= SYNC_RESOLUTION_MAX;
	const smallSafeSize = totalSizeBytes <= SYNC_SIZE_MAX_BYTES;
	if (smallSafeDuration && smallSafeResolution && smallSafeSize) {
		return {
			route: 'sync',
			reason: 'small-safe-sync',
			estimatedTotalBytes: totalSizeBytes,
			resolution
		};
	}
	if (activeSyncMuxed < SYNC_ACTIVE_SOFT_LIMIT) {
		return {
			route: 'sync',
			reason: 'mid-zone-sync-low-load',
			estimatedTotalBytes: totalSizeBytes,
			resolution
		};
	}
	return {
		route: 'jobs',
		reason: 'mid-zone-jobs-high-load',
		estimatedTotalBytes: totalSizeBytes,
		resolution
	};
}

export function estimateMuxedTotalBytes(
	videoStream: Stream,
	audioStream: Stream,
	durationSeconds?: number | null
): number | undefined {
	const videoSize = estimateStreamSizeBytes(videoStream, durationSeconds);
	const audioSize = estimateStreamSizeBytes(audioStream, durationSeconds);
	if (typeof videoSize !== 'number' || typeof audioSize !== 'number') return undefined;
	return videoSize + audioSize;
}

export function isTimeoutLikeMuxedSyncError(error: unknown): boolean {
	if (error instanceof DOMException && error.name === 'AbortError') return false;
	const status = getErrorStatus(error);
	if (typeof status === 'number' && TIMEOUT_STATUS_CODES.has(status)) return true;
	const message = error instanceof Error ? error.message.toLowerCase() : '';
	return TIMEOUT_MESSAGE_PATTERNS.some((token) => message.includes(token));
}

function estimateStreamSizeBytes(stream: Stream, durationSeconds?: number | null): number | undefined {
	if (typeof stream.size === 'number' && Number.isFinite(stream.size) && stream.size > 0) {
		return stream.size;
	}
	if (
		typeof stream.bitrate === 'number' &&
		Number.isFinite(stream.bitrate) &&
		stream.bitrate > 0 &&
		typeof durationSeconds === 'number' &&
		Number.isFinite(durationSeconds) &&
		durationSeconds > 0
	) {
		return Math.round((stream.bitrate * durationSeconds) / 8);
	}
	return undefined;
}

function resolutionScore(qualityLabel: string | undefined): number | undefined {
	if (!qualityLabel) return undefined;
	const lower = qualityLabel.toLowerCase();
	if (lower.includes('8k')) return 4320;
	if (lower.includes('4k')) return 2160;
	if (lower.includes('2k')) return 1440;
	const match = lower.match(/(\d{3,4})p/);
	if (!match) return undefined;
	const parsed = Number.parseInt(match[1], 10);
	return Number.isFinite(parsed) ? parsed : undefined;
}

function getErrorStatus(error: unknown): number | undefined {
	if (typeof error !== 'object' || error === null || !('status' in error)) return undefined;
	const raw = (error as { status?: unknown }).status;
	if (typeof raw === 'number' && Number.isFinite(raw)) return raw;
	if (typeof raw === 'string') {
		const parsed = Number.parseInt(raw, 10);
		if (Number.isFinite(parsed)) return parsed;
	}
	return undefined;
}
