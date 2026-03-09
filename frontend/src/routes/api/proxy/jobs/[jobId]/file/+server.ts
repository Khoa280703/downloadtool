import type { RequestHandler } from './$types';

import {
	buildRustApiHeaders,
	buildRustApiUrl,
	copyRustResponseHeaders,
	ensureDownloadSessionId
} from '$lib/server/rust-api-proxy';

export const GET: RequestHandler = async ({ params, request, fetch, cookies }) => {
	const downloadSessionId = ensureDownloadSessionId(cookies);
	const upstream = await fetch(buildRustApiUrl(`/api/jobs/${encodeURIComponent(params.jobId)}/file`), {
		headers: await buildRustApiHeaders(request, false, downloadSessionId),
		signal: request.signal
	});

	return new Response(upstream.body, {
		status: upstream.status,
		headers: copyRustResponseHeaders(upstream)
	});
};
