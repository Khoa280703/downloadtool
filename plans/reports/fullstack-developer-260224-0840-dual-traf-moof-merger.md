# Phase Implementation Report

### Executed Phase
- Phase: dual-traf-moof-merger
- Plan: none (direct implementation request)
- Status: completed

### Files Modified
- `crates/muxer/src/traf_merger.rs` — NEW, 415 lines: dual-traf merger logic
- `crates/muxer/src/fmp4_remuxer.rs` — MODIFIED, 265 lines: replaced interleave loop with video-led grouping
- `crates/muxer/src/lib.rs` — MODIFIED: added `pub mod traf_merger`

### Tasks Completed
- [x] Created `traf_merger.rs` with `merge_fragments()` public API
- [x] `extract_traf_bytes()`: extracts traf bytes from a moof (skips 8B header + 16B mfhd)
- [x] `patch_trun_data_offset()`: patches `trun.data_offset` inside a traf box; iterates traf children correctly by skipping outer traf header
- [x] `patch_mfhd_sequence_number()`: fast-path mfhd sequence patch (borrow-safe: collects offset before writing)
- [x] `merge_fragments()`: builds merged `[moof{mfhd+traf_V+traf_A...}][mdat{V+A...}]`; correctly computes cumulative audio data offsets
- [x] Updated `fmp4_remuxer.rs`: collects all video+audio fragments, then video-led grouping emits merged dual-traf fragments
- [x] Audio track_id patched to 2 before grouping (single-pass, not per-emission)
- [x] Remaining audio after all video emitted as-is with sequential seq numbers
- [x] Registered `pub mod traf_merger` in `lib.rs`
- [x] Fixed compile error: borrow conflict in `patch_mfhd_sequence_number` (iterator dropped before mutable write)
- [x] Fixed test failure: `patch_trun_data_offset` was iterating traf as a top-level box (seeing `traf` type, jumping to end); fixed by skipping outer traf header to descend into children

### Tests Status
- Type check: pass
- Unit tests: 33/33 pass (0 failed)
  - `traf_merger::tests::test_merge_fragments_empty_audio` — ok
  - `traf_merger::tests::test_merge_fragments_with_audio` — ok (1 moof, 1 mdat, 2 traf children)
  - `traf_merger::tests::test_data_offset_patching` — ok (video offset = moof_size+8, audio offset = moof_size+8+v_payload)
  - `fmp4_remuxer::tests::test_remux_streams_basic` — ok
  - All 30 pre-existing tests — ok

### Key Design Decisions
- `extract_traf_bytes` returns `&content[traf_offset..]` — includes all traf boxes from first traf to end of moof content. For YouTube DASH (single traf per moof) this equals exactly one traf box.
- `patch_trun_data_offset` handles multiple trun boxes (for compatibility) but YouTube DASH has one trun per traf.
- Video-led grouping uses u128 cross-multiplication for float-free tfdt comparison.
- Empty audio group → fast-path (no structural changes, just mfhd seq patch).

### Issues Encountered
1. Borrow checker: `iter_boxes(&moof[content_start..])` held immutable borrow preventing mutable write → fixed by collecting offset with `.find().map()` (temporary drops before write).
2. `patch_trun_data_offset` logic bug: was calling `read_box_header` on the traf buffer at `pos=0`, getting `box_type=traf`, advancing `pos = traf.total_size` → loop ended without visiting children → fixed by computing `content_start` from outer traf header and starting iteration there.

### Next Steps
- Integration test with real YouTube DASH fMP4 files to verify QuickTime duration display
- The moov_merger.rs duration patches (mvhd/tkhd/mehd) remain intact and complement this fix
