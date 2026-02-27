---
title: "Phase 04 - Lighthouse KPI and Cache Verification"
status: completed
priority: P2
effort: 20m
---

# Phase 04 - Lighthouse KPI and Cache Verification

## Context Links

- Baseline report: `docs/frontend-performance-baseline.md`
- Cache checklist: `docs/cloudflare-cache-checklist.md`

## Overview

Chạy benchmark local để lấy baseline sau tối ưu và chốt hướng verify cache.

## Implementation

1. Build frontend production.
2. Chạy local preview (`127.0.0.1:4173`).
3. Chạy Lighthouse performance category và lưu JSON report.
4. Trích xuất metrics chính (performance, LCP, FCP, CLS, TBT, Speed Index).
5. Cập nhật tài liệu baseline + steps verify.

## Success Criteria

- Có số đo baseline được ghi lại trong docs.
- Có checklist verify cache Cloudflare cho môi trường production.
