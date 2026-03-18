**Context Links**
- [plan.md](plan.md)
- [phase-03-implement-worker-playlist-item-execution.md](phase-03-implement-worker-playlist-item-execution.md)
- [playlist-download-worker.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/playlist-download-worker.ts)
- [+page.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/routes/+page.svelte)
- [BatchProgress.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/components/BatchProgress.svelte)

**Overview**
- Priority: high
- Status: proposed
- Brief: bỏ client orchestration, giữ UI/UX playlist quen thuộc

**Key Insights**
- UI hiện đã có model tốt cho queue/progress, có thể reuse visual shell.
- Thứ cần bỏ là orchestration logic trong `playlist-download-worker.ts`, không phải toàn bộ UI.

**Requirements**
- Frontend tạo playlist job và subscribe SSE.
- Hiển thị progress aggregate + từng item.
- Download item khi artifact ready.
- Support cancel/retry item/job từ UI.
- Không đụng single download flow.

**Architecture**
- Replace `enqueueDownload` model bằng `createPlaylistJob`.
- `BatchProgress` nhận state từ server events, không từ local worker state.
- Khi item `ready`, UI dùng artifact/ticket URL để tải.

**Related Code Files**
- Modify:
  - `frontend/src/routes/+page.svelte`
  - `frontend/src/components/BatchProgress.svelte`
  - `frontend/src/stores/batch.ts`
  - `frontend/src/lib/api.ts`
- Create:
  - `frontend/src/lib/playlist-job-client.ts`
- Delete:
  - `frontend/src/lib/playlist-download-worker.ts` sau khi migration hoàn tất

**Implementation Steps**
1. Add playlist job API client.
2. Wire create/status/events/cancel/retry.
3. Migrate BatchProgress to server state.
4. Remove local orchestration path.

**Todo List**
- [ ] Add playlist job client
- [ ] Update page submit handlers
- [ ] Update progress UI bindings
- [ ] Remove old worker path

**Success Criteria**
- Reload trang vẫn có thể reconnect job theo `playlistJobId`.
- UI playlist không còn phụ thuộc active tab để orchestration tiếp tục.

**Risk Assessment**
- Nếu cắt local worker quá sớm sẽ mất fallback dev path.

**Security Considerations**
- Playlist job actions phải đi qua ownership/session checks.

**Next Steps**
- Verify rollout, cleanup, docs.

**Unresolved questions**
- Có muốn persist `playlistJobId` vào URL/query/localStorage ở phase đầu không.
