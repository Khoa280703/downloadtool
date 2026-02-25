import type { BatchMessage, DownloadTask } from './types';

/**
 * Browser download pool manager - maintains max 3 concurrent downloads
 * Uses `<a download>` trigger for iOS Safari compatibility
 * @module download-pool
 */

export class DownloadPool {
	private queue: DownloadTask[] = [];
	private active = 0;
	private readonly maxConcurrent = 3;
	private taskIdCounter = 0;

	/**
	 * Add a download task to the queue
	 * @param url - Stream URL to download
	 * @param filename - Output filename
	 * @returns Task ID
	 */
	add(url: string, filename: string): string {
		const id = `task-${++this.taskIdCounter}`;
		const task: DownloadTask = {
			id,
			url,
			filename,
			status: 'pending'
		};
		this.queue.push(task);
		this.processQueue();
		return id;
	}

	/**
	 * Process the next items in queue if slots available
	 */
	private processQueue(): void {
		while (this.active < this.maxConcurrent && this.queue.length > 0) {
			const task = this.queue.find(t => t.status === 'pending');
			if (!task) break;

			task.status = 'downloading';
			this.active++;
			this.triggerDownload(task);
		}
	}

	/**
	 * Trigger browser download using anchor tag
	 * iOS Safari compatible - uses direct click on anchor
	 */
	private triggerDownload(task: DownloadTask): void {
		const anchor = document.createElement('a');
		anchor.href = task.url;
		anchor.download = task.filename;
		anchor.style.display = 'none';
		anchor.target = '_blank';

		document.body.appendChild(anchor);

		// Trigger download
		anchor.click();

		// Cleanup
		setTimeout(() => {
			document.body.removeChild(anchor);
			task.status = 'completed';
			this.active--;
			this.processQueue();
		}, 1000);
	}

	/**
	 * Get current queue status
	 */
	getStatus(): { queue: number; active: number; max: number } {
		return {
			queue: this.queue.filter(t => t.status === 'pending').length,
			active: this.active,
			max: this.maxConcurrent
		};
	}

	/**
	 * Clear all pending tasks
	 */
	clear(): void {
		this.queue = this.queue.filter(t => t.status !== 'pending');
	}
}

/**
 * Global download pool instance
 */
export const downloadPool = new DownloadPool();

/**
 * Subscribe to batch SSE stream and add downloads to pool
 * @param url - Batch API URL
 * @param onMessage - Callback for each message
 * @returns EventSource for cleanup
 */
export function subscribeBatch(
	url: string,
	onMessage: (msg: BatchMessage) => void
): EventSource {
	const es = new EventSource(url);

	es.onmessage = (event) => {
		try {
			const data: BatchMessage = JSON.parse(event.data);
			onMessage(data);

			if (data.type === 'progress' && data.url) {
				const title = data.title ?? 'video';
				const filename = `${title.replace(/[^a-z0-9]/gi, '_')}.mp4`;
				downloadPool.add(data.url, filename);
			}
		} catch (err) {
			console.error('Failed to parse SSE message:', err);
		}
	};

	es.onerror = (err) => {
		console.error('SSE error:', err);
		es.close();
	};

	return es;
}
