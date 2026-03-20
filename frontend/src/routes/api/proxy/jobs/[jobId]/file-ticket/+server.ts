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

	const ticket = await upstream.json();
	const backendUrl: string = ticket.download_url ?? '';
	const isDirectUrl = /^https?:\/\//i.test(backendUrl);
	if (!isDirectUrl) {
		await logAuditEvent(
			{ request, locals, cookies, url },
			{
				scope: 'download',
				eventType: 'job_file_ticket',
				entityId: params.jobId,
				targetLabel: params.jobId,
				statusCode: 502,
				outcome: 'failure',
				payload: {
					backendUrl,
					reason: 'backend_ticket_not_direct'
				}
			}
		);

		return json(
			{
				error: 'backend file ticket must be a direct signed URL'
			},
			{
				status: 502,
				headers: applyNoStoreCache(new Headers())
			}
		);
	}

	const downloadHost = (() => {
		try {
			return new URL(backendUrl).hostname;
		} catch {
			return 'unknown';
		}
	})();

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
				downloadUrl: '[signed-or-direct]',
				decisionReason: 'backend_ticket_direct',
				downloadHost
			}
		}
	);

	return json(
		{
			download_url: backendUrl
		},
		{
			headers: applyNoStoreCache(new Headers())
		}
	);
};
