# 🔬 Research: Giải Pháp Kiến Trúc Production Cho Video Download Service

> **Ngày**: 2026-03-17  
> **Mục tiêu**: Nghiên cứu các giải pháp kiến trúc production tốt hơn cho downloadtool, sẵn sàng thay đổi kiến trúc nếu cần.

---

## Mục lục

1. [Phân tích vấn đề cốt lõi](#1-phân-tích-vấn-đề-cốt-lõi)
2. [Giải pháp A: Hybrid Download Flow (★ Đề xuất chính)](#2-giải-pháp-a-hybrid-download-flow--đề-xuất-chính)
3. [Giải pháp B: Client-Side Muxing](#3-giải-pháp-b-client-side-muxing)
4. [Giải pháp C: Cloudflare Workers Edge Proxy](#4-giải-pháp-c-cloudflare-workers-edge-proxy)
5. [Giải pháp D: On-Device Processing (Cobalt Model)](#5-giải-pháp-d-on-device-processing-cobalt-model)
6. [Giải pháp cho vấn đề Bot Check & po_token](#6-giải-pháp-cho-vấn-đề-bot-check--po_token)
7. [So sánh tổng hợp](#7-so-sánh-tổng-hợp)
8. [Đề xuất kiến trúc mới](#8-đề-xuất-kiến-trúc-mới)

---

## 1. Phân tích vấn đề cốt lõi

Kiến trúc hiện tại của downloadtool có **2 nút thắt cổ chai chính**:

### 1.1. Chi phí Muxing trên Server

| Metric | Giá trị ước tính |
|---|---|
| Video 1080p 5 phút | ~100-200MB bandwidth server |
| Video 4K 10 phút | ~500MB-1GB bandwidth server |
| CPU mux time (Rust, `-c copy`) | 2-10 giây |
| Concurrent mux jobs | 1 (single-threaded loop) |

**Vấn đề**: Mỗi lần user tải video 1080p+, server phải:
1. Download video stream từ YouTube CDN (~100MB)
2. Download audio stream (~10MB)  
3. Mux 2 stream lại
4. Upload kết quả lên S3 (~100MB)
5. Serve file cho user (~100MB)

→ **Tổng bandwidth server = ~310MB cho 1 video 1080p**

### 1.2. Bot Check & IP Blocking

YouTube ngày càng siết chặt:
- `po_token` (Proof of Origin) bắt buộc cho format chất lượng cao
- Rate limit theo IP datacenter
- Bot detection qua BotGuard/DroidGuard challenge

→ Proxy dân cư giải quyết phần nào, nhưng chi phí cao và phức tạp vận hành.

---

## 2. Giải pháp A: Hybrid Download Flow (★ Đề xuất chính)

### Ý tưởng

Thay vì mux mọi thứ trên server, **tách thành 2 luồng**:

```
┌─────────────┐    ≤720p (pre-muxed)     ┌──────────────┐
│   Frontend   │ ◄──── Direct Link ─────► │ YouTube CDN  │
│   (Svelte)   │                          └──────────────┘
│              │    ≥1080p (cần mux)      ┌──────────────┐
│              │ ────► Mux Job ──────────► │ Worker Pool  │
└─────────────┘                           └──────────────┘
```

### Luồng 1: Direct Link cho ≤720p

- Server extract URL → trả direct link cho client
- Client download trực tiếp từ YouTube CDN
- **Server bandwidth = ~0** (chỉ vài KB cho JSON response)

**Vấn đề CORS**: YouTube CDN không cho client download trực tiếp qua Fetch API. Giải pháp:

| Cách | Mô tả | Khả thi |
|---|---|---|
| **Lightweight stream proxy** | Server pipe stream từ YouTube → client, không buffer | ✅ Tốt nhất |
| `<a>` tag redirect | Redirect browser tới CDN URL | ⚠️ Không ổn định |
| CF Workers proxy | Edge proxy thêm CORS headers | ⚠️ TOS risk |

**Lightweight stream proxy** là cách cobalt.tools đang dùng: Server không lưu file vào disk, chỉ pipe stream chunk-by-chunk từ upstream tới client. Bandwidth vẫn qua server nhưng **không cần mux, không cần S3, không cần job queue**.

### Luồng 2: Server Mux cho ≥1080p

- Giữ nguyên pipeline hiện tại (worker + job + S3)
- Nhưng **chỉ khi user chọn 1080p trở lên**
- Streaming mux: yt-dlp stream → Rust muxer → S3 multipart upload (đang có)

### Lợi ích

- **Giảm 60-80% workload** nếu đa số user tải 720p trở xuống
- **Không cần thay đổi kiến trúc lớn** — chỉ thêm stream proxy endpoint
- **Tương thích ngược** với hệ thống hiện tại

### Chi phí thay đổi: 🟢 Thấp

Thêm 1 endpoint `/api/stream-proxy` pipe từ upstream, frontend thêm logic chọn luồng.

---

## 3. Giải pháp B: Client-Side Muxing

### Ý tưởng

Server trả 2 direct link (video + audio) → Client dùng WebAssembly mux ngay trên browser.

### Công nghệ có sẵn

| Thư viện | Tốc độ | File size limit | Ghi chú |
|---|---|---|---|
| `ffmpeg.wasm` | 8-20% native | ~2GB (RAM) | Nặng 24MB WASM binary |
| `WebCodecs + mp4-muxer` | 10-20x nhanh hơn ffmpeg.wasm | Tùy RAM | Chỉ mux (không re-encode) |
| `mediabunny` | Hardware-accelerated | Tùy | Mới, ít community |

### Đánh giá

**Ưu điểm:**
- Server gần zero cost
- Scale tự nhiên — CPU của user

**Nhược điểm:**
- ❌ **CORS vẫn là vấn đề**: Client không thể fetch stream URL từ YouTube CDN
- ❌ **RAM giới hạn**: Video 4K > 2GB sẽ crash browser
- ❌ **UX kém**: Chậm trên thiết bị yếu, progress bar khó implement
- ❌ **Mobile**: Phone cũ sẽ rất chậm hoặc crash
- ⚠️ Cần `SharedArrayBuffer` (COOP/COEP headers) cho multi-threading

### Kết luận: ⚠️ Chỉ phù hợp cho use case nhỏ

Có thể làm option "fast mux" cho video ngắn < 500MB, nhưng **KHÔNG nên là giải pháp chính**. CORS khiến client vẫn phải fetch qua server proxy → chưa giảm bandwidth gốc.

---

## 4. Giải pháp C: Cloudflare Workers Edge Proxy

### Ý tưởng

Chuyển logic extract + stream proxy lên **Cloudflare Workers** (serverless edge).

```
Client → CF Worker (edge) → YouTube API → extract URL
Client → CF Worker (edge) → YouTube CDN → stream video (với CORS headers)
```

### Đánh giá

**Ưu điểm:**
- Auto-scale, không lo server
- Edge network phân tán IP (ít bị ban hơn single server)
- Workers có 30s CPU time (đủ cho extract, KHÔNG đủ cho mux)

**Nhược điểm:**
- ❌ **Cloudflare TOS**: Cấm stream video lớn qua Workers/CDN trừ khi dùng Cloudflare Stream ($5/1000 phút)
- ❌ **Không thể mux**: Workers không có ffmpeg, không đủ CPU time cho mux
- ❌ **Subnets giới hạn**: Workers IP cũng bị YouTube nhận diện
- ⚠️ Chi phí: Free tier 100K req/ngày, Paid $5/10M req/tháng

### Kết luận: ⚠️ Chỉ phù hợp cho extract, KHÔNG phù hợp cho stream/mux

Cloudflare Workers chỉ nên dùng cho **extract URL** (nhẹ, nhanh), KHÔNG stream video qua.

---

## 5. Giải pháp D: On-Device Processing (Cobalt Model)

### Cobalt.tools đang làm gì?

Cobalt.tools (open-source, GitHub: `imputnet/cobalt`) là một trong những video downloader thành công nhất hiện tại. Kiến trúc của họ:

1. **On-device processing (mặc định)**: Remux/transcode trên thiết bị user
2. **Server streaming (fallback)**: Nếu device không hỗ trợ, server stream trực tiếp, **KHÔNG lưu disk**
3. **Forced tunneling**: Ẩn IP user khi download

### Bài học từ Cobalt

| Aspect | Cobalt | downloadtool (hiện tại) |
|---|---|---|
| Pre-muxed (≤720p) | Stream qua server, no disk | Trả direct link hoặc mux |
| Mux (≥1080p) | On-device hoặc server stream | Server mux → S3 → serve |
| Lưu file server | ❌ Không bao giờ | ✅ S3/local storage |
| Privacy | Tunnel mọi thứ | N/A |
| Scale | Nhiều instance, stateless | Stateful (job queue, DB) |

### Key insight

Cobalt **không lưu file trên server**. Mọi thứ đều stream trực tiếp:
```
YouTube CDN → Cobalt Server (pipe) → User Browser
```

Ý nghĩa: **Server chỉ tốn bandwidth, KHÔNG tốn disk/S3/job overhead**. Đơn giản hơn nhiều nhưng user không thể resume download.

### Áp dụng cho downloadtool

Giữ job system cho 1080p+ (vì user cần resume, file lớn) nhưng thêm **streaming mode** cho 720p: pipe trực tiếp, không queue, không S3.

---

## 6. Giải pháp cho vấn đề Bot Check & po_token

### Thực trạng (2025-2026)

- YouTube bắt buộc `po_token` cho Innertube player requests
- Token **bound per video ID** — không reuse được
- Manual extraction không khả thi cho production

### Giải pháp production-grade

| Cách | Mô tả | Đánh giá |
|---|---|---|
| **`bgutil-ytdlp-pot-provider` plugin** | Auto-generate po_token qua BgUtils | ✅ Best practice hiện tại |
| **`yt-dlp-getpot-wpc` plugin** | Generate token qua browser headless | ✅ Stable fallback |
| **Cookie + OAuth** | Dùng cookie YouTube Premium/OAuth | ⚠️ Rủi ro ban account |
| **Residential proxy rotation** | Đổi IP liên tục | ✅ Đang dùng, nên tối ưu thêm |
| **Deno/JS runtime** | yt-dlp cần JS runtime cho po_token | ✅ Nên đảm bảo có Deno trong Docker |

### Đề xuất

1. **Tích hợp `bgutil-ytdlp-pot-provider`** vào Docker image → auto handle po_token
2. **Đảm bảo Deno runtime** có trong worker/api Docker image
3. **Giữ proxy rotation** hiện tại (đã rất tốt)
4. **Thêm monitoring** cho tỷ lệ bot-check để phát hiện sớm khi YouTube thay đổi

---

## 7. So sánh tổng hợp

| Giải pháp | Giảm BW | Complexity | Risk | Phù hợp |
|---|---|---|---|---|
| **A: Hybrid Flow** | 60-80% | 🟢 Thấp | 🟢 Thấp | ✅ **Best** |
| B: Client Mux | 0% (*) | 🔴 Cao | 🟡 TB | ⚠️ Niche |
| C: CF Workers | Extract only | 🟡 TB | 🟡 TOS | ⚠️ Partial |
| D: Cobalt Model | 100% mux (**) | 🟡 TB | 🟢 Thấp | ✅ Tốt |

(*) Client mux vẫn cần server proxy vì CORS → bandwidth không giảm  
(**) Cobalt model giảm 100% mux cost nhưng vẫn tốn bandwidth streaming

---

## 8. Đề xuất kiến trúc mới

### 8.1. Kiến trúc đề xuất: "Hybrid Tiered Download"

```
                         ┌──────────────────────────────────┐
                         │         Cloudflare / Reverse      │
                         │           Proxy Layer             │
                         └──────────┬───────────────────────┘
                                    │
                    ┌───────────────┴───────────────┐
                    │                               │
              ┌─────▼─────┐                  ┌──────▼──────┐
              │  API (Axum)│                  │  Frontend    │
              │            │                  │  (SvelteKit) │
              └─────┬──┬──┘                  └──────────────┘
                    │  │
         ┌──────────┘  └──────────────┐
         │                            │
    ┌────▼────┐              ┌────────▼────────┐
    │ Extract │              │ Stream Proxy    │ ← MỚI
    │ Service │              │ (lightweight)   │
    │ (yt-dlp)│              │ No disk, pipe   │
    └────┬────┘              └─────────────────┘
         │
    ┌────▼──────────────┐
    │ Decision Router   │ ← MỚI
    │  ≤720p → stream   │
    │  ≥1080p → mux job │
    └────┬──────────────┘
         │ (1080p+ only)
    ┌────▼──────────────┐
    │ Worker Pool (N)   │ ← Scale x3-5
    │ Redis Streams     │
    │ (concurrent mux)  │
    └────┬──────────────┘
         │
    ┌────▼──────────────┐
    │ S3 / Object Store │
    └───────────────────┘
```

### 8.2. Thay đổi cần làm (theo thứ tự ưu tiên)

#### Phase 1: Quick Wins (1-2 ngày) — Không cần thay đổi kiến trúc

| # | Thay đổi | File | Impact |
|---|---|---|---|
| 1 | Scale worker `--scale worker=3` | `docker-compose.server.yml` | Mux throughput x3 |
| 2 | Worker concurrent jobs (semaphore) | `worker/src/main.rs` | Mux throughput x3-5 per worker |
| 3 | Tăng DB pool 5 → 20-30 | `api/src/main.rs`, `worker/src/main.rs` | Tránh connection bottleneck |
| 4 | Restrict CORS | `api/src/main.rs` | Security |
| 5 | Tích hợp po_token plugin | `Dockerfile.worker`, `Dockerfile.api` | Tránh bot-check |

#### Phase 2: Stream Proxy (3-5 ngày) — Thay đổi kiến trúc nhỏ

| # | Thay đổi | Mô tả |
|---|---|---|
| 1 | Thêm `/api/stream-proxy` endpoint | Pipe stream từ YouTube CDN → client, không buffer disk |
| 2 | Decision Router trong extract response | Server trả thêm `download_strategy: "direct" | "mux_job"` |
| 3 | Frontend logic chọn luồng | ≤720p → stream proxy, ≥1080p → tạo mux job (đã có) |

#### Phase 3: Observability (2-3 ngày)

| # | Thay đổi | Mô tả |
|---|---|---|
| 1 | Prometheus metrics | Request latency, job processing time, proxy success rate |
| 2 | JSON structured logging | Dễ query trên Grafana/Loki |
| 3 | Deep health check | `/health` kiểm tra DB + Redis + S3 |
| 4 | Dashboard Grafana | Visualize metrics |

#### Phase 4: Advanced (tùy chọn, khi có đủ traffic)

| # | Thay đổi | Mô tả |
|---|---|---|
| 1 | Graceful shutdown | Worker drain jobs trước khi shutdown |
| 2 | Auto-scaling | Kubernetes/Docker Swarm auto-scale worker theo queue depth |
| 3 | CF Workers cho extract | Offload extract logic lên edge (nhẹ, nhanh) |
| 4 | WebCodecs mux option | Cho user chọn "mux trên browser" cho video ngắn |

---

## Tóm lại

> **Kiến trúc hiện tại CỦA BẠN đã rất tốt**. Rust backend, custom muxer, proxy pool, job system — đây là những thứ production-grade mà nhiều dự án lớn hơn không có.
> 
> **Thay đổi lớn nhất cần làm**: Thêm **streaming proxy mode** cho format ≤720p để giảm 60-80% workload. Đây là cách cobalt.tools, y2mate, và hầu hết các video download service lớn hoạt động.
>
> **KHÔNG nên**: Viết lại toàn bộ kiến trúc, chuyển hoàn toàn sang client-side mux, hoặc phụ thuộc Cloudflare Workers cho streaming. Kiến trúc hiện tại chỉ cần **bổ sung**, không cần **thay thế**.
