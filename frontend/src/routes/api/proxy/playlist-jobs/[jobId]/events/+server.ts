import type { RequestHandler } from './$types';

import {
	applyNoStoreCache,
	buildRustApiHeaders,
	buildRustApiUrl,
	ensureDownloadSessionId,
	normalizePlaylistProxyStream
} from '$lib/server/rust-api-proxy';

export const GET: RequestHandler = async ({ params, request, fetch, cookies }) => {
	const downloadSessionId = ensureDownloadSessionId(cookies);
	const upstream = await fetch(
		buildRustApiUrl(`/api/playlist-jobs/${encodeURIComponent(params.jobId)}/events`),
		{
			headers: await buildRustApiHeaders(request, false, downloadSessionId)
		}
	);

	const headers = applyNoStoreCache(new Headers());
	headers.set('content-type', upstream.headers.get('content-type') ?? 'text/event-stream');
	headers.set('connection', 'keep-alive');

	return new Response(normalizePlaylistProxyStream(upstream.body), {
		status: upstream.status,
		headers
	});
};
