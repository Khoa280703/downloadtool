# Phase Implementation Report

### Executed Phase
- Phase: SEO Events + llms.txt + Schema Updates
- Plan: ad-hoc (no plan dir)
- Status: completed

### Files Modified

| File | Action | Notes |
|------|--------|-------|
| `frontend/src/lib/analytics/seo-page-events.ts` | CREATED | 90 lines — 6 SEO event trackers |
| `frontend/src/routes/llms.txt/+server.ts` | CREATED | 52 lines — llms.txt endpoint |
| `frontend/src/lib/seo/structured-data.ts` | MODIFIED | Appended `buildArticleSchema` + `buildItemListSchema` (163 total) |
| `frontend/src/lib/seo/public-pages.ts` | MODIFIED | Added `/guides` + `/compare` entries |
| `frontend/src/routes/robots.txt/+server.ts` | MODIFIED | Added `Llms-Txt:` directive |

### Tasks Completed
- [x] Created `seo-page-events.ts` with 6 typed event trackers
- [x] Created `llms.txt/+server.ts` with AI Search readiness content
- [x] Appended `buildArticleSchema` to `structured-data.ts`
- [x] Appended `buildItemListSchema` to `structured-data.ts`
- [x] Added `/guides` + `/compare` to `PUBLIC_PAGES`
- [x] Added `Llms-Txt:` line to robots.txt
- [x] Fixed TypeScript type mismatch (`SeoEventParams` vs `Record<string, string | number | boolean>`) by building flat params via `buildParams()` helper

### Tests Status
- Type check: PASS (0 errors, 2 pre-existing CSS warnings in terms page)
- Unit tests: not applicable (event trackers are thin wrappers, no logic to unit test)

### Issues Encountered
- `SeoEventParams` with optional `cluster?: string` was not assignable to `Record<string, string | number | boolean>` (undefined not allowed). Fixed by introducing internal `buildParams()` helper that explicitly builds a flat, non-undefined object before passing to `trackEvent`.

### Next Steps
- Wire `trackSeoPageView` into `seo-landing-page-shell.svelte` `onMount` to start measuring organic traffic by page group
- Wire `trackSeoInputFocus` / `trackSeoExtractSubmit` into the URL input component when used inside SEO shell pages
- Use `buildArticleSchema` in guide/compare page `+page.ts` load functions once those routes exist
- Use `buildItemListSchema` in `/guides` and `/compare` hub pages

### Unresolved Questions
- None
