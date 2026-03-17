import type { RequestHandler } from './$types';

import {
	applyNoStoreCache,
	buildRustApiHeaders,
	buildRustApiUrl,
	ensureDownloadSessionId
} from '$lib/server/rust-api-proxy';
import { deriveAuditOutcome, logAuditEvent } from '$lib/server/audit-log';

export const GET: RequestHandler = async ({ params, request, fetch, cookies, locals, url }) => {
	const downloadSessionId = ensureDownloadSessionId(cookies);
	const upstream = await fetch(
		buildRustApiUrl(`/api/jobs/${encodeURIComponent(params.jobId)}/events`),
		{
			// Do not bind SSE proxy lifetime to request.signal here.
			// SvelteKit/undici can abort long-lived upstream streams immediately.
			headers: await buildRustApiHeaders(request, false, downloadSessionId)
		}
	);

	await logAuditEvent(
		{ request, locals, cookies, url },
		{
			scope: 'download',
			eventType: 'job_events_stream',
			entityId: params.jobId,
			targetLabel: params.jobId,
			statusCode: upstream.status,
			outcome: deriveAuditOutcome(upstream.status),
			payload: {
				contentType: upstream.headers.get('content-type'),
				downloadSessionId
			}
		}
	);

	const headers = applyNoStoreCache(new Headers());
	headers.set('content-type', upstream.headers.get('content-type') ?? 'text/event-stream');
	headers.set('connection', 'keep-alive');

	return new Response(upstream.body, {
		status: upstream.status,
		headers
	});
};
