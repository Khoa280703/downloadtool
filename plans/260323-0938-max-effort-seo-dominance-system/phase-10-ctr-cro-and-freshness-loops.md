# Phase 10 - CTR, CRO, and Freshness Loops

## Context Links
- Search Console
- analytics dashboards from phase 01

## Overview
- Priority: P0
- Status: proposed
- Brief: Ranking chưa đủ. Cần tối ưu click-through và conversion, rồi giữ content tươi và đúng intent theo query thực.

## Key Insights
- SEO tốt mà CTR kém -> mất click.
- SEO tốt mà conversion kém -> traffic vanity.
- Content không refresh -> tụt dần, nhất là ở queries có sự thay đổi UX / user expectations.

## Loops
- CTR loop:
  - title/meta tests
  - SERP wording
  - query cluster alignment
- CRO loop:
  - organic landing -> input focus
  - extract submit rate
  - download start rate
- Freshness loop:
  - quarterly review
  - update stale facts
  - enrich underperforming pages
- Comparison freshness loop (from Phase 05):
  - competitor feature changes → update comparison table within 2 weeks
  - competitor shutdown/rebrand → update + add notice
  - quarterly full comparison accuracy audit
- Featured snippet loop:
  - track "position 0" appearances in GSC
  - optimize quick-answer blocks on pages that rank #1-5 but lack snippet
  - A/B test snippet format (paragraph vs list vs table)

## Requirements
- Functional:
  - page-level refresh queue
  - title/meta test log
  - underperformer diagnosis playbook
- Non-functional:
  - avoid clickbait
  - avoid constant unnecessary churn on winning pages

## Implementation Steps
1. Create page health score.
2. Build refresh queue every month.
3. Run title/meta iteration for key pages.
4. Improve weak CTR pages before creating too many new pages.
5. Merge or rewrite pages with weak engagement.

## Todo List
- [ ] Build page health score
- [ ] Create monthly refresh queue
- [ ] Run title/meta experiments
- [ ] Improve low-CTR pages
- [ ] Improve low-conversion pages

## Success Criteria
- Organic pages improve both clicks and product engagement over time.

## Risk Assessment
- Over-testing titles can destabilize pages.
- Refreshing without reason creates churn.

## Security Considerations
- Never use deceptive titles/meta.

## Next Steps
- Phase 11 ensures scale does not become spam.
