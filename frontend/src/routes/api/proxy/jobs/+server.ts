import type { RequestHandler } from './$types';

import {
	applyNoStoreCache,
	buildRustApiHeaders,
	buildRustApiUrl,
	copyRustResponseHeaders,
	ensureDownloadSessionId
} from '$lib/server/rust-api-proxy';
import { deriveAuditOutcome, logAuditEvent, sanitizeAuditUrl } from '$lib/server/audit-log';

type CreateJobBody = {
	title?: string;
	source_url?: string;
	video_format_id?: string;
	audio_format_id?: string;
};

type CreateJobResponse = {
	job_id?: string;
	status?: string;
	file_ticket_url?: string;
	poll_url?: string;
};

export const POST: RequestHandler = async ({ request, fetch, cookies, locals, url }) => {
	const body = await request.text();
	const downloadSessionId = ensureDownloadSessionId(cookies);
	const upstream = await fetch(buildRustApiUrl('/api/jobs'), {
		method: 'POST',
		body,
		headers: await buildRustApiHeaders(request, true, downloadSessionId),
		signal: request.signal
	});

	let parsedRequest: CreateJobBody | null = null;
	let parsedResponse: CreateJobResponse | null = null;

	try {
		parsedRequest = JSON.parse(body) as CreateJobBody;
	} catch {
		parsedRequest = null;
	}

	try {
		if ((upstream.headers.get('content-type') ?? '').includes('application/json')) {
			parsedResponse = (await upstream.clone().json()) as CreateJobResponse;
		}
	} catch {
		parsedResponse = null;
	}

	await logAuditEvent(
		{ request, locals, cookies, url },
		{
			scope: 'download',
			eventType: 'job_create',
			entityId: parsedResponse?.job_id ?? null,
			targetLabel:
				parsedRequest?.title?.trim() || sanitizeAuditUrl(parsedRequest?.source_url?.trim() ?? null),
			statusCode: upstream.status,
			outcome: deriveAuditOutcome(upstream.status),
			payload: {
				sourceUrl: sanitizeAuditUrl(parsedRequest?.source_url ?? null),
				videoFormatId: parsedRequest?.video_format_id ?? null,
				audioFormatId: parsedRequest?.audio_format_id ?? null,
				jobId: parsedResponse?.job_id ?? null,
				jobStatus: parsedResponse?.status ?? null,
				fileTicketUrl: parsedResponse?.file_ticket_url ?? null,
				pollUrl: parsedResponse?.poll_url ?? null
			}
		}
	);

	return new Response(upstream.body, {
		status: upstream.status,
		headers: applyNoStoreCache(copyRustResponseHeaders(upstream))
	});
};
