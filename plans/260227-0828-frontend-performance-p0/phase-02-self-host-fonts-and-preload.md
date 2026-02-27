---
title: "Phase 02 - Self-host Fonts and Preload"
status: completed
priority: P1
effort: 30m
---

# Phase 02 - Self-host Fonts and Preload

## Context Links

- Fonts folder: `frontend/static/fonts/`
- Font-face: `frontend/src/app.css`
- Head preloads: `frontend/src/routes/+page.svelte`

## Overview

Loại bỏ Google Fonts runtime và chuyển font sang static assets `.woff2` nội bộ.

## Implementation

1. Download và lưu font vào `frontend/static/fonts/`:
- Fredoka (`latin`, `latin-ext`)
- Nunito (`normal/italic`: `latin`, `vietnamese`)
- Spline Sans (`latin`, `latin-ext`)
- Material Symbols Outlined (variable)

2. Khai báo `@font-face` trong `frontend/src/app.css`:
- dùng `font-display: swap` cho text fonts.
- giữ unicode-range theo subset.

3. Thêm class `.material-symbols-outlined` vào `app.css` để icon hoạt động không cần CDN.

4. Thêm preload các font critical trong `+page.svelte`:
- Fredoka latin
- Nunito latin
- Material Symbols Outlined

5. Xóa Google Fonts links khỏi `+page.svelte`.

## Success Criteria

- Không còn request `fonts.googleapis.com` / `fonts.gstatic.com` từ trang chủ.
- Font hiển thị đúng và icon Material Symbols hoạt động.
