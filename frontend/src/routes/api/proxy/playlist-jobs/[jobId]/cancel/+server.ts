import type { RequestHandler } from './$types';

import {
	applyNoStoreCache,
	buildRustApiHeaders,
	buildRustApiUrl,
	copyRustResponseHeaders,
	ensureDownloadSessionId
} from '$lib/server/rust-api-proxy';
import { deriveAuditOutcome, logAuditEvent } from '$lib/server/audit-log';

export const POST: RequestHandler = async ({ params, request, fetch, cookies, locals, url }) => {
	const downloadSessionId = ensureDownloadSessionId(cookies);
	const upstream = await fetch(
		buildRustApiUrl(`/api/playlist-jobs/${encodeURIComponent(params.jobId)}/cancel`),
		{
			method: 'POST',
			headers: await buildRustApiHeaders(request, false, downloadSessionId),
			signal: request.signal
		}
	);

	await logAuditEvent(
		{ request, locals, cookies, url },
		{
			scope: 'playlist',
			eventType: 'playlist_job.cancel',
			entityId: params.jobId,
			statusCode: upstream.status,
			outcome: deriveAuditOutcome(upstream.status),
			detail: upstream.ok ? null : `HTTP ${upstream.status}`
		}
	);

	return new Response(upstream.body, {
		status: upstream.status,
		headers: applyNoStoreCache(copyRustResponseHeaders(upstream))
	});
};
