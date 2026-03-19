import type { RequestHandler } from './$types';
import { SITE_URL } from '$lib/seo/public-pages';

function buildRobotsContent(): string {
	return `User-agent: *
Allow: /
Disallow: /api/

Sitemap: ${SITE_URL}/sitemap.xml
`;
}

export const GET: RequestHandler = async () =>
	new Response(buildRobotsContent(), {
		headers: {
			'content-type': 'text/plain; charset=utf-8',
			'cache-control': 'public, max-age=3600'
		}
	});
