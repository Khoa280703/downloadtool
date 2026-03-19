# Phase 04 - Internal Linking, Sitemap, Indexability

## Context Links
- [frontend/src/routes/sitemap.xml/+server.ts](../../frontend/src/routes/sitemap.xml/+server.ts)
- [frontend/src/routes/robots.txt/+server.ts](../../frontend/src/routes/robots.txt/+server.ts)
- [frontend/src/routes/+layout.svelte](../../frontend/src/routes/+layout.svelte)
- [frontend/src/routes/+page.svelte](../../frontend/src/routes/+page.svelte)

## Overview
- Priority: P1
- Status: complete
- Brief: Đảm bảo crawler tìm thấy, hiểu và ưu tiên đúng homepage + landing pages.

## Key Insights
- Có sitemap/robots rồi nhưng phải mở rộng đúng các page SEO mới.
- Internal links là trụ cột để cluster authority, đặc biệt khi chưa có nhiều backlink.
- Canonical/hreflang sai sẽ làm loãng index signals.
- Codebase đã có multilingual routing + hreflang injection, nên phase này phải tận dụng cái có sẵn thay vì build i18n SEO lại từ đầu.
- Vấn đề hiện tại là consistency: homepage đang duplicate hreflang, một số internal links chưa locale-aware, canonical chưa rõ ràng.
- Phase này chịu trách nhiệm “source of truth + wiring”; phase 09 chịu trách nhiệm “render/output validation + performance”.

## Requirements
- Functional:
  - Homepage link xuống landing pages trọng điểm.
  - Landing pages link qua lại logic.
  - Sitemap bao gồm toàn bộ public SEO pages.
  - Canonical tự-consistent.
  - Internal links chính tôn trọng locale hiện tại.
  - Hreflang không bị duplicate giữa page-level head và server-level injection.
- Non-functional:
  - Không link spam ở header/footer.
  - Không tạo crawl noise bằng pages không cần index.

## Architecture
- Homepage có 1 khu vực link gọn đến 4 SEO pages.
- Footer/header có thể thêm link nhưng ưu tiên contextual links trong main content.
- Sitemap generated route giữ source of truth cho public indexable pages.

## Related Code Files
- Modify:
  - `frontend/src/routes/+page.svelte`
  - `frontend/src/routes/+layout.svelte`
  - `frontend/src/routes/sitemap.xml/+server.ts`
  - `frontend/src/routes/robots.txt/+server.ts`
- Create:
  - `frontend/src/lib/seo/public-pages.ts`
- Delete:
  - none

## Implementation Steps
1. Tạo source of truth cho SEO pages public (`frontend/src/lib/seo/public-pages.ts`).
2. Cập nhật sitemap để include: 4 landing pages + trust pages (about/contact/terms/dmca).
3. **Update robots.txt** — thêm Disallow:
   ```
   Disallow: /admin
   Disallow: /account
   Disallow: /download/mux-job
   Disallow: /api/
   ```
4. **Add `<meta name="robots" content="noindex">` cho admin/account/mux-job pages.**
5. Add contextual links từ homepage sang các landing pages.
6. Add peer links từ mỗi landing page sang 2-3 pages gần intent.
7. Ensure canonical từng page trỏ đúng self URL.
8. Audit hreflang generation:
   - chỉ có 1 nguồn render thực tế
   - không duplicate ở homepage
   - chỉ trỏ tới locale/page thật sự tồn tại
9. Audit locale-aware internal links:
   - footer
   - header
   - CTA links
   - contextual links trong main content
10. Đảm bảo x-default + base locale consistent cho `/` và non-home pages.
11. Đưa public trust pages vào internal link graph ở mức vừa đủ.
12. **Tạo custom 404 page** với internal links về homepage + landing pages (giữ user + truyền link equity).
13. **Thêm breadcrumb structured data** cho landing pages + trust pages (tăng CTR trên SERP).

## Todo List
- [ ] Khai báo public SEO pages source
- [ ] Update sitemap (thêm landing + trust pages)
- [ ] Update robots.txt (Disallow /api/ only — no Disallow for pages with noindex)
- [ ] Add noindex meta cho admin/account/mux-job pages
- [ ] Add homepage internal links
- [ ] Add landing-page peer links
- [ ] Verify canonical/self-referencing
- [ ] Verify robots/indexability
- [ ] Remove hreflang duplication (đã fix ở Phase 01)
- [ ] Verify locale-aware internal links
- [ ] Add trust pages into crawl graph vừa đủ
- [ ] Tạo custom 404 page
- [ ] Thêm breadcrumb structured data
- [ ] Thêm `lastmod` date cho mỗi URL trong sitemap (freshness signal)
- [ ] Thêm image sitemap cho OG images

## Success Criteria
- Crawler có đường đi rõ ràng từ homepage đến toàn bộ landing pages.
- Sitemap chỉ liệt kê page thực sự muốn index.
- Không có canonical conflict.
- Không có hreflang duplicate trên page HTML.
- User và bot ở locale nào cũng đi qua internal links đúng locale đó.

## Risk Assessment
- Hreflang trỏ tới locale/page chưa tồn tại -> tín hiệu kém.
- Nhồi quá nhiều internal links sitewide -> loãng relevance.
- Giữ link hardcoded `/privacy`, `/#home`... sẽ làm locale graph yếu dù UI đã đa ngôn ngữ.

## Security Considerations
- Không có risk đặc biệt.

## Next Steps
- Phase 05 đo hiệu quả thực tế và gate quyết định expansion i18n.
