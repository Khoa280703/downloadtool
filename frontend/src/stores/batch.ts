/**
 * Batch download state management store
 * @module stores/batch
 */

import { writable } from 'svelte/store';
import type { BatchItem, BatchProgress } from '$lib/types';

/** Batch queue items */
export const batchQueue = writable<BatchItem[]>([]);

/** Batch progress state */
export const batchProgress = writable<BatchProgress>({ received: 0, total: 0 });

/** Whether batch is currently active */
export const isBatchActive = writable<boolean>(false);

/** Current SSE connection */
export const batchEventSource = writable<EventSource | null>(null);

/**
 * Reset batch state
 */
export function resetBatch(): void {
	batchQueue.set([]);
	batchProgress.set({ received: 0, total: 0 });
	isBatchActive.set(false);
	batchEventSource.update((es) => {
		if (es) {
			es.close();
		}
		return null;
	});
}

/**
 * Add item to batch queue
 */
export function addBatchItem(item: BatchItem): void {
	batchQueue.update((queue) => {
		const existingIndex = queue.findIndex((queued) => queued.videoId === item.videoId);
		if (existingIndex === -1) return [...queue, { ...item, selected: item.selected ?? true }];
		const next = [...queue];
		next[existingIndex] = {
			...next[existingIndex],
			...item,
			selected: item.selected ?? next[existingIndex].selected ?? true
		};
		return next;
	});
}

/**
 * Update batch item status
 */
export function updateBatchItem(url: string, status: BatchItem['status'], error?: string): void {
	batchQueue.update((queue) =>
		queue.map((item) => (item.url === url ? { ...item, status, error } : item))
	);
}

/**
 * Update batch item status by stable video ID.
 */
export function updateBatchItemByVideoId(
	videoId: string,
	status: BatchItem['status'],
	error?: string
): void {
	batchQueue.update((queue) =>
		queue.map((item) => (item.videoId === videoId ? { ...item, status, error } : item))
	);
}

/**
 * Toggle selection for a batch item.
 */
export function setBatchItemSelected(videoId: string, selected: boolean): void {
	batchQueue.update((queue) =>
		queue.map((item) => (item.videoId === videoId ? { ...item, selected } : item))
	);
}

/**
 * Set selection for all pending batch items.
 */
export function setAllPendingBatchItemsSelected(selected: boolean): void {
	batchQueue.update((queue) =>
		queue.map((item) => (item.status === 'pending' ? { ...item, selected } : item))
	);
}

/**
 * Set batch progress
 */
export function setBatchProgress(received: number, total: number): void {
	batchProgress.set({ received, total });
}

/**
 * Start batch download
 */
export function startBatch(): void {
	isBatchActive.set(true);
}

/**
 * Complete batch download
 */
export function completeBatch(): void {
	isBatchActive.set(false);
}

/**
 * Set SSE connection
 */
export function setEventSource(es: EventSource): void {
	batchEventSource.set(es);
}
