import { json } from '@sveltejs/kit';
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
		buildRustApiUrl(`/api/jobs/${encodeURIComponent(params.jobId)}/file-ticket`),
		{
			headers: await buildRustApiHeaders(request, false, downloadSessionId),
			signal: request.signal
		}
	);

	if (!upstream.ok) {
		await logAuditEvent(
			{ request, locals, cookies, url },
			{
				scope: 'download',
				eventType: 'job_file_ticket',
				entityId: params.jobId,
				targetLabel: params.jobId,
				statusCode: upstream.status,
				outcome: deriveAuditOutcome(upstream.status)
			}
		);
		return new Response(upstream.body, {
			status: upstream.status,
			headers: applyNoStoreCache(
				new Headers({
					'content-type': upstream.headers.get('content-type') ?? 'application/json'
				})
			)
		});
	}

	await logAuditEvent(
		{ request, locals, cookies, url },
		{
			scope: 'download',
			eventType: 'job_file_ticket',
			entityId: params.jobId,
			targetLabel: params.jobId,
			statusCode: 200,
			outcome: 'success',
			payload: {
				downloadUrl: `/api/proxy/jobs/${encodeURIComponent(params.jobId)}/file`
			}
		}
	);

	// Always force browser downloads through same-origin proxy.
	// Direct presigned R2 URLs cause some browsers to navigate instead of downloading.
	return json(
		{
			download_url: `/api/proxy/jobs/${encodeURIComponent(params.jobId)}/file`
		},
		{
			headers: applyNoStoreCache(new Headers())
		}
	);
};
