# Phase 04 - Guides Cluster Buildout

## Context Links
- existing support pages in `frontend/src/routes/*`

## Overview
- Priority: P0
- Status: proposed
- Brief: Xây cụm `/guides` như một help center / knowledge center mạnh, tập trung long-tail informational và pain-point queries.

## Key Insights
- Đây là phần nhiều site tool làm dở: có bài viết nhưng không có cluster.
- Guides phải giải quyết vấn đề thật, không phải chỉ viết cho có URL.
- Guides phải đẩy authority về money pages và giảm friction cho conversion.

## Cluster Buckets

### How-to
- how to use Snapvie
- how to download playlists
- how to download Shorts with audio
- how to save YouTube videos on desktop/mobile

### Troubleshooting
- why only 360p
- why no audio
- why 4K/8K not available
- why download fails
- why muxing is required

### Formats / quality
- mp4 vs webm
- hdr vs sdr
- 8k vs 4k
- video-only vs muxed

### Workflow / user intent
- backup workflows
- offline viewing
- playlist archiving
- extracting audio for study / podcast listening

## Content Standards
- Every guide must include:
  - quick answer box
  - main explanation
  - when/why section
  - action steps
  - CTA to relevant money page
  - related guides
  - FAQ if warranted
- Prefer screenshots / UI references later where useful.

## Featured Snippet Targeting (Added from review)
- Many guide queries are prime featured snippet candidates.
- Format rules per query type:
  - "what is X" → **definition snippet**: short paragraph (40-60 words) right after H1
  - "why X" → **paragraph/list snippet**: direct answer + numbered reasons
  - "how to X" → **ordered list snippet**: numbered steps with clear action verbs
  - "X vs Y" → **table snippet**: comparison table with clear headers
- Implementation:
  - Add `<p class="quick-answer">` or equivalent semantic block at top of each guide
  - Use `<ol>` for how-to steps (not `<ul>`)
  - Use `<table>` for comparisons (not CSS grids or custom layouts)
  - Keep quick answer concise — Google prefers 40-60 word snippet extracts
- Track: GSC "position 0" appearances per guide

## Device-Specific Guide Rules (Added from review)
- Decision: Keep 4 separate device pages (iPhone, Android, Mac, Windows)
- Each page MUST have ≥40% unique content. Unique content sources:
  - **iPhone**: Safari download limitations, Files app save flow, iOS no direct Camera Roll save → workaround, Shortcuts app integration tips
  - **Android**: Chrome download behavior, custom download folder, notification bar progress tracking, file manager access patterns
  - **Mac**: Safari vs Chrome behavior differences, Finder integration, AirDrop sharing workflow
  - **Windows**: Edge vs Chrome differences, Downloads folder management, batch download workflow tips
- Shared content (tool usage steps) should be brief — focus on device-specific UX
- If GSC data shows cannibalization between device pages → consolidate to 2 (mobile + desktop)
- Each page gets device-specific FAQ items (not shared FAQ)

## Related Code Files
- Create / modify:
  - `/guides` routes and components
  - content registry
  - existing support page migrations

## Implementation Steps
1. Migrate current support pages into `/guides`.
2. Launch 10 foundational guides.
3. Add 10 second-wave guides from GSC and user pain points.
4. Create category chips / browse patterns.
5. Build related-content loops.

## Initial 20 Guide Targets
- `how-to-use-snapvie`
- `how-to-download-youtube-playlists`
- `how-to-download-youtube-shorts-with-audio`
- `why-youtube-downloads-show-360p-only`
- `why-youtube-downloads-need-muxing`
- `best-format-for-youtube-downloads-mp4-vs-webm`
- `why-4k-not-available-on-youtube-download`
- `why-8k-hdr-does-not-show-up`
- `youtube-video-only-vs-video-with-audio`
- `how-to-download-youtube-on-iphone`
- `how-to-download-youtube-on-android`
- `how-to-download-youtube-on-mac`
- `how-to-download-youtube-on-windows`
- `how-to-download-youtube-audio-only`
- `how-to-download-long-youtube-playlists`
- `how-to-save-youtube-videos-for-offline-viewing`
- `what-is-hdr-video-download`
- `what-is-muxing-in-video-downloads`
- `why-some-youtube-videos-have-limited-quality`
- `how-to-choose-best-youtube-download-format`

## Todo List
- [ ] Build `/guides` hub
- [ ] Migrate existing guides
- [ ] Publish initial 10 guides
- [ ] Publish second-wave 10 guides
- [ ] Add related guides logic
- [ ] Add guide freshness metadata

## Success Criteria
- Guides cluster starts winning long-tail informational queries.
- Guides consistently route users back into money pages.

## Risk Assessment
- Too much topic overlap among device-specific guides.
- Thin content if each article is not clearly differentiated.

## Security Considerations
- Avoid legal/ToS claims not backed by counsel or documentation.

## Next Steps
- Phase 05 captures alternative-intent searches that often convert well.
