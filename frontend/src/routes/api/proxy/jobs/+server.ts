import type { RequestHandler } from './$types';

import { ensureDownloadSessionId, forwardRustJson } from '$lib/server/rust-api-proxy';

export const POST: RequestHandler = async ({ request, fetch, cookies }) => {
	const body = await request.text();
	const downloadSessionId = ensureDownloadSessionId(cookies);
	return forwardRustJson(request, fetch, '/api/jobs', {
		method: 'POST',
		body
	}, downloadSessionId);
};
