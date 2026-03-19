---
title: "UI-Preserving SEO for Snapvie"
description: "Tối ưu SEO tối đa cho snapvie.com theo hướng giữ nguyên UI chính, nhưng làm đủ 4 lớp: money pages, technical indexability/renderability/CWV, supporting content cluster, authority + entity trust + CTR optimization."
status: in_progress
priority: P1
effort: 2-4 tuần
branch: main
tags: [seo, frontend, content, metadata, schema, landing-pages, snapvie]
created: 2026-03-19
---

# UI-Preserving SEO for Snapvie

## Problem Statement
- `snapvie.com` đã có UX tải video tốt nhưng homepage hiện nghiêng về branding/cute copy hơn là search intent.
- Trang đã có H1, sitemap, robots, i18n, nhưng thiếu metadata/social tags/schema đủ mạnh và chưa có long-tail landing pages.
- User muốn đánh mạnh định vị `tải video YouTube chất lượng cao nhất`, nhấn `4K/8K HDR`, playlist, Shorts.
- User không muốn phá giao diện hiện tại hoặc biến homepage thành blog dài.

## Business Reality
- Head term như `tải video youtube` cạnh tranh rất nặng. Không nên đặt cược mọi thứ vào 1 homepage.
- `8K HDR` là moat/positioning tốt nhưng volume thấp hơn `playlist`, `shorts`, `4K`.
- FAQ rich results không phải trụ cột. Review stars/self-serving rating không nên kỳ vọng.
- EN nên là thị trường ưu tiên; i18n toàn cầu chỉ nên mở rộng sau khi template nội dung chứng minh hiệu quả.
- Codebase đã có multilingual UI foundation khá tốt: Paraglide URL strategy, locale routes, hreflang injection, sitemap alternates.
- Tuy vậy multilingual SEO chưa hoàn chỉnh: chưa có canonical chuẩn, còn hreflang duplicate ở homepage, còn internal links chưa locale-aware ở một số chỗ, và chưa có SEO landing pages per-locale.
- Trong ngách downloader, chỉ có metadata + vài landing pages vẫn chưa đủ. Google sẽ nhìn thêm renderability, Core Web Vitals, index hygiene, trust surface, brand/entity signals và external mentions.

## Strategy Summary
- Giữ homepage shell hiện tại, chỉ chỉnh copy + metadata + schema + thêm các content block gọn, utility-first.
- Tạo 5 landing pages long-tail bằng cách tái sử dụng 95% homepage UI.
- Dùng `8K HDR` làm positioning layer, không dùng làm keyword duy nhất.
- Dùng internal linking + sitemap + Search Console để đẩy index/cluster authority.
- Sau lớp on-page nền tảng, thêm supporting content cluster để tăng topical authority.
- Bổ sung authority/distribution layer và vòng tối ưu CTR, vì ngách downloader quá cạnh tranh để chỉ thắng bằng on-page.
- Siết technical SEO thật sự: renderability SSR, canonical/hreflang sạch, crawl/index control, CWV, JS payload, cache policy cho public pages.
- Bổ sung entity/trust surface công khai: about/contact/terms/dmca/brand consistency để site nhìn giống product thật, không giống thin tool page.

## SEO Pillars
- Pillar 1: Money pages thắng intent cao
  - Homepage + 5 EN landing pages + các page support có CTA rõ
- Pillar 2: Technical SEO sạch và nhanh
  - Canonical, hreflang, sitemap, renderability, Core Web Vitals, crawl/index hygiene
- Pillar 3: Topical authority
  - Supporting content trả lời pain points thật quanh playlist, 360p fallback, muxing, HDR, format
- Pillar 4: Trust + authority + CTR
  - Entity/legal pages, brand consistency, distribution, referring domains, title/meta testing

## Information Architecture
- Primary page: `/`
- Supporting EN landing pages:
  - `/download-youtube-8k-hdr`
  - `/download-youtube-playlist`
  - `/download-youtube-shorts`
  - `/download-youtube-4k`
  - `/download-youtube-mp3` (NEW — high volume query)
- Comparison/alternative pages (NEW):
  - `/snapvie-vs-y2mate`
  - `/snapvie-vs-ssyoutube`
  - (thêm khi có data từ Search Console)
- Public trust pages:
  - `/about`
  - `/contact`
  - `/terms`
  - `/privacy`
  - `/dmca`
- Deferred pages:
  - `/tai-video-youtube-8k-hdr`
  - `/tai-playlist-youtube`
  - `/tai-youtube-shorts`
  - `/tai-video-youtube-4k`

## Multilingual Readiness Snapshot
- Ready now:
  - Locale-aware routing qua Paraglide
  - `<html lang/dir>` theo locale
  - Hreflang injection ở server layer
  - Multilingual sitemap alternates
  - Message catalogs cho 30+ locale
- Not ready yet:
  - Self-referencing canonical cho từng public page
  - Xóa hreflang duplication ở homepage
  - Locale-aware internal links toàn site
  - OG/Twitter/schema theo locale
  - Non-thin landing pages cho VI/PT-BR
- Decision:
  - Ship EN SEO first
  - Sau khi template thắng ở EN, mở VI rồi PT-BR bằng content thật, không chỉ dịch UI

## Phases
| # | Phase | File | Status |
|---|-------|------|--------|
| 1 | Homepage SEO foundation | [phase-01-homepage-seo-foundation.md](./phase-01-homepage-seo-foundation.md) | complete |
| 2 | Homepage search-intent content blocks | [phase-02-homepage-search-intent-content-blocks.md](./phase-02-homepage-search-intent-content-blocks.md) | complete |
| 3 | EN long-tail landing pages | [phase-03-english-long-tail-landing-pages.md](./phase-03-english-long-tail-landing-pages.md) | complete |
| 4 | Internal linking, sitemap, indexability | [phase-04-internal-linking-sitemap-indexability.md](./phase-04-internal-linking-sitemap-indexability.md) | complete |
| 5 | Measurement, rollout, expansion gate | [phase-05-measurement-rollout-expansion-gate.md](./phase-05-measurement-rollout-expansion-gate.md) | partial |
| 6 | Supporting content cluster | [phase-06-supporting-content-cluster.md](./phase-06-supporting-content-cluster.md) | partial |
| 7 | Authority, distribution, link acquisition | [phase-07-authority-distribution-link-acquisition.md](./phase-07-authority-distribution-link-acquisition.md) | partial |
| 8 | SERP CTR optimization loop | [phase-08-serp-ctr-optimization-loop.md](./phase-08-serp-ctr-optimization-loop.md) | partial |
| 9 | Technical renderability, CWV, crawl efficiency | [phase-09-technical-renderability-cwv-crawl-efficiency.md](./phase-09-technical-renderability-cwv-crawl-efficiency.md) | partial |
| 10 | Entity trust, legal surface, brand consistency | [phase-10-entity-trust-legal-surface.md](./phase-10-entity-trust-legal-surface.md) | complete |

## Non-goals
- Không redesign homepage.
- Không thêm blog engine/CMS ở đợt đầu.
- Không làm 20+ locale landing pages ngay.
- Không tạo programmatic pages theo từng video/query.
- Không thêm fake testimonials, fake ratings, fake review schema.

## Success Criteria
- Homepage có metadata/search copy rõ intent hơn nhưng UI gần như giữ nguyên.
- 5 landing pages EN live, indexable, canonical đúng, dùng lại phần lớn UI.
- Homepage + landing pages có JSON-LD hợp lệ và social metadata đầy đủ.
- Sitemap liệt kê toàn bộ public SEO pages quan trọng.
- Có supporting content pages đủ để phủ pain-point cluster chính quanh downloader / playlist / shorts / 4K / 8K HDR / mux / quality fallback.
- Có authority plan và distribution checklist thực thi được, không chỉ nói chung.
- Search Console bắt đầu ghi nhận impression/click từ keyword clusters: playlist, shorts, 4K, 8K HDR.
- CTR trên homepage và landing pages được tối ưu lặp lại dựa trên query thật từ Search Console.
- Audit multilingual technical SEO pass cho các page public trọng điểm trước khi scale locale.
- Public pages indexable có render HTML/head sạch, không lệ thuộc JS mới đọc được nội dung chính.
- Core Web Vitals và asset strategy của public pages không kéo site vào vùng “tool nặng + chậm”.
- Có trust surface công khai đủ mạnh để site trông như product thật: about/contact/terms/privacy/dmca rõ ràng và đồng bộ brand.
- Không tăng đáng kể complexity frontend hoặc phá conversion flow hiện tại.

## Brainstorm Findings (2026-03-19)
Từ phân tích code thực tế, phát hiện các lỗi SEO cần fix xuyên suốt phases:

**Bugs hiện tại (tích hợp vào Phase 01):**
- Brand mismatch: title/meta/i18n còn ghi "FetchTube" → phải rename hết sang "Snapvie"
- localStorage keys/events dùng `fetchtube-*` → rename sang `snapvie-*` (39 files)
- Duplicate hreflang ở homepage: `hooks.server.ts` inject + `+page.svelte` cũng render
- Cache-control `private, no-store` áp dụng cho MỌI HTML, kể cả homepage prerendered
- Thiếu canonical tag toàn site
- Thiếu OG/Twitter tags
- Thiếu JSON-LD
- Admin/account/mux-job pages đang indexable (cần noindex + robots Disallow)
- Hidden locale links `display:none` trong `+layout.svelte` — anti-pattern

**Bổ sung plan thiếu (tích hợp vào phase tương ứng):**
- Comparison/alternative pages ("Snapvie vs y2mate") → Phase 06
- Custom 404 page với internal links → Phase 04
- Breadcrumb structured data → Phase 03
- Font optimization: 3 families (Material Symbols + Nunito + Fredoka) = bottleneck → Phase 09
- SW HTML cache exclusion → Phase 09
- robots.txt Disallow cho /admin, /account, /download/mux-job, /api/ → Phase 04

**Bổ sung từ SEO plan review (2026-03-19):**

Must-do (đã tích hợp):
- Landing page `/download-youtube-mp3` — query volume rất cao, tool đã support audio-only → Phase 03
- GEO (Generative Engine Optimization) — optimize cho AI Overviews, ChatGPT, Perplexity → Phase 08
- `HowTo` schema cho supporting pages (how-to-download-*, best-format-*) → Phase 06
- Content freshness strategy — refresh content mỗi 3-6 tháng → Phase 05
- Schema nesting: Organization → WebSite → WebApplication liên kết → Phase 01

Should-do (đã tích hợp):
- IndexNow protocol ping khi deploy → Phase 09
- Competitor ranking tracking (y2mate, ssyoutube, savefrom) → Phase 05
- `/how-to-use-snapvie` tutorial page + HowTo schema → Phase 06
- Image sitemap cho OG images → Phase 04
- `SameAs` trong Organization schema (link tới GitHub, social profiles) → Phase 01
- Preload strategy cho critical fonts (`<link rel="preload">`) → Phase 09
- `fetchpriority="high"` cho hero LCP element → Phase 09
- `lastmod` trong sitemap URLs → Phase 04
- Favicon optimization cho SERP mobile → Phase 08

**Confirmed:**
- Tool thật sự support 8K HDR ✓
- Google Search Console đã verify ✓
- Không có domain cũ fetchtube cần redirect
- Logo SVG đã có tại `public/logo.svg` — dùng làm base cho OG image

## Key Risks
- Nhồi copy quá nhiều làm trang mất chất clean.
- Landing pages quá giống nhau -> thin/duplicate content.
- Overusing FAQ/schema tạo kỳ vọng sai về rich results.
- Mở i18n landing pages quá sớm -> translated thin pages.
- Locale pages có hreflang/canonical/internal links lệch nhau -> loãng index signals.
- Làm supporting content kiểu blog chung chung -> loãng topical authority thay vì tăng.
- Chỉ tập trung on-page mà bỏ authority/distribution -> rank tăng chậm hoặc không giữ được.
- Tối ưu CTR quá đà bằng clickbait title -> giảm conversion/trust.
- Quên đo conversion SEO -> không biết keyword nào ra user thật.
- Public pages render bằng JS nặng hoặc cache/head sai -> bot đọc yếu, crawl kém, snippet sai.
- Thiếu trust/legal surface -> site trông như thin affiliate/download tool, khó rank bền.

## Rollout Order
1. Homepage metadata/canonical/OG/Twitter/JSON-LD
2. Homepage copy rewrite nhẹ + content blocks gọn
3. 5 landing pages EN
4. Internal links + sitemap update + multilingual consistency cleanup
5. Ship measurement baseline cùng money pages và indexing checklist
6. Technical renderability/CWV/crawl efficiency pass
7. Entity/legal/trust surface pass
8. Supporting content cluster
9. Authority/distribution execution
10. SERP CTR optimization loop, rồi review 3-4 tuần trước khi quyết định VI/PT-BR

## Related Code
- `frontend/src/routes/+page.svelte`
- `frontend/src/routes/+page.ts`
- `frontend/src/routes/+layout.svelte`
- `frontend/src/app.html`
- `frontend/src/routes/sitemap.xml/+server.ts`
- `frontend/src/routes/robots.txt/+server.ts`
- `frontend/src/routes/privacy/+page.svelte`
- `frontend/src/components/SiteHeader.svelte`
- `frontend/messages/vi.json`
- `frontend/messages/en.json`
- future content/supporting pages created in `frontend/src/routes/*`

## Decisions Locked
- Public trust surface sẽ được triển khai luôn: `/about`, `/contact`, `/terms`, `/privacy`, `/dmca`.
- SEO execution vẫn giữ EN-first; VI và PT-BR chỉ mở sau khi EN template có traction thật.
- Phase rollout đầu sẽ dùng EN-only slugs (`/download-*`); localized slugs chỉ mở khi qua expansion gate.
- Logo SVG đã có (`public/logo.svg`), dùng làm base để tạo OG image 1200x630px cho social/SEO.
- Authority execution sẽ theo hướng white-hat core + controlled distribution:
  - directories/listings sạch
  - community seeding có kiểm soát
  - không spam, không mua backlink rác

## Remaining Open Questions
- Không còn blocker ở mức plan. Các quyết định còn lại chỉ là execution detail trong implementation.
