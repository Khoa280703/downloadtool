import type { RequestHandler } from './$types';

import { ensureDownloadSessionId, forwardRustJson } from '$lib/server/rust-api-proxy';

export const POST: RequestHandler = async ({ params, request, fetch, cookies }) =>
	forwardRustJson(
		request,
		fetch,
		`/api/jobs/${encodeURIComponent(params.jobId)}/release`,
		{ method: 'POST' },
		ensureDownloadSessionId(cookies)
	);
