import * as m from '$lib/paraglide/messages';

type SaveDirectoryHandle = {
	getFileHandle: (
		name: string,
		options?: { create?: boolean }
	) => Promise<{
		createWritable: () => Promise<FileWritableHandle>;
	}>;
};

type FileWritableHandle = WritableStream & {
	close?: () => Promise<void>;
	abort?: (reason?: unknown) => Promise<void>;
};

export interface SaveDownloadOptions {
	requireFsaa?: boolean;
	allowAnchorFallback?: boolean;
}

let saveDirectoryHandle: SaveDirectoryHandle | null = null;
const RETRYABLE_STATUS_CODES = new Set([429, 502, 503, 504]);
const MAX_RETRY_ATTEMPTS = 5;
const RETRY_BASE_DELAY_MS = 600;
const RETRY_MAX_DELAY_MS = 12_000;

type DownloadErrorWithStatus = Error & {
	status?: number;
	retryAfterMs?: number;
};

export function hasSelectedSaveDirectory(): boolean {
	return saveDirectoryHandle !== null;
}

export async function pickSaveDirectory(): Promise<boolean> {
	if (typeof window === 'undefined') return false;

	const picker = (window as Window & { showDirectoryPicker?: () => Promise<SaveDirectoryHandle> })
		.showDirectoryPicker;
	if (!picker) return false;

	try {
		saveDirectoryHandle = await picker();
		return true;
	} catch {
		return false;
	}
}

export function isAbortError(error: unknown): boolean {
	return error instanceof DOMException && error.name === 'AbortError';
}

export async function saveDownload(
	url: string,
	filename: string,
	signal: AbortSignal,
	options: SaveDownloadOptions = {}
): Promise<void> {
	const requireFsaa = options.requireFsaa === true;
	const allowAnchorFallback = options.allowAnchorFallback !== false;

	if (saveDirectoryHandle) {
		try {
			console.info('[downloadtool] saveDownload using File System Access API', {
				filename,
				url
			});
			await saveWithDirectory(url, filename, saveDirectoryHandle, signal);
			return;
		} catch (error) {
			if (isAbortError(error)) throw error;
			if (!allowAnchorFallback) throw error;
			console.warn('[downloadtool] saveDownload falling back to anchor after FSAA failure', {
				filename,
				url,
				error
			});
			downloadViaAnchor(url, filename);
			return;
		}
	}

	if (requireFsaa) {
		throw new Error(m.file_saver_error_no_directory_selected());
	}

	if (!allowAnchorFallback) {
		throw new Error(m.file_saver_error_anchor_fallback_disabled());
	}

	console.info('[downloadtool] saveDownload using anchor fallback', {
		filename,
		url
	});
	downloadViaAnchor(url, filename);
}

async function saveWithDirectory(
	url: string,
	filename: string,
	dirHandle: SaveDirectoryHandle,
	signal: AbortSignal
): Promise<void> {
	const response = await fetchWithRetry(url, signal);

	const fileHandle = await dirHandle.getFileHandle(filename, { create: true });
	const writable = await fileHandle.createWritable();

	if (!response.body) {
		if (typeof writable.close === 'function') {
			await writable.close();
		}
		throw new Error(m.file_saver_error_stream_unavailable());
	}

	try {
		await response.body.pipeTo(writable as WritableStream, { signal });
	} catch (error) {
		if (typeof writable.abort === 'function') {
			try {
				await writable.abort(error);
			} catch {
				// Ignore secondary abort errors.
			}
		}
		throw error;
	}
}

async function fetchWithRetry(url: string, signal: AbortSignal): Promise<Response> {
	let lastError: unknown;

	for (let attempt = 1; attempt <= MAX_RETRY_ATTEMPTS; attempt += 1) {
		try {
			const response = await fetch(url, { signal });
			if (response.ok) return response;

			const status = response.status;
			const retryAfterMs = parseRetryAfterHeader(response.headers.get('retry-after'));
			if (response.body) {
				try {
					await response.body.cancel();
				} catch {
					// Ignore body cancellation errors.
				}
			}

			if (!isRetryableStatus(status) || attempt >= MAX_RETRY_ATTEMPTS) {
				const error = new Error(
					m.file_saver_error_download_failed_with_status({ status: String(status) })
				) as DownloadErrorWithStatus;
				error.status = status;
				error.retryAfterMs = retryAfterMs;
				throw error;
			}

			const delayMs = clampRetryDelay(
				retryAfterMs ?? computeBackoffWithJitter(attempt),
				RETRY_MAX_DELAY_MS
			);
			await sleep(delayMs, signal);
		} catch (error) {
			if (isAbortError(error)) throw error;
			lastError = error;
			if (attempt >= MAX_RETRY_ATTEMPTS) break;

			const delayMs = computeDelayFromError(error, attempt);
			await sleep(delayMs, signal);
		}
	}

	if (lastError instanceof Error) throw lastError;
	throw new Error(m.file_saver_error_download_failed_after_retries());
}

function isRetryableStatus(status: number): boolean {
	return RETRYABLE_STATUS_CODES.has(status);
}

function parseRetryAfterHeader(value: string | null): number | undefined {
	if (!value) return undefined;
	const seconds = Number.parseInt(value, 10);
	if (Number.isFinite(seconds) && seconds > 0) return seconds * 1000;

	const asDate = Date.parse(value);
	if (!Number.isNaN(asDate)) {
		const diff = asDate - Date.now();
		if (diff > 0) return diff;
	}
	return undefined;
}

function clampRetryDelay(delayMs: number, maxMs: number): number {
	return Math.max(0, Math.min(delayMs, maxMs));
}

function computeBackoffWithJitter(attempt: number): number {
	const base = Math.min(RETRY_BASE_DELAY_MS * Math.pow(2, attempt - 1), RETRY_MAX_DELAY_MS);
	const jitter = Math.floor(Math.random() * Math.min(400, Math.floor(base * 0.3)));
	return base + jitter;
}

function computeDelayFromError(error: unknown, attempt: number): number {
	const retryAfterMs = (error as DownloadErrorWithStatus | null | undefined)?.retryAfterMs;
	if (typeof retryAfterMs === 'number' && Number.isFinite(retryAfterMs) && retryAfterMs > 0) {
		return clampRetryDelay(retryAfterMs, RETRY_MAX_DELAY_MS);
	}
	return computeBackoffWithJitter(attempt);
}

async function sleep(ms: number, signal: AbortSignal): Promise<void> {
	if (ms <= 0) return;
	if (signal.aborted) throw new DOMException('Operation aborted', 'AbortError');
	await new Promise<void>((resolve, reject) => {
		const timeout = setTimeout(() => {
			signal.removeEventListener('abort', onAbort);
			resolve();
		}, ms);

		const onAbort = () => {
			clearTimeout(timeout);
			signal.removeEventListener('abort', onAbort);
			reject(new DOMException('Operation aborted', 'AbortError'));
		};

		signal.addEventListener('abort', onAbort, { once: true });
	});
}

function downloadViaAnchor(url: string, filename: string): void {
	console.info('[downloadtool] anchor download click', {
		filename,
		url
	});
	const anchor = document.createElement('a');
	anchor.href = url;
	anchor.download = filename;
	anchor.style.display = 'none';
	document.body.appendChild(anchor);
	anchor.click();
	document.body.removeChild(anchor);
}
