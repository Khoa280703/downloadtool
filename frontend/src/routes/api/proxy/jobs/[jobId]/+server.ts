import type { RequestHandler } from './$types';

import { forwardRustJson } from '$lib/server/rust-api-proxy';

export const GET: RequestHandler = async ({ params, request, fetch }) =>
	forwardRustJson(
		request,
		fetch,
		`/api/jobs/${encodeURIComponent(params.jobId)}`
	);
