import type { RequestHandler } from './$types';

import { ensureDownloadSessionId, forwardRustJson } from '$lib/server/rust-api-proxy';

export const GET: RequestHandler = async ({ params, request, fetch, cookies }) =>
	forwardRustJson(
		request,
		fetch,
		`/api/jobs/${encodeURIComponent(params.jobId)}`,
		undefined,
		ensureDownloadSessionId(cookies)
	);
