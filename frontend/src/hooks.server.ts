import { building } from '$app/environment';
import type { Handle } from '@sveltejs/kit';
import { svelteKitHandler } from 'better-auth/svelte-kit';

import { auth } from '$lib/server/auth';

const HTML_CACHE_CONTROL = 'private, no-store';

export const handle: Handle = async ({ event, resolve }) => {
	if (building) {
		event.locals.session = null;
		event.locals.user = null;
	} else {
		const authSession = await auth.api.getSession({
			headers: event.request.headers
		});

		event.locals.session = authSession?.session ?? null;
		event.locals.user = authSession?.user ?? null;
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
	}

	return response;
};
