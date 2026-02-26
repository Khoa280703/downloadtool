/**
 * Type definitions for video downloader frontend
 * @module types
 */

/** Supported video platforms */
export type Platform = 'youtube' | 'unknown';

/** Video stream information */
export interface Stream {
	/** Direct stream URL */
	url: string;
	/** Quality label (e.g., '4K', '1080p', '720p') */
	quality: string;
	/** Format (e.g., 'mp4', 'webm', 'mp3') */
	format: string;
	/** Whether this stream has an audio track */
	hasAudio: boolean;
	/** Whether this stream is audio-only (no video) */
	isAudioOnly: boolean;
	/** Human-readable codec label (e.g., 'H.264', 'VP9', 'AV1', 'AAC') */
	codecLabel?: string;
	/** Bitrate in bits per second */
	bitrate?: number;
	/** File size in bytes (if known) */
	size?: number;
}

/** Result from extract API */
export interface ExtractResult {
	/** Video title */
	title: string;
	/** Channel/author name */
	channel?: string;
	/** View count */
	viewCount?: number;
	/** Video description */
	description?: string;
	/** Available streams */
	streams: Stream[];
	/** Detected platform */
	platform: Platform;
	/** Video thumbnail URL */
	thumbnail?: string;
	/** Video duration in seconds */
	duration?: number;
}

/** Batch message from SSE stream */
export interface BatchMessage {
	/** Message type */
	type: 'link' | 'done' | 'progress' | 'error';
	/** Video URL */
	url?: string;
	/** Video title */
	title?: string;
	/** Index in batch (1-based) */
	index?: number;
	/** Total videos in batch */
	total?: number;
	/** Error message (if type is 'error') */
	message?: string;
}

/** Download task state */
export interface DownloadTask {
	/** Unique task ID */
	id: string;
	/** Stream URL */
	url: string;
	/** Output filename */
	filename: string;
	/** Current status */
	status: 'pending' | 'downloading' | 'completed' | 'error';
	/** Error message if failed */
	error?: string;
}

/** Download state for single download */
export interface DownloadState {
	/** Video URL being processed */
	videoUrl: string;
	/** Selected stream */
	selectedStream: Stream | null;
	/** Whether extracting info */
	isExtracting: boolean;
	/** Whether downloading */
	isDownloading: boolean;
	/** Error message */
	error: string | null;
}

/** Batch item in queue */
export interface BatchItem {
	/** Video URL */
	url: string;
	/** Video title */
	title: string;
	/** Current status */
	status: 'pending' | 'downloading' | 'completed' | 'error';
	/** Error message if failed */
	error?: string;
}

/** Batch progress state */
export interface BatchProgress {
	/** Number of items processed */
	received: number;
	/** Total items to process */
	total: number;
}
