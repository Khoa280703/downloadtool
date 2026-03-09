import { env } from '$env/dynamic/private';
import type { Cookies } from '@sveltejs/kit';

import { auth } from '$lib/server/auth';
import { getJwtForRequest } from '$lib/server/auth-utils';

const DOWNLOAD_SESSION_COOKIE = 'downloadtool_session';
const DOWNLOAD_SESSION_HEADER = 'x-download-session-id';

function normalizeBaseUrl(url: string | undefined): string {
	if (!url) return 'http://127.0.0.1:3068';
	return url.trim().replace(/\/+$/, '');
}

export function buildRustApiUrl(path: string): string {
	return `${normalizeBaseUrl(env.RUST_API_URL ?? env.VITE_API_URL)}${path}`;
}

export function ensureDownloadSessionId(cookies: Cookies): string {
	const existing = cookies.get(DOWNLOAD_SESSION_COOKIE)?.trim();
	if (existing) return existing;

	const sessionId = crypto.randomUUID();
	cookies.set(DOWNLOAD_SESSION_COOKIE, sessionId, {
		path: '/',
		httpOnly: true,
		sameSite: 'lax',
		secure: (env.ORIGIN ?? '').startsWith('https://'),
		maxAge: 60 * 60 * 24 * 30
	});
	return sessionId;
}

export async function buildRustApiHeaders(
	request: Request,
	hasJsonBody = false,
	downloadSessionId?: string
): Promise<Headers> {
	const headers = new Headers();
	if (hasJsonBody) headers.set('content-type', 'application/json');
	if (downloadSessionId) headers.set(DOWNLOAD_SESSION_HEADER, downloadSessionId);

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
	init?: RequestInit,
	downloadSessionId?: string
): Promise<Response> {
	const upstreamHeaders = await buildRustApiHeaders(
		request,
		init?.body !== undefined || init?.method === 'POST',
		downloadSessionId
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
