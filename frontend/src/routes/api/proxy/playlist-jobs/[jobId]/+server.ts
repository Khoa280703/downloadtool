import type { RequestHandler } from './$types';

import {
	applyNoStoreCache,
	buildRustApiHeaders,
	buildRustApiUrl,
	copyRustResponseHeaders,
	ensureDownloadSessionId,
	normalizePlaylistProxyPayload
} from '$lib/server/rust-api-proxy';

export const GET: RequestHandler = async ({ params, request, fetch, cookies }) => {
	const downloadSessionId = ensureDownloadSessionId(cookies);
	const upstream = await fetch(
		buildRustApiUrl(`/api/playlist-jobs/${encodeURIComponent(params.jobId)}`),
		{
			headers: await buildRustApiHeaders(request, false, downloadSessionId),
			signal: request.signal
		}
	);

	const body = normalizePlaylistProxyPayload(await upstream.text());

	return new Response(body, {
		status: upstream.status,
		headers: applyNoStoreCache(copyRustResponseHeaders(upstream))
	});
};
