/**
 * Download state management store
 * @module stores/download
 */

import { writable } from 'svelte/store';
import type { DownloadState, Stream } from '$lib/types';

/** Initial download state */
const initialState: DownloadState = {
	videoUrl: '',
	selectedStream: null,
	isExtracting: false,
	isDownloading: false,
	error: null
};

/** Current download state */
export const currentDownload = writable<DownloadState>({ ...initialState });

/** Download progress (0-100) */
export const downloadProgress = writable<number>(0);

/**
 * Reset download state to initial
 */
export function resetDownload(): void {
	currentDownload.set({ ...initialState });
	downloadProgress.set(0);
}

/**
 * Set video URL and start extracting
 */
export function setVideoUrl(url: string): void {
	currentDownload.update((state) => ({
		...state,
		videoUrl: url,
		isExtracting: true,
		error: null
	}));
}

/**
 * Set extracted streams and stop loading
 */
export function setExtracted(streams: Stream[]): void {
	currentDownload.update((state) => ({
		...state,
		selectedStream: streams[0] || null,
		isExtracting: false
	}));
}

/**
 * Select a specific stream
 */
export function selectStream(stream: Stream): void {
	currentDownload.update((state) => ({
		...state,
		selectedStream: stream
	}));
}

/**
 * Set downloading state
 */
export function setDownloading(isDownloading: boolean): void {
	currentDownload.update((state) => ({
		...state,
		isDownloading
	}));
}

/**
 * Set error state
 */
export function setError(error: string): void {
	currentDownload.update((state) => ({
		...state,
		error,
		isExtracting: false,
		isDownloading: false
	}));
}
