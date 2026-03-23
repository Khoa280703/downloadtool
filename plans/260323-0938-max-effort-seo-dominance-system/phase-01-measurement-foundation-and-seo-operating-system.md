# Phase 01 - Measurement Foundation and SEO Operating System

## Context Links
- [docs/project-overview-pdr.md](../../docs/project-overview-pdr.md)
- [plans/260319-0929-ui-preserving-seo-snapvie/phase-05-measurement-rollout-expansion-gate.md](../260319-0929-ui-preserving-seo-snapvie/phase-05-measurement-rollout-expansion-gate.md)

## Overview
- Priority: P0
- Status: proposed
- Brief: Không đo thì không có SEO strategy, chỉ có cảm giác. Phase này dựng toàn bộ nền điều hành SEO theo page-level, cluster-level, query-level.

## Key Insights
- Tool site phải đo cả traffic lẫn hành vi sản phẩm:
  - page view
  - scroll / interaction
  - input focus
  - extract submit
  - extract success
  - download start
  - download success
- GSC cho biết query. Product analytics cho biết giá trị thật.
- Nếu không có join giữa hai lớp dữ liệu này, sẽ tối ưu nhầm pages kéo traffic rác.

## Requirements
- Functional:
  - GSC property hygiene hoàn chỉnh
  - page-level event naming chuẩn
  - keyword cluster dashboard
  - content inventory sheet / registry tracking
  - monthly scorecard
- Non-functional:
  - không log URL người dùng paste
  - không log PII
  - event naming ổn định đủ cho 12 tháng

## Architecture
- Inputs:
  - Search Console
  - Cloudflare Web Analytics
  - app telemetry / custom events
- Output:
  - dashboard by page cluster
  - dashboard by query theme
  - landing page -> conversion map

## Related Code Files
- Modify:
  - `frontend/src/routes/+page.svelte`
  - all money pages
  - all guide/comparison templates later
- Create:
  - `frontend/src/lib/analytics/seo-page-events.ts`
  - `docs/seo-ops-scorecard.md`
  - `docs/seo-page-inventory.md`

## Implementation Steps
1. Define canonical page groups:
   - homepage
   - money pages
   - guides
   - comparison pages
   - trust pages
2. Define event taxonomy:
   - `seo_page_view`
   - `seo_input_focus`
   - `seo_extract_submit`
   - `seo_extract_success`
   - `seo_download_start`
   - `seo_playlist_start`
3. Add page metadata to events:
   - page type
   - cluster
   - locale
   - canonical slug
4. Build weekly review template:
   - impressions
   - clicks
   - avg position
   - organic conversion
5. Create growth decision rules:
   - promote
   - refresh
   - merge
   - prune

## Core Web Vitals Baseline (Added from review)
- Measure CWV baseline BEFORE content scale begins, not after.
- Targets:
  - LCP < 2.5s on all public pages
  - INP < 200ms
  - CLS < 0.1
- Steps:
  1. Run Lighthouse CI on all money pages + homepage
  2. Run PageSpeed Insights for field data if available
  3. Document JS bundle size per page type (money page vs guide vs compare)
  4. Set performance budget: JS < 100KB gzipped for SEO pages
  5. Identify biggest LCP blockers (fonts, images, SSR delays)
  6. Create CWV dashboard or tracking sheet
- This baseline feeds into Phase 06 (Technical SEO Hardening) for fixes.

## Todo List
- [ ] Define SEO event taxonomy
- [ ] Add page-group metadata
- [ ] Create content inventory
- [ ] Create weekly SEO review template
- [ ] Create monthly scorecard
- [ ] Define page promotion / prune rules
- [ ] Run CWV baseline audit on all public pages
- [ ] Document JS bundle size per page type
- [ ] Set performance budget

## Success Criteria
- Every SEO page has measurable query + behavior signal.
- Team can say which pages drive downloads, not just traffic.

## Risk Assessment
- Over-instrumentation without clear naming causes noisy data.
- Missing query-to-page mapping causes false prioritization.

## Security Considerations
- Never store pasted URLs or user private content in analytics.

## Next Steps
- Phase 02 builds the architecture that can scale under this measurement system.
