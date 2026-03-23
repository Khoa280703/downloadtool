# Code Review: SEO Content Platform (Phase 01 + 02)

**Score: 7.5/10** -- Solid architecture, clean patterns, a few issues need fixing before merge.

## Scope
- Files reviewed: 20 (16 new, 4 modified)
- LOC: ~1,200 new
- Focus: content registry, schema builders, hub/detail routes, sitemap/robots integration

## Overall Assessment

Well-structured content platform with registry-driven approach (add content = add entry, no route boilerplate). Type safety is good, SEO schema is mostly correct. DRY principles followed -- hub and detail pages share builders. A few issues need attention before merge.

---

## Critical Issues

### 1. DUPLICATE SITEMAP ENTRIES for `/guides` and `/compare`
**Severity: Critical (SEO)**
**Files:** `public-pages.ts` L32-33 + `sitemap.xml/+server.ts` L83-85

`/guides` and `/compare` are added to sitemap TWICE:
- Once via `PUBLIC_PAGES` array (L32-33 in public-pages.ts)
- Again manually in sitemap.xml generator (L83-85)

Google Search Console will flag duplicate `<url>` entries. **Fix: remove lines 83-85 from sitemap.xml/+server.ts** since they're already in PUBLIC_PAGES.

### 2. DUPLICATE `buildArticleSchema` FUNCTION
**Severity: High (maintainability, potential schema drift)**
**Files:** `structured-data.ts` L130 vs `content/build-page-schema.ts` L19

Two different `buildArticleSchema` functions exist with different signatures and output:
- `structured-data.ts` version: takes `opts` object, uses inline Organization, includes `mainEntityOfPage`
- `content/build-page-schema.ts` version: takes `ContentEntry`, uses `@id` references, includes `isPartOf`

The content version uses `@id` references (correct for linked data graph). The structured-data.ts version inlines Organization data. Schema drift risk if one is updated and the other is not.

**Fix:** Either consolidate to one function, or clearly document that they serve different page types and should not be mixed.

---

## High Priority

### 3. DUPLICATE CONTENT: flat routes vs hub routes serve SAME content
**Severity: High (SEO cannibalization)**

Old flat routes still exist (e.g., `/how-to-use-snapvie/+page.svelte`) AND the same slug is now served at `/guides/how-to-use-snapvie`. Both URLs are in the sitemap. This causes:
- Keyword cannibalization (Google indexes both)
- Duplicate content penalty risk

**Fix:** Either 301-redirect old flat routes to `/guides/{slug}` or remove old routes entirely. The `legacySlug` field in ContentEntry already anticipates this -- implement the redirects.

### 4. Svelte 5 `state_referenced_locally` warnings (14 warnings)
**Severity: Medium-High (correctness)**
**Files:** All 4 `.svelte` files in guides/ and compare/

```
"This reference only captures the initial value of `data`"
```

In Svelte 5, `const meta = buildHubPageMeta(data.config)` outside `$derived` captures initial value only. If `data` changes (e.g., client-side navigation between pages), `meta` and `jsonLd` won't update.

Since these are prerendered static pages, this is **not a runtime bug** right now. But it will become a bug if prerender is removed or if these pages are ever SSR'd with client-side nav.

**Fix:** Wrap derived values in `$derived`:
```ts
const meta = $derived(buildHubPageMeta(data.config));
const jsonLd = $derived(buildHubPageJsonLd(data.config, data.entries));
```

### 5. `{@html section.content}` -- XSS surface
**Severity: Medium-High (security)**
**Files:** `guides/[slug]/+page.svelte` L77, `compare/[slug]/+page.svelte` L77

`section.content` is rendered via `{@html}`. Currently safe because all content is hardcoded in the registry. BUT:
- If registry is ever populated from a CMS, database, or user input, this becomes an XSS vector
- No sanitization layer exists

**Mitigation (acceptable for now):** Content is developer-controlled in `content-registry.ts`. Add a code comment warning future developers:
```ts
// SECURITY: Content rendered via {@html} — must be developer-authored only.
// If content source changes to CMS/API, add DOMPurify sanitization.
```

---

## Medium Priority

### 6. `SITE_URL` defined in 3 places
**Files:** `build-page-seo.ts` L8, `build-page-schema.ts` L10, `public-pages.ts` L10

`const SITE_URL = 'https://snapvie.com'` is defined independently in 3 files. Violates DRY.

**Fix:** Import from `public-pages.ts` (already exported) in the other two files.

### 7. `hubPath()` duplicated
**Files:** `build-page-seo.ts` L12, `build-page-schema.ts` L14

Same `hubPath()` function exists in two files.

**Fix:** Export from one file and import in the other, or move to content-types.ts.

### 8. `buildItemListSchema` in structured-data.ts is UNUSED
**File:** `structured-data.ts` L156

Added in this PR but never imported anywhere. Dead code -- hub pages use `buildHubPageJsonLd` from `build-page-schema.ts` instead.

**Fix:** Remove to avoid confusion.

### 9. Content registry file is 454 lines
**File:** `content-registry.ts`

Already exceeds the 200-line modularization guideline. Will grow significantly as more content is added.

**Fix:** Consider splitting content entries into per-category or per-pageType files and re-exporting from registry.

### 10. `seo-page-events.ts` is defined but UNUSED
**File:** `frontend/src/lib/analytics/seo-page-events.ts`

No component imports `trackSeoPageView`, `trackSeoInputFocus`, etc. These are dead code in the current PR.

**Acceptable:** If Phase 03+ will wire these up, this is fine as scaffold. Otherwise remove.

---

## Low Priority

### 11. OG images reference non-existent files
**File:** `build-page-seo.ts` L24

```ts
const ogImage = `${SITE_URL}/og/${entry.pageType}-${entry.slug}.png`;
```

These per-page OG images (`/og/guide-how-to-use-snapvie.png`) don't exist yet. Social sharing will show broken images.

**Mitigation:** Fall back to `DEFAULT_OG_IMAGE` until per-page images are generated.

### 12. Compare hub shows empty state but compare registry has 0 entries
Currently `CONTENT_REGISTRY` has 0 entries with `pageType: 'compare'`. The compare hub will show "No comparisons yet" which is fine UX, but `/compare` is in the sitemap with priority 0.7 pointing to an empty page.

---

## Positive Observations

1. **Registry-driven architecture** is excellent -- adding content requires no route changes
2. **Type safety** is solid -- `ContentEntry`, `ContentCategory`, `ContentPageType` all properly typed
3. **JSON-LD schema** correctly uses `@graph` pattern with `@id` entity linking
4. **FAQ schema** conditionally included only when faqItems present
5. **Prerender strategy** is correct -- hub pages link to detail pages enabling SvelteKit crawl discovery
6. **llms.txt** implementation is clean and follows the emerging spec
7. **robots.txt** correctly adds `Llms-Txt` directive
8. **Dark mode** support included via `:global(.page-root.theme-dark)` selectors
9. **Related content** resolver with explicit + category fallback is well designed

---

## Recommended Actions (Priority Order)

1. **Remove duplicate sitemap entries** for `/guides` and `/compare` (Critical, 2 lines)
2. **Plan 301 redirects** from flat support page URLs to `/guides/{slug}` (High, SEO)
3. **Fix `$derived` warnings** in all 4 Svelte components (Medium-High, 8 lines)
4. **Consolidate `SITE_URL`** imports (Medium, 4 lines)
5. **Remove unused `buildItemListSchema`** from structured-data.ts (Medium, 10 lines)
6. **Add `{@html}` security comment** in content-registry.ts (Low, 2 lines)
7. **Fall back OG image** when per-page image doesn't exist (Low)

## Metrics
- Type Coverage: ~95% (all new code well-typed)
- svelte-check: 0 ERRORS, 14 WARNINGS (all `state_referenced_locally`)
- Dead code: 2 functions (buildItemListSchema, seo-page-events module)

## Unresolved Questions
1. Are the old flat support page routes (`/how-to-use-snapvie`, `/why-youtube-downloads-show-360p-only`, etc.) intended to coexist with the new `/guides/{slug}` routes long-term, or should they 301-redirect?
2. When will per-page OG images be generated? Social shares currently broken for content pages.
3. Will `seo-page-events.ts` be wired in Phase 03?
