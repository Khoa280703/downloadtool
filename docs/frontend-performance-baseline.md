# Frontend Performance Baseline

Ngày đo: 2026-02-27 (local lab)

## Scope

- URL: `http://127.0.0.1:4173/`
- Build mode: production (`pnpm --filter frontend build` + `pnpm --filter frontend preview`)
- Tool: Lighthouse CLI
- Command:

```bash
npx -y lighthouse http://127.0.0.1:4173 \
  --only-categories=performance \
  --chrome-flags='--headless --no-sandbox' \
  --quiet --output=json --output-path=/tmp/lh-frontend.json
```

## Result (median of 2 runs)

- Performance: `75`
- LCP: `~7.51s`
- FCP: `~1.66s`
- CLS: `~0.046`
- TBT: `0ms`

## Readout

- Điểm nghẽn chính còn lại là `LCP` cao.
- Nghi vấn chính: tài nguyên ảnh lớn từ nguồn bên ngoài và ảnh hero/content dùng remote URL.
- `CLS` đang ổn dưới ngưỡng `0.1`.
- `TBT` tốt, không có JS blocking lớn trong lab run.

## Next P1 Optimizations

1. Chuyển ảnh hero/section quan trọng về local/CDN tối ưu hơn, giảm thời gian tải ảnh đầu tiên.
2. Dùng kích thước ảnh phù hợp theo viewport (responsive image strategy).
3. Kiểm tra lại Cloudflare edge cache trên domain production theo `docs/cloudflare-cache-checklist.md`.
