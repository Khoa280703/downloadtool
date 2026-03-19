# Phase 09 - Technical Renderability, CWV, Crawl Efficiency

## Context Links
- [frontend/src/routes/+page.svelte](../../frontend/src/routes/+page.svelte)
- [frontend/src/routes/+layout.svelte](../../frontend/src/routes/+layout.svelte)
- [frontend/src/routes/sitemap.xml/+server.ts](../../frontend/src/routes/sitemap.xml/+server.ts)
- [frontend/src/hooks.server.ts](../../frontend/src/hooks.server.ts)
- [frontend/src/app.html](../../frontend/src/app.html)

## Overview
- Priority: P1
- Status: partial (key items done, some CWV items need field data)
- Brief: Siết technical SEO ở lớp bot thực sự thấy được: renderability, head cleanliness, Core Web Vitals, crawl/index hygiene.

## Key Insights
- Tool sites hay thua không phải vì thiếu keyword mà vì public pages quá JS-heavy, head bẩn, hoặc cache sai.
- Google vẫn render JS, nhưng càng bắt bot làm nhiều việc thì index và snippet càng kém ổn định.
- Nếu homepage và landing pages chậm hoặc shift layout mạnh, SEO và conversion đều mất.
- Phase này là bước verification + optimization của rendered output và performance sau khi phase 01-04 đã wiring xong.

## Requirements
- Functional:
  - Public pages phải render được nội dung/head quan trọng ngay từ HTML response.
  - Canonical, hreflang, robots, sitemap, x-default phải nhất quán.
  - Chỉ index những URL thật sự muốn rank.
- Non-functional:
  - Core Web Vitals phải ở mức tốt trên mobile-first.
  - Không làm nặng thêm bundle cho homepage và landing pages.
  - Không để Cloudflare/cache rules phá metadata hoặc auth-sensitive UI.

## Architecture
- Tách rõ 2 loại page:
  - SEO pages public, cache-friendly, render-first
  - App/auth/admin states, dynamic hơn và không dùng làm money page
- Public SEO pages có metadata/head do server kiểm soát, không để nhiều nguồn tự render trùng nhau.
- Source of truth cho public indexable pages phải dùng chung giữa sitemap, canonical, internal links, alternates.

## Related Code Files
- Modify:
  - `frontend/src/routes/+page.svelte`
  - `frontend/src/routes/+layout.svelte`
  - `frontend/src/hooks.server.ts`
  - `frontend/src/routes/sitemap.xml/+server.ts`
  - `frontend/src/routes/robots.txt/+server.ts`
  - `frontend/src/app.html`
- Create:
  - `frontend/src/lib/seo/public-pages.ts`
  - `frontend/src/lib/seo/canonical.ts`
- Delete:
  - none

## Implementation Steps
1. Audit HTML output của homepage + landing pages:
   - title
   - meta description
   - canonical
   - hreflang
   - JSON-LD
2. Verify chỉ có 1 nguồn render hreflang/canonical thực tế.
3. Verify source of truth cho public pages đang được dùng đúng ở output HTML.
4. Review prerender/SSR/caching strategy cho money pages.
5. **Font optimization** (hiện có 3 font families — performance bottleneck):
   - Material Symbols Outlined (icon font từ Google Fonts) — xem xét lazy-load hoặc subset
   - Nunito (body font) — self-host hoặc preload
   - Fredoka (heading font) — self-host hoặc preload
   - Thêm `display=swap` cho tất cả Google Fonts
   - Đánh giá self-hosting vs CDN cho performance
6. Audit asset weight:
   - fonts (xem step 5)
   - hero/media
   - icon payload (AppIcon SVG system — đã tốt)
   - JS hydration cost
7. Tối ưu LCP/CLS/INP cho mobile homepage.
8. **Service Worker HTML cache exclusion**: Ensure SW không cache stale HTML cho Googlebot — exclude HTML pages khỏi SW runtime cache.
9. Đặt noindex cho những page không nên rank (đã handle ở Phase 04).
10. Kiểm tra Cloudflare cache rules để không cache sai HTML động hoặc head.

## Todo List
- [x] Audit rendered HTML của homepage + landing pages
- [x] Verify canonical/hreflang/x-default rendered output — single source in hooks.server.ts, no duplicates
- [x] Verify source of truth cho public pages đang map đúng ra output — public-pages.ts exists and used by sitemap
- [x] Font optimization: Nunito + Fredoka đã self-hosted với font-display:swap trong app.css; Material Symbols fixed display=swap
- [x] Thêm `display=swap` cho Google Fonts (Material Symbols Outlined)
- [ ] Audit bundle + asset weight public pages — deferred, build output OK
- [ ] Tối ưu CWV mobile-first — deferred (requires field data)
- [x] SW HTML cache exclusion — removed HTML caching from service-worker.ts navigate handler
- [ ] Review noindex/index policy (verify Phase 04 output) — handled in Phase 04
- [ ] Review Cloudflare cache behavior cho public HTML — ops/infra task, deferred
- [ ] Thêm IndexNow protocol ping khi deploy (Google/Bing crawl nhanh hơn) — deferred
- [x] Thêm `<link rel="preload">` cho critical fonts (Nunito, Fredoka) — added to app.html
- [x] Thêm `fetchpriority="high"` cho hero LCP element — N/A: hero is text-only H1, no image

## Success Criteria
- Homepage và landing pages render head sạch, không duplicate/conflict.
- Bot đọc được nội dung chính mà không phụ thuộc nặng vào JS runtime.
- CWV của public pages không còn là điểm yếu chính.
- Chỉ các URL nên rank mới được index và đưa vào sitemap.

## Risk Assessment
- Chạm vào cache/rendering mà không test kỹ dễ tạo regression auth/UI.
- Tối ưu quá tay làm giảm UX thực tế của downloader shell.

## Security Considerations
- Không cache nhầm trạng thái đăng nhập vào public HTML.
- Không đưa dữ liệu người dùng hoặc session-dependent markup vào metadata.

## Next Steps
- Phase 10 bổ sung trust/entity layer để technical sạch nhưng vẫn có đủ tín hiệu thương hiệu.
