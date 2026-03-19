# Brainstorm: SEO Optimization cho snapvie.com

**Date:** 2026-03-19 | **Plan:** 260319-0929-ui-preserving-seo-snapvie

---

## Lỗi SEO đang tồn tại (Fix ngay)

| # | Vấn đề | File | Severity |
|---|--------|------|----------|
| 1 | Brand mismatch: title = "FetchTube" nhưng domain = snapvie.com | `messages/en.json:5` | Critical |
| 2 | Meta description không chứa keyword SEO nào | `messages/en.json:45` | High |
| 3 | Thiếu canonical tag toàn site — 30+ locale = duplicate content risk | Toàn site | High |
| 4 | Duplicate hreflang homepage — `hooks.server.ts` inject + `+page.svelte` cũng render | 2 files | High |
| 5 | Cache-control `private, no-store` cho MỌI HTML kể cả homepage prerendered | `hooks.server.ts:9` | High |
| 6 | Thiếu OG/Twitter/JSON-LD hoàn toàn | Toàn site | High |
| 7 | Admin/account/mux-job indexable — robots.txt chỉ `Allow: /` | `robots.txt/+server.ts` | Medium |
| 8 | Hidden locale links `display:none` — Google có thể coi hidden links | `+layout.svelte:333-337` | Medium |

---

## Phân tích theo từng Phase

### Phase 1: Homepage SEO Foundation
**Việc cần làm:**
- Fix brand: "FetchTube" → "Snapvie" trong title, meta, tất cả i18n keys
- Title mới: `Snapvie - Download YouTube Videos in 4K/8K HDR | Free Online Tool`
- Meta description có keyword: mention YouTube, download, 4K, playlist
- Self-referencing canonical cho mọi page/locale
- OG tags: og:title, og:description, og:image, og:url, og:type, og:site_name
- Twitter Card: twitter:card=summary_large_image, twitter:title, twitter:description, twitter:image
- JSON-LD `WebApplication` schema (cẩn thận với `SoftwareApplication` — chỉ dùng nếu tool đúng nghĩa app)
- Cần OG image asset — design 1200x630px

**Lưu ý:**
- H1 hiện tại "Save videos in a snap." — quá generic, cần mention YouTube/quality
- Không overclaim trong schema nếu tool không support đầy đủ

### Phase 2: Homepage Search-Intent Content Blocks
**Việc cần làm:**
- Thêm content blocks utility-first vào homepage (không phá UI)
- FAQ block với câu hỏi thật từ Google auto-suggest / People Also Ask
- Feature cards mention keyword: "4K/8K HDR", "Playlist", "Shorts", "No watermark"
- How-it-works section có keyword trong heading/text

**Lưu ý:**
- Giữ clean, không nhồi copy quá nhiều
- FAQ dùng `<details>/<summary>` HTML native, không cần JS accordion

### Phase 3: EN Long-tail Landing Pages
**4 pages:** `/download-youtube-8k-hdr`, `/download-youtube-playlist`, `/download-youtube-shorts`, `/download-youtube-4k`

**Việc cần làm:**
- Reuse 95% homepage UI (hero + download box + feature highlights)
- MỖI page phải có nội dung riêng biệt — nếu chỉ khác H1/meta thì Google coi doorway pages
- Nội dung riêng: hướng dẫn cụ thể, FAQ riêng, feature comparison table
- Canonical, OG, JSON-LD riêng cho mỗi page
- Prerender = true cho tất cả

**Rủi ro:**
- Thin/duplicate content nếu share quá nhiều nội dung giống nhau
- Mỗi page cần ≥300 words unique content

### Phase 4: Internal Linking, Sitemap, Indexability
**Việc cần làm:**
- Sitemap hiện chỉ list `/` và `/privacy` — cần thêm tất cả landing pages, trust pages
- robots.txt thêm Disallow: `/admin`, `/account`, `/download/mux-job`, `/api/`
- Internal links từ homepage → landing pages (footer hoặc feature cards)
- Internal links giữa landing pages (related downloads section)
- Breadcrumb structured data cho landing pages

**Lưu ý:**
- Xóa hidden locale links `display:none` div trong `+layout.svelte` — chuyển thành visible language switcher hoặc xóa hẳn

### Phase 5: Measurement, Rollout, Expansion Gate
**Việc cần làm:**
- Google Search Console verify (nếu chưa)
- GA4 tracking cho landing pages (đã có `initGA`)
- Conversion tracking: search → homepage → download action
- Baseline metrics: impressions, clicks, CTR, position cho keyword clusters
- Gate criteria trước khi mở i18n: ≥X impressions/tuần từ EN landing pages

### Phase 6: Supporting Content Cluster
**Việc cần làm:**
- Pain-point articles: "Why YouTube gives 360p instead of 1080p", "How video muxing works", "Playlist download limits"
- Utility pages: format comparison table, quality guide
- Keep ≤800 words, utility-first, có CTA về tool

**Bổ sung plan thiếu:**
- **Comparison/alternative pages**: "Snapvie vs y2mate", "Snapvie vs ssyoutube" — keyword cluster conversion cao, plan hiện tại không mention

### Phase 7: Authority, Distribution, Link Acquisition
**Việc cần làm:**
- Submit lên directories/listings sạch (AlternativeTo, Product Hunt, etc.)
- Community seeding có kiểm soát (Reddit, forums)
- White-hat chỉ — không spam, không mua backlink rác

### Phase 8: SERP CTR Optimization Loop
**Việc cần làm:**
- Monitor Search Console query data
- A/B test title tags dựa trên query thật
- Optimize meta descriptions cho CTR (power words, CTA)
- Structured data để nhận rich snippets (FAQ, How-to)

### Phase 9: Technical Renderability, CWV, Crawl Efficiency
**Việc cần làm:**
- Fix cache-control: split public pages (`public, s-maxage=3600`) vs auth pages (`private, no-store`)
- Google Fonts optimization: Material Symbols Outlined + Nunito + Fredoka = 3 font families → render-blocking
  - Self-host hoặc subset
  - Thêm `display=swap`
  - Lazy-load Material Symbols
- Service Worker: exclude HTML pages khỏi SW cache (tránh serve HTML cũ cho Googlebot)
- Audit LCP, CLS, INP cho homepage và landing pages
- `<meta name="robots" content="noindex">` cho admin/account/mux-job pages

### Phase 10: Entity Trust, Legal Surface
**Việc cần làm:**
- Tạo public pages: `/about`, `/contact`, `/terms`, `/dmca`
- `/privacy` đã có — review và update
- Brand consistency: tất cả nơi phải ghi "Snapvie", không "FetchTube"
- Structured data Organization/WebSite cho brand entity

---

## Bổ sung plan chưa có

1. **Comparison/alternative pages** — capture "y2mate alternative" searches → conversion cao
2. **Custom 404 page** với internal links về homepage/landing pages
3. **Breadcrumb structured data** cho landing/trust pages
4. **Font optimization** — 3 font families là performance bottleneck
5. **Redirect audit** — nếu có domain cũ fetchtube cần 301
6. **SW HTML cache exclusion** — tránh serve stale HTML cho crawler
7. **noindex meta tags** cho app pages (admin, account, mux-job)

---

## "8K HDR" Positioning

- Volume thấp (<500/tháng globally) — **positioning signal, không phải traffic play**
- Dùng làm sub-messaging/badge, **không phải primary keyword**
- Primary keywords: "YouTube video downloader", "download YouTube playlist", "YouTube to MP4"
- Điều kiện: tool phải thật sự support 8K HDR

---

## Rủi ro chính

1. **DMCA/Legal**: Google có thể de-rank downloader sites → cần trust surface rõ ràng
2. **Thin landing pages**: share 95% UI chỉ khác H1 = doorway pages risk
3. **Schema overclaim**: `SoftwareApplication` sai info → manual action
4. **i18n thin content**: 30+ locales chỉ có UI translation → không mở landing pages per-locale chưa có content thật
5. **3 font families**: ảnh hưởng CWV, đặc biệt LCP

---

## Câu hỏi mở

1. Domain `fetchtube` trước đây có tồn tại? Nếu có → cần 301 redirect
2. Có OG image asset chưa? Cần design 1200x630px trước khi ship
3. Tool có thật sự support 8K HDR download?
4. Google Search Console đã verify domain chưa?
5. Cloudflare Page Rules/Cache Rules ảnh hưởng HTML caching?
6. localStorage keys vẫn dùng `fetchtube-*` — có muốn rename luôn?
