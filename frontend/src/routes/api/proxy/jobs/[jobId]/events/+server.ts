import type { RequestHandler } from './$types';

import {
	applyNoStoreCache,
	buildRustApiHeaders,
	buildRustApiUrl,
	ensureDownloadSessionId
} from '$lib/server/rust-api-proxy';

export const GET: RequestHandler = async ({ params, request, fetch, cookies }) => {
	const downloadSessionId = ensureDownloadSessionId(cookies);
	const upstream = await fetch(
		buildRustApiUrl(`/api/jobs/${encodeURIComponent(params.jobId)}/events`),
		{
			// Do not bind SSE proxy lifetime to request.signal here.
			// SvelteKit/undici can abort long-lived upstream streams immediately.
			headers: await buildRustApiHeaders(request, false, downloadSessionId)
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
