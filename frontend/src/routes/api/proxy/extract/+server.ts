import { env } from '$env/dynamic/private';
import { error, json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

import { auth } from '$lib/server/auth';
import { getJwtForRequest } from '$lib/server/auth-utils';
import { deriveAuditOutcome, logAuditEvent, sanitizeAuditUrl } from '$lib/server/audit-log';

const RETRYABLE_UPSTREAM_STATUS = new Set([429, 502, 503, 504]);
const MAX_UPSTREAM_ATTEMPTS = 4;
const RETRY_BASE_DELAY_MS = 300;
const RETRY_MAX_DELAY_MS = 2_000;

function normalizeBaseUrl(url: string | undefined): string {
	if (!url) return 'http://127.0.0.1:3068';
	return url.trim().replace(/\/+$/, '');
}

function parseRetryAfterMs(value: string | null): number | undefined {
	if (!value) return undefined;
	const seconds = Number.parseInt(value, 10);
	if (Number.isFinite(seconds) && seconds > 0) return seconds * 1000;

	const asDate = Date.parse(value);
	if (!Number.isNaN(asDate)) {
		const diff = asDate - Date.now();
		if (diff > 0) return diff;
	}
	return undefined;
}

function computeBackoffWithJitter(attempt: number): number {
	const base = Math.min(RETRY_BASE_DELAY_MS * Math.pow(2, attempt - 1), RETRY_MAX_DELAY_MS);
	const jitter = Math.floor(Math.random() * Math.min(200, Math.floor(base * 0.25)));
	return Math.min(RETRY_MAX_DELAY_MS, base + jitter);
}

async function sleep(ms: number): Promise<void> {
	if (ms <= 0) return;
	await new Promise<void>((resolve) => setTimeout(resolve, ms));
}

export const POST: RequestHandler = async ({ request, locals, fetch, cookies, url }) => {
	const body = await request.text();
	const rustApiUrl = normalizeBaseUrl(env.INTERNAL_RUST_API_URL ?? env.RUST_API_URL ?? env.VITE_API_URL);
	let requestUrl: string | null = null;

	try {
		const parsed = JSON.parse(body) as { url?: string };
		requestUrl = sanitizeAuditUrl(parsed.url?.trim() ?? null);
	} catch {
		requestUrl = null;
	}

	const upstreamHeaders: Record<string, string> = {
		'Content-Type': 'application/json'
	};

	if (locals.session) {
		const jwt = await getJwtForRequest(auth, request.headers);
		if (jwt) {
			upstreamHeaders.Authorization = `Bearer ${jwt}`;
		}
	}

	let upstream: Response | null = null;
	let lastUpstreamError: unknown = null;

	for (let attempt = 1; attempt <= MAX_UPSTREAM_ATTEMPTS; attempt += 1) {
		try {
			const response = await fetch(`${rustApiUrl}/api/extract`, {
				method: 'POST',
				headers: upstreamHeaders,
				body,
				signal: request.signal
			});

			if (
				response.ok ||
				!RETRYABLE_UPSTREAM_STATUS.has(response.status) ||
				attempt >= MAX_UPSTREAM_ATTEMPTS
			) {
				upstream = response;
				break;
			}

			const retryAfterMs = parseRetryAfterMs(response.headers.get('retry-after'));
			const delayMs = retryAfterMs ?? computeBackoffWithJitter(attempt);
			try {
				await response.arrayBuffer();
			} catch {
				// Ignore body drain errors before retry.
			}
			await sleep(delayMs);
		} catch (upstreamError) {
			lastUpstreamError = upstreamError;
			if (attempt >= MAX_UPSTREAM_ATTEMPTS) break;
			await sleep(computeBackoffWithJitter(attempt));
		}
	}

	if (!upstream) {
		console.error('BFF proxy extract upstream error after retries:', lastUpstreamError);
		await logAuditEvent(
			{ request, locals, cookies, url },
			{
				scope: 'download',
				eventType: 'extract',
				targetLabel: requestUrl,
				statusCode: 502,
				outcome: 'error',
				detail: 'Upstream API unavailable after retries',
				payload: {
					requestUrl,
					attempts: MAX_UPSTREAM_ATTEMPTS,
					error:
						lastUpstreamError instanceof Error
							? lastUpstreamError.message
							: String(lastUpstreamError ?? 'unknown')
				}
			}
		);
		throw error(502, 'Upstream API unavailable after retries');
	}

	const responseHeaders = new Headers();
	const contentType = upstream.headers.get('content-type') ?? 'application/json';
	responseHeaders.set('content-type', contentType);

	if (!upstream.body) {
		await logAuditEvent(
			{ request, locals, cookies, url },
			{
				scope: 'download',
				eventType: 'extract',
				targetLabel: requestUrl,
				statusCode: 502,
				outcome: 'error',
				detail: 'Empty upstream response',
				payload: {
					requestUrl,
					upstreamStatus: upstream.status
				}
			}
		);
		return json(
			{
				error: 'Empty upstream response'
			},
			{ status: 502, headers: responseHeaders }
		);
	}

	await logAuditEvent(
		{ request, locals, cookies, url },
		{
			scope: 'download',
			eventType: 'extract',
			targetLabel: requestUrl,
			statusCode: upstream.status,
			outcome: deriveAuditOutcome(upstream.status),
			payload: {
				requestUrl,
				upstreamStatus: upstream.status,
				contentType,
				authenticated: Boolean(locals.user?.id)
			}
		}
	);

	return new Response(upstream.body, {
		status: upstream.status,
		headers: responseHeaders
	});
};
