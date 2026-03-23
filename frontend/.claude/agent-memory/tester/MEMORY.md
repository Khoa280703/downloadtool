# Snapvie Frontend Tester Memory

## Build & Verification (2026-03-23)

### Frontend Build Status
- **TypeScript/Svelte Check:** 0 errors, 18 non-blocking warnings (state reactivity suggestions)
- **Vite Build:** Completes in ~22.6s, produces valid production artifacts
- **Circular Dependencies:** 47 from external packages (zod, kysely, better-auth) — non-blocking
- **Build Command:** `pnpm run build` (runs paraglide:compile → vite build)

### Content Registry Architecture
- **Location:** `src/lib/seo/content/content-registry.ts`
- **Exports:** 26 total entries = 20 guides + 6 comparisons
- **Guide Entries:** `src/lib/seo/content/guide-entries.ts` (1,455 lines, 20 slugs)
- **Compare Entries:** `src/lib/seo/content/compare-entries.ts` (449 lines, 6 slugs)
- **Query Helpers:** getContentBySlug(), getContentByType(), getContentByCategory()

### Sitemap Generation
- **Route:** `src/routes/sitemap.xml/+server.ts` (95 lines)
- **Logic:** Dynamically generates from CONTENT_REGISTRY via getContentByType()
- **Coverage:** Guide + Compare entries loop over content registry (lines 73-81)
- **i18n:** Full support for 34 languages with hreflang alternates
- **Includes:** Guides at `/guides/{slug}` + Comparisons at `/compare/{slug}`
- **Priority:** 0.6 for all content, monthly changefreq, dynamic lastmod dates

### Build Artifacts Verified
- `content-registry.js` — 134.74 kB ✓
- `structured-data.js` — 207.65 kB ✓
- All server chunks compiled successfully

## Testing Recommendations
- Warnings in terms page & guide/compare pages are Svelte 5 migration hints (non-blocking)
- Content registry properly integrated into sitemap generation
- No TypeScript errors blocking production deployment
