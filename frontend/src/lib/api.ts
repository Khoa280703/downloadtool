/**
 * Typed API client for video downloader backend
 * @module api
 */

import type { ExtractResult, BatchMessage } from './types';

const RAW_API_BASE = import.meta.env.VITE_API_URL || '';

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
	const response = await fetch('/api/proxy/extract', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ url: requestUrl }),
		signal
	});

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
	format: string = 'mp4'
): string {
	const params = new URLSearchParams({
		url: streamUrl,
		title,
		format
	});
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
	title: string
): string {
	const params = new URLSearchParams({
		video_url: videoUrl,
		audio_url: audioUrl,
		title
	});
	return `${buildApiUrl('/api/stream/muxed')}?${params.toString()}`;
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
): EventSource {
	const encodedUrl = encodeURIComponent(url);
	const es = new EventSource(`${buildApiUrl('/api/batch')}?url=${encodedUrl}`);

	es.onmessage = (event) => {
		try {
			const data: BatchMessage = JSON.parse(event.data);
			onMessage(data);
		} catch (err) {
			console.error('Failed to parse SSE message:', err);
		}
	};

	es.onerror = (err) => {
		console.error('SSE error:', err);
		onError?.(err);
		es.close();
	};

	return es;
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
