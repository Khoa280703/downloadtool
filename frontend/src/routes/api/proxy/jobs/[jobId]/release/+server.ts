import type { RequestHandler } from './$types';

import { forwardRustJson } from '$lib/server/rust-api-proxy';

export const POST: RequestHandler = async ({ params, request, fetch }) =>
	forwardRustJson(
		request,
		fetch,
		`/api/jobs/${encodeURIComponent(params.jobId)}/release`,
		{ method: 'POST' }
	);
