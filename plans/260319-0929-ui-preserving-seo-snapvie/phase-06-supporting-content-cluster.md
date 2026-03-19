# Phase 06 - Supporting Content Cluster

## Context Links
- [frontend/src/routes/+page.svelte](../../frontend/src/routes/+page.svelte)
- [plans/260319-0929-ui-preserving-seo-snapvie/phase-03-english-long-tail-landing-pages.md](./phase-03-english-long-tail-landing-pages.md)
- Search Console query data after initial rollout

## Overview
- Priority: P1
- Status: complete
- Brief: Tạo lớp supporting content rất sát pain points thật để tăng topical authority mà không biến site thành blog rác.

## Key Insights
- Trong ngách tool tải video, homepage + money pages thôi thường chưa đủ để thắng lâu dài.
- Supporting pages phải trả lời câu hỏi thật quanh chất lượng, playlist, shorts, HDR, mux, quality fallback.
- Content cluster đúng sẽ kéo long-tail, tăng internal links, và giải thích tốt các vấn đề user đang gặp.

## Requirements
- Functional:
  - Tạo 6-10 supporting pages EN nhắm pain-point / informational intent thật.
  - Mỗi page phải link logic về homepage hoặc landing page liên quan.
  - Mỗi page phải có title/H1/meta/FAQ theo intent riêng.
- Non-functional:
  - Không làm blog chung chung.
  - Không viết content mỏng chỉ để có thêm URL.
  - Nội dung phải hỗ trợ conversion hoặc trust, không chỉ tăng page count.

## Recommended Cluster

### Pain-point / informational pages:
- `/why-youtube-downloads-show-360p-only`
- `/how-to-download-youtube-playlists`
- `/download-youtube-video-only`
- `/download-youtube-hdr-video`
- `/youtube-8k-vs-4k-download-quality`
- `/why-youtube-downloads-need-muxing`
- `/download-youtube-shorts-with-audio`
- `/best-format-for-youtube-downloads-mp4-vs-webm`

### Comparison/alternative pages (NEW — high conversion intent):
- `/snapvie-vs-y2mate`
- `/snapvie-vs-ssyoutube`
- `/snapvie-vs-savefrom`
- (thêm khi có Search Console data cho "alternative" queries)

## Architecture
- Reuse content page shell nhẹ:
  - intro
  - quick answer / summary box
  - main explanation sections
  - FAQ
  - internal links to related money pages
- Có thể tạo shared config cho supporting pages nếu structure lặp nhiều.
- Không làm CMS ở giai đoạn đầu; hand-crafted pages cho cluster đầu là đủ.

## Related Code Files
- Modify:
  - `frontend/src/routes/sitemap.xml/+server.ts`
  - `frontend/src/routes/+page.svelte` (nếu thêm contextual links sang support pages)
- Create:
  - `frontend/src/lib/seo/supporting-content-config.ts`
  - `frontend/src/lib/components/seo-supporting-page-shell.svelte`
  - `frontend/src/routes/<supporting-pages>/+page.svelte`
- Delete:
  - none

## Implementation Steps
1. Lấy query thật từ Search Console + knowledge pain points hiện có.
2. Chọn 6-10 pages đầu, tránh overlap với money pages.
3. Chốt pattern cho từng page:
   - quick answer
   - explanation
   - proof/examples
   - CTA sang downloader page
4. Viết content theo người dùng thật, không SEO-essay.
5. Thêm internal links theo cluster:
   - support -> money page
   - money page -> 1-2 support pages
6. Đưa các page vào sitemap nếu đủ chất lượng và muốn index.

## Todo List
- [ ] Chốt 6-10 supporting pages đầu
- [ ] Chốt content template
- [ ] Viết copy cho từng page
- [ ] Thêm `HowTo` JSON-LD schema cho how-to pages (how-to-download-*, best-format-*) — eligible cho rich snippets
- [ ] Tạo comparison/alternative pages (Snapvie vs y2mate/ssyoutube/savefrom)
- [ ] Thêm internal links theo cluster
- [ ] Tạo `/how-to-use-snapvie` tutorial page (+ HowTo schema candidate)
- [ ] Đưa pages đủ chất lượng vào sitemap

## Success Criteria
- Cluster quanh downloader có topical coverage sâu hơn rõ rệt.
- Supporting pages kéo thêm long-tail impressions/clicks.
- Mỗi page có vai trò rõ, không cannibalize money page.

## Risk Assessment
- Nếu topic chọn sai, page sẽ kéo traffic rác hoặc không có demand.
- Nếu copy quá chung chung, pages thành thin content.

## Security Considerations
- Không nêu claim kỹ thuật sai hoặc hứa capability không defend được.

## Next Steps
- Phase 07 dùng content cluster + money pages làm nền để đi authority/distribution.
