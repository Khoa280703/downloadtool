# Phase 03 - English Long-Tail Landing Pages

## Context Links
- [frontend/src/routes/+page.svelte](../../frontend/src/routes/+page.svelte)
- [frontend/src/routes/sitemap.xml/+server.ts](../../frontend/src/routes/sitemap.xml/+server.ts)
- [frontend/messages/en.json](../../frontend/messages/en.json)

## Overview
- Priority: P1
- Status: complete
- Brief: Tạo 4 landing pages EN nhắm long-tail intent, tái sử dụng gần như toàn bộ UI homepage.

## Key Insights
- Head term khó, landing pages là nơi đánh cụm intent sâu.
- 8K HDR là moat nhưng không đủ volume để làm chiến lược duy nhất.
- 4 page đầu đủ để test cluster trước khi scale.

## Requirements
- Functional:
  - Create 5 EN SEO pages:
    - `/download-youtube-8k-hdr`
    - `/download-youtube-playlist`
    - `/download-youtube-shorts`
    - `/download-youtube-4k`
    - `/download-youtube-mp3` (NEW — "download youtube mp3" là query volume rất cao, tool đã support audio-only)
- Mỗi page có metadata riêng, H1 riêng, intro riêng, FAQ riêng.
- Mỗi page có schema phù hợp + `BreadcrumbList` JSON-LD (tăng CTR trên SERP).
  - Reuse same downloader UI/shell.
- Mỗi page cần ≥300 words unique content để tránh thin/doorway pages risk.
- Non-functional:
  - Tránh duplicate/thin content.
  - Shared implementation phải simple, không over-abstract.

## Architecture
- Tạo shared landing-page content config:
  - slug
  - title
  - meta description
  - H1
  - hero subtitle
  - usp bullets
  - faq items
  - internal links
- Reuse homepage shell qua shared component hoặc page template tối giản.
- Chỉ build 5 pages cụ thể, không dynamic catch-all.

## Related Code Files
- Modify:
  - `frontend/src/routes/+page.svelte`
  - `frontend/src/routes/sitemap.xml/+server.ts`
- Create:
  - `frontend/src/lib/seo/landing-page-config.ts`
  - `frontend/src/lib/components/seo-landing-page-shell.svelte`
  - `frontend/src/routes/download-youtube-8k-hdr/+page.svelte`
  - `frontend/src/routes/download-youtube-playlist/+page.svelte`
  - `frontend/src/routes/download-youtube-shorts/+page.svelte`
  - `frontend/src/routes/download-youtube-4k/+page.svelte`
  - `frontend/src/routes/download-youtube-mp3/+page.svelte`
- Delete:
  - none

## Implementation Steps
1. Chốt keyword-to-page mapping:
   - 8K HDR = moat
   - playlist = feature strength
   - shorts = mass demand
   - 4K = bridge keyword giữa mass + premium quality
2. Tạo shared page content config thay vì copy-paste toàn trang.
3. Tạo landing page shell tái sử dụng hero/input/download UI.
4. Mỗi page thêm 2-3 blocks riêng:
   - intro theo intent
   - use-case/benefits
   - FAQ theo intent
   - proof/trust micro-copy theo intent
5. Thêm cross-links giữa các landing pages.
6. Thêm CTA quay về homepage tool nếu user muốn tải ngay.

## Todo List
- [x] Chốt slugs EN
- [x] Tạo shared landing config
- [x] Tạo shell/component tái sử dụng
- [x] Tạo page 8K HDR
- [x] Tạo page playlist
- [x] Tạo page shorts
- [x] Tạo page 4K
- [x] Tạo page MP3
- [x] Thêm schema/breadcrumb strategy cho landing pages
- [x] Review duplicate content risk

## Success Criteria
- 5 landing pages EN indexable, unique enough, giữ style đồng nhất.
- Mỗi page match một intent cụ thể, không cannibalize homepage quá mạnh.
- Shared code vừa đủ, không biến SEO pages thành framework mới.

## Risk Assessment
- Dùng chung quá nhiều content => duplicate pages.
- Abstraction quá sớm => chậm implementation, khó maintain.

## Security Considerations
- Không có risk đặc biệt; chỉ tránh expose runtime-only data trong page template.

## Next Steps
- Phase 04 kết nối các page bằng internal links và sitemap/canonical strategy.
- Nếu template EN thắng, clone chiến lược sang VI rồi PT-BR với copy riêng.
