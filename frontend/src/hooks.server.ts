import { building } from '$app/environment';
import type { Handle } from '@sveltejs/kit';
import { svelteKitHandler } from 'better-auth/svelte-kit';

import { auth } from '$lib/server/auth';

const HTML_CACHE_CONTROL = 'private, no-store';
const AUTH_API_CACHE_CONTROL = 'private, no-store, max-age=0, must-revalidate';

function normalizeRedirectTarget(value: string | null): string {
	if (!value || !value.startsWith('/') || value.startsWith('//')) {
		return '/';
	}

	return value;
}

export const handle: Handle = async ({ event, resolve }) => {
	if (building) {
		event.locals.session = null;
		event.locals.user = null;
	} else {
		// Skip DB session lookup when no Better Auth cookie is present (unauthenticated users).
		// auth.api.getSession() queries the sessions table — avoid this on every unauthenticated request.
		const cookieHeader = event.request.headers.get('cookie') ?? '';
		const hasBetterAuthCookie = cookieHeader.includes('better-auth.');

		if (hasBetterAuthCookie) {
			const authSession = await auth.api.getSession({
				headers: event.request.headers
			});
			event.locals.session = authSession?.session ?? null;
			event.locals.user = authSession?.user ?? null;
		} else {
			event.locals.session = null;
			event.locals.user = null;
		}
	}

	if (event.url.pathname === '/login') {
		const redirectTo = normalizeRedirectTarget(event.url.searchParams.get('redirectTo') ?? '/account');
		const destination = event.locals.session
			? redirectTo
			: `/?auth=required&redirectTo=${encodeURIComponent(redirectTo)}`;
		return Response.redirect(new URL(destination, event.url), 302);
	}

	const response = await svelteKitHandler({
		auth,
		event,
		resolve,
		building
	});

	if (
		(event.request.method === 'GET' || event.request.method === 'HEAD') &&
		!event.url.pathname.startsWith('/api/') &&
		response.headers.get('content-type')?.startsWith('text/html')
	) {
		response.headers.set('cache-control', HTML_CACHE_CONTROL);
		response.headers.set('vary', 'cookie');
	}

	if (event.url.pathname.startsWith('/api/auth/')) {
		response.headers.set('cache-control', AUTH_API_CACHE_CONTROL);
		response.headers.set('pragma', 'no-cache');
		response.headers.set('expires', '0');
		response.headers.set('vary', 'cookie');
	}

	return response;
};
