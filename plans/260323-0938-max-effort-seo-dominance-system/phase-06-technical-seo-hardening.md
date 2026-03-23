# Phase 06 - Technical SEO Hardening

## Context Links
- `frontend/src/routes/sitemap.xml/+server.ts`
- `frontend/src/routes/robots.txt/+server.ts`
- `frontend/src/app.html`
- public page routes

## Overview
- Priority: P0
- Status: proposed
- Brief: Khóa technical SEO ở mức "khó lỗi", đủ để scale số lượng page và locale mà không tạo index mess.

## Key Insights
- Technical debt ở SEO thường không giết site ngay, nhưng nó làm mọi content effort kém hiệu quả.
- Khi page count tăng, một lỗi nhỏ ở canonical/hreflang/sitemap sẽ nhân lên thành thảm họa.

## Requirements
- Functional:
  - canonical sạch mọi public page
  - hreflang sạch mọi locale page
  - sitemap từ content registry
  - breadcrumb schema
  - robots strategy rõ
  - noindex cho auth/internal pages
  - clean status codes and redirect mapping
- Non-functional:
  - fast SSR/public rendering
  - crawl-friendly head/html

## Hardening Checklist
- canonical self-reference
- alternate locale mapping
- clean OG/Twitter on all public pages
- breadcrumb schema
- FAQ / HowTo / Article schema only when eligible
- image optimization
- favicon stability
- SW caching does not corrupt HTML/indexability
- no blocking resources harming render
- 404 page useful and crawl-safe
- no accidental soft-404 for thin pages

## CWV / Performance Work
- LCP focus
- preload discipline
- image sizing
- font strategy
- JS pruning on SEO pages
- avoid shipping interactive weight to purely informational pages where unnecessary
- Use Phase 01 CWV baseline as starting point for fixes

## Schema Strategy Matrix (Added from review)
- Page type → eligible schema types:
  | Page Type | Schema Types |
  |-----------|-------------|
  | Homepage | Organization, WebSite, WebApplication |
  | Money pages | WebApplication, FAQPage, BreadcrumbList, HowTo (if step-based) |
  | Guides | Article (datePublished, dateModified, author), FAQPage, HowTo, BreadcrumbList |
  | Comparison pages | Article, ItemList (feature matrix), BreadcrumbList |
  | Hub pages (/guides, /compare) | ItemList, CollectionPage, BreadcrumbList |
  | Trust pages | Organization, ContactPoint, BreadcrumbList |
- Rules:
  - Only add schema types the page genuinely qualifies for
  - Every Article schema needs datePublished + dateModified + author
  - SoftwareApplication schema: add aggregateRating when real user reviews exist (not before)
  - Never use Review/Rating schema without genuine user data

## Video SEO (Added from review)
- Snapvie is a video tool — natural competitive advantage for video SERP.
- Strategy:
  1. **VideoObject schema** on guide pages that have embedded demo videos
  2. **Video sitemap** (`/video-sitemap.xml`) for all pages with video content
  3. **YouTube channel** for Snapvie:
     - Upload tool walkthrough/demo videos
     - Link channel → snapvie.com (brand entity signal)
     - Embed these videos on relevant guide pages
  4. **Thumbnail optimization**: Custom thumbnails for all video content
- Priority video content:
  - "How to use Snapvie" demo (30-60s)
  - "How to download YouTube playlist" walkthrough
  - "4K vs 8K download quality comparison" visual demo
  - "Why 360p only — explained" quick explainer
- Schema example for guide with video:
  ```json
  { "@type": "VideoObject", "name": "...", "description": "...",
    "thumbnailUrl": "...", "uploadDate": "...", "duration": "...",
    "contentUrl": "..." }
  ```

## AI Search / SGE Readiness (Added from review)
- Google SGE / AI Overviews extract structured answers from pages.
- Optimization:
  1. **Quick answer blocks**: Every guide starts with a concise 2-3 sentence direct answer
  2. **Structured headings**: Use clear H2/H3 hierarchy that maps to sub-questions
  3. **FAQ pairs**: Q&A format naturally feeds AI extraction
  4. **Data tables**: Comparison tables are highly extractable by AI
  5. **`llms.txt`**: Add `/llms.txt` file describing site purpose, key pages, and capabilities
  6. **Avoid**: walls of text, ambiguous headings, content hidden behind JS interactions

## OG Image Strategy (Added from review)
- Current: placeholder `/og-image.png` (not deployed)
- Target:
  1. **Unique OG image per money page**: Show page-specific visual (4K badge, playlist icon, etc.)
  2. **Dynamic OG images for guides/compare**: Generate via edge function or build-time
  3. **OG image specs**: 1200x630px, clear text overlay, brand colors, readable at small size
  4. **Twitter card**: Use `summary_large_image` for all public pages
- Implementation options:
  - Build-time: generate static images per page during build (simplest)
  - Edge: Satori/Vercel OG-like approach for dynamic generation
  - Manual: design 5-6 money page images + 1 template for guides/compare
- Priority: money pages first (5 images), then guide template (1 reusable design)

## Related Code Files
- Modify:
  - app/head files
  - sitemap route
  - robots route
  - layout
  - public page templates

## Implementation Steps
1. Technical audit template for every public page type.
2. Move sitemap generation to registry-driven approach.
3. Verify canonical/hreflang.
4. Add quality gates in CI where practical.
5. Review CWV bottlenecks on money and guide templates.

## Todo List
- [ ] Canonical audit all page types
- [ ] Hreflang audit all locale variants
- [ ] Registry-driven sitemap
- [ ] Breadcrumb schema across content pages
- [ ] Noindex audit for non-public surfaces
- [ ] CWV budget for public pages (use Phase 01 baseline)
- [ ] Implement schema strategy matrix across all page types
- [ ] Add Article schema with dates/author to guides and compare pages
- [ ] Add ItemList schema to hub pages
- [ ] Create VideoObject schema builder
- [ ] Create video sitemap route
- [ ] Set up YouTube channel + upload initial demo videos
- [ ] Add AI Search quick-answer blocks to all guides
- [ ] Create `/llms.txt` file
- [ ] Design + deploy unique OG images for 5 money pages
- [ ] Create OG image template for guides/compare pages
- [ ] Switch all public pages to `summary_large_image` Twitter card

## Success Criteria
- Technical SEO remains clean as page count grows.
- New content types inherit good defaults automatically.

## Risk Assessment
- Locale growth breaks canonical/hreflang.
- Too many schema types create invalid markup or wasted effort.

## Security Considerations
- Redirect and canonical generation must not accept user-controlled destinations.

## Next Steps
- Phase 07 strengthens trust and entity signals.
