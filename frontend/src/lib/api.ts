/**
 * Typed API client for video downloader backend
 * @module api
 */

import type { ExtractResult, BatchMessage } from './types';

const RAW_API_BASE = import.meta.env.VITE_API_URL || '';
const RETRYABLE_HTTP_STATUS = new Set([429, 502, 503, 504]);
const EXTRACT_MAX_RETRY_ATTEMPTS = 4;
const EXTRACT_RETRY_BASE_DELAY_MS = 500;
const EXTRACT_RETRY_MAX_DELAY_MS = 8_000;
const BATCH_MAX_RECONNECT_ATTEMPTS = 8;
const BATCH_RECONNECT_BASE_DELAY_MS = 1_000;
const BATCH_RECONNECT_MAX_DELAY_MS = 12_000;
const MUX_JOB_POLL_INTERVAL_MS = 1_200;
const MUX_JOB_MAX_WAIT_MS = 10 * 60 * 1000;

export type BatchSubscription = {
	close: () => void;
};

type CreateMuxJobApiResponse = {
	job_id: string;
	status: string;
	status_url: string;
	file_url: string;
};

type MuxJobStatusApiResponse = {
	job_id: string;
	status: 'queued' | 'processing' | 'ready' | 'failed';
	error?: string | null;
	file_url?: string | null;
};

function stripErrorPrefix(message: string): string {
	return message.replace(/^extract(?:ion)?\s+failed:\s*/i, '').replace(/^error:\s*/i, '').trim();
}

function isBatchLikeYoutubeUrl(url: string): boolean {
	const lower = url.toLowerCase();
	return (
		lower.includes('youtube.com/playlist') ||
		lower.includes('youtube.com/channel/') ||
		lower.includes('youtube.com/user/') ||
		lower.includes('youtube.com/c/') ||
		lower.includes('youtube.com/@') ||
		lower.includes('list=')
	);
}

function mapExtractErrorMessage(status: number, rawMessage: string, inputUrl: string): string {
	const clean = stripErrorPrefix(rawMessage);
	const text = clean.toLowerCase();
	const urlLower = inputUrl.toLowerCase();
	const hasSingleVideoId = extractYouTubeVideoId(inputUrl) !== null;

	if (
		(!hasSingleVideoId && isBatchLikeYoutubeUrl(urlLower)) ||
		(!hasSingleVideoId && (text.includes('playlist') || text.includes('channel')))
	) {
		return 'Link này là playlist/channel. Hiện tool đang tải từng video; hãy mở 1 video cụ thể rồi thử lại.';
	}

	if (
		text.includes('age-restricted') ||
		text.includes('age restricted') ||
		text.includes('require authentication') ||
		text.includes('sign in') ||
		text.includes('login') ||
		text.includes('private') ||
		text.includes('members-only') ||
		text.includes('restricted')
	) {
		return 'Video này đang bị giới hạn quyền truy cập (private/age/members). Hãy thử video public khác.';
	}

	if (
		text.includes('region') ||
		text.includes('country') ||
		text.includes('geo') ||
		text.includes('not available')
	) {
		return 'Video này có thể bị giới hạn khu vực hoặc tạm thời không khả dụng.';
	}

	if (
		status >= 502 ||
		text.includes('timeout') ||
		text.includes('timed out') ||
		text.includes('network') ||
		text.includes('failed to fetch') ||
		text.includes('gateway') ||
		text.includes('connection')
	) {
		return 'Kết nối đến nguồn video đang chậm hoặc lỗi tạm thời. Vui lòng thử lại sau vài giây.';
	}

	if (
		status === 400 ||
		text.includes('invalid or unsupported url') ||
		text.includes('invalid youtube url') ||
		text.includes('could not extract video id')
	) {
		return 'Link chưa đúng định dạng YouTube video. Hãy dán link dạng /watch?v=... hoặc youtu.be/...';
	}

	if (status >= 500) {
		return 'Không thể phân tích video lúc này. Vui lòng thử lại sau.';
	}

	if (clean) return clean;
	return 'Không thể phân tích link này. Vui lòng thử link YouTube khác.';
}

function normalizeApiBase(base: string): string {
	const trimmed = base.trim().replace(/\/+$/, '');
	if (!trimmed) {
		// Browser fallback: keep API calls on current origin if env is missing.
		if (typeof window !== 'undefined') return window.location.origin;
		return '';
	}

	// If app is loaded on HTTPS, force API endpoint to HTTPS as well.
	if (
		typeof window !== 'undefined' &&
		window.location.protocol === 'https:' &&
		trimmed.startsWith('http://')
	) {
		return `https://${trimmed.slice('http://'.length)}`;
	}

	return trimmed;
}

function buildApiUrl(path: string): string {
	const base = normalizeApiBase(RAW_API_BASE);
	return `${base}${path}`;
}

function isAbortError(error: unknown): boolean {
	return error instanceof DOMException && error.name === 'AbortError';
}

function toAbsoluteApiUrl(url: string): string {
	if (url.startsWith('http://') || url.startsWith('https://')) return url;
	if (url.startsWith('/')) return `${normalizeApiBase(RAW_API_BASE)}${url}`;
	return `${normalizeApiBase(RAW_API_BASE)}/${url}`;
}

async function parseApiError(response: Response, fallbackMessage: string): Promise<Error & { status?: number }> {
	let message = fallbackMessage;
	try {
		const text = await response.text();
		if (text) {
			try {
				const parsed = JSON.parse(text) as { error?: string; message?: string };
				message = parsed.error || parsed.message || text;
			} catch {
				message = text;
			}
		}
	} catch {
		// Ignore parse/read failures and keep fallback message.
	}

	const error = new Error(message) as Error & { status?: number };
	error.status = response.status;
	return error;
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

function computeBackoffWithJitter(
	attempt: number,
	baseDelayMs: number,
	maxDelayMs: number,
	maxJitterMs: number
): number {
	const base = Math.min(baseDelayMs * Math.pow(2, attempt - 1), maxDelayMs);
	const jitter = Math.floor(Math.random() * Math.min(maxJitterMs, Math.floor(base * 0.3)));
	return Math.min(maxDelayMs, base + jitter);
}

async function sleep(ms: number, signal?: AbortSignal): Promise<void> {
	if (ms <= 0) return;
	if (signal?.aborted) throw new DOMException('Operation aborted', 'AbortError');

	await new Promise<void>((resolve, reject) => {
		const timeout = setTimeout(() => {
			if (signal) signal.removeEventListener('abort', onAbort);
			resolve();
		}, ms);

		const onAbort = () => {
			clearTimeout(timeout);
			if (signal) signal.removeEventListener('abort', onAbort);
			reject(new DOMException('Operation aborted', 'AbortError'));
		};

		if (signal) signal.addEventListener('abort', onAbort, { once: true });
	});
}

async function fetchExtractWithRetry(requestUrl: string, signal?: AbortSignal): Promise<Response> {
	let lastError: unknown;

	for (let attempt = 1; attempt <= EXTRACT_MAX_RETRY_ATTEMPTS; attempt += 1) {
		try {
			const response = await fetch('/api/proxy/extract', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ url: requestUrl }),
				signal
			});

			if (response.ok) return response;
			if (!RETRYABLE_HTTP_STATUS.has(response.status) || attempt >= EXTRACT_MAX_RETRY_ATTEMPTS) {
				return response;
			}

			const retryAfterMs = parseRetryAfterHeader(response.headers.get('retry-after'));
			const delayMs =
				retryAfterMs ??
				computeBackoffWithJitter(
					attempt,
					EXTRACT_RETRY_BASE_DELAY_MS,
					EXTRACT_RETRY_MAX_DELAY_MS,
					500
				);

			try {
				await response.text();
			} catch {
				// Ignore body read errors between retry attempts.
			}

			await sleep(delayMs, signal);
		} catch (error) {
			if (isAbortError(error)) throw error;
			lastError = error;
			if (attempt >= EXTRACT_MAX_RETRY_ATTEMPTS) break;
			const delayMs = computeBackoffWithJitter(
				attempt,
				EXTRACT_RETRY_BASE_DELAY_MS,
				EXTRACT_RETRY_MAX_DELAY_MS,
				500
			);
			await sleep(delayMs, signal);
		}
	}

	if (lastError instanceof Error) throw lastError;
	throw new Error('Extraction request failed after retries');
}

function normalizeExtractUrl(url: string): string {
	const videoId = extractYouTubeVideoId(url);
	if (!videoId) return url;
	return `https://www.youtube.com/watch?v=${videoId}`;
}

/**
 * Extract video information from URL
 * @param url - Video URL to extract
 * @returns Extracted video metadata and streams
 * @throws Error if extraction fails
 */
export async function extract(url: string, signal?: AbortSignal): Promise<ExtractResult> {
	const requestUrl = normalizeExtractUrl(url);
	const response = await fetchExtractWithRetry(requestUrl, signal);

	if (!response.ok) {
		const status = response.status;
		const rawBody = await response.text();
		let message = rawBody;

		try {
			const parsed = JSON.parse(rawBody) as { error?: string; message?: string };
			message = parsed.error || parsed.message || rawBody;
		} catch {
			// Keep plain text body as-is when response is not JSON.
		}

		const apiError = new Error(mapExtractErrorMessage(status, message, url)) as Error & {
			status?: number;
		};
		apiError.status = status;
		throw apiError;
	}

	const raw = await response.json();

	// Map backend { status, metadata: { title, formats, ... } } to ExtractResult
	const meta = raw.metadata || {};
	const streams = (meta.formats || []).map((f: any) => {
		// Backend StreamFormat fields: quality, ext, url, has_audio, is_audio_only, codec_label, bitrate, filesize
		return {
			url: f.url,
			formatId: f.format_id,
			quality: f.quality || 'Unknown',
			format: f.ext || 'mp4',
			hasAudio: !!f.has_audio,
			isAudioOnly: !!f.is_audio_only,
			codecLabel: f.codec_label,
			bitrate: f.bitrate,
			size: f.filesize
		};
	});

	return {
		title: meta.title || 'Unknown',
		channel: meta.channel,
		viewCount: meta.view_count,
		description: meta.description,
		streams,
		originalUrl: meta.original_url,
		platform: getPlatform(url),
		thumbnail: meta.thumbnail,
		duration: meta.duration
	};
}

/**
 * Build stream download URL with query parameters
 * @param streamUrl - Direct stream URL from extract
 * @param title - Video title for filename
 * @param format - File format extension
 * @returns Full download URL
 */
export function buildStreamUrl(
	streamUrl: string,
	title: string,
	format: string = 'mp4',
	options?: {
		sourceUrl?: string;
		formatId?: string;
	}
): string {
	const params = new URLSearchParams({
		url: streamUrl,
		title,
		format
	});
	if (options?.sourceUrl) params.set('source_url', options.sourceUrl);
	if (options?.formatId) params.set('format_id', options.formatId);
	return `${buildApiUrl('/api/stream')}?${params.toString()}`;
}

/**
 * Build muxed stream download URL (video + audio → fMP4)
 * @param videoUrl - Video-only stream URL
 * @param audioUrl - Audio-only stream URL
 * @param title - Video title for filename
 * @returns Full muxed download URL
 */
export function buildMuxedStreamUrl(
	videoUrl: string,
	audioUrl: string,
	title: string,
	options?: {
		sourceUrl?: string;
		videoFormatId?: string;
		audioFormatId?: string;
	}
): string {
	const params = new URLSearchParams({
		video_url: videoUrl,
		audio_url: audioUrl,
		title
	});
	if (options?.sourceUrl) params.set('source_url', options.sourceUrl);
	if (options?.videoFormatId) params.set('video_format_id', options.videoFormatId);
	if (options?.audioFormatId) params.set('audio_format_id', options.audioFormatId);
	return `${buildApiUrl('/api/stream/muxed')}?${params.toString()}`;
}

export async function createMuxedDownloadJob(
	videoUrl: string,
	audioUrl: string,
	title: string,
	options?: {
		sourceUrl?: string;
		videoFormatId?: string;
		audioFormatId?: string;
	},
	signal?: AbortSignal
): Promise<{ jobId: string; statusUrl: string; fileUrl: string }> {
	const response = await fetch(buildApiUrl('/api/stream/muxed/jobs'), {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({
			video_url: videoUrl,
			audio_url: audioUrl,
			title,
			source_url: options?.sourceUrl,
			video_format_id: options?.videoFormatId,
			audio_format_id: options?.audioFormatId
		}),
		signal
	});

	if (!response.ok) {
		throw await parseApiError(response, `Failed to create mux job (${response.status})`);
	}

	const data = (await response.json()) as CreateMuxJobApiResponse;
	return {
		jobId: data.job_id,
		statusUrl: toAbsoluteApiUrl(data.status_url),
		fileUrl: toAbsoluteApiUrl(data.file_url)
	};
}

export async function waitForMuxedDownloadJobReady(
	jobId: string,
	options?: {
		timeoutMs?: number;
		pollIntervalMs?: number;
	},
	signal?: AbortSignal
): Promise<string> {
	const startedAt = Date.now();
	const timeoutMs = options?.timeoutMs ?? MUX_JOB_MAX_WAIT_MS;
	const pollIntervalMs = options?.pollIntervalMs ?? MUX_JOB_POLL_INTERVAL_MS;
	const statusUrl = buildApiUrl(`/api/stream/muxed/jobs/${encodeURIComponent(jobId)}`);

	while (true) {
		if (signal?.aborted) throw new DOMException('Operation aborted', 'AbortError');
		if (Date.now() - startedAt > timeoutMs) {
			throw new Error(`Mux job timed out after ${Math.ceil(timeoutMs / 1000)} seconds`);
		}

		const response = await fetch(statusUrl, { signal });
		if (!response.ok) {
			throw await parseApiError(response, `Failed to query mux job status (${response.status})`);
		}

		const status = (await response.json()) as MuxJobStatusApiResponse;
		if (status.status === 'ready' && status.file_url) {
			return toAbsoluteApiUrl(status.file_url);
		}
		if (status.status === 'failed') {
			throw new Error(status.error || 'Mux job failed');
		}

		await sleep(pollIntervalMs, signal);
	}
}

/**
 * Subscribe to batch download SSE stream
 * @param url - Channel/playlist URL
 * @param onMessage - Callback for each batch message
 * @param onError - Callback for errors
 * @returns EventSource instance for cleanup
 */
export function subscribeBatch(
	url: string,
	onMessage: (msg: BatchMessage) => void,
	onError?: (error: Event) => void
): BatchSubscription {
	const encodedUrl = encodeURIComponent(url);
	let es: EventSource | null = null;
	let closed = false;
	let reconnectAttempts = 0;
	let reconnectTimer: ReturnType<typeof setTimeout> | null = null;

	const clearReconnectTimer = () => {
		if (!reconnectTimer) return;
		clearTimeout(reconnectTimer);
		reconnectTimer = null;
	};

	const connect = () => {
		if (closed) return;

		es = new EventSource(`${buildApiUrl('/api/batch')}?url=${encodedUrl}`);
		es.onopen = () => {
			reconnectAttempts = 0;
		};

		es.onmessage = (event) => {
			try {
				const data: BatchMessage = JSON.parse(event.data);
				onMessage(data);
			} catch (err) {
				console.error('Failed to parse SSE message:', err);
			}
		};

		es.onerror = (err) => {
			if (closed) return;
			// Browser EventSource already auto-reconnects while CONNECTING.
			if (es?.readyState === EventSource.CONNECTING) return;

			if (reconnectAttempts >= BATCH_MAX_RECONNECT_ATTEMPTS) {
				onError?.(err);
				return;
			}

			reconnectAttempts += 1;
			const delayMs = computeBackoffWithJitter(
				reconnectAttempts,
				BATCH_RECONNECT_BASE_DELAY_MS,
				BATCH_RECONNECT_MAX_DELAY_MS,
				800
			);
			try {
				es?.close();
			} catch {
				// Ignore close errors while reconnecting.
			}
			clearReconnectTimer();
			reconnectTimer = setTimeout(() => {
				reconnectTimer = null;
				connect();
			}, delayMs);
		};
	};

	connect();

	return {
		close: () => {
			closed = true;
			clearReconnectTimer();
			try {
				es?.close();
			} catch {
				// Ignore close errors.
			}
			es = null;
		}
	};
}

function isYouTubeHost(hostname: string): boolean {
	const host = hostname.toLowerCase().replace(/^www\./, '');
	return host === 'youtube.com' || host.endsWith('.youtube.com') || host === 'youtu.be';
}

export function extractYouTubeVideoId(url: string): string | null {
	try {
		const parsed = new URL(url.trim());
		if (!isYouTubeHost(parsed.hostname)) return null;
		const host = parsed.hostname.toLowerCase().replace(/^www\./, '');
		const isValidId = (id: string | null): id is string =>
			!!id && /^[A-Za-z0-9_-]{11}$/.test(id);

		if (host === 'youtu.be') {
			const firstSegment = parsed.pathname.split('/').filter(Boolean)[0] ?? null;
			return isValidId(firstSegment) ? firstSegment : null;
		}

		const v = parsed.searchParams.get('v');
		if (isValidId(v)) return v;

		const shortsMatch = parsed.pathname.match(/^\/shorts\/([A-Za-z0-9_-]{11})(?:[/?#]|$)/);
		if (shortsMatch) return shortsMatch[1];

		const liveMatch = parsed.pathname.match(/^\/live\/([A-Za-z0-9_-]{11})(?:[/?#]|$)/);
		if (liveMatch) return liveMatch[1];

		const embedMatch = parsed.pathname.match(/^\/embed\/([A-Za-z0-9_-]{11})(?:[/?#]|$)/);
		if (embedMatch) return embedMatch[1];

		return null;
	} catch {
		return null;
	}
}

/**
 * Validate video URL (YouTube only)
 * @param url - URL to validate
 * @returns True if valid video URL
 */
export function isValidVideoUrl(url: string): boolean {
	try {
		const parsed = new URL(url.trim());
		if (!(parsed.protocol === 'http:' || parsed.protocol === 'https:')) return false;
		if (!isYouTubeHost(parsed.hostname)) return false;

		// Accept real video URLs and playlist links (needed by BatchInput).
		if (extractYouTubeVideoId(url)) return true;
		return parsed.searchParams.has('list');
	} catch {
		return false;
	}
}

/**
 * Get platform from URL
 * @param url - Video URL
 * @returns Platform name or 'unknown'
 */
export function getPlatform(url: string): 'youtube' | 'unknown' {
	return isValidVideoUrl(url) ? 'youtube' : 'unknown';
}
