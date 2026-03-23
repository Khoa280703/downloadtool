# Phase 07 - Entity, Trust, Legal, and Brand Surface

## Context Links
- existing about/contact/privacy/terms/dmca pages

## Overview
- Priority: P0
- Status: complete
- Brief: Trong ngách downloader, trust không phải nice-to-have. Nó là điều kiện để site không trông như low-quality tool clone.

## Key Insights
- Google đánh giá trust ở cấp site, không chỉ page.
- Legal/trust pages tồn tại rồi nhưng chưa đủ để trở thành entity layer thật sự mạnh.

## Requirements
- Functional:
  - brand consistency mọi bề mặt
  - about page thực chất hơn
  - contact surface rõ
  - DMCA/terms/privacy cập nhật
  - org schema + sameAs sạch
  - author/editorial ownership model cho content pages nếu cần
- Non-functional:
  - không fake team/persona
  - không bịa organization signals

## Trust Surface Enhancements
- explicit product mission
- how Snapvie works
- why qualities differ
- infrastructure reliability explanations
- update timestamps on content
- content review ownership
- changelog visibility when useful

## Related Code Files
- Modify public trust pages
- SEO schema builders

## Implementation Steps
1. Audit trust gaps from SERP + page view perspective.
2. Enrich About and Contact.
3. Standardize brand voice and legal consistency.
4. Add structured entity references where valid.
5. Tie trust pages back into guides and homepage/footer.

## Todo List
- [x] Expand About page depth (added How Snapvie Works, Built for Quality sections)
- [x] Strengthen Contact page (added BreadcrumbList schema, OG tags)
- [x] Add/update organization schema fields (description, foundingDate, contactPoint)
- [x] Review legal page consistency (About/Contact now have consistent OG tags + schema)
- [x] Standardize footer/header trust links (Privacy, Terms, Contact in layout.svelte)

## Success Criteria
- Site no longer looks like a thin anonymous downloader tool.
- Brand/entity signals are coherent across pages.

## Risk Assessment
- Overstating company or team details.
- Disconnected legal pages with stale data.

## Security Considerations
- Any published business/contact details must be intentional and safe.

## Next Steps
- Phase 08 builds the off-page proof layer.
