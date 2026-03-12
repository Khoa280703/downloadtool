import type { RequestHandler } from './$types';

import {
	applyNoStoreCache,
	buildRustApiHeaders,
	buildRustApiUrl,
	copyRustResponseHeaders,
	ensureDownloadSessionId
} from '$lib/server/rust-api-proxy';

export const GET: RequestHandler = async ({ params, request, fetch, cookies }) => {
	const downloadSessionId = ensureDownloadSessionId(cookies);
	const rustHeaders = await buildRustApiHeaders(request, false, downloadSessionId);
	const ticketUpstream = await fetch(
		buildRustApiUrl(`/api/jobs/${encodeURIComponent(params.jobId)}/file-ticket`),
		{
			headers: rustHeaders,
			signal: request.signal
		}
	);

	if (!ticketUpstream.ok) {
		return new Response(ticketUpstream.body, {
			status: ticketUpstream.status,
			headers: applyNoStoreCache(copyRustResponseHeaders(ticketUpstream))
		});
	}

	const ticketPayload = (await ticketUpstream.json()) as { download_url?: string };
	const rustFallbackPath = `/api/jobs/${encodeURIComponent(params.jobId)}/file`;
	const ticketUrl = ticketPayload.download_url?.trim();
	const upstreamUrl =
		ticketUrl && /^https?:\/\//i.test(ticketUrl)
			? ticketUrl
			: buildRustApiUrl(ticketUrl && ticketUrl.startsWith('/api/jobs/') ? ticketUrl : rustFallbackPath);
	const upstream = await fetch(upstreamUrl, {
		headers: /^https?:\/\//i.test(upstreamUrl) ? undefined : rustHeaders,
		signal: request.signal
	});

	const headers = copyRustResponseHeaders(upstream);
	applyNoStoreCache(headers);

	return new Response(upstream.body, {
		status: upstream.status,
		headers
	});
};
