/**
 * Client API for backend playlist job orchestration.
 * Creates jobs, subscribes to SSE events, and triggers file saves when items are ready.
 * @module playlist-job-api
 */

import { toAbsoluteDownloadUrl } from './api';

export type PlaylistJobItem = {
	id: string;
	video_id: string;
	title: string | null;
	thumbnail: string | null;
	ordinal: number;
	status: string;
	attempt_count: number;
	last_error: string | null;
	selected: boolean;
	mux_job_id: string | null;
	download_url: string | null;
	progress: PlaylistJobItemProgress | null;
};

export type PlaylistJobItemProgress = {
	phase: string;
	percent: number | null;
	uploaded_bytes: number;
	total_bytes: number | null;
	updated_at_ms: number;
};

export type PlaylistJobSnapshot = {
	job_id: string;
	status: string;
	source_url: string;
	title: string | null;
	total_items: number;
	completed_items: number;
	failed_items: number;
	requested_quality: string;
	requested_mode: string;
	created_at_ms: number;
	updated_at_ms: number;
	items: PlaylistJobItem[];
};

type CreatePlaylistJobResponse = {
	job_id: string;
	status: string;
};

type StartPlaylistJobResponse = {
	started: boolean;
	selected_items: number;
};

export type PlaylistJobEventSubscription = {
	close: () => void;
};

/**
 * Create a backend playlist job.
 */
export async function createPlaylistJob(
	url: string,
	signal?: AbortSignal
): Promise<CreatePlaylistJobResponse> {
	const response = await fetch('/api/proxy/playlist-jobs', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ url }),
		signal
	});

	if (!response.ok) {
		let message = `Failed to create playlist job (${response.status})`;
		try {
			const body = await response.json();
			message = (body as { message?: string }).message || message;
		} catch {
			// keep default message
		}
		throw new Error(message);
	}

	return (await response.json()) as CreatePlaylistJobResponse;
}

/**
 * Fetch current playlist job status + items.
 */
export async function getPlaylistJob(
	jobId: string,
	signal?: AbortSignal
): Promise<PlaylistJobSnapshot> {
	const response = await fetch(
		`/api/proxy/playlist-jobs/${encodeURIComponent(jobId)}?t=${Date.now()}`,
		{ signal }
	);

	if (!response.ok) {
		throw new Error(`Failed to fetch playlist job (${response.status})`);
	}

	return (await response.json()) as PlaylistJobSnapshot;
}

/**
 * Cancel a running playlist job.
 */
export async function cancelPlaylistJob(
	jobId: string,
	signal?: AbortSignal
): Promise<{ cancelled: boolean; cancelled_items: number }> {
	const response = await fetch(
		`/api/proxy/playlist-jobs/${encodeURIComponent(jobId)}/cancel`,
		{
			method: 'POST',
			signal
		}
	);

	if (!response.ok) {
		throw new Error(`Failed to cancel playlist job (${response.status})`);
	}

	return (await response.json()) as { cancelled: boolean; cancelled_items: number };
}

/**
 * Start processing the selected playlist items.
 */
export async function startPlaylistJob(
	jobId: string,
	selectedVideoIds: string[],
	quality?: string,
	mode?: string,
	signal?: AbortSignal
): Promise<StartPlaylistJobResponse> {
	const response = await fetch(
		`/api/proxy/playlist-jobs/${encodeURIComponent(jobId)}/start`,
		{
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				selected_video_ids: selectedVideoIds,
				quality,
				mode
			}),
			signal
		}
	);

	if (!response.ok) {
		let message = `Failed to start playlist job (${response.status})`;
		try {
			const body = await response.json();
			message = (body as { message?: string; error?: string }).message || (body as { error?: string }).error || message;
		} catch {
			// keep default message
		}
		throw new Error(message);
	}

	return (await response.json()) as StartPlaylistJobResponse;
}

/**
 * Subscribe to SSE events for a playlist job.
 * Calls onSnapshot with each status update from the backend.
 */
export function subscribePlaylistJobEvents(
	jobId: string,
	onSnapshot: (snapshot: PlaylistJobSnapshot) => void,
	onError?: (error: Event) => void
): PlaylistJobEventSubscription {
	const url = `/api/proxy/playlist-jobs/${encodeURIComponent(jobId)}/events?t=${Date.now()}`;
	const es = new EventSource(url);
	let closed = false;

	es.addEventListener('status', (rawEvent) => {
		try {
			const event = rawEvent as MessageEvent<string>;
			const snapshot = JSON.parse(event.data) as PlaylistJobSnapshot;
			onSnapshot(snapshot);

			// Auto-close on terminal status
			if (snapshot.status === 'completed' || snapshot.status === 'failed' || snapshot.status === 'cancelled') {
				closed = true;
				es.close();
			}
		} catch {
			// ignore parse errors
		}
	});

	es.onerror = (err) => {
		if (closed) return;
		onError?.(err);
	};

	return {
		close: () => {
			if (closed) return;
			closed = true;
			es.close();
		}
	};
}

/**
 * Resolve download URL from a playlist item to an absolute URL.
 */
export async function resolvePlaylistItemDownloadUrl(
	item: PlaylistJobItem,
	signal?: AbortSignal
): Promise<string | null> {
	if (!item.download_url) return null;

	const absoluteUrl = toAbsoluteDownloadUrl(item.download_url);
	if (!absoluteUrl.endsWith('/file-ticket') && !absoluteUrl.includes('/file-ticket?')) {
		return absoluteUrl;
	}

	const ticketResponse = await fetch(
		`${absoluteUrl}${absoluteUrl.includes('?') ? '&' : '?'}t=${Date.now()}`,
		{ signal }
	);

	if (!ticketResponse.ok) {
		throw new Error(`Failed to resolve playlist file ticket (${ticketResponse.status})`);
	}

	const ticket = (await ticketResponse.json()) as { download_url?: string };
	if (!ticket.download_url) {
		throw new Error('Playlist file ticket is missing download_url');
	}

	return toAbsoluteDownloadUrl(ticket.download_url);
}
