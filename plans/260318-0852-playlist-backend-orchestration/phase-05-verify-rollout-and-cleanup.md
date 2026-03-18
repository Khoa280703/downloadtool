**Context Links**
- [plan.md](plan.md)
- [phase-04-migrate-frontend-to-backend-playlist-jobs.md](phase-04-migrate-frontend-to-backend-playlist-jobs.md)
- [README.md](/home/khoa2807/working-sources/downloadtool/README.md)
- [docs/system-architecture.md](/home/khoa2807/working-sources/downloadtool/docs/system-architecture.md)

**Overview**
- Priority: medium
- Status: proposed
- Brief: verify thật kỹ rồi mới xóa code cũ

**Key Insights**
- Playlist orchestration là luồng dễ regression nhất vì chạm API, worker, storage, frontend cùng lúc.
- Cần rollout từng bước, có feature flag nếu cần.

**Requirements**
- Verify single download không vỡ.
- Verify playlist lớn, playlist ngắn, cancel, retry, reload browser.
- Verify proxy behavior, progress realtime, artifact reuse.

**Architecture**
- Test matrix:
1. single direct stream
2. single mux job
3. playlist direct-only item
4. playlist mixed mux/direct items
5. cancel whole playlist
6. retry failed item
7. page reload + reconnect

**Related Code Files**
- Modify:
  - `README.md`
  - `docs/system-architecture.md`
  - `docs/codebase-summary.md`
- Create:
  - test cases if missing
- Delete:
  - legacy playlist orchestration code after stable

**Implementation Steps**
1. Add integration tests where possible.
2. Smoke test local and production-like env.
3. Update docs.
4. Remove dead code and unused runtime limits.

**Todo List**
- [ ] Add verification checklist
- [ ] Run frontend + backend checks
- [ ] Update docs
- [ ] Delete obsolete playlist worker code

**Success Criteria**
- Playlist backend flow stable trên local và production.
- Single flow unchanged.
- Docs phản ánh đúng kiến trúc mới.

**Risk Assessment**
- Cleanup quá sớm có thể mất fallback khi production bug.

**Security Considerations**
- Recheck access control cho artifact URLs và ticket TTL.

**Next Steps**
- Nếu ổn, cân nhắc admin controls cho playlist jobs sau.

**Unresolved questions**
- Có cần feature flag rollout giữa old playlist flow và new flow trong 1-2 deploy đầu không.
