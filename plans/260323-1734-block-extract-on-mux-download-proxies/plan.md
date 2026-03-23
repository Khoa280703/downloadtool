---
title: "Block extract on mux-download-busy proxies"
description: "Plan toi thieu de khong cho extract chon proxy dang duoc worker dung de fetch/download media."
status: completed
priority: P1
effort: 1.5h
branch: main
tags: [proxy, worker, extract, mux]
created: 2026-03-23
---

## Scope

Muc tieu: khi worker dang dung proxy cho mux fetch/download, extract tam thoi khong duoc chon lai proxy do.

Khong lam trong scope nay:
- Khong doi env.
- Khong them config moi vao `runtime-limit-profiles.json`.
- Khong redesign shared-state toan he thong.

## Thay doi toi thieu de xuat

1. Them busy-state cho "download in flight" trong `ProxyPool`.
2. Khi worker vao doan fetch/download media, acquire mot lease tren proxy video/audio; release ngay sau khi stream YouTube duoc doc xong, khong giu qua buoc finalize upload.
3. Sua logic chon proxy cho extract de bo qua proxy dang co download lease.
4. Them test unit cho 2 invariant:
   - extract skip proxy dang download
   - preferred proxy cung bi tu choi neu dang download

## File can sua

- `crates/proxy/src/proxy_pool.rs`
  - Them counter/lease cho download-busy state.
  - Sua `try_acquire_next_owned()` va `try_acquire_preferred_owned()` de skip proxy dang busy.
  - Them test cho invariant moi.
- `crates/worker/src/mux_pipeline.rs`
  - Acquire/release download lease bao quanh doan `fetch_both_with_refresh_and_proxy(...)`.

## Ly do cach nay la toi thieu

- Dung lai abstraction san co: `ProxyPool`, `ProxyLease`, worker fetch pipeline.
- Khong dong vao anti-bot flow, DB schema, Redis schema, env, hay routing policy.
- Giam pressure len pool ma it nguy co regression nhat.

## Rui ro chinh

1. Process-local only:
   - Neu `api` va `worker` la 2 process/container rieng, busy-state in-memory khong tu dong dong bo giua hai ben.
   - Ban fix toi thieu nay chi chac chan trong pham vi cung process/runtime pool.
2. Race condition:
   - Neu download-busy check va extract-slot acquire khong duoc thiet ke thanh mot invariant chat, co the van co cua so race nho.
3. Over-blocking tam thoi:
   - Neu lease scope giu qua dai, extract pool bi giam proxy kha dung khong can thiet.

## Cach verify

1. Unit tests:
   - Proxy dang co download lease => `try_acquire_next_owned()` khong lay trung.
   - Preferred proxy dang download => `try_acquire_preferred_owned()` tra `None`.
2. Build/test:
   - `cargo check -p proxy -p worker`
   - `cargo test -p proxy proxy_pool::tests::`
3. Manual log check:
   - Chay nhieu playlist dong thoi.
   - Xac nhan log extract khong reuse proxy dang duoc worker dung de fetch media trong cung runtime.
4. Regression watch:
   - Theo doi xem throughput extract co giam qua manh khi so proxy it.

## Tieu chi done

- Extract bo qua proxy dang duoc mux fetch/download giu.
- Download lease roi ngay sau EOF cua upstream stream, khong giu qua `complete_multipart_upload`.
- Khong can them env/config moi.
- `proxy` va `worker` build/test pass.

## Unresolved questions

- Co can giai bai toan cross-container (`api`/`worker`) ngay bay gio khong, hay chap nhan fix toi thieu process-local truoc?
