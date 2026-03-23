# Phase 03 - Money Page Domination

## Context Links
- [frontend/src/routes/+page.svelte](../../frontend/src/routes/+page.svelte)
- `frontend/src/routes/download-youtube-*`

## Overview
- Priority: P0
- Status: proposed
- Brief: Củng cố các money pages hiện có để thắng intent mạnh nhất trước khi scale content cluster.

## Key Insights
- Money pages là nơi SEO phải quy đổi thành action.
- Mỗi money page cần khác nhau đủ rõ để tránh cannibalization.
- Không page nào được chỉ là "homepage clone đổi vài câu".

## Requirements
- Functional:
  - mỗi money page có value proposition riêng
  - title/meta riêng
  - schema phù hợp
  - FAQ riêng
  - internal links vào guides/comparison pages
  - strong CTA + tool interaction above the fold
- Non-functional:
  - giữ UX conversion nhanh
  - không biến money page thành essay dài vô ích

## Target Pages
- `/`
- `/download-youtube-4k`
- `/download-youtube-8k-hdr`
- `/download-youtube-playlist`
- `/download-youtube-shorts`
- `/download-youtube-mp3`

## Expansion Candidates
- `/download-youtube-video-only`
- `/download-youtube-hdr-video`
- `/download-youtube-audio-only`
- `/download-youtube-1080p`
- only if query and differentiation justify them

## Related Code Files
- Modify:
  - existing money page routes
  - `frontend/src/components/seo-landing-page-shell.svelte`
  - shared FAQ / related links components

## Implementation Steps
1. Audit each money page against target query.
2. Rewrite copy to sharpen unique search intent.
3. Add links to exact relevant guides and comparisons.
4. Ensure schema + breadcrumb + metadata are unique.
5. Track page-level conversion from organic.
6. Test title/meta variants over time.

## Internal Link Architecture (Added from review)
- Money pages are the top of the link equity pyramid.
- Link flow rules:
  - Guides → link UP to relevant money page (contextual anchor text)
  - Comparison pages → link UP to relevant money page
  - Money pages → link ACROSS to related money pages (sparingly)
  - Money pages → link DOWN to 2-3 most relevant guides
  - Homepage → link to ALL money pages (already done via ExploreMoreSnapvieTools)
- Anchor text rules:
  - Use descriptive, keyword-relevant anchors (not "click here")
  - Vary anchor text — don't repeat exact match across all pages
  - Example: "download YouTube in 4K" from guide, not just "4K page"
- Sidebar / footer:
  - Footer: all money pages + /guides + /compare hubs
  - No sidebar link spam — keep contextual only
- Hub-spoke model:
  - Hub: `/guides` → spokes: individual guide pages
  - Hub: `/compare` → spokes: individual comparison pages
  - Each spoke links back to hub + 2-3 related spokes

## Todo List
- [ ] Audit unique intent of each money page
- [ ] Tighten H1 + meta descriptions
- [ ] Add exact-match FAQs where helpful
- [ ] Add contextual guide links
- [ ] Add comparison links where useful
- [ ] Add page-level conversion tracking
- [ ] Implement internal link architecture rules
- [ ] Add footer navigation with money + hub links
- [ ] Audit anchor text diversity

## Success Criteria
- Each money page ranks for its own cluster and drives tool usage.
- Cannibalization among money pages remains controlled.

## Risk Assessment
- Overlap between mp3 / audio-only / playlist / shorts clusters.
- Too much content below the fold hurting clarity.

## Security Considerations
- Claims must match real product capability.

## Next Steps
- Phase 04 supports these pages with deep informational cluster.
