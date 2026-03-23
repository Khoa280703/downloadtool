# Phase 02 - Content Platform Architecture for Scale

## Context Links
- [frontend/src/routes/how-to-use-snapvie/+page.svelte](../../frontend/src/routes/how-to-use-snapvie/+page.svelte)
- [frontend/src/components/seo-landing-page-shell.svelte](../../frontend/src/components/seo-landing-page-shell.svelte)

## Overview
- Priority: P0
- Status: proposed
- Brief: Nếu muốn hàng chục đến hàng trăm pages + nhiều locale, phải bỏ cách route thủ công và chuyển sang kiến trúc content data-driven.

## Key Insights
- Vấn đề lớn nhất khi scale SEO không phải viết bài, mà là quản lý content entropy.
- Route thủ công phù hợp cho 5-10 pages, không phù hợp cho 50-150 pages.
- Cần một system hỗ trợ:
  - content registry
  - locale routing
  - canonical governance
  - legacy slug handling
  - sitemap generation
  - related content graph

## Requirements
- Functional:
  - registry typed cho guides/comparisons/learn pages
  - category / intent / locale model
  - shared templates
  - route generation logic
  - sitemap source of truth
- Non-functional:
  - KISS enough to ship in current repo
  - but strong enough to survive year-scale growth

## Architecture
- Core files:
  - `frontend/src/lib/seo/content/content-types.ts`
  - `frontend/src/lib/seo/content/content-registry.ts`
  - `frontend/src/lib/seo/content/content-taxonomy.ts`
  - `frontend/src/lib/seo/content/related-content.ts`
  - `frontend/src/lib/seo/content/build-page-seo.ts`
  - `frontend/src/lib/seo/content/build-page-schema.ts`
- Routes:
  - `/guides`
  - `/guides/[slug]`
  - `/compare`
  - `/compare/[slug]`
  - optional later `/learn/[slug]`
- Locale-aware extension:
  - locale field in content model
  - canonical master slug
  - alternate locale mapping

## Related Code Files
- Create:
  - `frontend/src/lib/seo/content/*`
  - `frontend/src/components/content/*`
  - `frontend/src/routes/guides/*`
  - `frontend/src/routes/compare/*`
- Modify:
  - `frontend/src/routes/sitemap.xml/+server.ts`
  - `frontend/src/routes/robots.txt/+server.ts`

## Implementation Steps
1. Define universal content entry schema.
2. Create registry and helpers.
3. Create shared hub templates and detail templates.
4. Migrate existing support pages into the registry.
5. Add locale and legacy slug support from day one.
6. Ensure every page can be resolved from registry for sitemap and related links.

## Todo List
- [ ] Define universal content model
- [ ] Implement registry
- [ ] Add related-content graph
- [ ] Add shared templates
- [ ] Migrate existing support pages
- [ ] Wire sitemap from registry

## Success Criteria
- Adding a new content page is mostly data entry, not route boilerplate.
- Locale and content scale do not force architecture rewrite later.

## Risk Assessment
- Over-engineering too early.
- Under-modeling and falling back into hand-crafted route chaos.

## Security Considerations
- Redirect and slug mapping must only use internal vetted values.

## Next Steps
- Phase 03 and 04 fill this platform with money pages + guides.
