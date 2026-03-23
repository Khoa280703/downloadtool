# Phase 05 - Comparison and Alternatives Cluster

## Context Links
- competitor observation outside repo

## Overview
- Priority: P1
- Status: proposed
- Brief: Đây là cluster có ý định cao nhưng hay bị bỏ qua. Người tìm `y2mate alternative` hoặc `snapvie vs ...` thường sắp chuyển đổi.

## Key Insights
- Comparison pages convert mạnh nếu làm honest và specific.
- Những page này cần cực kỳ cẩn thận để không thành doorway spam.
- Chỉ làm cho đối thủ có demand thực sự.

## Target Pages
- `/compare/snapvie-vs-y2mate`
- `/compare/snapvie-vs-ssyoutube`
- `/compare/snapvie-vs-savefrom`
- `/compare/best-youtube-downloader-for-4k`
- `/compare/best-youtube-downloader-for-playlists`
- `/compare/best-youtube-downloader-for-shorts`

## Content Standards
- honest comparison
- feature matrix
- speed / quality / safety / ads / audio / playlist / 4k/8k support
- no fake claims
- no fabricated benchmarks
- clear "best for" segmentation

## Related Code Files
- create compare routes / templates / registry entries

## Implementation Steps
1. Validate which competitors have real query demand.
2. Create comparison template.
3. Publish 3 core competitor pages.
4. Publish 3 intent-based "best for X" pages.
5. Link comparisons from money pages and guides where appropriate.

## Comparison Freshness Strategy (Added from review)
- Competitors change features, shut down, rebrand frequently in this niche.
- Update triggers:
  - Competitor adds/removes major feature → update comparison table within 2 weeks
  - Competitor shuts down or rebrands → update page + add "discontinued" note
  - Quarterly scheduled review of all comparison data accuracy
  - GSC shows declining CTR on comparison page → refresh content
- Monitoring:
  - Monthly manual check of top 5 competitors' landing pages
  - Set Google Alerts for competitor brand names
  - Track competitor Wayback Machine snapshots quarterly
- Freshness signals:
  - Show "Last verified: [date]" on each comparison page
  - Update `dateModified` in Article schema on each refresh
  - Add "Update log" section at bottom of each comparison page

## Todo List
- [ ] Validate comparison keywords
- [ ] Build comparison template
- [ ] Publish first 3 competitor pages
- [ ] Publish first 3 best-for pages
- [ ] Add internal links from money/guides
- [ ] Set up competitor monitoring alerts
- [ ] Create quarterly comparison review ritual
- [ ] Add "Last verified" display on comparison pages

## Success Criteria
- Snapvie appears for alternative and comparison intent queries.
- Comparison pages assist trust and conversion instead of sounding like SEO bait.

## Risk Assessment
- Overpromising against competitors.
- Legal or reputational issues if comparisons are misleading.

## Security Considerations
- Only compare verifiable product features.

## Next Steps
- Phase 06 ensures the whole growing site remains technically clean.
