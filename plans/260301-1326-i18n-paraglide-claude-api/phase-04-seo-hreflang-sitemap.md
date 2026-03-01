# Phase 4: SEO — hreflang Tags + Sitemap

**Status:** completed | **Priority:** high | **Can parallel with Phase 5**

> **CRITICAL WARNING — Prerendered Homepage:**
> The homepage has `export const prerender = true` in its `+page.server.ts` (or `+page.ts`).
> `transformPageChunk` in `hooks.server.ts` does **NOT** run for prerendered static HTML — it only fires during SSR.
> Therefore, hreflang injection via hooks does NOT work for the homepage.
>
> **Solution:** hreflang tags for the homepage MUST be added statically in `+page.svelte`'s `<svelte:head>` as hardcoded `<link>` tags (see Section 4.5).
> Only SSR routes (`/privacy`, `/account`) benefit from the hooks injection approach.

## Overview

Paraglide does NOT auto-generate hreflang. Must manually inject into every HTML response via `hooks.server.ts` for SSR routes, and statically in `<svelte:head>` for prerendered routes. Also generate `sitemap.xml` with all language variants.

## 4.1 hreflang Implementation in hooks.server.ts

```ts
// src/hooks.server.ts
import { i18n } from '$lib/i18n';
import { sequence } from '@sveltejs/kit/hooks';
import type { Handle } from '@sveltejs/kit';

const LANGUAGE_TAGS = [
  'ar', 'bg', 'cs', 'da', 'de', 'el', 'en', 'es', 'et', 'fi',
  'fr', 'hu', 'id', 'it', 'ja', 'ko', 'lt', 'lv', 'nb', 'nl',
  'pl', 'pt', 'pt-BR', 'ro', 'ru', 'sk', 'sl', 'sv', 'tr', 'uk',
  'vi', 'zh', 'zh-TW',
];

// BCP 47 → hreflang mapping (most are identical, some need adjustment)
const HREFLANG_MAP: Record<string, string> = {
  'pt-BR': 'pt-BR',
  'zh-TW': 'zh-TW',
  'nb': 'no',  // Norwegian Bokmål → "no" for Google
};

function buildHreflangTags(origin: string, pathname: string): string {
  // Strip existing lang prefix to get the canonical path
  const langPrefixRegex = new RegExp(`^/(${LANGUAGE_TAGS.join('|')})(/|$)`);
  const canonicalPath = pathname.replace(langPrefixRegex, '/');

  const tags = LANGUAGE_TAGS.map((lang) => {
    const hreflang = HREFLANG_MAP[lang] ?? lang;
    const url = lang === 'en'
      ? `${origin}${canonicalPath}`  // default (no prefix for en)
      : `${origin}/${lang}${canonicalPath === '/' ? '' : canonicalPath}`;
    return `<link rel="alternate" hreflang="${hreflang}" href="${url}" />`;
  }).join('\n    ');

  // x-default points to English (no prefix)
  const xDefault = `<link rel="alternate" hreflang="x-default" href="${origin}${canonicalPath}" />`;
  return `${tags}\n    ${xDefault}`;
}

const hreflangHandle: Handle = async ({ event, resolve }) => {
  const response = await resolve(event, {
    transformPageChunk: ({ html }) => {
      if (!html.includes('</head>')) return html;
      const hreflangTags = buildHreflangTags(
        event.url.origin,
        event.url.pathname
      );
      return html.replace('</head>', `  ${hreflangTags}\n</head>`);
    },
  });
  return response;
};

// Add hreflangHandle to the existing sequence chain from Phase 1.
// Do not overwrite the existing custom handle body.
// Reuse `existingHandle` defined in Phase 1 (which already wraps all auth/cache logic).
export const handle = sequence(
  i18n.handle(),
  hreflangHandle,
  existingHandle,
);
```

## 4.2 Per-locale Meta Tags (title, description)

In each page's `<svelte:head>`, use translated messages for title/description:

```svelte
<!-- src/routes/+page.svelte -->
<svelte:head>
  <title>{m.home_meta_title()}</title>
  <meta name="description" content={m.home_meta_description()} />
</svelte:head>
```

Add to `messages/en.json`:
```json
{
  "home_meta_title": "FetchTube — Free YouTube Video Downloader",
  "home_meta_description": "Download YouTube videos in any format. Fast, free, no ads.",
  "privacy_meta_title": "Privacy Policy — FetchTube",
  "account_meta_title": "Your Account — FetchTube"
}
```

## 4.3 Sitemap with Language Variants

Create `src/routes/sitemap.xml/+server.ts`:

```ts
import type { RequestHandler } from './$types';

const ORIGIN = process.env.ORIGIN ?? 'https://download.khoadangbui.online';
const LANGUAGES = [
  'ar', 'bg', 'cs', 'da', 'de', 'el', 'en', 'es', 'et', 'fi',
  'fr', 'hu', 'id', 'it', 'ja', 'ko', 'lt', 'lv', 'nb', 'nl',
  'pl', 'pt', 'pt-BR', 'ro', 'ru', 'sk', 'sl', 'sv', 'tr', 'uk',
  'vi', 'zh', 'zh-TW',
];

const PAGES = ['/', '/privacy'];  // user-facing pages (no /account — auth-required)

function buildUrlEntry(path: string): string {
  const alternates = LANGUAGES.map((lang) => {
    const url = lang === 'en' ? `${ORIGIN}${path}` : `${ORIGIN}/${lang}${path === '/' ? '' : path}`;
    return `    <xhtml:link rel="alternate" hreflang="${lang}" href="${url}"/>`;
  }).join('\n');

  const xDefault = `    <xhtml:link rel="alternate" hreflang="x-default" href="${ORIGIN}${path}"/>`;

  return `
  <url>
    <loc>${ORIGIN}${path}</loc>
    <changefreq>weekly</changefreq>
    <priority>${path === '/' ? '1.0' : '0.7'}</priority>
${alternates}
${xDefault}
  </url>`;
}

export const GET: RequestHandler = () => {
  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<urlset
  xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
  xmlns:xhtml="http://www.w3.org/1999/xhtml">
${PAGES.map(buildUrlEntry).join('\n')}
</urlset>`;

  return new Response(xml, {
    headers: {
      'Content-Type': 'application/xml',
      'Cache-Control': 'public, max-age=3600',
    },
  });
};
```

## 4.5 hreflang for Prerendered Homepage (STATIC)

Because the homepage is prerendered, hreflang must be added directly in `src/routes/+page.svelte`'s `<svelte:head>` as hardcoded tags. All 33 locale variants:

```svelte
<!-- src/routes/+page.svelte -->
<svelte:head>
  <title>{m.home_meta_title()}</title>
  <meta name="description" content={m.home_meta_description()} />

  <!-- hreflang: static for prerendered homepage — hooks transformPageChunk does NOT run -->
  <link rel="alternate" hreflang="x-default" href="https://download.khoadangbui.online/" />
  <link rel="alternate" hreflang="en" href="https://download.khoadangbui.online/" />
  <link rel="alternate" hreflang="ar" href="https://download.khoadangbui.online/ar/" />
  <link rel="alternate" hreflang="bg" href="https://download.khoadangbui.online/bg/" />
  <link rel="alternate" hreflang="cs" href="https://download.khoadangbui.online/cs/" />
  <link rel="alternate" hreflang="da" href="https://download.khoadangbui.online/da/" />
  <link rel="alternate" hreflang="de" href="https://download.khoadangbui.online/de/" />
  <link rel="alternate" hreflang="el" href="https://download.khoadangbui.online/el/" />
  <link rel="alternate" hreflang="es" href="https://download.khoadangbui.online/es/" />
  <link rel="alternate" hreflang="et" href="https://download.khoadangbui.online/et/" />
  <link rel="alternate" hreflang="fi" href="https://download.khoadangbui.online/fi/" />
  <link rel="alternate" hreflang="fr" href="https://download.khoadangbui.online/fr/" />
  <link rel="alternate" hreflang="hu" href="https://download.khoadangbui.online/hu/" />
  <link rel="alternate" hreflang="id" href="https://download.khoadangbui.online/id/" />
  <link rel="alternate" hreflang="it" href="https://download.khoadangbui.online/it/" />
  <link rel="alternate" hreflang="ja" href="https://download.khoadangbui.online/ja/" />
  <link rel="alternate" hreflang="ko" href="https://download.khoadangbui.online/ko/" />
  <link rel="alternate" hreflang="lt" href="https://download.khoadangbui.online/lt/" />
  <link rel="alternate" hreflang="lv" href="https://download.khoadangbui.online/lv/" />
  <link rel="alternate" hreflang="no" href="https://download.khoadangbui.online/nb/" />
  <link rel="alternate" hreflang="nl" href="https://download.khoadangbui.online/nl/" />
  <link rel="alternate" hreflang="pl" href="https://download.khoadangbui.online/pl/" />
  <link rel="alternate" hreflang="pt" href="https://download.khoadangbui.online/pt/" />
  <link rel="alternate" hreflang="pt-BR" href="https://download.khoadangbui.online/pt-BR/" />
  <link rel="alternate" hreflang="ro" href="https://download.khoadangbui.online/ro/" />
  <link rel="alternate" hreflang="ru" href="https://download.khoadangbui.online/ru/" />
  <link rel="alternate" hreflang="sk" href="https://download.khoadangbui.online/sk/" />
  <link rel="alternate" hreflang="sl" href="https://download.khoadangbui.online/sl/" />
  <link rel="alternate" hreflang="sv" href="https://download.khoadangbui.online/sv/" />
  <link rel="alternate" hreflang="tr" href="https://download.khoadangbui.online/tr/" />
  <link rel="alternate" hreflang="uk" href="https://download.khoadangbui.online/uk/" />
  <link rel="alternate" hreflang="vi" href="https://download.khoadangbui.online/vi/" />
  <link rel="alternate" hreflang="zh" href="https://download.khoadangbui.online/zh/" />
  <link rel="alternate" hreflang="zh-TW" href="https://download.khoadangbui.online/zh-TW/" />
</svelte:head>
```

> Note: `nb` (Norwegian Bokmål) maps to hreflang `no` per Google guidelines.

## 4.4 robots.txt — Add Sitemap Reference

Update `static/robots.txt`:
```
User-agent: *
Allow: /

Sitemap: https://download.khoadangbui.online/sitemap.xml
```

## Files Modified/Created

- `src/hooks.server.ts` — add hreflangHandle + sequence()
- `src/routes/+page.svelte` — add translated meta title/description
- `src/routes/privacy/+page.svelte` — add translated meta
- `src/routes/sitemap.xml/+server.ts` — new file
- `static/robots.txt` — add Sitemap line
- `messages/en.json` — add meta_title, meta_description keys

## Success Criteria

- [x] `curl https://download.khoadangbui.online/` (prerendered static file) -> hreflang tags present in raw HTML (static head)
- [x] `curl http://127.0.0.1:4173/vi/privacy` (SSR route) -> `<link rel="alternate" hreflang="vi">` present via hooks injection
- [x] `x-default` hreflang points to `/` (English, no prefix)
- [x] `/sitemap.xml` returns valid XML with all 33 locale variants
- [ ] Google Search Console validates sitemap (post-deploy)
- [x] Locale meta title keys wired for pages in scope
