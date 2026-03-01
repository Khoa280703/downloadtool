import type { RequestHandler } from './$types';

const ORIGIN = process.env.ORIGIN ?? 'https://download.khoadangbui.online';
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
const PAGES = ['/', '/privacy'] as const;

function localizedHref(pathname: string, locale: string): string {
	if (locale === 'en') return `${ORIGIN}${pathname}`;
	if (pathname === '/') return `${ORIGIN}/${locale}/`;
	return `${ORIGIN}/${locale}${pathname}`;
}

function buildEntry(pathname: string): string {
	const links = LANGUAGES.map((locale) => {
		const hreflang = HREFLANG_MAP[locale] ?? locale;
		return `    <xhtml:link rel="alternate" hreflang="${hreflang}" href="${localizedHref(pathname, locale)}" />`;
	}).join('\n');

	const xDefault = `    <xhtml:link rel="alternate" hreflang="x-default" href="${localizedHref(pathname, 'en')}" />`;

	return `<url>
    <loc>${localizedHref(pathname, 'en')}</loc>
    <changefreq>weekly</changefreq>
    <priority>${pathname === '/' ? '1.0' : '0.7'}</priority>
${links}
${xDefault}
  </url>`;
}

export const GET: RequestHandler = () => {
	const body = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">
${PAGES.map(buildEntry).join('\n')}
</urlset>`;

	return new Response(body, {
		headers: {
			'content-type': 'application/xml; charset=utf-8',
			'cache-control': 'public, max-age=3600'
		}
	});
};
