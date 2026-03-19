import { building } from '$app/environment';
import type { Handle } from '@sveltejs/kit';
import { sequence } from '@sveltejs/kit/hooks';
import { getTextDirection } from '$lib/paraglide/runtime';
import { paraglideMiddleware } from '$lib/paraglide/server';
import { svelteKitHandler } from 'better-auth/svelte-kit';
import { auth } from '$lib/server/auth';

const PUBLIC_SEO_CACHE_CONTROL = 'public, s-maxage=3600, stale-while-revalidate=86400';
const PRIVATE_CACHE_CONTROL = 'private, no-store';
const AUTH_API_CACHE_CONTROL = 'private, no-store, max-age=0, must-revalidate';

/** Paths that are public SEO pages — gets CDN-cacheable headers */
const PUBLIC_SEO_PATHS = ['/', '/privacy', '/about', '/terms', '/contact', '/dmca'];

function isPublicSeoPage(pathname: string): boolean {
	if (PUBLIC_SEO_PATHS.includes(pathname)) return true;
	if (pathname.startsWith('/download-youtube-')) return true;
	return false;
}
const LANGUAGE_TAGS = [
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

const HREFLANG_MAP: Record<string, string> = {
	nb: 'no'
};

function stripLocalePrefix(pathname: string): string {
	for (const lang of LANGUAGE_TAGS) {
		if (lang === 'en') continue;
		if (pathname === `/${lang}`) return '/';
		if (pathname.startsWith(`/${lang}/`)) return pathname.slice(lang.length + 1) || '/';
	}

	return pathname || '/';
}

function buildLocalizedHref(origin: string, locale: string, canonicalPath: string): string {
	if (locale === 'en') return `${origin}${canonicalPath}`;
	if (canonicalPath === '/') return `${origin}/${locale}/`;
	return `${origin}/${locale}${canonicalPath}`;
}

function buildHreflangTags(origin: string, pathname: string): string {
	const canonicalPath = stripLocalePrefix(pathname);
	const tags = LANGUAGE_TAGS.map((lang) => {
		const hreflang = HREFLANG_MAP[lang] ?? lang;
		const href = buildLocalizedHref(origin, lang, canonicalPath);
		return `<link rel="alternate" hreflang="${hreflang}" href="${href}" />`;
	});

	tags.push(
		`<link rel="alternate" hreflang="x-default" href="${buildLocalizedHref(origin, 'en', canonicalPath)}" />`
	);

	return tags.join('\n');
}

function normalizeRedirectTarget(value: string | null): string {
	if (!value || !value.startsWith('/') || value.startsWith('//')) {
		return '/';
	}

	return value;
}

const existingHandle: Handle = async ({ event, resolve }) => {
	if (building) {
		event.locals.session = null;
		event.locals.user = null;
	} else {
		// Skip DB session lookup when no Better Auth cookie is present (unauthenticated users).
		// auth.api.getSession() queries the sessions table — avoid this on every unauthenticated request.
		const cookieHeader = event.request.headers.get('cookie') ?? '';

		const hasBetterAuthCookie = cookieHeader.includes('better-auth.');

		if (hasBetterAuthCookie) {
			const authSession = await auth.api.getSession({ headers: event.request.headers });

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

	const response = await svelteKitHandler({ auth, event, resolve, building });

	if ((event.request.method === 'GET' || event.request.method === 'HEAD') && !event.url.pathname.startsWith('/api/') && response.headers.get('content-type')?.startsWith('text/html')) {
		const isPublic = isPublicSeoPage(event.url.pathname);
		response.headers.set('cache-control', isPublic ? PUBLIC_SEO_CACHE_CONTROL : PRIVATE_CACHE_CONTROL);
		if (!isPublic) {
			response.headers.set('vary', 'cookie');
		}
	}

	if (event.url.pathname.startsWith('/api/auth/')) {
		response.headers.set('cache-control', AUTH_API_CACHE_CONTROL);
		response.headers.set('pragma', 'no-cache');
		response.headers.set('expires', '0');
		response.headers.set('vary', 'cookie');
	}

	return response;
};

const handleParaglide: Handle = ({ event, resolve }) =>
	paraglideMiddleware(event.request, ({ request, locale }: { request: Request; locale: string }) => {
		event.request = request;

		return resolve(event, {
			transformPageChunk: ({ html }) => {
				const withLocaleAttrs = html
					.replace('%paraglide.lang%', locale)
					.replace('%paraglide.dir%', getTextDirection(locale));

				if (!withLocaleAttrs.includes('</head>')) return withLocaleAttrs;

				// During prerender, use production origin instead of the internal sveltekit-prerender origin
				const origin = event.url.origin.includes('sveltekit-prerender')
					? 'https://snapvie.com'
					: event.url.origin;
				const hreflangTags = buildHreflangTags(origin, event.url.pathname);
				return withLocaleAttrs.replace(
					'</head>',
					`  ${hreflangTags
						.split('\n')
						.join('\n  ')}
</head>`
				);
			}
		});
	});

export const handle = sequence(handleParaglide, existingHandle);
