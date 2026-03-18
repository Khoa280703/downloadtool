import type { RequestHandler } from './$types';

import {
	applyNoStoreCache,
	buildRustApiHeaders,
	buildRustApiUrl,
	copyRustResponseHeaders,
	ensureDownloadSessionId
} from '$lib/server/rust-api-proxy';
import { deriveAuditOutcome, logAuditEvent } from '$lib/server/audit-log';

type StartPlaylistBody = {
	selected_video_ids?: string[];
};

export const POST: RequestHandler = async ({ params, request, fetch, cookies, locals, url }) => {
	const body = await request.text();
	const downloadSessionId = ensureDownloadSessionId(cookies);
	const upstream = await fetch(
		buildRustApiUrl(`/api/playlist-jobs/${encodeURIComponent(params.jobId)}/start`),
		{
			method: 'POST',
			body,
			headers: await buildRustApiHeaders(request, true, downloadSessionId),
			signal: request.signal
		}
	);

	let parsedBody: StartPlaylistBody | null = null;
	try {
		parsedBody = JSON.parse(body) as StartPlaylistBody;
	} catch {
		parsedBody = null;
	}

	await logAuditEvent(
		{ request, locals, cookies, url },
		{
			scope: 'playlist',
			eventType: 'playlist_job.start',
			entityId: params.jobId,
			statusCode: upstream.status,
			outcome: deriveAuditOutcome(upstream.status),
			detail: upstream.ok ? null : `HTTP ${upstream.status}`,
			payload: {
				selectedVideoIds: parsedBody?.selected_video_ids ?? []
			}
		}
	);

	return new Response(upstream.body, {
		status: upstream.status,
		headers: applyNoStoreCache(copyRustResponseHeaders(upstream))
	});
};
