import { env } from '$env/dynamic/private';
import type { Cookies } from '@sveltejs/kit';

import { auth } from '$lib/server/auth';
import { getJwtForRequest } from '$lib/server/auth-utils';

const DOWNLOAD_SESSION_COOKIE = 'downloadtool_session';
const DOWNLOAD_SESSION_HEADER = 'x-download-session-id';
const NO_STORE_CACHE_CONTROL = 'no-store, no-cache, must-revalidate';
const RAW_MUX_JOB_PATH = '/api/jobs/';
const PROXIED_MUX_JOB_PATH = '/api/proxy/jobs/';
const RAW_MUX_FILE_TICKET_SUFFIX = '/file-ticket';
const PROXIED_MUX_FILE_SUFFIX = '/file';

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

export function applyNoStoreCache(headers: Headers): Headers {
	headers.set('cache-control', NO_STORE_CACHE_CONTROL);
	return headers;
}

export function normalizePlaylistProxyPayload(payload: string): string {
	if (!payload.includes(RAW_MUX_JOB_PATH)) {
		return payload;
	}

	return payload
		.replaceAll(RAW_MUX_JOB_PATH, PROXIED_MUX_JOB_PATH)
		.replaceAll(RAW_MUX_FILE_TICKET_SUFFIX, PROXIED_MUX_FILE_SUFFIX);
}

export function normalizePlaylistProxyStream(
	stream: ReadableStream<Uint8Array> | null
): ReadableStream<Uint8Array> | null {
	if (!stream) return null;

	const decoder = new TextDecoder();
	const encoder = new TextEncoder();
	let buffer = '';

	return stream.pipeThrough(
		new TransformStream<Uint8Array, Uint8Array>({
			transform(chunk, controller) {
				buffer += decoder.decode(chunk, { stream: true });
				let boundaryIndex = buffer.indexOf('\n\n');

				while (boundaryIndex !== -1) {
					const eventChunk = buffer.slice(0, boundaryIndex + 2);
					controller.enqueue(encoder.encode(normalizePlaylistProxyPayload(eventChunk)));
					buffer = buffer.slice(boundaryIndex + 2);
					boundaryIndex = buffer.indexOf('\n\n');
				}
			},
			flush(controller) {
				buffer += decoder.decode();
				if (!buffer) return;
				controller.enqueue(encoder.encode(normalizePlaylistProxyPayload(buffer)));
			}
		})
	);
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
		headers: applyNoStoreCache(copyRustResponseHeaders(upstream))
	});
}
