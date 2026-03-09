import { env } from '$env/dynamic/private';

import { auth } from '$lib/server/auth';
import { getJwtForRequest } from '$lib/server/auth-utils';

function normalizeBaseUrl(url: string | undefined): string {
	if (!url) return 'http://127.0.0.1:3068';
	return url.trim().replace(/\/+$/, '');
}

export function buildRustApiUrl(path: string): string {
	return `${normalizeBaseUrl(env.RUST_API_URL ?? env.VITE_API_URL)}${path}`;
}

export async function buildRustApiHeaders(request: Request, hasJsonBody = false): Promise<Headers> {
	const headers = new Headers();
	if (hasJsonBody) headers.set('content-type', 'application/json');

	const cookieHeader = request.headers.get('cookie') ?? '';
	if (!cookieHeader.includes('better-auth.')) {
		return headers;
	}

	try {
		const jwt = await getJwtForRequest(auth, request.headers);
		if (jwt) headers.set('Authorization', `Bearer ${jwt}`);
	} catch (error) {
		console.error('Failed to resolve Better Auth JWT for Rust API proxy:', error);
	}
	return headers;
}

export function copyRustResponseHeaders(upstream: Response): Headers {
	const headers = new Headers();
	const contentType = upstream.headers.get('content-type');
	if (contentType) headers.set('content-type', contentType);
	const contentLength = upstream.headers.get('content-length');
	if (contentLength) headers.set('content-length', contentLength);
	const disposition = upstream.headers.get('content-disposition');
	if (disposition) headers.set('content-disposition', disposition);
	return headers;
}

export async function forwardRustJson(
	request: Request,
	fetchFn: typeof fetch,
	path: string,
	init?: RequestInit
): Promise<Response> {
	const upstreamHeaders = await buildRustApiHeaders(
		request,
		init?.body !== undefined || init?.method === 'POST'
	);
	const upstream = await fetchFn(buildRustApiUrl(path), {
		...init,
		headers: upstreamHeaders,
		signal: request.signal
	});
	return new Response(upstream.body, {
		status: upstream.status,
		headers: copyRustResponseHeaders(upstream)
	});
}
