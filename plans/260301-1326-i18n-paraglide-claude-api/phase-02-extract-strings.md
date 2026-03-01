# Phase 2: Extract Hardcoded Strings -> messages/en.json

**Status:** completed | **Priority:** critical | **Effort:** high

## Overview

Đã chuyển các chuỗi UI hardcoded chính sang `m.*` keys và gom vào `frontend/messages/en.json`, bao gồm home, account, privacy, theme toggle, language menu, và download CTA text.

## Implemented Scope

- `frontend/src/routes/+page.svelte`
- `frontend/src/routes/privacy/+page.svelte`
- `frontend/src/routes/(auth)/account/+page.svelte`
- `frontend/src/routes/+layout.svelte`
- `frontend/src/components/LanguageSwitcher.svelte`
- `frontend/src/components/DownloadBtn.svelte`

## Key Outcomes

- Trang privacy đã được i18n hóa toàn bộ headings/paragraph/list items.
- Các chuỗi còn sót trên home (thumbnail alt, recommended badge, bookmarklet CTA) đã chuyển sang `m.*`.
- Theme labels/aria text đã dùng i18n keys dùng chung ở home + layout.
- `en.json` đã mở rộng lên bộ key mới phục vụ privacy + shared UI strings.

## Files Modified

- `frontend/messages/en.json`
- `frontend/src/routes/+page.svelte`
- `frontend/src/routes/privacy/+page.svelte`
- `frontend/src/routes/(auth)/account/+page.svelte`
- `frontend/src/routes/+layout.svelte`
- `frontend/src/components/LanguageSwitcher.svelte`
- `frontend/src/components/DownloadBtn.svelte`

## Success Criteria

- [x] Chuỗi hardcoded trong phạm vi phase đã được trích xuất vào `messages/en.json`
- [x] Code dùng `m.key()` thay cho text literal ở các vị trí đã nêu
- [x] `pnpm --filter frontend check` pass (0 errors, 0 warnings)
- [x] `pnpm --filter frontend build` pass
