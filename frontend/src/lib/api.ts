/**
 * Typed API client for video downloader backend
 * @module api
 */

import * as m from '$lib/paraglide/messages';
import type { ExtractResult, BatchMessage } from './types';
import { apiClientLimitConfig, muxJobClientLimitConfig } from './runtime-limit-config';

const RAW_API_BASE = import.meta.env.VITE_API_URL || '';
const RETRYABLE_HTTP_STATUS = new Set([429, 502, 503, 504]);
const EXTRACT_MAX_RETRY_ATTEMPTS = apiClientLimitConfig.extractMaxRetryAttempts;
const EXTRACT_RETRY_BASE_DELAY_MS = apiClientLimitConfig.extractRetryBaseDelayMs;
const EXTRACT_RETRY_MAX_DELAY_MS = apiClientLimitConfig.extractRetryMaxDelayMs;
const BATCH_MAX_RECONNECT_ATTEMPTS = apiClientLimitConfig.batchMaxReconnectAttempts;
const BATCH_RECONNECT_BASE_DELAY_MS = apiClientLimitConfig.batchReconnectBaseDelayMs;
const BATCH_RECONNECT_MAX_DELAY_MS = apiClientLimitConfig.batchReconnectMaxDelayMs;
const MUX_JOB_POLL_INTERVAL_MS = muxJobClientLimitConfig.pollIntervalMs;
const MUX_JOB_MAX_WAIT_MS = muxJobClientLimitConfig.maxWaitMs;

export type BatchSubscription = {
	close: () => void;
};

type CreateMuxJobApiResponse = {
	job_id: string;
	status: string;
	poll_url?: string;
	status_url?: string;
	file_ticket_url?: string;
	file_url?: string;
};

type MuxJobStatusApiResponse = {
	job_id: string;
	status: 'queued' | 'leased' | 'processing' | 'ready' | 'failed' | 'expired';
	error?: string | null;
	file_ticket_url?: string | null;
	file_url?: string | null;
};

export type MuxJobLifecycleStatus = MuxJobStatusApiResponse['status'];

export type MuxJobStatusUpdate = {
	jobId: string;
	status: MuxJobLifecycleStatus;
	elapsedMs: number;
	pollCount: number;
};

type FileTicketApiResponse = {
	job_id: string;
	download_url: string;
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
		return m.api_extract_error_playlist_channel();
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
		return m.api_extract_error_restricted();
	}

	if (
		text.includes('region') ||
		text.includes('country') ||
		text.includes('geo') ||
		text.includes('not available')
	) {
		return m.api_extract_error_region_blocked();
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
		return m.api_extract_error_temporary_network();
	}

	if (
		status === 400 ||
		text.includes('invalid or unsupported url') ||
		text.includes('invalid youtube url') ||
		text.includes('could not extract video id')
	) {
		return m.api_extract_error_invalid_youtube_url();
	}

	if (status >= 500) {
		return m.api_extract_error_server_unavailable();
	}

	if (clean) return clean;
	return m.api_extract_error_unknown();
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

function toAbsoluteDownloadUrl(url: string): string {
	if (url.startsWith('http://') || url.startsWith('https://')) return url;
	if (url.startsWith('/api/proxy/')) {
		if (typeof window !== 'undefined') return `${window.location.origin}${url}`;
		return url;
	}
	return toAbsoluteApiUrl(url);
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
	throw new Error(m.api_extract_request_failed_after_retries());
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
			quality: f.quality || m.api_unknown_quality(),
			format: f.ext || 'mp4',
			hasAudio: !!f.has_audio,
			isAudioOnly: !!f.is_audio_only,
			codecLabel: f.codec_label,
			bitrate: f.bitrate,
			size: f.filesize
		};
	});

	return {
		title: meta.title || m.api_unknown_title(),
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
		patchInitMetadata?: boolean;
	}
): string {
	const params = new URLSearchParams({
		url: streamUrl,
		title,
		format
	});
	if (options?.sourceUrl) params.set('source_url', options.sourceUrl);
	if (options?.formatId) params.set('format_id', options.formatId);
	if (options?.patchInitMetadata) params.set('patch_init_metadata', 'true');
	return `${buildApiUrl('/api/stream')}?${params.toString()}`;
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
	const response = await fetch('/api/proxy/jobs', {
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
		throw await parseApiError(
			response,
			m.api_mux_create_failed({ status: String(response.status) })
		);
	}

	const data = (await response.json()) as CreateMuxJobApiResponse;
	return {
		jobId: data.job_id,
		statusUrl: toAbsoluteApiUrl(data.poll_url ?? data.status_url ?? `/api/jobs/${data.job_id}`),
		fileUrl: toAbsoluteApiUrl(
			data.file_ticket_url ?? data.file_url ?? `/api/jobs/${data.job_id}/file-ticket`
		)
	};
}

export async function waitForMuxedDownloadJobReady(
	jobId: string,
	options?: {
		timeoutMs?: number;
		pollIntervalMs?: number;
		onStatus?: (update: MuxJobStatusUpdate) => void;
	},
	signal?: AbortSignal
): Promise<string> {
	const startedAt = Date.now();
	const timeoutMs = options?.timeoutMs ?? MUX_JOB_MAX_WAIT_MS;
	const pollIntervalMs = options?.pollIntervalMs ?? MUX_JOB_POLL_INTERVAL_MS;
	const jobPath = `/api/proxy/jobs/${encodeURIComponent(jobId)}`;
	let pollCount = 0;

	while (true) {
		if (signal?.aborted) throw new DOMException('Operation aborted', 'AbortError');
		if (Date.now() - startedAt > timeoutMs) {
			throw new Error(
				m.api_mux_job_timed_out({ seconds: String(Math.ceil(timeoutMs / 1000)) })
			);
		}

		const cacheBuster = Date.now().toString();
		const response = await fetch(`${jobPath}?t=${cacheBuster}`, { signal });
		if (!response.ok) {
			throw await parseApiError(
				response,
				m.api_mux_query_status_failed({ status: String(response.status) })
			);
		}

		const status = (await response.json()) as MuxJobStatusApiResponse;
		pollCount += 1;
		options?.onStatus?.({
			jobId: status.job_id,
			status: status.status,
			elapsedMs: Date.now() - startedAt,
			pollCount
		});
		if (status.status === 'ready') {
			const ticketResponse = await fetch(`${jobPath}/file-ticket?t=${cacheBuster}`, { signal });
			if (!ticketResponse.ok) {
				throw await parseApiError(
					ticketResponse,
					m.api_mux_file_ticket_failed({ status: String(ticketResponse.status) })
				);
			}
			const ticket = (await ticketResponse.json()) as FileTicketApiResponse;
			return toAbsoluteDownloadUrl(appendCacheBuster(ticket.download_url, cacheBuster));
		}
		if (status.status === 'failed') {
			throw new Error(status.error || m.api_mux_job_failed());
		}
		if (status.status === 'expired') {
			throw new Error(status.error || m.api_mux_job_expired());
		}

		await sleep(pollIntervalMs, signal);
	}
}

function appendCacheBuster(url: string, value: string): string {
	const separator = url.includes('?') ? '&' : '?';
	return `${url}${separator}t=${encodeURIComponent(value)}`;
}

export async function releaseMuxedDownloadJob(
	jobId: string,
	signal?: AbortSignal
): Promise<boolean> {
	const response = await fetch(`/api/proxy/jobs/${encodeURIComponent(jobId)}/release`, {
		method: 'POST',
		signal
	});

	if (!response.ok) {
		throw await parseApiError(
			response,
			m.api_mux_release_failed({ status: String(response.status) })
		);
	}

	const payload = (await response.json()) as { released?: boolean };
	return payload.released === true;
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

		// Accept real video URLs and playlist links for homepage playlist mode.
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
