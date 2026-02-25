# Docker Deploy Debug Report
**Date**: 25/02/2026
**Project**: downloadtool (Rust API + SvelteKit frontend + CUDA GPU worker)
**Target**: Coolify → homeserver, domain `download.khoadangbui.online`
**Khởi điểm**: Commit `262ab87` thêm Docker setup
**Tổng số fix commits**: 22
**Kết quả cuối**: ✅ DEPLOY THÀNH CÔNG — 3 containers running (04:11 UTC)

---

## Kết quả deploy cuối (commit `4964f9a`)

```
Container gpu-worker  ✅ Started
Container api         ✅ Started
Container frontend    ✅ Started
New container started.
```

---

## Tổng quan các nhóm lỗi

| Nhóm | Service | Commits | Status |
|------|---------|---------|--------|
| 1 | frontend — vite build fail | 9 | ✅ Fixed (b192ae1) |
| 2 | git — *.ts gitignore | 1 | ✅ Fixed (74e943e) |
| 3 | api — binary name + Rust version | 2 | ✅ Fixed |
| 4 | api — injector JS embed | 1 | ✅ Fixed (738f65a) |
| 5 | api — extractor scripts COPY | 1 | ✅ Fixed (913a548) |
| 6 | gpu-worker — binary + feature flag | 2 | ✅ Fixed |
| 7 | gpu-worker — FFmpeg system libs | 2 | ✅ Fixed |
| 8 | gpu-worker — FFmpeg 7.x mismatch | 1 | ⚠️ Workaround (disabled --features gpu) |

---

## Chi tiết từng commit

### `262ab87` — feat(deploy): add homeserver docker compose
Thêm `Dockerfile.frontend`, `Dockerfile.homeserver`, `Dockerfile.vps`, `docker-compose.homeserver.yml`.
**Vấn đề**: Nhiều tên binary, feature flags, thư viện, build steps sai → chuỗi 22 commits fix.

---

### `4ba8b02` — fix: context: . in compose files
**Lỗi**: Coolify không tìm được Dockerfile — `context:` path sai.
**Fix**: Đổi tất cả `context:` về `.` (root workspace).

---

### `88723f3` — fix: track Cargo.lock
**Lỗi**: Build không reproducible.
**Fix**: Bỏ `Cargo.lock` khỏi gitignore.

---

### `1e9be71` — fix: allow esbuild install scripts in pnpm
**Lỗi**: pnpm v10 strict mode block postinstall scripts của esbuild.
**Fix**: Thêm `onlyBuiltDependencies: [esbuild]` vào `.npmrc`.

---

### `2fbf1c9` — fix: copy frontend source before pnpm install
**Lỗi**: `svelte-kit sync` không tìm thấy `svelte.config.js`.
**Fix**: COPY source trước rồi mới `pnpm install`.

---

### `b12ea91` — fix: cd into frontend dir before build
**Lỗi**: `pnpm build` chạy từ root, không tìm thấy config frontend.
**Fix**: `WORKDIR /app/frontend && pnpm build`.

---

### `0d7ba5a` — fix: use explicit binary paths
**Lỗi**: pnpm workspace binary resolve không đúng.
**Fix**: Dùng explicit `./node_modules/.bin/svelte-kit` và `./node_modules/.bin/vite`.

---

### `f3aa086` — fix: use local node_modules/.bin
**Lỗi**: Binary không tìm thấy sau khi đổi WORKDIR.
**Fix**: Đảm bảo WORKDIR = `/app/frontend`.

---

### `591f513` — fix: switch to npm install
**Lỗi**: pnpm v10 tiếp tục block esbuild binary.
**Fix**: Chuyển sang `npm install`.

---

### `51be96b` — fix: build frontend standalone
**Lỗi**: Vẫn "0 modules transformed".
**Fix thử**: Tách frontend khỏi pnpm workspace. → Không giải quyết root cause.

---

### `c995f7a` — fix: downgrade vite@7→6
**Lỗi**: Vẫn "0 modules transformed".
**Fix thử**: Downgrade `vite@7→6`, `@sveltejs/vite-plugin-svelte@6→5`. → Sai hướng.

---

### `cfd287b` — fix: restructure Dockerfile + Rust 1.82→1.85
**Lỗi 1**: Vẫn "0 modules transformed".
**Lỗi 2 MỚI**: `time-core-0.1.8` requires Rust edition 2024, `rust:1.82` không hỗ trợ.
**Fix**: Restructure copy order; nâng Rust `1.82→1.85`.

---

### `b192ae1` — **fix: bypass vite config loading** ✅ FRONTEND FIXED
**Root cause phát hiện**: Vite dùng esbuild để bundle `vite.config.ts`. Trên **Alpine Linux (musl libc)**, esbuild native binary fail **silently** → config rỗng → SvelteKit plugin không load → "0 modules transformed".
**Fix**: Tạo `frontend/build-docker.mjs` với `configFile: false` + inject plugin programmatically.
**Kết quả**: ✅ 123 modules transformed.
**Lỗi mới**: `ENOENT: analytics.ts` — TypeScript files missing.

---

### `74e943e` — fix: remove *.ts from gitignore ✅
**Root cause**: `.gitignore` có dòng `*.ts` (vốn cho video MPEG-2 TS files) vô tình **exclude toàn bộ TypeScript source** (29 files).
**Fix**: Xóa `*.ts`, add lại 29 TS files.

---

### `87ab6d3` — fix build (hotfix nhỏ)

---

### `56f9304` — fix: correct binary names ✅
**Lỗi**: `no bin target 'gpu-worker-server'`, `no bin target 'api-server'`.
**Root cause**: Tên binary trong Dockerfiles khác với `Cargo.toml`:
- `gpu-worker-server` → `gpu-node`
- `api-server` → `vps-gateway`
**Fix**: Sửa tên binary + COPY + CMD.

---

### `e51c065` — fix: Rust 1.85→1.88 ✅
**Lỗi**: `time@0.3.47 requires rustc 1.88.0`.
**Fix**: `rust:1.85-bookworm` → `rust:1.88-bookworm`.

---

### `d2250c6` — fix: feature flag gpu-support→gpu ✅
**Lỗi**: `unknown feature 'gpu-support'`.
**Root cause**: Feature tên là `gpu` trong `Cargo.toml`, không phải `gpu-support`.
**Fix**: `--features gpu-support` → `--features gpu`.

---

### `0f4bfc6` — fix: add libavfilter-dev, libavdevice-dev ✅
**Lỗi**: `The system library 'libavfilter' required by crate 'ffmpeg-sys-next' was not found`.
**Root cause**: Dockerfile.homeserver thiếu `libavfilter-dev` và `libavdevice-dev`.
**Fix**: Thêm vào builder stage + `libavfilter7` + `libavdevice58` vào runtime.

---

### `738f65a` — fix: js-builder stage for injector in Dockerfile.vps ✅
**Lỗi**:
```
error: couldn't read /app/apps/injector/dist/bm.js: No such file
error: couldn't read /app/apps/injector/dist/youtube-downloader.user.js: No such file
```
**Root cause**: `crates/api/src/routes/static_files.rs` dùng `include_str!()` embed 2 file JS lúc compile time. Đây là build artifact của `apps/injector/`, không có trong git, và `Dockerfile.vps` không build injector trước.
**Fix**: Thêm `js-builder` stage: Node.js → `pnpm --filter @downloadtool/injector build` → copy `dist/` vào Rust builder.

---

### `990e02f` — fix: add clang + libclang-dev ✅
**Lỗi**: `Unable to find libclang`.
**Root cause**: `ffmpeg-sys-next` dùng `bindgen` generate Rust FFI bindings từ C headers — cần `libclang`. CUDA image không có sẵn.
**Fix**: Thêm `clang` + `libclang-dev` vào Dockerfile.homeserver builder.

---

### `913a548` — fix: remove COPY extractor scripts ✅
**Lỗi**: `failed to compute cache key: "/app/crates/extractor/scripts": not found`.
**Root cause**: `COPY --from=builder ... || true` — `|| true` không có tác dụng trong Dockerfile `COPY` instruction. Directory không tồn tại trong repo.
**Fix**: Xóa dòng COPY.

---

### `4964f9a` — fix: disable --features gpu (FFmpeg version mismatch) ⚠️ WORKAROUND
**Lỗi**:
```
error[E0433]: failed to resolve: use of unresolved module 'async_stream'
error[E0425]: cannot find value 'AVHWDeviceType_AV_HWDEVICE_TYPE_CUDA' in 'ffmpeg_next::ffi'
error[E0308]: mismatched types (AV_PIX_FMT_CUDA)
```
**Root cause**: `gpu-pipeline/Cargo.toml` có `ffmpeg-next = { version = "7" }` — crate version 7 yêu cầu **FFmpeg 7.x** được cài trên hệ thống. Nhưng Ubuntu 22.04 apt chỉ có **FFmpeg 4.4.x** → API không khớp → CUDA constants không tồn tại trong bindgen output. Ngoài ra thiếu dep `async-stream` trong `gpu-pipeline`.
**Workaround**: Bỏ `--features gpu` → code GPU-specific bị `cfg`-gate, không compile. Binary vẫn chạy nhưng không có hardware acceleration.
**Kết quả**: ✅ BUILD THÀNH CÔNG. 3 containers deployed.

---

## Trạng thái hiện tại

| Service | Tình trạng | Ghi chú |
|---------|-----------|---------|
| `frontend` | ✅ Running | SvelteKit Node server |
| `api` (vps-gateway) | ✅ Running | Rust API, port 3068 |
| `gpu-worker` (gpu-node) | ✅ Running | **Không có GPU acceleration** |

---

## Việc cần làm để restore GPU features

1. **Thêm `async-stream`** vào `gpu-pipeline/Cargo.toml`
2. **Nâng FFmpeg**: Chọn một trong:
   - Đổi base image sang `nvidia/cuda:12.6.0-devel-ubuntu24.04` (có FFmpeg 6.x) + đổi `ffmpeg-next = "7"` → `"6"` + update API calls
   - Build FFmpeg 7 từ source trong Dockerfile.homeserver (build time rất lâu)
3. **Re-enable** `--features gpu` sau khi FFmpeg version match

---

## 3 nguyên nhân gốc rễ của toàn bộ

| # | Nguyên nhân | Ảnh hưởng |
|---|-------------|-----------|
| 1 | **esbuild/Alpine musl incompatibility** — Vite config loading fail silent | 9 commits frontend |
| 2 | **`*.ts` glob trong .gitignore** — Pattern video file exclude TypeScript source | 1 commit, 29 files missing |
| 3 | **Dockerfiles viết không verify với codebase** — Binary names, feature flags, deps, build steps đều sai | 12 commits Rust/infra |

---

## Unresolved questions

1. GPU acceleration: cần upgrade FFmpeg hoặc thay base image — plan riêng cần thiết
2. `async-stream` crate chưa được add vào `gpu-pipeline/Cargo.toml` — cần add khi enable `--features gpu` trở lại
3. Traefik routing (`/api/*` → api, `/*` → frontend) cần verify hoạt động đúng sau deploy

---

*Last updated: 2026-02-25 11:20*
