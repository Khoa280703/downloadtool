import { env } from '$env/dynamic/private';
import { error, json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

import { auth } from '$lib/server/auth';
import { getJwtForRequest } from '$lib/server/auth-utils';

function normalizeBaseUrl(url: string | undefined): string {
	if (!url) return 'http://127.0.0.1:3068';
	return url.trim().replace(/\/+$/, '');
}

export const POST: RequestHandler = async ({ request, locals, fetch }) => {
	const body = await request.text();
	const rustApiUrl = normalizeBaseUrl(env.RUST_API_URL ?? env.VITE_API_URL);

	const upstreamHeaders: Record<string, string> = {
		'Content-Type': 'application/json'
	};

	if (locals.session) {
		const jwt = await getJwtForRequest(auth, request.headers);
		if (jwt) {
			upstreamHeaders.Authorization = `Bearer ${jwt}`;
		}
	}

	let upstream: Response;
	try {
		upstream = await fetch(`${rustApiUrl}/api/extract`, {
			method: 'POST',
			headers: upstreamHeaders,
			body
		});
	} catch (upstreamError) {
		console.error('BFF proxy extract upstream error:', upstreamError);
		throw error(502, 'Upstream API unavailable');
	}

	const responseHeaders = new Headers();
	const contentType = upstream.headers.get('content-type') ?? 'application/json';
	responseHeaders.set('content-type', contentType);

	if (!upstream.body) {
		return json(
			{
				error: 'Empty upstream response'
			},
			{ status: 502, headers: responseHeaders }
		);
	}

	return new Response(upstream.body, {
		status: upstream.status,
		headers: responseHeaders
	});
};
