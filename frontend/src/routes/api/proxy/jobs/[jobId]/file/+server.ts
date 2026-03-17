import type { RequestHandler } from './$types';

import {
	applyNoStoreCache,
	buildRustApiHeaders,
	buildRustApiUrl,
	copyRustResponseHeaders,
	ensureDownloadSessionId
} from '$lib/server/rust-api-proxy';
import { deriveAuditOutcome, logAuditEvent } from '$lib/server/audit-log';

export const GET: RequestHandler = async ({ params, request, fetch, cookies, locals, url }) => {
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
		await logAuditEvent(
			{ request, locals, cookies, url },
			{
				scope: 'download',
				eventType: 'job_file',
				entityId: params.jobId,
				targetLabel: params.jobId,
				statusCode: ticketUpstream.status,
				outcome: deriveAuditOutcome(ticketUpstream.status),
				detail: 'Failed to fetch file ticket'
			}
		);
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
	const viaSignedUrl = /^https?:\/\//i.test(upstreamUrl);
	const upstream = await fetch(upstreamUrl, {
		headers: viaSignedUrl ? undefined : rustHeaders,
		signal: request.signal
	});

	await logAuditEvent(
		{ request, locals, cookies, url },
		{
			scope: 'download',
			eventType: 'job_file',
			entityId: params.jobId,
			targetLabel: params.jobId,
			statusCode: upstream.status,
			outcome: deriveAuditOutcome(upstream.status),
			payload: {
				viaSignedUrl,
				upstreamTarget: viaSignedUrl ? new URL(upstreamUrl).host : rustFallbackPath
			}
		}
	);

	const headers = copyRustResponseHeaders(upstream);
	applyNoStoreCache(headers);

	return new Response(upstream.body, {
		status: upstream.status,
		headers
	});
};
