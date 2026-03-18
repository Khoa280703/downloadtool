import type { RequestHandler } from './$types';

import {
	applyNoStoreCache,
	buildRustApiHeaders,
	buildRustApiUrl,
	copyRustResponseHeaders,
	ensureDownloadSessionId
} from '$lib/server/rust-api-proxy';
import { deriveAuditOutcome, logAuditEvent, sanitizeAuditUrl } from '$lib/server/audit-log';

type CreatePlaylistBody = {
	url?: string;
	quality?: string;
	mode?: string;
};

export const POST: RequestHandler = async ({ request, fetch, cookies, locals, url }) => {
	const body = await request.text();
	const downloadSessionId = ensureDownloadSessionId(cookies);
	const upstream = await fetch(buildRustApiUrl('/api/playlist-jobs'), {
		method: 'POST',
		body,
		headers: await buildRustApiHeaders(request, true, downloadSessionId),
		signal: request.signal
	});

	let parsedBody: CreatePlaylistBody | null = null;
	try {
		parsedBody = JSON.parse(body) as CreatePlaylistBody;
	} catch {
		parsedBody = null;
	}

	let parsedResponse: Record<string, unknown> | null = null;
	if (upstream.ok) {
		try {
			const cloned = upstream.clone();
			parsedResponse = (await cloned.json()) as Record<string, unknown>;
		} catch {
			parsedResponse = null;
		}
	}

	await logAuditEvent(
		{ request, locals, cookies, url },
		{
			scope: 'playlist',
			eventType: 'playlist_job.create',
			entityId: (parsedResponse?.job_id as string) ?? null,
			targetLabel: sanitizeAuditUrl(parsedBody?.url),
			statusCode: upstream.status,
			outcome: deriveAuditOutcome(upstream.status),
			detail: upstream.ok ? null : `HTTP ${upstream.status}`,
			payload: {
				sourceUrl: sanitizeAuditUrl(parsedBody?.url),
				quality: parsedBody?.quality ?? null,
				mode: parsedBody?.mode ?? null,
				jobId: parsedResponse?.job_id ?? null
			}
		}
	);

	return new Response(upstream.body, {
		status: upstream.status,
		headers: applyNoStoreCache(copyRustResponseHeaders(upstream))
	});
};
