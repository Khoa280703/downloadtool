# Phase 02 - Homepage Search-Intent Content Blocks

## Context Links
- [frontend/src/routes/+page.svelte](../../frontend/src/routes/+page.svelte)
- [frontend/messages/vi.json](../../frontend/messages/vi.json)
- [frontend/messages/en.json](../../frontend/messages/en.json)

## Overview
- Priority: P1
- Status: complete
- Brief: Điều chỉnh homepage copy và thêm content blocks rất gọn để tăng topical relevance mà vẫn giữ UI clean.

## Key Insights
- Hero hiện đẹp nhưng wording chưa match mạnh với query EN như `youtube downloader`, `download youtube playlist`, `download youtube shorts`, `download youtube 4k/8k hdr`.
- Trang đã có how-it-works; không cần thêm section dư thừa.
- Nội dung dưới fold nên là utility content, không phải blog copy.

## Requirements
- Functional:
  - Rewrite H1/subtitle/chips theo intent.
  - Add `Why Snapvie` block.
  - Add `Supported quality & format` block.
  - Add compact FAQ block.
- Non-functional:
  - Không tăng visual clutter.
  - Không tạo thêm card/section màu mè trùng lặp.
  - Nội dung phải support conversion, không chỉ SEO.

## Architecture
- Reuse section styles đang có trong homepage.
- FAQ render inline, không cần accordion framework nếu chưa cần.
- EN copy là source of truth trước; locale files khác chỉ mở sau khi EN chứng minh hiệu quả.

## Related Code Files
- Modify:
  - `frontend/src/routes/+page.svelte`
  - `frontend/messages/vi.json`
  - `frontend/messages/en.json`
- Create:
  - `frontend/src/lib/seo/homepage-copy.ts` (optional nếu copy dài và cần tách)
- Delete:
  - none

## Implementation Steps
1. Rewrite H1 thành intent-first headline:
   - phải chứa `YouTube`, `playlist`, `4K/8K HDR` hoặc phân bổ giữa H1 + subtitle.
2. Rewrite hero subtitle theo problem-solution, bỏ bớt tone cute.
3. Keep existing `How it works` block, chỉ sửa text cho transactional hơn.
4. Add `Why Snapvie` section với 4-6 USP bullets:
   - chất lượng cao
   - playlist
   - shorts
   - video only / audio only / mux when needed
   - no shady ads claim nếu đúng thực tế
5. Add `Supported Qualities` section:
   - 360p → 4320p
   - SDR / HDR
   - combined / video-only / audio-only
6. Add compact FAQ section, ưu tiên câu hỏi có intent thật:
   - có tải 8K HDR không
   - tại sao có video chỉ hiện 360p/1080p
   - playlist tải thế nào
   - shorts hỗ trợ không
7. Do not expect FAQ rich results; use FAQ for content relevance.

## Todo List
- [ ] Chốt messaging homepage mới
- [ ] Rewrite H1/subtitle
- [ ] Điều chỉnh copy how-it-works
- [ ] Thêm block `Why Snapvie`
- [ ] Thêm block `Supported quality & format`
- [ ] Thêm compact FAQ block
- [ ] Giữ spacing/layout clean sau khi thêm content
- [ ] Giữ EN copy làm bản chuẩn, chưa scale long-form sang các locale khác

## Success Criteria
- Homepage rõ intent hơn khi đọc lướt 5 giây đầu.
- User vẫn nhận ra đây là cùng một giao diện hiện tại.
- Content mới tăng semantic coverage mà không làm trang nặng nề.

## Risk Assessment
- Quá nhiều text dưới fold làm mất cảm giác tool-first.
- Claim chất lượng quá mạnh nhưng không có section chứng minh -> phản tác dụng.

## Security Considerations
- Không có risk đặc biệt; chỉ tránh claim sai về capability/legal.

## Next Steps
- Phase 03 tách các intent sâu thành landing pages EN riêng để tránh nhồi homepage.
