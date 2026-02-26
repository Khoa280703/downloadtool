/**
 * Typed API client for video downloader backend
 * @module api
 */

import type { ExtractResult, BatchMessage } from './types';

const API_BASE = import.meta.env.VITE_API_URL || '';

/**
 * Extract video information from URL
 * @param url - Video URL to extract
 * @returns Extracted video metadata and streams
 * @throws Error if extraction fails
 */
export async function extract(url: string): Promise<ExtractResult> {
	const response = await fetch(`${API_BASE}/api/extract`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ url })
	});

	if (!response.ok) {
		const error = await response.text();
		throw new Error(`Extract failed: ${error}`);
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
	return `${API_BASE}/api/stream?${params.toString()}`;
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
	return `${API_BASE}/api/stream/muxed?${params.toString()}`;
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
	const es = new EventSource(`${API_BASE}/api/batch?url=${encodedUrl}`);

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

/**
 * Validate video URL (YouTube only)
 * @param url - URL to validate
 * @returns True if valid video URL
 */
export function isValidVideoUrl(url: string): boolean {
	const patterns = [/^https?:\/\/(www\.)?(youtube\.com|youtu\.be)\/.+/];
	return patterns.some((p) => p.test(url));
}

/**
 * Get platform from URL
 * @param url - Video URL
 * @returns Platform name or 'unknown'
 */
export function getPlatform(url: string): 'youtube' | 'unknown' {
	if (/^https?:\/\/(www\.)?(youtube\.com|youtu\.be)\/.+/.test(url)) {
		return 'youtube';
	}
	return 'unknown';
}
