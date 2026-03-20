import {
	buildStreamUrl,
	createMuxedDownloadJob,
	extract,
	type MuxJobStatusUpdate,
	waitForMuxedDownloadJobReady
} from '$lib/api';
import { playlistWorkerLimitConfig } from '$lib/runtime-limit-config';
import {
	clearBatchItemProgressByVideoId,
	updateBatchItemByVideoId,
	updateBatchItemProgressByVideoId
} from '$stores/batch';
import {
	getStoredPlaylistDownloadMode,
	getStoredPlaylistQuality,
	isSingleCombined360pMp4Fallback,
	pickBestStreams,
	resolutionScore,
	safeFilename,
	toWatchUrl,
	type PlaylistDownloadMode,
	type PlaylistQuality
} from './playlist-download-stream-selection';
import type { Stream } from './types';
import {
	hasSelectedSaveDirectory,
	isAbortError,
	pickSaveDirectory,
	saveDownload
} from './playlist-download-file-saver';
import * as m from '$lib/paraglide/messages';

export type QueueEntry = {
	videoId: string;
	title: string;
	thumbnail?: string;
};

type ReadyEntry = {
	entry: QueueEntry;
	downloadUrl: string;
	filename: string;
	muxJobId?: string;
};

const MAX_CONCURRENT = playlistWorkerLimitConfig.maxConcurrent;
const READY_QUEUE_MAX = playlistWorkerLimitConfig.readyQueueMax;
const EXTRACT_JITTER_MIN_MS = playlistWorkerLimitConfig.extractJitterMinMs;
const EXTRACT_JITTER_RANGE_MS = playlistWorkerLimitConfig.extractJitterRangeMs;
const CIRCUIT_COOLDOWN_MS = playlistWorkerLimitConfig.circuitCooldownMs;

const pendingQueue: QueueEntry[] = [];
const readyQueue: ReadyEntry[] = [];
const activeEntries = new Map<string, QueueEntry>();
const activeControllers = new Map<string, AbortController>();

let activeCount = 0;
let prefetchActive = false;
let processActive = false;
let preferredQuality: PlaylistQuality = getStoredPlaylistQuality();
let preferredDownloadMode: PlaylistDownloadMode = getStoredPlaylistDownloadMode();
let hasManualQuality = false;
let hasManualMode = false;
let strictFsaaMode = false;
let circuitOpenUntil = 0;
let circuitResumeTimer: ReturnType<typeof setTimeout> | null = null;
let extractAbortController = new AbortController();
let workerEpoch = 0;

if (typeof window !== 'undefined') {
	window.addEventListener('beforeunload', () => {
		cancelAll();
	});
}

export { hasSelectedSaveDirectory, pickSaveDirectory };

export function setPreferredQuality(quality: PlaylistQuality): void {
	preferredQuality = quality;
	hasManualQuality = true;
}

export function setPreferredDownloadMode(mode: PlaylistDownloadMode): void {
	preferredDownloadMode = mode;
	hasManualMode = true;
}

export function setStrictFsaaMode(enabled: boolean): void {
	strictFsaaMode = enabled;
}

export function resetWorkerState(): void {
	cancelAll();
	closeCircuit();
	prefetchActive = false;
	processActive = false;
	preferredQuality = getStoredPlaylistQuality();
	preferredDownloadMode = getStoredPlaylistDownloadMode();
	hasManualQuality = false;
	hasManualMode = false;
}

export function enqueueDownload(entry: QueueEntry): void {
	const normalized: QueueEntry = {
		videoId: entry.videoId,
		title: entry.title,
		thumbnail: entry.thumbnail
	};

	if (isQueued(normalized.videoId)) return;
	pendingQueue.push(normalized);

	void fillPrefetchBuffer();
	void processDownloadSlots();
}

export function cancelAll(): void {
	workerEpoch += 1;

	for (const controller of activeControllers.values()) {
		controller.abort();
	}
	activeControllers.clear();

	extractAbortController.abort();
	extractAbortController = new AbortController();
	pendingQueue.length = 0;
	readyQueue.length = 0;

	for (const entry of activeEntries.values()) {
		updateBatchItemByVideoId(entry.videoId, 'pending');
	}
	activeEntries.clear();
	activeCount = 0;
}

export function getStatus(): {
	active: number;
	pending: number;
	ready: number;
	max: number;
	circuitOpen: boolean;
	cooldownMs: number;
} {
	const cooldownMs = Math.max(0, circuitOpenUntil - Date.now());
	return {
		active: activeCount,
		pending: pendingQueue.length,
		ready: readyQueue.length,
		max: MAX_CONCURRENT,
		circuitOpen: cooldownMs > 0,
		cooldownMs
	};
}

async function fillPrefetchBuffer(): Promise<void> {
	if (prefetchActive) return;
	prefetchActive = true;
	const epoch = workerEpoch;

	try {
		while (readyQueue.length < READY_QUEUE_MAX && pendingQueue.length > 0) {
			if (epoch !== workerEpoch) break;
			if (isCircuitOpen()) {
				scheduleCircuitResume();
				break;
			}

			const entry = pendingQueue.shift();
			if (!entry || activeEntries.has(entry.videoId)) continue;

			try {
				await sleep(extractJitterMs());
				if (epoch !== workerEpoch) break;
				const ready = await createReadyEntry(entry, extractAbortController.signal);
				if (epoch !== workerEpoch) break;
				readyQueue.push(ready);
			} catch (error) {
				if (epoch !== workerEpoch) break;
				if (isAbortError(error)) {
					break;
				}

				if (shouldOpenCircuit(error)) {
					openCircuit();
					pendingQueue.unshift(entry);
					updateBatchItemByVideoId(entry.videoId, 'pending');
					break;
				}

				const message =
					error instanceof Error ? error.message : m.playlist_worker_error_prepare_failed();
				updateBatchItemByVideoId(entry.videoId, 'error', message);
			}
		}
	} finally {
		prefetchActive = false;
		// After filling the buffer, kick off the download processor.
		// This is the only place that chains fillPrefetchBuffer → processDownloadSlots,
		// preventing the infinite microtask loop that occurs when processDownloadSlots
		// calls fillPrefetchBuffer().then(processDownloadSlots) while prefetchActive=true
		// (fillPrefetchBuffer returns immediately → .then fires immediately → loop).
		void processDownloadSlots();
	}
}

async function processDownloadSlots(): Promise<void> {
	if (processActive || isCircuitOpen()) {
		if (isCircuitOpen()) scheduleCircuitResume();
		return;
	}

	processActive = true;

	try {
		while (activeCount < MAX_CONCURRENT && readyQueue.length > 0) {
			const ready = readyQueue.shift();
			if (!ready) break;
			void startDownload(ready);
		}

		if (activeCount < MAX_CONCURRENT && readyQueue.length === 0 && pendingQueue.length > 0) {
			// Just trigger fillPrefetchBuffer — do NOT chain .then(processDownloadSlots) here.
			// fillPrefetchBuffer's finally block will call processDownloadSlots when done.
			// Chaining causes an infinite loop when fillPrefetchBuffer is already running.
			void fillPrefetchBuffer();
		}
	} finally {
		processActive = false;
	}
}

async function startDownload(ready: ReadyEntry): Promise<void> {
	const epoch = workerEpoch;
	const { entry } = ready;
	activeCount += 1;
	activeEntries.set(entry.videoId, entry);

	updateBatchItemByVideoId(entry.videoId, 'downloading');
	updateBatchItemProgressByVideoId(entry.videoId, {
		label: m.download_btn_progress_preparing_browser(),
		percent: null,
		indeterminate: true
	});

	const controller = new AbortController();
	activeControllers.set(entry.videoId, controller);

	try {
		await saveReadyEntry(ready, controller.signal);
		if (epoch !== workerEpoch) return;
		updateBatchItemByVideoId(entry.videoId, 'completed');
		clearBatchItemProgressByVideoId(entry.videoId);
	} catch (error) {
		if (epoch !== workerEpoch) return;
		if (isAbortError(error)) {
			updateBatchItemByVideoId(entry.videoId, 'pending');
			clearBatchItemProgressByVideoId(entry.videoId);
		} else {
			const message =
				error instanceof Error ? error.message : m.playlist_worker_error_download_failed();
			updateBatchItemByVideoId(entry.videoId, 'error', message);
		}
	} finally {
		activeControllers.delete(entry.videoId);
		activeEntries.delete(entry.videoId);
		activeCount = Math.max(0, activeCount - 1);
		if (epoch === workerEpoch) {
			void fillPrefetchBuffer();
			void processDownloadSlots();
		}
	}
}

async function saveReadyEntry(ready: ReadyEntry, signal: AbortSignal): Promise<void> {
	const saveOptions = {
		requireFsaa: strictFsaaMode,
		allowAnchorFallback: true,
		onProgress: (progress: {
			receivedBytes: number;
			totalBytes: number | null;
			percent: number | null;
		}) => {
			updateBatchItemProgressByVideoId(ready.entry.videoId, {
				label: m.download_btn_progress_starting_browser(),
				percent: progress.percent,
				indeterminate: progress.percent === null
			});
		}
	};

	await saveDownload(ready.downloadUrl, ready.filename, signal, saveOptions);
}

async function createReadyEntry(entry: QueueEntry, signal?: AbortSignal): Promise<ReadyEntry> {
	if (!hasManualQuality) {
		preferredQuality = getStoredPlaylistQuality();
	}
	if (!hasManualMode) {
		preferredDownloadMode = getStoredPlaylistDownloadMode();
	}

	const result = await extract(toWatchUrl(entry.videoId), signal);
	const useFsaa = hasSelectedSaveDirectory();
	const { video, audio } = pickBestStreams(result.streams, preferredQuality, {
		preferCombinedStream: !useFsaa,
		mode: preferredDownloadMode
	});
	const selectedResolution = video ? resolutionScore(video.quality) : null;
	const requestedResolution =
		preferredQuality === 'best' ? null : Number.parseInt(preferredQuality, 10);
	const fallbackBelowRequested =
		selectedResolution !== null &&
		requestedResolution !== null &&
		Number.isFinite(requestedResolution) &&
		selectedResolution < requestedResolution;
	const degradedSingleStream = isSingleCombined360pMp4Fallback(result.streams);

	console.info('[downloadtool][playlist] stream selection', {
		videoId: entry.videoId,
		title: entry.title,
		requestedQuality: preferredQuality,
		requestedResolution,
		mode: preferredDownloadMode,
		preferCombinedStream: !useFsaa,
		availableVideoStreams: summarizeVideoStreams(result.streams),
		selectedVideo: summarizeStream(video),
		selectedAudio: summarizeStream(audio),
		fallbackBelowRequested,
		degradedSingleStream,
		willMux: Boolean(video && !video.hasAudio && audio),
		willDirectStream: Boolean(video && video.hasAudio) || Boolean(!video && audio)
	});

	if (fallbackBelowRequested && degradedSingleStream) {
		console.warn('[downloadtool][playlist] degraded extract rejected below requested quality', {
			videoId: entry.videoId,
			title: entry.title,
			requestedQuality: preferredQuality,
			requestedResolution,
			selectedVideo: summarizeStream(video),
			availableVideoStreams: summarizeVideoStreams(result.streams)
		});
		throw new Error(m.playlist_worker_error_no_streams());
	}

	if (video && !video.hasAudio && audio) {
		console.info('[downloadtool][playlist] mux path selected', {
			videoId: entry.videoId,
			requestedQuality: preferredQuality,
			selectedVideo: summarizeStream(video),
			selectedAudio: summarizeStream(audio),
			fallbackBelowRequested
		});
		updateBatchItemProgressByVideoId(entry.videoId, {
			label: m.download_btn_progress_queueing_mux(),
			percent: null,
			indeterminate: true
		});
		const { jobId } = await createMuxedDownloadJob(
			video.url,
			audio.url,
			entry.title,
			{
				sourceUrl: result.originalUrl,
				videoFormatId: video.formatId,
				audioFormatId: audio.formatId
			}
		);
		const muxedFileUrl = await waitForMuxedDownloadJobReady(
			jobId,
			{
				onStatus: (update) => {
					const nextState = formatMuxStatus(update);
					updateBatchItemProgressByVideoId(entry.videoId, {
						label: nextState.label,
						percent: nextState.value,
						indeterminate: nextState.indeterminate
					});
				}
			},
			signal
		);

		return {
			entry,
			downloadUrl: muxedFileUrl,
			filename: safeFilename(entry.title, 'mp4'),
			muxJobId: jobId
		};
	}

	const stream = video ?? audio;
	if (!stream) throw new Error(m.playlist_worker_error_no_streams());
	console.info('[downloadtool][playlist] direct stream path selected', {
		videoId: entry.videoId,
		requestedQuality: preferredQuality,
		selectedStream: summarizeStream(stream),
		fallbackBelowRequested
	});
	updateBatchItemProgressByVideoId(entry.videoId, {
		label: m.download_btn_progress_preparing_browser(),
		percent: null,
		indeterminate: true
	});

	const downloadUrl = buildStreamUrl(stream.url, entry.title, stream.format || 'mp4', {
		sourceUrl: result.originalUrl,
		formatId: stream.formatId,
		patchInitMetadata: !stream.hasAudio && (stream.format || 'mp4').toLowerCase() === 'mp4'
	});

	return {
		entry,
		downloadUrl,
		filename: safeFilename(entry.title, stream.format || 'mp4')
	};
}

function extractJitterMs(): number {
	return EXTRACT_JITTER_MIN_MS + Math.random() * EXTRACT_JITTER_RANGE_MS;
}

function shouldOpenCircuit(error: unknown): boolean {
	const status = getErrorStatus(error);
	if (status === 429 || status === 403) return true;

	const message = error instanceof Error ? error.message.toLowerCase() : '';
	return (
		message.includes('429') ||
		message.includes('403') ||
		message.includes('too many requests') ||
		message.includes('rate limit') ||
		message.includes('forbidden')
	);
}

function getErrorStatus(error: unknown): number | undefined {
	if (typeof error !== 'object' || error === null || !('status' in error)) return undefined;
	const raw = (error as { status?: unknown }).status;
	if (typeof raw === 'number') return raw;
	if (typeof raw === 'string') {
		const parsed = Number.parseInt(raw, 10);
		if (Number.isFinite(parsed)) return parsed;
	}
	return undefined;
}

function isCircuitOpen(): boolean {
	return Date.now() < circuitOpenUntil;
}

function openCircuit(): void {
	circuitOpenUntil = Date.now() + CIRCUIT_COOLDOWN_MS;
	scheduleCircuitResume();
}

function closeCircuit(): void {
	circuitOpenUntil = 0;
	if (circuitResumeTimer) {
		clearTimeout(circuitResumeTimer);
		circuitResumeTimer = null;
	}
}

function scheduleCircuitResume(): void {
	if (!isCircuitOpen()) return;
	if (circuitResumeTimer) clearTimeout(circuitResumeTimer);

	const waitMs = Math.max(0, circuitOpenUntil - Date.now()) + 50;
	circuitResumeTimer = setTimeout(() => {
		closeCircuit();
		void fillPrefetchBuffer();
		void processDownloadSlots();
	}, waitMs);
}

function sleep(ms: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

function summarizeVideoStreams(streams: Stream[]): Array<{
	formatId?: string;
	quality: string;
	format: string;
	hasAudio: boolean;
	isAudioOnly: boolean;
	codecLabel?: string;
}> {
	return streams
		.filter((stream) => !stream.isAudioOnly)
		.map((stream) => summarizeStream(stream))
		.filter((stream): stream is NonNullable<typeof stream> => stream !== null);
}

function summarizeStream(stream: Stream | null): {
	formatId?: string;
	quality: string;
	format: string;
	hasAudio: boolean;
	isAudioOnly: boolean;
	codecLabel?: string;
} | null {
	if (!stream) return null;
	return {
		formatId: stream.formatId,
		quality: stream.quality,
		format: stream.format,
		hasAudio: stream.hasAudio,
		isAudioOnly: stream.isAudioOnly,
		codecLabel: stream.codecLabel
	};
}

function clampProgressPercent(
	value: number | null | undefined,
	status: MuxJobStatusUpdate['status']
): number | null {
	if (typeof value !== 'number' || Number.isNaN(value)) return null;
	const max = status === 'ready' ? 100 : 99;
	return Math.max(0, Math.min(max, value));
}

function resolveMuxPhaseLabel(update: MuxJobStatusUpdate): string | null {
	switch (update.phase) {
		case 'starting':
			return m.download_btn_mux_status_processing_running();
		case 'fetching_streams':
			return m.download_btn_mux_status_processing_fetching();
		case 'muxing_uploading':
			return m.download_btn_mux_status_processing_muxing();
		case 'completing_upload':
			return m.download_btn_mux_status_processing_finalizing();
		case 'ready':
			return m.download_btn_mux_status_ready();
		case 'failed':
			return m.download_btn_mux_status_failed();
		default:
			return null;
	}
}

function resolveQueuedMuxLabel(update: MuxJobStatusUpdate, elapsedSeconds: number): string {
	const queuePosition = update.queuePosition ?? null;
	if (typeof queuePosition === 'number' && queuePosition > 1) {
		return m.download_btn_mux_status_queued_position({ count: String(queuePosition - 1) });
	}
	if (queuePosition === 1) {
		return m.download_btn_mux_status_queued_front();
	}
	return elapsedSeconds >= 8
		? m.download_btn_mux_status_queued_waiting()
		: m.download_btn_mux_status_queued();
}

function formatMuxStatus(
	update: MuxJobStatusUpdate
): { label: string; value: number | null; indeterminate: boolean } {
	const elapsedSeconds = Math.floor(update.elapsedMs / 1000);
	const livePercent = clampProgressPercent(update.percent, update.status);
	const phaseLabel = resolveMuxPhaseLabel(update);

	if (phaseLabel) {
		return {
			label: phaseLabel,
			value:
				update.status === 'ready'
					? 100
					: update.status === 'failed'
						? 0
						: livePercent,
			indeterminate: livePercent === null && update.status !== 'ready' && update.status !== 'failed'
		};
	}

	if (update.status === 'queued') {
		return {
			label: resolveQueuedMuxLabel(update, elapsedSeconds),
			value: livePercent,
			indeterminate: livePercent === null
		};
	}

	if (update.status === 'leased') {
		return {
			label:
				elapsedSeconds >= 10
					? m.download_btn_mux_status_leased_waiting()
					: m.download_btn_mux_status_leased(),
			value: livePercent,
			indeterminate: livePercent === null
		};
	}

	if (update.status === 'processing') {
		const phase = Math.floor(elapsedSeconds / 6) % 4;
		if (phase === 0) {
			return {
				label: m.download_btn_mux_status_processing_fetching(),
				value: livePercent,
				indeterminate: livePercent === null
			};
		}
		if (phase === 1) {
			return {
				label: m.download_btn_mux_status_processing_muxing(),
				value: livePercent,
				indeterminate: livePercent === null
			};
		}
		if (phase === 2) {
			return {
				label: m.download_btn_mux_status_processing_finalizing(),
				value: livePercent,
				indeterminate: livePercent === null
			};
		}
		return {
			label: m.download_btn_mux_status_processing_running(),
			value: livePercent,
			indeterminate: livePercent === null
		};
	}

	if (update.status === 'ready') {
		return { label: m.download_btn_mux_status_ready(), value: 100, indeterminate: false };
	}

	if (update.status === 'failed') {
		return { label: m.download_btn_mux_status_failed(), value: 0, indeterminate: false };
	}

	return { label: m.download_btn_mux_status_expired(), value: 0, indeterminate: false };
}

function isQueued(videoId: string): boolean {
	if (pendingQueue.some((entry) => entry.videoId === videoId)) return true;
	if (readyQueue.some((ready) => ready.entry.videoId === videoId)) return true;
	if (activeEntries.has(videoId)) return true;
	return false;
}
