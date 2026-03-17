import type { RequestHandler } from './$types';

import {
	applyNoStoreCache,
	buildRustApiHeaders,
	buildRustApiUrl,
	copyRustResponseHeaders,
	ensureDownloadSessionId
} from '$lib/server/rust-api-proxy';
import { deriveAuditOutcome, logAuditEvent } from '$lib/server/audit-log';

type JobStatusResponse = {
	status?: string;
	error?: string | null;
	file_ticket_url?: string | null;
	progress?: {
		phase?: string | null;
		percent?: number | null;
	};
};

export const GET: RequestHandler = async ({ params, request, fetch, cookies, locals, url }) => {
	const downloadSessionId = ensureDownloadSessionId(cookies);
	const upstream = await fetch(buildRustApiUrl(`/api/jobs/${encodeURIComponent(params.jobId)}`), {
		headers: await buildRustApiHeaders(request, false, downloadSessionId),
		signal: request.signal
	});

	let parsedResponse: JobStatusResponse | null = null;
	try {
		if ((upstream.headers.get('content-type') ?? '').includes('application/json')) {
			parsedResponse = (await upstream.clone().json()) as JobStatusResponse;
		}
	} catch {
		parsedResponse = null;
	}

	await logAuditEvent(
		{ request, locals, cookies, url },
		{
			scope: 'download',
			eventType: 'job_status',
			entityId: params.jobId,
			targetLabel: params.jobId,
			statusCode: upstream.status,
			outcome: deriveAuditOutcome(upstream.status),
			detail: parsedResponse?.error ?? null,
			payload: {
				jobStatus: parsedResponse?.status ?? null,
				fileTicketUrl: parsedResponse?.file_ticket_url ?? null,
				progressPhase: parsedResponse?.progress?.phase ?? null,
				progressPercent: parsedResponse?.progress?.percent ?? null
			}
		}
	);

	return new Response(upstream.body, {
		status: upstream.status,
		headers: applyNoStoreCache(copyRustResponseHeaders(upstream))
	});
};
