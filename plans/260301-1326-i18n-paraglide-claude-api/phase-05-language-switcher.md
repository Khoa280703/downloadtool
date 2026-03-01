# Phase 5: Language Switcher UI

**Status:** completed | **Priority:** medium | **Can parallel with Phase 4**

## Overview

Đã triển khai language switcher dropdown dùng Paraglide runtime, lưu lựa chọn vào `localStorage` và điều hướng URL theo locale tương ứng.

## Implemented Behavior

- Current locale hiển thị ngay trên trigger.
- Chọn locale -> lưu `preferred-lang` -> điều hướng bằng `localizeHref(...)`.
- Dropdown đóng khi click outside.
- Locale hiện tại có trạng thái selected.
- Tự redirect theo `preferred-lang` ở lần mở trang sau (khi URL chưa có explicit locale prefix).

## Architecture Notes

- `LanguageSwitcher` được tích hợp vào `SiteHeader` để dùng chung cho home + non-home.
- `+layout.svelte` giữ logic `preferred-lang` redirect/persistence.
- Không locale-wrap API paths (`/api/*`) nên auth/proxy flow không bị ảnh hưởng.

## Files Created/Modified

- `frontend/src/components/LanguageSwitcher.svelte` (new)
- `frontend/src/components/SiteHeader.svelte`
- `frontend/src/routes/+layout.svelte`

## Success Criteria

- [x] Language switcher hiển thị ổn định trên cả home và non-home pages
- [x] Click đổi ngôn ngữ điều hướng đúng URL locale
- [x] Locale hiện tại được highlight
- [x] Persist lựa chọn ngôn ngữ qua reload bằng `localStorage`
- [x] Dropdown đóng khi click outside
