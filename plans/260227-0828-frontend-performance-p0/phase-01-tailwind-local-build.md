---
title: "Phase 01 - Tailwind Local Build"
status: completed
priority: P1
effort: 35m
---

# Phase 01 - Tailwind Local Build

## Context Links

- Source: `frontend/src/routes/+page.svelte`
- Config: `frontend/tailwind.config.cjs`, `frontend/postcss.config.cjs`
- Entry CSS: `frontend/src/app.css`

## Overview

Loại bỏ Tailwind CDN runtime và chuyển sang build-time Tailwind qua Vite/PostCSS.

## Implementation

1. Thêm dependencies vào `frontend/package.json`:
- `tailwindcss@3.4.x`
- `postcss`
- `autoprefixer`
- `@tailwindcss/forms`
- `@tailwindcss/container-queries`

2. Tạo `frontend/tailwind.config.cjs`:
- map đầy đủ theme tokens đang dùng (`colors`, `fontFamily`, `borderRadius`, `boxShadow`, `animation`, `keyframes`).

3. Tạo `frontend/postcss.config.cjs` với plugin `tailwindcss` + `autoprefixer`.

4. Tạo `frontend/src/app.css` với:
- `@tailwind base/components/utilities`.

5. Import `app.css` vào `frontend/src/routes/+layout.svelte`.

6. Xóa khỏi `+page.svelte`:
- `<script src="https://cdn.tailwindcss.com?...">`
- `<script id="tailwind-config">...</script>` inline.

## Success Criteria

- Không còn dùng Tailwind CDN trong runtime.
- Build pass với style tương đương trước.
