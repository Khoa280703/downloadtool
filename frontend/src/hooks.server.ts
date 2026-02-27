import type { Handle } from '@sveltejs/kit';

const HTML_CACHE_CONTROL = 'public, max-age=300';

export const handle: Handle = async ({ event, resolve }) => {
	const response = await resolve(event);

	if (
		(event.request.method === 'GET' || event.request.method === 'HEAD') &&
		!event.url.pathname.startsWith('/api/') &&
		response.headers.get('content-type')?.startsWith('text/html')
	) {
		response.headers.set('cache-control', HTML_CACHE_CONTROL);
	}

	return response;
};
