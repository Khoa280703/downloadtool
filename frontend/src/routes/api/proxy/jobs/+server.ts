import type { RequestHandler } from './$types';

import { forwardRustJson } from '$lib/server/rust-api-proxy';

export const POST: RequestHandler = async ({ request, fetch }) => {
	const body = await request.text();
	return forwardRustJson(request, fetch, '/api/jobs', {
		method: 'POST',
		body
	});
};
