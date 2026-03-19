import type { RequestHandler } from './$types';

const ORIGIN = process.env.ORIGIN ?? 'https://snapvie.com';

function buildRobotsContent(): string {
	return `User-agent: *
Allow: /

Sitemap: ${ORIGIN}/sitemap.xml
`;
}

export const GET: RequestHandler = async () =>
	new Response(buildRobotsContent(), {
		headers: {
			'content-type': 'text/plain; charset=utf-8',
			'cache-control': 'public, max-age=3600'
		}
	});
