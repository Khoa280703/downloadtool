# Cloudflare Cache Checklist

Áp dụng cho domain frontend tĩnh (ví dụ: `download.khoadangbui.online`) và API domain riêng (ví dụ: `api-download.khoadangbui.online`).

## 1) Rule cho frontend (cache mạnh)

- Bật rule `Cache Everything` cho:
  - `https://download.khoadangbui.online/*`
- Edge Cache TTL gợi ý:
  - HTML prerender: `5m` đến `30m`
  - Asset fingerprint (`/_app/*`, `*.js`, `*.css`, `*.woff2`, ảnh): `1d` đến `7d`
- Browser Cache TTL gợi ý:
  - HTML: ngắn (`5m`)
  - Asset hash: dài (`7d` hoặc hơn)

## 2) Rule cho API (không cache)

- Tạo bypass rule cho:
  - `https://api-download.khoadangbui.online/api/*`
  - `https://download.khoadangbui.online/api/*` (nếu frontend proxy cùng domain)
- Thiết lập:
  - `Cache Level: Bypass`
  - `Origin Cache Control: On`

## 3) Header gợi ý từ origin

- API responses:
  - `Cache-Control: no-store, no-cache, must-revalidate`
- Static files hash:
  - `Cache-Control: public, max-age=31536000, immutable`
- HTML:
  - `Cache-Control: public, max-age=300`

## 4) Verify sau khi cấu hình

- Kiểm tra HTML:
  - `curl -I https://download.khoadangbui.online/`
  - mong đợi `cf-cache-status: HIT` (sau lần truy cập thứ 2)
- Kiểm tra API:
  - `curl -I https://api-download.khoadangbui.online/api/health`
  - mong đợi `cf-cache-status: BYPASS` hoặc `DYNAMIC`
- Nếu luôn `MISS`:
  - kiểm tra rule thứ tự ưu tiên
  - kiểm tra có cookie/authorization làm bypass
  - purge cache rồi thử lại

## 5) Lưu ý vận hành

- Không cache endpoint download stream (`/api/stream*`, `/api/extract`, `/api/batch`).
- Sau mỗi lần deploy frontend lớn:
  - purge URL chính (`/`) hoặc purge toàn zone nếu cần.
