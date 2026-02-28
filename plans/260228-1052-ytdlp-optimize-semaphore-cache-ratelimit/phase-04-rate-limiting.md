# Phase 04 — Rate Limiting per IP on /api/extract

## Overview

- **Priority:** P1
- **Status:** pending
- **Effort:** ~45 min
- **Optimization:** #4

## Key Insights

- `tower_governor 0.4` phụ thuộc `axum 0.7` — **không tương thích** với `axum 0.8` workspace → dùng `governor` crate trực tiếp (không phụ thuộc Axum version)
- `governor` là token bucket thuần Rust, `tower_governor` chỉ là wrapper mỏng → dùng thẳng cũng không phức tạp hơn đáng kể
- Implement custom Axum middleware (30 dòng) thay vì Tower middleware → fully compatible Axum 0.8
- Apply only to `/api/extract`; `/api/stream` và `/api/stream/muxed` unlimited
- Cloudflare in front: extract IP từ `CF-Connecting-IP` → fallback `ConnectInfo<SocketAddr>` → 403 nếu không xác định được
- Fallback về `127.0.0.1` là sai: all unidentified requests share one bucket → có thể DoS chéo

## Requirements

- Token bucket: 1 token / 6 giây, burst_size = 5 (≈10 req/min sustained, 5 burst)
- Apply to `POST /api/extract` only
- HTTP 429 khi vượt limit; HTTP 403 khi không xác định được IP
- `ConnectInfo<SocketAddr>` injected vào Axum — cần `.into_make_service_with_connect_info::<SocketAddr>()` ở cuối

## Files to Modify

- `crates/api/Cargo.toml`
- `crates/api/src/main.rs`

## New Dependency

```toml
# crates/api/Cargo.toml [dependencies]
governor = "0.8"
```

`governor` crate là thuần Rust, không có axum feature flag, không bị lock axum version.

## Token Bucket Parameters

- `Quota::with_period(Duration::from_secs(6))` → 1 token / 6s = 10/min sustained
- `.allow_burst(NonZeroU32::new(5))` → bucket capacity 5
- Hiệu ứng thực: 5 request liên tiếp đầu tiên được phép ngay (drain burst), request thứ 6 phải chờ ~6s
- **Success criteria đúng:** gửi 6 request cực nhanh → request thứ 6 nhận 429 ngay lập tức

**Lý do burst_size=5:** Frontend `MAX_CONCURRENT=1` + jitter 2-5s → ~1 req/5s. burst_size=5 đủ headroom cho UI interactions mà không mở gap lạm dụng.

## Implementation Steps

### 1. Add dependency to `crates/api/Cargo.toml`

```toml
governor = "0.8"
```

### 2. Implement rate limiter type alias + factory

```rust
use governor::{Quota, RateLimiter, clock::DefaultClock, state::keyed::DefaultKeyedStateStore};
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use std::net::IpAddr;

type KeyedLimiter = RateLimiter<IpAddr, DefaultKeyedStateStore<IpAddr>, DefaultClock>;

fn make_rate_limiter() -> Arc<KeyedLimiter> {
    let quota = Quota::with_period(Duration::from_secs(6))
        .expect("valid period")
        .allow_burst(NonZeroU32::new(5).unwrap());
    Arc::new(RateLimiter::keyed(quota))
}
```

### 3. Implement middleware function

```rust
use axum::{
    extract::{ConnectInfo, State, Request},
    middleware::Next,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use std::net::SocketAddr;

/// Extract real visitor IP: CF-Connecting-IP → ConnectInfo socket addr → None
fn extract_ip(req: &Request) -> Option<IpAddr> {
    // Cloudflare sets CF-Connecting-IP — real visitor IP, cannot be spoofed
    if let Some(ip) = req.headers()
        .get("cf-connecting-ip")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
    {
        return Some(ip);
    }
    // Fallback: TCP socket addr injected by Axum (requires into_make_service_with_connect_info)
    req.extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ConnectInfo(addr)| addr.ip())
}

async fn rate_limit_middleware(
    State(limiter): State<Arc<KeyedLimiter>>,
    request: Request,
    next: Next,
) -> Response {
    let Some(ip) = extract_ip(&request) else {
        // Request bypassed Cloudflare AND has no socket info — misconfiguration or direct attack
        return (
            StatusCode::FORBIDDEN,
            [("content-type", "application/json")],
            r#"{"error":"Unable to identify client IP"}"#,
        ).into_response();
    };

    if limiter.check_key(&ip).is_err() {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            [("content-type", "application/json")],
            r#"{"error":"Rate limit exceeded"}"#,
        ).into_response();
    }

    next.run(request).await
}
```

### 4. Wire up in `main.rs`

```rust
use axum::middleware;

// In main():
let limiter = make_rate_limiter();

let app = Router::new()
    .route("/health", get(routes::health_check))
    .route("/openapi.json", get(routes::openapi_handler))
    .route("/bm.js", get(routes::bm_js_handler))
    .route("/userscript", get(routes::userscript_handler))
    .route(
        "/api/extract",
        post(routes::extract_handler)
            .route_layer(middleware::from_fn_with_state(limiter, rate_limit_middleware)),
    )
    .route("/api/stream", get(routes::stream_handler))
    .route("/api/stream/muxed", get(routes::muxed_stream_handler))
    .route("/api/batch", get(routes::batch_handler))
    .layer(CorsLayer::permissive())
    .layer(TraceLayer::new_for_http());

// IMPORTANT: ConnectInfo requires this instead of .into_make_service()
let listener = tokio::net::TcpListener::bind(&addr).await?;
axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;
```

**Note:** Thay `into_make_service()` bằng `into_make_service_with_connect_info::<SocketAddr>()` để inject `ConnectInfo` extension vào mọi request.

## Todo List

- [ ] Add `governor = "0.8"` to `crates/api/Cargo.toml`
- [ ] Add `KeyedLimiter` type alias + `make_rate_limiter()` fn
- [ ] Add `extract_ip()` helper + `rate_limit_middleware()` fn
- [ ] Replace `into_make_service()` → `into_make_service_with_connect_info::<SocketAddr>()`
- [ ] Apply `.route_layer(middleware::from_fn_with_state(...))` on `/api/extract` only
- [ ] Run `cargo check -p api`
- [ ] Run `cargo build -p api`
- [ ] Manual test 429: `for i in {1..6}; do curl -s -o /dev/null -w "%{http_code}\n" -X POST http://localhost:PORT/api/extract -H "Content-Type: application/json" -d '{"url":"https://youtube.com/watch?v=test"}'; done` → request thứ 6 = 429
- [ ] Verify `/api/stream` không bị rate limit: gửi 10+ request stream → tất cả 200
- **Note 403:** Không test được locally — `into_make_service_with_connect_info` inject ConnectInfo vào mọi request, nên branch 403 chỉ trigger trong môi trường wiring sai (server không dùng ConnectInfo). 403 là defensive fallback, không cần test thường xuyên.

## Success Criteria

- `cargo check -p api` clean
- 6 rapid-fire requests → request thứ 6 nhận `HTTP 429 Too Many Requests`
- `/api/stream/muxed` không bị ảnh hưởng
- ~~Request thiếu CF header → 403~~ (không test được khi dùng `into_make_service_with_connect_info`; branch là defensive fallback, không phải happy path)

## Risk Assessment

- `governor` lưu IP state trong memory → mất khi restart (acceptable)
- `DefaultKeyedStateStore` không tự purge entries cũ → có thể tích lũy IP theo thời gian. Với homeserver (few thousand unique IPs/day), không đáng lo; nếu cần, dùng `DashMapStateStore` với periodic cleanup
- Nếu Cloudflare IP range chưa được lock ở infra (UFW/nginx), kẻ tấn công có thể direct-hit và tự fake `CF-Connecting-IP` — phải lock ở tầng infra, không phải code

## Security Considerations

- `CF-Connecting-IP` chỉ đáng tin khi origin lock chỉ nhận traffic từ Cloudflare IP range
- Fallback `ConnectInfo` chỉ hoạt động khi Axum served trực tiếp (không qua reverse proxy thuần TCP)
- 403 thay vì 429 cho unidentified IP: phân biệt rõ "rate limited" vs "security reject"

## Resolved Decisions

- **Reverse proxy:** Cloudflare in front → `CF-Connecting-IP` primary, `ConnectInfo` fallback, 403 nếu cả hai fail
- **burst_size:** 5 — frontend MAX_CONCURRENT=1 + jitter ~1 req/5s, không cần cao hơn
- **tower_governor:** loại bỏ — incompatible axum 0.8; dùng `governor` trực tiếp
- **127.0.0.1 fallback:** không dùng — shared bucket gây DoS chéo; 403 là đúng
