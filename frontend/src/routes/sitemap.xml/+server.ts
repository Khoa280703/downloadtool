import type { RequestHandler } from './$types';
import { PUBLIC_PAGES, SITE_URL } from '$lib/seo/public-pages';

const LANGUAGES = [
	'ar',
	'bg',
	'cs',
	'da',
	'de',
	'el',
	'en',
	'es',
	'et',
	'fi',
	'fr',
	'hu',
	'id',
	'it',
	'ja',
	'ko',
	'lt',
	'lv',
	'nb',
	'nl',
	'pl',
	'pt',
	'pt-BR',
	'ro',
	'ru',
	'sk',
	'sl',
	'sv',
	'tr',
	'uk',
	'vi',
	'zh',
	'zh-TW'
] as const;

const HREFLANG_MAP: Record<string, string> = { nb: 'no' };

function localizedHref(pathname: string, locale: string): string {
	if (locale === 'en') return `${SITE_URL}${pathname}`;
	if (pathname === '/') return `${SITE_URL}/${locale}/`;
	return `${SITE_URL}/${locale}${pathname}`;
}

function buildEntry(pathname: string, priority: number, changefreq: string, lastmod?: string): string {
	const links = LANGUAGES.map((locale) => {
		const hreflang = HREFLANG_MAP[locale] ?? locale;
		return `    <xhtml:link rel="alternate" hreflang="${hreflang}" href="${localizedHref(pathname, locale)}" />`;
	}).join('\n');

	const xDefault = `    <xhtml:link rel="alternate" hreflang="x-default" href="${localizedHref(pathname, 'en')}" />`;

	const lastmodTag = lastmod ? `\n    <lastmod>${lastmod}</lastmod>` : '';

	return `<url>
    <loc>${localizedHref(pathname, 'en')}</loc>
    <changefreq>${changefreq}</changefreq>
    <priority>${priority.toFixed(1)}</priority>${lastmodTag}
${links}
${xDefault}
  </url>`;
}

export const GET: RequestHandler = () => {
	const entries = PUBLIC_PAGES.map((page) =>
		buildEntry(page.path, page.priority, page.changefreq, page.lastmod)
	);

	const body = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">
${entries.join('\n')}
</urlset>`;

	return new Response(body, {
		headers: {
			'content-type': 'application/xml; charset=utf-8',
			'cache-control': 'public, max-age=3600'
		}
	});
};
