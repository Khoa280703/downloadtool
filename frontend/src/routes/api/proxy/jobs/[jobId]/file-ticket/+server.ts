import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

import {
	buildRustApiHeaders,
	buildRustApiUrl,
	ensureDownloadSessionId
} from '$lib/server/rust-api-proxy';

export const GET: RequestHandler = async ({ params, request, fetch, cookies }) => {
	const downloadSessionId = ensureDownloadSessionId(cookies);
	const upstream = await fetch(
		buildRustApiUrl(`/api/jobs/${encodeURIComponent(params.jobId)}/file-ticket`),
		{
			headers: await buildRustApiHeaders(request, false, downloadSessionId),
			signal: request.signal
		}
	);

	if (!upstream.ok) {
		return new Response(upstream.body, {
			status: upstream.status,
			headers: { 'content-type': upstream.headers.get('content-type') ?? 'application/json' }
		});
	}

	// Always force browser downloads through same-origin proxy.
	// Direct presigned R2 URLs cause some browsers to navigate instead of downloading.
	return json({
		download_url: `/api/proxy/jobs/${encodeURIComponent(params.jobId)}/file`
	});
};
