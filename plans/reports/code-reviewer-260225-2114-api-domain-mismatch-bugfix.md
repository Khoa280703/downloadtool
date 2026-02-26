## Code Review Summary

### Scope
- Files:
  - /home/khoa2807/working-sources/downloadtool/frontend/src/components/BatchInput.svelte
  - /home/khoa2807/working-sources/downloadtool/frontend/src/lib/download-pool.ts
  - /home/khoa2807/working-sources/downloadtool/docker/Dockerfile.frontend
  - /home/khoa2807/working-sources/downloadtool/docker/docker-compose.homeserver.yml
- LOC reviewed (approx): 30 lines changed
- Focus: recent bugfix patch
- Scout findings: API base-path usage in batch SSE + stream URL was inconsistent before; Docker runtime env for Vite public var was ineffective

### Overall Assessment
Patch đi đúng hướng và **khả năng cao đã fix đúng lỗi production 404 do API domain mismatch**. Cách chuyển `VITE_API_URL` sang build-arg là đúng bản chất Vite (`import.meta.env` được embed lúc build), và `BatchInput` đã đổi từ relative `/api/*` sang helper dùng `API_BASE`.

### Critical Issues
- Không thấy issue mức Critical trong phạm vi patch này.

### High Priority
- Không thấy issue mức High rõ ràng.

### Medium Priority
1. Hardcode domain trong compose build arg làm giảm portability
   - File: /home/khoa2807/working-sources/downloadtool/docker/docker-compose.homeserver.yml
   - Hiện tại: `VITE_API_URL=https://api-download.khoadangbui.online`
   - Tác động: khi deploy môi trường khác dễ quên sửa, tạo regression dạng “deploy thành công nhưng frontend gọi sai API”.

2. Duplicated concern/overlap về `subscribeBatch`
   - File: /home/khoa2807/working-sources/downloadtool/frontend/src/lib/download-pool.ts
   - Vẫn còn `subscribeBatch()` trong download-pool nhưng `BatchInput` đã dùng `subscribeBatch` từ `/frontend/src/lib/api.ts`.
   - Tác động: tăng cognitive load, dễ gây drift logic SSE về sau (không DRY).

### Low Priority
1. Callback semantics có thể gây hiểu nhầm
   - File: /home/khoa2807/working-sources/downloadtool/frontend/src/components/BatchInput.svelte
   - `onComplete?.()` được gọi cả ở nhánh lỗi kết nối. Nếu consumer coi `onComplete` là “thành công”, có thể phát sinh behavior không mong muốn.

### Edge Cases Found by Scout
- Boundary: thiếu `VITE_API_URL` ở build hiện đã fail-fast bằng `RUN test -n "$VITE_API_URL"` (tốt).
- Data-flow: `BatchInput` đã dùng `buildStreamUrl()` nên stream URL đồng nhất domain với `extract`/`batch` APIs.
- State/async: `subscribeBatch` trong `/frontend/src/lib/api.ts` tự `close()` ở `onerror`; `BatchInput` không close lần nữa trong callback lỗi (an toàn).
- Payload guard: `BatchInput` đã check null/undefined cho link payload trước khi enqueue download (giảm crash runtime).

### Positive Observations
- Fix đúng trọng tâm bug (Vite env timing + relative URL).
- Có guard payload tốt hơn ở event `link`.
- Tuân KISS: thay đổi nhỏ, đúng điểm lỗi.

### Recommended Actions
1. Giữ patch hiện tại để unblock production (verdict: pass với lưu ý).
2. Sau đó chuẩn hóa `VITE_API_URL` qua variable substitution trong compose/.env deploy pipeline để tránh hardcode domain.
3. Dọn/merge logic `subscribeBatch` để tránh trùng trách nhiệm giữa `api.ts` và `download-pool.ts`.

### Metrics
- Type Coverage: N/A (chưa chạy type-check trong review này)
- Test Coverage: N/A (không có test run trong phạm vi review)
- Linting Issues: N/A (không chạy lint trong phạm vi review)

### Unresolved Questions
- `onComplete` có nghĩa là “kết thúc bất kể success/fail” hay “hoàn tất thành công”? Cần contract rõ để tránh regression UI state.
- Hệ thống deploy có cơ chế inject `VITE_API_URL` theo môi trường (staging/prod) chưa, hay còn phụ thuộc sửa tay compose?