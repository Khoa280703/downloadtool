# Phase 08 - SERP CTR Optimization Loop

## Context Links
- Search Console data outside repo
- Homepage + landing pages + supporting pages from earlier phases

## Overview
- Priority: P1
- Status: complete (framework ready, testing post-deploy)
- Brief: Tối ưu liên tục title/meta/snippet/proof blocks để tăng CTR mà không làm clickbait.

## Key Insights
- Ở SERP cạnh tranh, lên hạng chưa đủ; CTR quyết định lượng traffic thật.
- `8K HDR`, `playlist`, `shorts`, `video only`, `no shady ads`, `quality fallback explained` là các trust hooks đáng test.
- Snippet optimization phải dựa trên query thật, không dựa trên cảm giác.

## Requirements
- Functional:
  - Có framework test title/meta theo cluster page.
  - Có OG image strategy đủ tốt cho social sharing.
  - Có proof blocks / trust snippets trong page để hỗ trợ title promise.
- Non-functional:
  - Không clickbait sai kỳ vọng.
  - Không thay title quá thường xuyên gây nhiễu.

## Optimization Surface
- Homepage
- 4 EN landing pages
- Supporting content pages có impression cao

## Candidate Tests
- Include vs exclude `8K HDR`
- Include vs exclude `Playlist`
- `Free` vs `High Quality`
- `YouTube Downloader` vs `Download YouTube Videos`
- `4K/8K HDR` order
- `with audio` vs `video only / audio only`

## Implementation Steps
1. Pull Search Console query/page report.
2. Chọn pages có impression đủ lớn.
3. Viết 2-3 title/meta variants per cluster.
4. Ship từng batch nhỏ, chờ đủ data.
5. So sánh:
   - CTR
   - position
   - conversion into extract/download
6. Chốt winners và cập nhật style guide snippet.

## Todo List
- [ ] Xây title/meta testing sheet
- [ ] Chốt trust hooks per cluster
- [ ] Tạo OG image spec
- [ ] Review snippet CTR every 2 weeks
- [ ] Chốt snippet style guide sau 1-2 vòng test
- [ ] Favicon optimization cho SERP mobile (hiển thị favicon trên kết quả mobile)
- [ ] GEO (Generative Engine Optimization): optimize content cho AI Overviews / ChatGPT / Perplexity
  - Structure content dạng "quick answer + explanation" để AI dễ trích dẫn
  - Đảm bảo entity signals rõ ràng (Organization schema, brand mentions nhất quán)
  - Tạo concise, factual content blocks dễ citeable
  - Monitor AI Overview appearances cho target keywords

## Success Criteria
- CTR tăng có ý nghĩa trên homepage và top landing pages.
- Search snippet phản ánh đúng USP mạnh nhất của Snapvie.
- Không giảm conversion sau click.

## Risk Assessment
- CTR tăng nhưng conversion giảm nếu title quá hứa hẹn.
- Thay đổi đồng loạt nhiều page cùng lúc làm khó đọc dữ liệu.

## Security Considerations
- Không có risk đặc biệt.

## Next Steps
- Sau khi EN cluster ổn định, áp dụng framework tương tự cho VI rồi PT-BR.
