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

	const headers = copyRustResponseHeaders(upstream);
	if (!headers.has('content-disposition')) {
		headers.set(
			'content-disposition',
			`attachment; filename="${encodeURIComponent(params.jobId)}.mp4"`
		);
	}
	if (!headers.has('cache-control')) {
		headers.set('cache-control', 'no-store, no-cache, must-revalidate');
	}

	return new Response(upstream.body, {
		status: upstream.status,
		headers
	});
};
