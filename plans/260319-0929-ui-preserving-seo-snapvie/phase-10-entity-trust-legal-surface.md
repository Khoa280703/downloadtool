# Phase 10 - Entity Trust, Legal Surface, Brand Consistency

## Context Links
- [frontend/src/routes/+page.svelte](../../frontend/src/routes/+page.svelte)
- [frontend/src/routes/privacy/+page.svelte](../../frontend/src/routes/privacy/+page.svelte)
- future public trust pages in `frontend/src/routes/*`

## Overview
- Priority: P1
- Status: partial (trust pages shipped, brand/schema/internal-link consistency still open)
- Brief: Xây lớp trust công khai để Snapvie trông như một product thật, không giống thin downloader page.

## Key Insights
- Trong ngách nhạy cảm như downloader, trust và entity signals ảnh hưởng rất nhiều tới khả năng rank bền.
- Chỉ có homepage + privacy là quá mỏng cho một thương hiệu muốn SEO lớn.
- Google không cần bạn “giới thiệu công ty dài dòng”, nhưng cần thấy brand, purpose, policies, contact points, consistency.

## Requirements
- Functional:
  - Public đầy đủ bộ trust pages công khai.
  - Brand naming, title patterns, logo/OG image, organization schema nhất quán.
  - Footer/header có đường dẫn hợp lý tới các trust pages quan trọng.
- Non-functional:
  - Không biến site thành legal maze.
  - Copy phải ngắn, rõ, đáng tin, không corporate-sounding giả tạo.

## Public Surface
- `/about`
- `/contact`
- `/terms`
- `/privacy`
- `/dmca` hoặc equivalent copyright/contact policy

## Architecture
- Trust pages dùng chung shell/layout hiện tại, cực ít design overhead.
- Metadata + canonical vẫn chuẩn như money pages, nhưng noindex chỉ dùng nếu thật sự cần.
- Organization schema, WebSite schema, logo/OG asset dùng cùng source of truth.

## Related Code Files
- Modify:
  - `frontend/src/routes/+layout.svelte`
  - `frontend/src/routes/privacy/+page.svelte`
  - `frontend/src/components/SiteHeader.svelte`
- Create:
  - `frontend/src/routes/about/+page.svelte`
  - `frontend/src/routes/contact/+page.svelte`
  - `frontend/src/routes/terms/+page.svelte`
  - `frontend/src/routes/dmca/+page.svelte`
  - `frontend/src/lib/seo/site-brand.ts`
- Delete:
  - none

## Implementation Steps
1. Public đầy đủ trust surface đã chốt.
2. Viết copy ngắn gọn cho từng page:
   - product purpose
   - contact path
   - privacy/terms basics
   - copyright/report flow
3. Chuẩn hóa brand consistency:
   - site name
   - logo
   - OG image
   - Organization schema
4. Thêm footer/header links vừa đủ, không noise.
5. Link trust pages từ homepage/footer để crawler và user đều thấy.

## Todo List
- [x] Public about/contact/terms/privacy/dmca
- [x] Viết copy cho about/contact/terms/dmca
- [ ] Chuẩn hóa brand/OG/logo/schema naming
- [x] Verify brand consistency: không còn "FetchTube" ở bất kỳ đâu (đã rename ở Phase 01)
- [x] Verify localStorage keys đã chuyển từ `fetchtube-*` → `snapvie-*` (đã rename ở Phase 01)
- [ ] Thêm internal links tới trust pages
- [x] Review tone để vẫn gọn, không corporate giả

## Success Criteria
- Snapvie có surface công khai đủ đáng tin cho user và bot.
- Brand signals nhất quán giữa page titles, schema, OG, footer/legal pages.
- Site bớt cảm giác “single-page tool site” và giống product thật hơn.

## Risk Assessment
- Copy pháp lý sơ sài hoặc mơ hồ có thể phản tác dụng.
- Thêm quá nhiều legal text làm site nặng nề, giảm độ sạch của UI.

## Security Considerations
- Không công khai email/số điện thoại nếu chưa muốn; có thể dùng contact form hoặc alias phù hợp.
- Terms/privacy/DMCA phải phản ánh đúng vận hành thật, không được viết cho có.

## Next Steps
- Sau khi trust layer ổn, Phase 07 và 08 sẽ tận dụng brand/entity tốt hơn cho distribution và CTR.
