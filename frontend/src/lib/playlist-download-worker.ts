import { buildMuxedStreamUrl, buildStreamUrl, extract } from '$lib/api';
import { updateBatchItemByVideoId } from '$stores/batch';
import {
	getStoredPlaylistQuality,
	pickBestStreams,
	safeFilename,
	toWatchUrl,
	type PlaylistQuality
} from './playlist-download-stream-selection';
import {
	hasSelectedSaveDirectory,
	isAbortError,
	pickSaveDirectory,
	saveDownload
} from './playlist-download-file-saver';

export type QueueEntry = {
	videoId: string;
	title: string;
	thumbnail?: string;
};

type ReadyEntry = {
	entry: QueueEntry;
	downloadUrl: string;
	filename: string;
};

const MAX_CONCURRENT = 1;
const READY_QUEUE_MAX = 1;
const EXTRACT_JITTER_MIN_MS = 2000;
const EXTRACT_JITTER_RANGE_MS = 3000;
const CIRCUIT_COOLDOWN_MS = 5 * 60 * 1000;

const pendingQueue: QueueEntry[] = [];
const readyQueue: ReadyEntry[] = [];
const activeEntries = new Map<string, QueueEntry>();
const activeControllers = new Map<string, AbortController>();

let activeCount = 0;
let prefetchActive = false;
let processActive = false;
let preferredQuality: PlaylistQuality = getStoredPlaylistQuality();
let hasManualQuality = false;
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

export function setStrictFsaaMode(enabled: boolean): void {
	strictFsaaMode = enabled;
}

export function resetWorkerState(): void {
	cancelAll();
	closeCircuit();
	prefetchActive = false;
	processActive = false;
	preferredQuality = getStoredPlaylistQuality();
	hasManualQuality = false;
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

				const message = error instanceof Error ? error.message : 'Failed to prepare download';
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
	const { entry, downloadUrl, filename } = ready;
	activeCount += 1;
	activeEntries.set(entry.videoId, entry);

	updateBatchItemByVideoId(entry.videoId, 'downloading');

	const controller = new AbortController();
	activeControllers.set(entry.videoId, controller);

	try {
		await saveDownload(downloadUrl, filename, controller.signal, {
			requireFsaa: strictFsaaMode,
			allowAnchorFallback: true
		});
		if (epoch !== workerEpoch) return;
		updateBatchItemByVideoId(entry.videoId, 'completed');
	} catch (error) {
		if (epoch !== workerEpoch) return;
		if (isAbortError(error)) {
			updateBatchItemByVideoId(entry.videoId, 'pending');
		} else {
			const message = error instanceof Error ? error.message : 'Download failed';
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

async function createReadyEntry(entry: QueueEntry, signal?: AbortSignal): Promise<ReadyEntry> {
	if (!hasManualQuality) {
		preferredQuality = getStoredPlaylistQuality();
	}

	const result = await extract(toWatchUrl(entry.videoId), signal);
	const useFsaa = hasSelectedSaveDirectory();
	const { video, audio } = pickBestStreams(result.streams, preferredQuality, {
		preferMuxed: !useFsaa
	});

	if (video && !video.hasAudio && audio) {
		return {
			entry,
			downloadUrl: buildMuxedStreamUrl(video.url, audio.url, entry.title),
			filename: safeFilename(entry.title, 'mp4')
		};
	}

	const stream = video ?? audio;
	if (!stream) throw new Error('No downloadable stream found');

	const downloadUrl = useFsaa
		? buildStreamUrl(stream.url, entry.title, stream.format || 'mp4')
		: stream.url;

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

function isQueued(videoId: string): boolean {
	if (pendingQueue.some((entry) => entry.videoId === videoId)) return true;
	if (readyQueue.some((ready) => ready.entry.videoId === videoId)) return true;
	if (activeEntries.has(videoId)) return true;
	return false;
}
