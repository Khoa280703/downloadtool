import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

import {
	applyNoStoreCache,
	buildRustApiHeaders,
	buildRustApiUrl,
	ensureDownloadSessionId
} from '$lib/server/rust-api-proxy';
import { deriveAuditOutcome, logAuditEvent } from '$lib/server/audit-log';
import {
	classifyUserAgentFamily,
	resolveDeliveryMode
} from '$lib/server/delivery-mode-resolver';

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

	// Parse backend ticket to get the real download_url (may be R2 signed URL or local path)
	const ticket = await upstream.json();
	const backendUrl: string = ticket.download_url ?? '';
	const userAgent = request.headers.get('user-agent');

	const decision = resolveDeliveryMode(userAgent, backendUrl);
	const uaFamily = classifyUserAgentFamily(userAgent);

	// Resolve final download URL based on delivery decision
	const proxyPath = `/api/proxy/jobs/${encodeURIComponent(params.jobId)}/file`;
	const finalUrl = decision.deliveryMode === 'direct' ? backendUrl : proxyPath;

	// Audit: never log full R2 signed URL
	const auditDownloadUrl =
		decision.deliveryMode === 'direct' ? '[r2-signed]' : proxyPath;
	const downloadHost = decision.deliveryMode === 'direct'
		? (() => { try { return new URL(backendUrl).hostname; } catch { return 'unknown'; } })()
		: 'self';

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
				downloadUrl: auditDownloadUrl,
				ticketDelivery: decision.deliveryMode,
				decisionReason: decision.reason,
				userAgentFamily: uaFamily,
				downloadHost
			}
		}
	);

	return json(
		{
			download_url: finalUrl,
			ticket_delivery: decision.deliveryMode
		},
		{
			headers: applyNoStoreCache(new Headers())
		}
	);
};
