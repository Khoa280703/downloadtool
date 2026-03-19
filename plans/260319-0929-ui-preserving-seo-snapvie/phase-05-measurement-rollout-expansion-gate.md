# Phase 05 - Measurement, Rollout, Expansion Gate

## Context Links
- [docs/project-overview-pdr.md](../../docs/project-overview-pdr.md)
- [frontend/src/routes/+page.svelte](../../frontend/src/routes/+page.svelte)
- Search Console / analytics setup outside repo

## Overview
- Priority: P1
- Status: deferred (requires post-deploy data)
- Brief: Đo đúng hiệu quả SEO và khóa chặt điều kiện trước khi mở rộng VI/PT-BR sau EN.

## Key Insights
- Không đo -> không biết page nào kéo impression, page nào kéo conversion.
- SEO tool site phải gắn với business outcome: extract starts, successful downloads, retained usage.
- i18n expansion quá sớm rất dễ tạo translated thin pages.
- Trước khi scale locale, phải pass multilingual technical SEO audit cho homepage + landing page template.
- Instrumentation phải ship cùng rollout, không phải đợi ranking rồi mới thêm.

## Requirements
- Functional:
  - Define KPI dashboard tối thiểu.
  - Define rollout checklist sau deploy.
  - Define gate để mở rộng sang VI/PT-BR sau EN.
  - Define multilingual readiness checklist trước khi mở locale SEO pages.
- Non-functional:
  - Không đòi hỏi data platform lớn ở giai đoạn đầu.
  - Dùng Search Console + app analytics hiện có là đủ.

## Architecture
- Combine 3 lớp dữ liệu:
  - Search Console: query/impression/click/position
  - Web analytics: landing page views, CTR into tool usage
  - Product analytics/logs: extract submit, extract success, download start, playlist start
- Mỗi SEO page có page-level attribution cơ bản.

## Related Code Files
- Modify:
  - `frontend/src/routes/+page.svelte` (nếu cần page tagging)
  - landing pages created in phase 03
- Create:
  - `docs/seo-rollout-checklist.md` (optional)
  - lightweight analytics constants/helpers if needed
- Delete:
  - none

## Implementation Steps
1. Define keyword clusters and success mapping:
   - homepage
   - 8K HDR
   - playlist
   - shorts
   - 4K
2. Track page-level user journey:
   - page view
   - click/focus on input
   - extract request
   - download start
3. Ship measurement baseline cùng homepage + landing pages rollout.
4. After deploy, request indexing on homepage + 4 landing pages.
5. Review after 2 weeks:
   - indexed page count
   - impressions by page
   - clicks by page
   - average position by cluster
   - conversion from organic landing page to tool usage
6. Expansion gate for VI/PT-BR only if:
   - EN pages index ổn
   - ít nhất 1-2 pages có traction rõ
   - content template proven, not thin
   - canonical/hreflang/internal links pass audit trên page template
   - locale metadata/social tags strategy đã chuẩn hóa
7. Nếu pass gate:
   - mở VI trước
   - PT-BR sau VI
   - không mở đồng loạt 30 locale

## Todo List
- [ ] Chốt KPI theo page cluster
- [ ] Add page-level analytics tags/events nếu thiếu
- [ ] Ship measurement baseline cùng rollout đầu
- [ ] Viết post-deploy indexing checklist
- [ ] Review Search Console after 2 weeks
- [ ] Set expansion gate cho VI/PT-BR
- [ ] Viết multilingual readiness checklist
- [ ] Lập content freshness schedule (refresh mỗi 3-6 tháng) — Google 2025-2026 ưu tiên content mới (<10 tháng tuổi)
- [ ] Set up competitor ranking tracking (y2mate, ssyoutube, savefrom) cho cùng keyword clusters

## Success Criteria
- Có dashboard/tập số đủ để quyết định giữ, sửa, hay scale cluster nào.
- Không mở rộng locale dựa trên cảm giác.
- Team biết rõ page nào kéo traffic thật, page nào kéo user thật.
- Quyết định mở VI/PT-BR dựa trên cả traction lẫn technical readiness, không chỉ vì đã có message catalogs.

## Risk Assessment
- Impression tăng nhưng conversion thấp nếu copy quá SEO-first.
- Đọc sai tín hiệu trong 1-2 tuần đầu vì index chưa ổn định.

## Security Considerations
- Không log dữ liệu nhạy cảm vào analytics.
- Event naming không được để lộ URL người dùng paste.

## Next Steps
- Nếu EN chứng minh hiệu quả: clone strategy sang VI và PT-BR với copy riêng, không machine-translated raw.
