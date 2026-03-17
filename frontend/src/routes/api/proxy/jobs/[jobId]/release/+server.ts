import type { RequestHandler } from './$types';

import {
	applyNoStoreCache,
	buildRustApiHeaders,
	buildRustApiUrl,
	copyRustResponseHeaders,
	ensureDownloadSessionId
} from '$lib/server/rust-api-proxy';
import { deriveAuditOutcome, logAuditEvent } from '$lib/server/audit-log';

type ReleaseResponse = {
	released?: boolean;
};

export const POST: RequestHandler = async ({ params, request, fetch, cookies, locals, url }) => {
	const downloadSessionId = ensureDownloadSessionId(cookies);
	const upstream = await fetch(
		buildRustApiUrl(`/api/jobs/${encodeURIComponent(params.jobId)}/release`),
		{
			method: 'POST',
			headers: await buildRustApiHeaders(request, false, downloadSessionId),
			signal: request.signal
		}
	);

	let parsedResponse: ReleaseResponse | null = null;
	try {
		if ((upstream.headers.get('content-type') ?? '').includes('application/json')) {
			parsedResponse = (await upstream.clone().json()) as ReleaseResponse;
		}
	} catch {
		parsedResponse = null;
	}

	await logAuditEvent(
		{ request, locals, cookies, url },
		{
			scope: 'download',
			eventType: 'job_release',
			entityId: params.jobId,
			targetLabel: params.jobId,
			statusCode: upstream.status,
			outcome: deriveAuditOutcome(upstream.status),
			payload: {
				released: parsedResponse?.released ?? null
			}
		}
	);

	return new Response(upstream.body, {
		status: upstream.status,
		headers: applyNoStoreCache(copyRustResponseHeaders(upstream))
	});
};
