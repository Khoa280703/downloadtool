---
title: "Frontend Performance P0 (Tailwind Local + Font Self-host + Render Tuning)"
description: "Loại bỏ Tailwind CDN và Google Fonts runtime, tối ưu tải tài nguyên dưới fold, và chốt baseline Lighthouse + cache verify."
status: completed
priority: P1
effort: 2h
branch: main
tags: [frontend, performance, tailwind, fonts, lighthouse, cloudflare]
created: 2026-02-27
---

# Frontend Performance P0

## Overview

Mục tiêu P0: tăng tốc first load và giảm phụ thuộc runtime bên thứ ba mà không đổi giao diện Stitch-style hiện tại.

Phạm vi thực hiện:
- Chuyển Tailwind từ CDN sang build local (PostCSS/Vite).
- Self-host fonts `.woff2` và preload font quan trọng.
- Tối ưu tải ảnh dưới fold (`fetchpriority="low"`) và giữ `content-visibility` cho section dài.
- Ghi baseline Lighthouse + checklist verify cache.

## Phases

| # | Phase | Status |
|---|-------|--------|
| 1 | [Tailwind local build](./phase-01-tailwind-local-build.md) | completed |
| 2 | [Self-host fonts + preload](./phase-02-self-host-fonts-and-preload.md) | completed |
| 3 | [Below-fold render tuning](./phase-03-below-fold-render-and-bundle.md) | completed |
| 4 | [Lighthouse KPI + cache verify](./phase-04-lighthouse-kpi-and-cache-verification.md) | completed |

## Key Constraints

- Không đổi flow chính: fetch URL, chọn format, download.
- Không đổi backend API.
- Ưu tiên patch nhỏ, rollback dễ.
- Mọi thay đổi phải qua `pnpm --filter frontend check` + `pnpm --filter frontend build`.
