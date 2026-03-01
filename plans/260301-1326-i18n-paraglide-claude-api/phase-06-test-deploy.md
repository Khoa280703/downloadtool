# Phase 6: Test & Deploy

**Status:** completed | **Priority:** high | **Blocks:** nothing

## Overview

Đã chạy vòng verify kỹ thuật cho frontend i18n flow ở local build + preview và xác nhận các đường SEO/auth quan trọng không bị vỡ.

## Validation Run (đã thực hiện)

```bash
pnpm --filter frontend check
pnpm --filter frontend build
```

Kết quả:
- `check`: pass (0 errors, 0 warnings)
- `build`: pass

Preview validation:
- `curl http://127.0.0.1:4173/vi/privacy` -> `200` + có `hreflang` tags injected từ hooks
- `curl http://127.0.0.1:4173/sitemap.xml` -> `200` + XML có đầy đủ alternate links
- `curl http://127.0.0.1:4173/api/auth/get-session` -> `200` (API route không bị locale-wrap)

## Notes

- `sitemap.xml` dùng giá trị `ORIGIN` từ env, nên ở local có thể thấy domain local env (`http://localhost:5168`), đây là behavior expected.
- Warnings circular dependency từ dependency tree (`kysely`, `zod`) vẫn tồn tại nhưng không làm fail build.

## Success Criteria

- [x] `pnpm --filter frontend check` pass
- [x] `pnpm --filter frontend build` pass
- [x] SSR route hreflang injection verified
- [x] `/sitemap.xml` endpoint verified
- [x] `/api/auth/*` route vẫn hoạt động bình thường
- [ ] Search Console + Lighthouse verification (post-deploy/manual)
