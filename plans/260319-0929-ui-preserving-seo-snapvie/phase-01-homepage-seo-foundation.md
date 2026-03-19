# Phase 01 - Homepage SEO Foundation

## Context Links
- [docs/project-overview-pdr.md](../../docs/project-overview-pdr.md)
- [docs/code-standards.md](../../docs/code-standards.md)
- [docs/system-architecture.md](../../docs/system-architecture.md)
- [frontend/src/routes/+page.svelte](../../frontend/src/routes/+page.svelte)
- [frontend/src/routes/+page.ts](../../frontend/src/routes/+page.ts)

## Overview
- Priority: P1
- Status: complete
- Brief: Siết technical SEO base cho homepage mà không đổi layout chính.

## Key Insights
- Homepage đang có H1 nhưng copy còn thiên branding.
- Chưa có canonical/OG/Twitter/JSON-LD đủ chuẩn.
- Tool site muốn rank phải rõ intent ngay từ title, meta, H1, subtitle.
- OG image không nên để pending quá lâu; nếu chưa có asset chuẩn thì phải tạo ngay ở đợt rollout đầu.

## Requirements
- Functional:
  - Title, meta description, canonical, OG, Twitter tags đầy đủ.
  - Thêm JSON-LD phù hợp cho homepage.
  - Không đổi flow tải, không đổi component tree lớn.
- Non-functional:
  - Không tạo regression hydration/SSR.
  - Metadata phải bám `snapvie.com`.
  - Dễ tái sử dụng cho landing pages.

## Architecture
- Tách metadata constants/schema builders thành module dùng chung nếu số page > 2.
- Homepage render metadata/server-safe.
- JSON-LD chèn bằng `<script type="application/ld+json">` trên page hoặc shared helper.

## Related Code Files
- Modify:
  - `frontend/src/routes/+page.svelte`
  - `frontend/src/routes/+page.ts`
  - `frontend/src/routes/+layout.svelte` (hidden locale links, cache)
  - `frontend/src/hooks.server.ts` (cache-control split, hreflang guard)
  - `frontend/src/components/SiteHeader.svelte` (brand name)
  - `frontend/src/components/FormatPicker.svelte` (localStorage key)
  - `frontend/src/components/CookieConsent.svelte` (localStorage key)
  - `frontend/src/lib/playlist-download-stream-selection.ts` (localStorage keys)
  - `frontend/messages/*.json` (30+ files — FetchTube → Snapvie)
- Create:
  - `frontend/src/lib/seo/site-metadata.ts`
  - `frontend/src/lib/seo/structured-data.ts`
- Delete:
  - none

## Implementation Steps

### Step 0: Fix existing SEO bugs (CRITICAL — do first)
1. **Brand rename FetchTube → Snapvie**: Replace "FetchTube" in all 30+ i18n message files (`messages/*.json`) — keys: `home_meta_title`, `home_meta_description`, `privacy_meta_title`, `privacy_meta_description`, `account_meta_title`, `footer_copyright`, `home_playlist_subtitle`, `download_btn_legal_prefix`, `privacy_section_1_body`
2. **Rename localStorage keys/events** across 9 code files:
   - `fetchtube-theme` → `snapvie-theme` (layout, page, CookieConsent)
   - `fetchtube-theme-change` → `snapvie-theme-change` (layout, page)
   - `fetchtube.preferred-format.v1` → `snapvie.preferred-format.v1` (FormatPicker, playlist-download-stream-selection)
   - `fetchtube.playlist-quality.v1` → `snapvie.playlist-quality.v1`
   - `fetchtube.playlist-download-mode.v1` → `snapvie.playlist-download-mode.v1`
   - `fetchtube.playlist-job-id.v1` → `snapvie.playlist-job-id.v1`
3. **Fix SiteHeader.svelte**: "FetchTube" text → "Snapvie"
4. **Fix duplicate hreflang on homepage**: Remove `HOMEPAGE_HREFLANG_LINKS` from `+page.svelte`; keep only `hooks.server.ts` injection. Fix the prerender guard (`sveltekit-prerender` skip) so hreflang still works for prerendered pages.
5. **Fix cache-control**: Split `hooks.server.ts` — public SEO pages get `public, s-maxage=3600, stale-while-revalidate=86400`; auth/admin/app pages keep `private, no-store`.
6. **Remove hidden locale links**: Delete `display:none` locale links div from `+layout.svelte:333-337`.

### Step 1: Homepage metadata
7. Replace cute title/meta bằng intent-first variants:
   - Title: `Snapvie - Download YouTube Videos in 4K/8K HDR | Free Online Tool`
   - Meta description: mention YouTube, download, playlist, 4K/8K HDR, free, no ads
8. Add self-referencing canonical URL for homepage.
9. Add OG tags: `og:type`, `og:title`, `og:description`, `og:url`, `og:image`, `og:site_name`.
10. Add Twitter tags: `twitter:card`, `twitter:title`, `twitter:description`, `twitter:image`.
11. Add JSON-LD blocks với schema nesting (Organization → WebSite → WebApplication liên kết):
   - `Organization` (root entity, `@id` anchor)
   - `WebSite` (publisher = Organization `@id`, + `SearchAction` if truthful)
   - `WebApplication` (author/publisher = Organization `@id`)
   - Dùng `@id` references để Google hiểu entity relationships
   - Thêm `SameAs` trong Organization (link tới GitHub, social profiles) để tăng entity resolution
12. Do **not** add `AggregateRating` unless có nguồn review thật.
13. Validate output HTML + structured data.

## Todo List
- [ ] Rename FetchTube → Snapvie in all 30+ i18n message files
- [ ] Rename localStorage keys/events fetchtube → snapvie (9 code files)
- [ ] Fix SiteHeader "FetchTube" text
- [ ] Fix duplicate hreflang (remove from +page.svelte, keep hooks.server.ts)
- [ ] Fix cache-control split (public vs private pages)
- [ ] Remove hidden locale links from +layout.svelte
- [ ] Chốt final homepage title/meta copy theo EN market
- [ ] Thêm canonical + OG + Twitter tags
- [ ] Thêm `WebSite` + `Organization` + `WebApplication` schema
- [ ] Cân nhắc `SearchAction` nếu phản ánh đúng homepage flow
- [ ] OG image: dùng `frontend/static/logo.svg` làm base, tạo OG image 1200x630px
- [ ] Validate schema bằng Rich Results Test / Schema validator

## Success Criteria
- Homepage có technical SEO head hoàn chỉnh.
- JSON-LD hợp lệ, không dùng risky rating markup.
- UI không thay đổi đáng kể.

## Risk Assessment
- Rủi ro lớn nhất: overclaim bằng schema không có dữ liệu thật.
- Nếu metadata hardcode trùng lặp trên nhiều page, maintenance cost tăng.

## Security Considerations
- Không nhúng dữ liệu người dùng hoặc runtime-sensitive data vào structured data.
- OG image phải là asset public tĩnh.

## Next Steps
- Phase 02 dùng bộ messaging mới để rewrite copy trên homepage theo EN-first search intent.
