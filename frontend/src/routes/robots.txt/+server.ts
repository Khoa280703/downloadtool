import type { RequestHandler } from './$types';

const ROBOTS_CONTENT = `User-agent: *
Disallow:

Sitemap: https://download.khoadangbui.online/sitemap.xml
`;

export const GET: RequestHandler = async () =>
	new Response(ROBOTS_CONTENT, {
		headers: {
			'content-type': 'text/plain; charset=utf-8',
			'cache-control': 'public, max-age=3600'
		}
	});
