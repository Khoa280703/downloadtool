# Brainstorm Report: Outstanding Issues quanh extract/rate-limit

- Scope: phân tích 3 vấn đề tồn đọng + đánh giá khuyến nghị tiếp theo
- Date: 2026-02-28 12:20 (+07)
- Mode: brainstorm (no implementation)

## Problem Statement

Hệ thống hiện chạy được end-to-end extract + download. Nhưng còn 3 vấn đề kỹ thuật:
1. `/api/extract` đôi khi trả rất ít format (chỉ còn progressive 18), giảm lựa chọn chất lượng.
2. Rate limit đang in-memory (`governor keyed store`) nên reset khi restart, không share state giữa nhiều instance.
3. Nhánh `403` (không xác định IP) là defensive; local gần như không kích hoạt khi đã inject `ConnectInfo`.

Mục tiêu: tăng độ ổn định format output, tăng khả năng quan sát và vận hành khi scale, giữ hệ thống đơn giản vừa đủ (KISS/YAGNI).

---

## Vấn đề 1: `/api/extract` trả ít format

### Quan sát thực tế

- Có case trả nhiều format, có case chỉ trả format 18.
- Tình trạng này không phá luồng download cơ bản, nhưng UX giảm rõ (ít chất lượng để chọn).
- Đặc thù YouTube/yt-dlp thay đổi theo thời điểm, IP reputation, client profile, cookie/session, PO-token constraints.

### Brutal honesty

- Không có giải pháp “đảm bảo luôn đủ format” 100% chỉ bằng 1 profile yt-dlp.
- Nếu cố hard-code nhiều hack client profile không có telemetry thì sẽ khó debug, tăng debt nhanh.

### Root-cause candidates (theo xác suất)

1. Upstream behavior variance của YouTube theo IP/session/client profile.
2. `player_client` không tối ưu cho mọi môi trường (region/network khác nhau).
3. Không có observability đủ sâu để biết hit profile nào, fallback nào, số format mỗi request.
4. Parser policy đang bỏ một số loại format theo protocol/shape, làm “mỏng” danh sách hơn kỳ vọng (cần đo, chưa đủ dữ liệu để kết luận).

### Phương án

#### Approach A — giữ nguyên logic, tăng observability trước
- Thêm metrics/log có cấu trúc:
  - số format raw vs parsed
  - số lần fallback retry
  - profile used (`android,web` vs fallback none)
  - latency extract, tỷ lệ 200/500
- Chưa mở rộng chiến lược profile lúc này.

Pros:
- Rẻ, nhanh, ít rủi ro.
- Có dữ liệu thật để ra quyết định đúng.

Cons:
- Không cải thiện ngay số format cho người dùng.

#### Approach B — profile cascade có kiểm soát
- Thay vì 1 profile cố định, dùng chuỗi thử:
  - primary: `android,web`
  - secondary: no extractor args
  - tertiary (nếu cần): profile khác theo benchmark
- Chọn output “best quality set” giữa các attempt (không chỉ success/fail).

Pros:
- Tăng xác suất có nhiều format ngay.

Cons:
- Tăng latency và tải upstream.
- Logic phức tạp, khó vận hành nếu thiếu metrics.

#### Approach C — dual mode API
- `mode=fast` (default): hành vi hiện tại.
- `mode=full_formats`: cho phép retry/cascade mạnh hơn để lấy format list dày.

Pros:
- Cân bằng UX vs hiệu năng theo nhu cầu.

Cons:
- API complexity tăng.
- Cần client/UI xử lý mode.

### Recommendation cho Vấn đề 1

- **Chọn A ngay** (1-2 ngày): instrument metrics + log trước.
- Sau khi có dữ liệu 3-7 ngày production, nếu tỷ lệ “format_count <= 1” vẫn cao, chuyển **B có giới hạn** (tối đa 2 attempt).
- Chỉ dùng C nếu product thực sự cần “show nhiều format mọi giá”.

---

## Vấn đề 2: rate limit in-memory không phù hợp scale ngang

### Brutal honesty

- Với single instance/homeserver: in-memory là đủ tốt, đúng KISS.
- Với multi-instance thật: in-memory app-level rate limit **không nhất quán**; user có thể “né” bằng load balancer distribution.

### Phương án

#### Approach A — giữ in-memory, đẩy rate-limit chính ra edge (Cloudflare)
- Dùng Cloudflare Rate Limiting/WAF làm global gate.
- App-level governor giữ vai trò second guard.

Pros:
- Đúng bài cho internet-facing service.
- Ít thay đổi code backend.

Cons:
- Phụ thuộc cấu hình hạ tầng.

#### Approach B — distributed limiter (Redis)
- Token bucket/sliding window bằng Redis script/atomic ops.
- App instances share state thật.

Pros:
- Nhất quán giữa instances.

Cons:
- Thêm hạ tầng, độ phức tạp vận hành, failure mode mới.

#### Approach C — sticky session + in-memory
- Dùng LB sticky theo IP/cookie để “giả distributed”.

Pros:
- Nhanh hơn B.

Cons:
- Không bền vững, dễ lệch khi scale/rolling restart.

### Recommendation cho Vấn đề 2

- **Ngắn hạn**: A (edge-first + app fallback).
- **Khi scale thật (>1 backend active nhận traffic đều)**: B.
- Không khuyến nghị C như giải pháp chính.

---

## Vấn đề 3: nhánh 403 defensive khó kích hoạt local

### Đánh giá

- Branch này hợp lý về security semantics: không xác định IP => reject.
- Đúng là local wiring chuẩn có `ConnectInfo` nên branch hiếm hit.

### Rủi ro nếu bỏ

- Mất guard khi deploy sai wiring/proxy chain.
- Lúc sự cố sẽ trả behavior mơ hồ hơn.

### Recommendation

- **Giữ branch 403**.
- Bổ sung test mức middleware/unit để cover branch nhân tạo (build Request không có CF header + không có ConnectInfo extension).
- Tăng log field `ip_source=none|cf|connect_info` để phát hiện misconfig sớm.

---

## Đánh giá 3 khuyến nghị bạn đưa ra

### 1) Integration test tự động `/api/extract` + `/api/stream` với URL ổn định

Đúng hướng, nhưng cần phân lớp:
- CI stable test: dùng fixture/mock-like test local hoặc URL nội bộ ổn định kiểm soát được.
- Scheduled canary (không block merge): test URL public YouTube vì upstream biến động cao.

Lý do: nếu ép URL public vào CI gating dễ flaky fail giả.

### 2) Thêm metric log cho cache hit/miss và fallback retry count

Đây là việc **nên làm trước tiên**. Không có metrics thì tối ưu profile chỉ là đoán.

Metric tối thiểu:
- `extract.cache.hit`, `extract.cache.miss`
- `extract.retry.count`
- `extract.format.count`
- `extract.profile.used`
- `extract.error.kind`
- latency histogram cho `/api/extract`

### 3) Benchmark thêm profile client nếu cần nhiều format

Đúng, nhưng làm sau khi có metrics baseline.
Benchmark matrix tối thiểu:
- profile: `android,web` | none | (candidate khác)
- network region/IP pool
- mẫu URL đa dạng (music, short, long-form, age-gated nếu hợp lệ)

KPIs:
- success rate
- median format_count
- p95 latency
- error rate by kind

---

## Final Recommended Path (ưu tiên)

### P0 (ngay)
1. Chốt giữ kiến trúc hiện tại (đang hoạt động, đã push).
2. Bổ sung observability cho extract + rate-limit.
3. Viết middleware tests cho 429/403 branch (không cần đợi integration test full).

### P1 (sau khi có dữ liệu 3-7 ngày)
1. Nếu format_count thấp kéo dài: rollout profile cascade giới hạn 2 attempt.
2. Bổ sung canary E2E scheduled job cho URL public.

### P2 (khi scale multi-instance)
1. Đẩy global rate limit về Cloudflare làm lớp chính.
2. Nếu cần consistency backend-level, thêm Redis distributed limiter.

---

## Risks & Mitigation

1. Risk: thêm retry profile làm tăng latency.
- Mitigation: cap max attempts = 2, timeout cứng, metric p95/p99.

2. Risk: CI flaky nếu dùng URL public làm gating.
- Mitigation: tách CI stable vs scheduled canary.

3. Risk: over-engineer distributed limiter quá sớm.
- Mitigation: chỉ bật khi scale ngang thực sự.

---

## Success Metrics

1. `extract.success_rate` >= hiện tại, không tăng lỗi 5xx.
2. `format_count_p50/p90` tăng rõ sau tối ưu profile (nếu rollout).
3. `rate_limit_false_positive` thấp (ít user hợp lệ bị 429).
4. Không phát sinh incident do missing IP detection; 403 branch có telemetry rõ.

---

## Unresolved questions

1. Sản phẩm có cần “nhiều format” như một requirement bắt buộc, hay “download ổn định” mới là ưu tiên số 1?
2. Hạ tầng hiện tại đã có Cloudflare Rate Limiting rule production chưa, hay mới rely vào app limiter?
3. Có chấp nhận tách canary test khỏi CI gating để tránh flaky không?
