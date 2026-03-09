---
title: "Remove Sync Mux Route"
description: "Delete `/api/stream/muxed` and keep `/api/jobs/*` as the only mux architecture."
status: pending
priority: P1
effort: 3h
branch: main
tags: [api, frontend, worker, cleanup]
created: 2026-03-09
---

# Goal

Xoa han sync mux `/api/stream/muxed`. Giu `GET /api/stream` cho direct stream thuong. Tat ca luong can mux phai di qua durable `/api/jobs/*`.

# Runtime Files Can Sua

- `crates/api/src/main.rs`
  Xoa route registration cu, giu router chi con `/api/stream` va `/api/jobs/*`.
- `crates/api/src/routes/stream.rs`
  Xoa toan bo handler/query/helper cua sync mux; file nay chi con stream proxy thuong.
- `frontend/src/components/DownloadBtn.svelte`
  Chot rule: neu can mux thi tao job + poll ready; neu khong can mux thi goi direct `/api/stream`.
- `frontend/src/lib/api.ts`
  Giu 2 nhom API ro rang: direct stream va mux jobs. Khong de helper nao tro lai `/api/stream/muxed`.
- `frontend/src/routes/download/mux-job/+page.svelte`
  Launcher canonical cho bookmarklet/userscript/extension de tai qua auth + BFF.
- `apps/injector/src/shared/stream-utils.ts`
  Doi het mux launch URL sang app launcher, khong build sync mux URL nua.
- `apps/extension/src/shared/stream-utils.ts`
  Cung logic nhu injector; popup/background mo launcher page thay vi tai truc tiep.

# Docs Scripts Config Can Don

- `frontend/vite.config.ts`
  Xoa proxy dev cho `/api/stream/muxed`.
- `scripts/stress-test-all-endpoints.sh`
- `scripts/stress-test-comprehensive-suite.sh`
- `scripts/stress-test-refresh-probe.sh`
- `scripts/stress-test-muxed-only.sh`
- `scripts/stress-test-muxed-jobs.mjs`
  Doi naming/assertion cho jobs-only, bo case sync mux neu con.
- `config/runtime-limit-profiles.json`
  Giu `frontend.mux_job_*`; xoa moi guard/runtime key chi danh cho sync mux neu con sot lai.
- `docs/codebase-summary.md`
- `docs/system-architecture.md`
- `docs/project-roadmap.md`
- `docs/project-overview-pdr.md`
- `docs/code-standards.md`
  Cap nhat tai lieu theo 1 kien truc mux duy nhat.

# Gaps Can Chan Truoc Khi Cat Hoan Toan

- `crates/api/src/services/job_control_plane.rs`
- `crates/job-system/src/repository_create.rs`
  Can chan case publish Redis fail lam job reuse bi kẹt `queued` vinh vien. Neu row da ton tai ma chua duoc enqueue thanh cong, flow tao job phai co co che republish/an toan retry.
- `scripts/stress-test-muxed-jobs.mjs`
- `scripts/stress-test-comprehensive-suite.sh`
  Scripts load test jobs hien goi thang `/api/jobs/*` khong auth; can doi sang cookie/BFF flow hoac truyen JWT hop le, neu khong ket qua sau cutover chi la `401`.
- `frontend/src/components/DownloadBtn.svelte`
  Main app can xu ly `401` ro rang cho mux jobs: redirect login hoac thong diep xac thuc, khong duoc nuot thanh loi chung.
- `crates/api/src/routes/jobs.rs`
- `frontend/src/routes/download/mux-job/+page.svelte`
  Chot lai semantics `release`: chi la access hint, hay thuc su rut ngan TTL/mark cleanup. Jobs-only nen co nghia cleanup ro rang.

# Thu Tu Trien Khai An Toan Nhat

1. Gia co jobs pipeline truoc.
   Fix queue republish, auth UX, load-test auth, va release semantics de jobs path du khoe khi tro thanh duong mux duy nhat.
2. Chuyen tat ca caller sang jobs truoc.
   Frontend, launcher, injector, extension, scripts test phai het build URL `/api/stream/muxed` truoc khi cat route.
3. Cat runtime sync mux trong API.
   Xoa route o `main.rs`, xoa handler/helper o `routes/stream.rs`, bo dependency/import khong con dung.
4. Don dev proxy va config.
   Xoa vite proxy cu, xac nhan `runtime-limit-profiles.json` chi con bien cho direct stream va mux jobs.
5. Don docs va stress scripts.
   Sua mo ta architecture, endpoint inventory, roadmap, test scripts de phan anh jobs-only.
6. Verify bang grep + build + test.
   `rg "/api/stream/muxed|buildMuxedUrl"` phai sach trong runtime active.
   Chay `cargo check`, `cargo test --workspace`, `pnpm --filter frontend check`, build injector/extension.

# Done Khi

- Runtime active khong con route, helper, proxy, hay client nao goi `/api/stream/muxed`.
- External channels chi mo `/download/mux-job` de vao durable flow.
- Docs/config/scripts active mo ta dung 1 kien truc mux duy nhat.
