# Code Review: SEO Content Platform (Guides + Compare Clusters)

**Score: 8/10**

## Scope
- Files reviewed: 12 (content-registry, guide-entries, compare-entries, content-types, build-page-seo, build-page-schema, related-content, seo-page-events, hooks.server, public-pages, guides/[slug]/+page, compare/[slug]/+page, sitemap.xml/+server)
- Focus: content registry modularization, SEO correctness, security, internal links
- Type check: 0 errors, 18 warnings (all benign `state_referenced_locally`)

## Overall Assessment
Well-structured content platform. Registry pattern is clean, type-safe, and scales well. SEO schema, canonical URLs, and sitemap generation are correct. Legacy redirects are properly 301'd. Content quality is high — honest competitor comparisons, no thin content.

## Critical Issues
None.

## High Priority

### H1. `/guides/*` and `/compare/*` routes not covered by `isPublicSeoPage()` cache headers
`hooks.server.ts` only matches `PUBLIC_SEO_PATHS` and `/download-youtube-*`. New `/guides/` and `/compare/` routes get `private, no-store` cache-control headers, which means CDN will not cache these pages. Since they are prerendered static content, they should be CDN-cacheable.

**Fix**: extend `isPublicSeoPage()`:
```ts
if (pathname.startsWith('/guides/') || pathname.startsWith('/compare/')) return true;
if (pathname === '/guides' || pathname === '/compare') return true;
```

### H2. OG images reference non-existent per-page assets
`build-page-seo.ts` line 24 generates `ogImage = /og/guide-{slug}.png` and `/og/compare-{slug}.png`. These files likely do not exist yet. Social shares will show broken OG images until generated. Low risk for SEO but poor for social sharing CTR.

**Recommendation**: Fall back to `DEFAULT_OG_IMAGE` until per-page assets exist:
```ts
const ogImage = DEFAULT_OG_IMAGE; // TODO: generate per-page OG images
```

## Medium Priority

### M1. `{@html section.content}` XSS surface
Both `guides/[slug]/+page.svelte` (line 88) and `compare/[slug]/+page.svelte` (line 88) render raw HTML via `{@html}`. Currently safe because all content is hardcoded in TypeScript files (no user input). However, if content is ever loaded from a CMS or API, this becomes an XSS vector.

**Current risk**: None (content is compile-time static).
**Future risk**: Medium — add a comment noting the trust boundary.

### M2. Duplicate `hubPath()` function
`hubPath()` is defined identically in both `build-page-seo.ts` (line 12) and `build-page-schema.ts` (line 14). Should be extracted to `content-types.ts` or a shared util.

### M3. `SITE_URL` defined in 3 places
`SITE_URL = 'https://snapvie.com'` appears in `public-pages.ts`, `build-page-seo.ts`, and `build-page-schema.ts`. Single source of truth preferred.

### M4. guide-entries.ts is ~1450 lines
Exceeds the 200-line guideline. Each entry is ~70 lines. Consider splitting by category (how-to-entries.ts, troubleshooting-entries.ts, etc.) if more entries are added.

## Low Priority

### L1. Svelte 5 `state_referenced_locally` warnings
16 warnings across hub/detail pages for `const` derived from `$props()`. These are benign for static prerendered pages but could be resolved by wrapping in `$derived()` if reactivity is needed later.

### L2. Related guides link routing assumes same pageType
In `guides/[slug]/+page.svelte` line 100, related links hardcode `/guides/{rel.slug}`. If `relatedSlugs` ever includes a `compare` entry, the link will 404. Currently safe because all guide `relatedSlugs` reference other guides. Same issue in compare page.

**Mitigation**: Use `hubPath(rel.pageType)` instead of hardcoded path prefix.

## Edge Cases Found

1. **All 23 `relatedSlugs` references resolve** to existing entries in the registry -- verified programmatically.
2. **All 5 `relatedMoneyPage` values** map to existing routes (`download-youtube-4k`, `download-youtube-playlist`, etc.) -- verified.
3. **Legacy redirect set matches exactly** the 6 old support page slugs that were removed from `public-pages.ts` -- no orphaned redirects, no missing redirects.
4. **Sitemap correctly auto-includes** all guide and compare entries from content registry with proper `lastmod` dates.
5. **No duplicate slugs** across guide-entries and compare-entries (26 unique slugs total).

## Positive Observations

1. **Clean modularization**: content-registry.ts is 24 lines, delegates to guide-entries.ts and compare-entries.ts. Adding a page = adding an entry.
2. **Type system is solid**: ContentEntry, ContentPageType, ContentCategory are well-defined union types. Loaders validate pageType match.
3. **SEO schema is correct**: Article + BreadcrumbList + FAQPage graph, proper @id anchoring to Organization/WebSite entities.
4. **Legacy redirects done right**: 301 status, placed first in `sequence()` so they fire before paraglide/auth overhead.
5. **Content quality**: competitor comparisons are genuinely fair (acknowledge Y2mate brand recognition, ssyoutube URL trick convenience). No thin content -- each entry has 4-5 sections + 4-5 FAQ items.
6. **Analytics integration**: seo_page_view tracking on all page types with consistent params structure.
7. **Prerender enabled**: all content pages use `export const prerender = true` -- good for performance.

## Recommended Actions (prioritized)

1. **[HIGH]** Update `isPublicSeoPage()` to include `/guides/` and `/compare/` paths for CDN caching
2. **[HIGH]** Fix OG image fallback in `build-page-seo.ts` to avoid broken social previews
3. **[MED]** Extract shared `hubPath()` and `SITE_URL` to single source
4. **[LOW]** Consider splitting guide-entries.ts by category when adding more entries

## Metrics
- Type coverage: Strong (all content types fully typed)
- Build: Pass (0 errors)
- Lint warnings: 18 (all benign state_referenced_locally)
- Content entries: 26 (20 guides + 6 comparisons)
- Cross-ref integrity: 100% (all links resolve)
