import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

import { buildRustApiHeaders, buildRustApiUrl } from '$lib/server/rust-api-proxy';

export const GET: RequestHandler = async ({ params, request, fetch }) => {
	const upstream = await fetch(
		buildRustApiUrl(`/api/jobs/${encodeURIComponent(params.jobId)}/file-ticket`),
		{
			headers: await buildRustApiHeaders(request),
			signal: request.signal
		}
	);

	if (!upstream.ok) {
		return new Response(upstream.body, {
			status: upstream.status,
			headers: { 'content-type': upstream.headers.get('content-type') ?? 'application/json' }
		});
	}

	const payload = (await upstream.json()) as { download_url?: string };
	const downloadUrl = payload.download_url ?? null;
	if (!downloadUrl) {
		return json({ download_url: `/api/proxy/jobs/${encodeURIComponent(params.jobId)}/file` });
	}

	if (downloadUrl.startsWith('/api/jobs/')) {
		return json({
			download_url: `/api/proxy/jobs/${encodeURIComponent(params.jobId)}/file`
		});
	}

	return json({ download_url: downloadUrl });
};
