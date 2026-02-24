# Phase 01: Streaming Muxer Refactor

## Context Links
- Plan overview: `./plan.md`
- Current buffer-based implementation: `crates/muxer/src/fmp4_remuxer.rs`
- Existing fragment parser (REUSE): `crates/muxer/src/fragment_stream.rs` — `FragmentReader`
- Existing fragment merger (REUSE): `crates/muxer/src/traf_merger.rs` — `merge_fragments()`
- Existing moov merger (REUSE): `crates/muxer/src/moov_merger.rs` — `merge_moov()`
- Module registry: `crates/muxer/src/lib.rs`

## Overview

- **Priority:** P1 (blocking production stability)
- **Status:** pending
- **Problem:** `fmp4_remuxer.rs::remux_streams()` calls `collect_stream()` twice (lines 43-48), buffering the entire video + audio in RAM before any output. A 4K 2-hr video = ~5GB RAM per request.
- **Solution:** Replace the buffer-collect approach with a two-stage streaming pipeline. RAM usage drops to ~5-10MB per connection (one window of fragments).

## Key Insights

1. **`FragmentReader` already exists** (`fragment_stream.rs`) — reads moof+mdat pairs one at a time from a byte stream. No rewrite needed.
2. **`merge_fragments()` and `merge_moov()` are pure functions on `&[u8]`** — fully reusable in streaming context.
3. **Init segment (ftyp+moov) must be emitted first** — must wait for both init segments before streaming fragments. This is unavoidable; but moov boxes are small (~1-5KB), not the RAM problem.
4. **Fragment alignment window:** Only need to buffer ~1-2 video fragments worth of audio (~2-5 audio fragments). Not the full stream.
5. **Public API `remux_streams()` signature must not change** — callers in `mux_router.rs` and HTTP handlers depend on it.
6. **`ftyp` brand patch (dash→isom)** must be preserved — existing logic in `fmp4_remuxer.rs` lines 72-78.
7. **Audio `track_id` patch to 2** must be preserved — done per-fragment in current code lines 111-120.

## Requirements

### Functional
- `remux_streams(video, audio) -> MuxedStream` signature unchanged
- Output format unchanged: `ftyp → merged_moov → [moof{mfhd+traf_V+traf_A}][mdat{V+A}]...`
- Timestamp alignment logic unchanged (cross-multiply normalization, lines 143-154 of current `fmp4_remuxer.rs`)
- EOS handling: flush remaining audio fragments after video stream ends
- All existing tests in `fmp4_remuxer.rs`, `fragment_stream.rs`, `traf_merger.rs`, `moov_merger.rs` must still pass

### Non-Functional
- Peak RAM per muxer connection: ≤ 10MB (vs current ~5GB for 4K)
- No intermediate `Vec<u8>` accumulation of full streams
- Files ≤ 200 LOC each (split into `atom_framer.rs` + `fragment_aligner.rs`)

## Architecture

```
remux_streams(video_stream, audio_stream)
│
├── Phase INIT: Collect ftyp+moov from both streams (small, ~1-5KB each)
│   ├── AtomFramer(video_stream) → read until moov complete
│   ├── AtomFramer(audio_stream) → read until moov complete
│   ├── merge_moov(video_moov, audio_moov)     [existing fn, reuse]
│   ├── patch ftyp brand dash→isom             [existing logic, move here]
│   └── yield ftyp_bytes, yield merged_moov
│
└── Phase STREAM: Align and emit fragments on-the-fly
    ├── FragmentAligner {
    │     video_reader: FragmentReader<V>,      [existing struct, reuse]
    │     audio_reader: FragmentReader<A>,      [existing struct, reuse]
    │     audio_window: VecDeque<Fragment>,     [small buffer, ~2-5 fragments]
    │     seq: u32,
    │   }
    └── for each video fragment Vi:
          ├── drain audio_window of frags where tfdt_norm < next_v_norm
          ├── pull new audio frags until next_v_norm threshold
          ├── merge_fragments(Vi, audio_group, seq)   [existing fn, reuse]
          └── yield merged moof+mdat bytes
```

### New Files

**`crates/muxer/src/atom_framer.rs`** (~80 LOC)
- `struct AtomFramer<S>` wrapping a byte stream + `BytesMut` internal buffer
- `async fn read_box(&mut self) -> Option<Result<(BoxType, Bytes), MuxerError>>`
  - reads 8B header, handles extended size (length==1 → next 8B = u64 size)
  - fills internal buffer to `total_size`, returns owned `Bytes`
- `async fn collect_init_segment(&mut self) -> Result<(Bytes, Bytes), MuxerError>`
  - reads boxes until both `ftyp` and `moov` found; returns `(ftyp_bytes, moov_bytes)`
  - skips unknown boxes (e.g., `styp`, `sidx`)

**`crates/muxer/src/fragment_aligner.rs`** (~120 LOC)
- `struct FragmentAligner<V, A>` holding two `FragmentReader`s + `VecDeque<Fragment>` audio window
- `async fn next_merged(&mut self, seq: u32, vts: u128, ats: u128) -> Option<Result<Bytes, MuxerError>>`
  - drives the video-led alignment loop (logic from `fmp4_remuxer.rs` lines 156-199)
  - patches audio `track_id → 2` per fragment (from lines 111-120)
  - calls `merge_fragments()` from `traf_merger`

**`crates/muxer/src/fmp4_remuxer.rs`** (replace body, ~80 LOC)
- `pub fn remux_streams(...)` same signature, same `MuxedStream` return type
- Body becomes an `async_stream::try_stream!` with two phases:
  1. INIT: use `AtomFramer::collect_init_segment()` on both streams, emit ftyp+moov
  2. STREAM: create `FragmentAligner`, loop calling `next_merged()`, yield each result

## Related Code Files

**Modify:**
- `crates/muxer/src/fmp4_remuxer.rs` — replace `collect_stream` / buffer logic with streaming pipeline
- `crates/muxer/src/lib.rs` — add `pub mod atom_framer;` and `pub mod fragment_aligner;`

**Create:**
- `crates/muxer/src/atom_framer.rs`
- `crates/muxer/src/fragment_aligner.rs`

**Keep unchanged (reuse as-is):**
- `crates/muxer/src/fragment_stream.rs` — `FragmentReader`, `Fragment`
- `crates/muxer/src/traf_merger.rs` — `merge_fragments()`
- `crates/muxer/src/moov_merger.rs` — `merge_moov()`
- `crates/muxer/src/box_parser.rs` — all helpers

## Implementation Steps

### Step 1 — Create `atom_framer.rs`

Create `/home/khoa2807/working-sources/downloadtool/crates/muxer/src/atom_framer.rs`:

```rust
//! MP4 atom-level streaming reader.
//! Reads one complete box at a time from an async byte stream.
use crate::MuxerError;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use futures::{Stream, StreamExt};

pub struct AtomFramer<S> {
    stream: S,
    buf: BytesMut,
    done: bool,
}

impl<S, E> AtomFramer<S>
where
    E: std::error::Error + Send + Sync + 'static,
    S: Stream<Item = Result<Bytes, E>> + Unpin,
{
    pub fn new(stream: S) -> Self {
        Self { stream, buf: BytesMut::with_capacity(64 * 1024), done: false }
    }

    /// Fill buffer to at least `n` bytes. Returns false if stream ends first.
    async fn fill_to(&mut self, n: usize) -> Result<bool, MuxerError> { ... }

    /// Read the next complete box. Returns (box_type_4bytes, full_box_bytes).
    pub async fn read_box(&mut self) -> Option<Result<([u8; 4], Bytes), MuxerError>> { ... }
    // Handles extended size: if size_field == 1, read next 8 bytes as u64.

    /// Collect ftyp and moov boxes, skipping others (styp, sidx, etc.).
    /// Returns (ftyp_bytes, moov_bytes).
    pub async fn collect_init_segment(&mut self) -> Result<(Bytes, Bytes), MuxerError> {
        let (mut ftyp, mut moov) = (None, None);
        while ftyp.is_none() || moov.is_none() {
            match self.read_box().await {
                None => return Err(MuxerError::InvalidInput("Stream ended before init".into())),
                Some(Err(e)) => return Err(e),
                Some(Ok((box_type, data))) => match &box_type {
                    b"ftyp" => ftyp = Some(data),
                    b"moov" => moov = Some(data),
                    _ => {} // skip styp, sidx, etc.
                },
            }
        }
        Ok((ftyp.unwrap(), moov.unwrap()))
    }
}
```

Key details:
- Extended size: `if u32::from_be_bytes(header[0..4]) == 1 { read 8 more bytes as u64 total_size }`
- `fill_to` is identical pattern to `FragmentReader::fill_to` — copy that impl

### Step 2 — Create `fragment_aligner.rs`

Create `/home/khoa2807/working-sources/downloadtool/crates/muxer/src/fragment_aligner.rs`:

```rust
//! Video-led fragment alignment for streaming dual-track muxing.
use crate::fragment_stream::{Fragment, FragmentReader};
use crate::traf_merger::merge_fragments;
use crate::fmp4_remuxer::patch_tfhd_track_id;  // make pub(crate)
use crate::MuxerError;
use bytes::Bytes;
use futures::Stream;
use std::collections::VecDeque;

pub struct FragmentAligner<V, A> {
    video_reader: FragmentReader<V>,
    audio_reader: FragmentReader<A>,
    audio_window: VecDeque<Fragment>,  // small: ~2-5 audio frags at most
    audio_done: bool,
    next_video: Option<Fragment>,      // lookahead for boundary calc
}

impl<V, A, VE, AE> FragmentAligner<V, A>
where
    VE: std::error::Error + Send + Sync + 'static,
    AE: std::error::Error + Send + Sync + 'static,
    V: Stream<Item = Result<Bytes, VE>> + Unpin,
    A: Stream<Item = Result<Bytes, AE>> + Unpin,
{
    pub fn new(video: V, audio: A) -> Self { ... }

    /// Pull next merged fragment. Returns None when video stream exhausted + audio flushed.
    pub async fn next_merged(
        &mut self,
        seq: u32,
        vts: u128,  // video timescale
        ats: u128,  // audio timescale
    ) -> Option<Result<Bytes, MuxerError>> { ... }
}
```

Alignment algorithm (port from `fmp4_remuxer.rs` lines 143-199):
- `v_tfdt_norm = v_frag.tfdt as u128 * ats`
- `a_tfdt_norm = a_frag.tfdt as u128 * vts`
- Next video fragment is pre-fetched (lookahead) to determine `next_v_norm`
- Audio frags from `audio_window` where `a_tfdt_norm < next_v_norm` → form `audio_group`
- Pull more audio frags from `audio_reader` until `a_tfdt_norm >= next_v_norm` or audio done
- Call `merge_fragments(v_frag.moof, v_frag.mdat, audio_group, seq)` → yield `Bytes::from(result)`
- EOS: when `video_reader` exhausted → flush remaining `audio_window` + drain `audio_reader`

Audio track_id patch: before pushing to `audio_window`, call `patch_tfhd_track_id(&mut moof, 2)` — same as current lines 111-120. Move `patch_tfhd_track_id` to `pub(crate)` in `fmp4_remuxer.rs` or inline in `fragment_aligner.rs`.

### Step 3 — Rewrite `fmp4_remuxer.rs` body

Keep `pub fn remux_streams(...)` and `pub type MuxedStream`. Remove `collect_stream()` and `skip_init_bytes()`. New body:

```rust
pub fn remux_streams<V, A, VE, AE>(video: V, audio: A) -> MuxedStream
where ... // same bounds as today
{
    let video_mapped = video.map(|r| r.map_err(...));
    let audio_mapped = audio.map(|r| r.map_err(...));

    Box::pin(async_stream::try_stream! {
        // PHASE 1: collect init segments (small, ~1-5KB each)
        let mut v_framer = AtomFramer::new(video_mapped);
        let mut a_framer = AtomFramer::new(audio_mapped);

        let ((v_ftyp, v_moov), (_, a_moov)) = tokio::join!(
            v_framer.collect_init_segment(),
            a_framer.collect_init_segment(),
        );
        let (v_ftyp, v_moov) = v_ftyp?;  // ? propagates MuxerError
        let (_, a_moov) = (_, a_moov?);  // audio ftyp discarded

        let (merged_moov, video_timescale, audio_timescale) = merge_moov(&v_moov, &a_moov)?;

        // Patch ftyp brand dash→isom (preserve existing behaviour)
        let ftyp_patched = patch_ftyp_brand(v_ftyp);
        yield ftyp_patched;
        yield Bytes::from(merged_moov);

        // PHASE 2: stream-align fragments
        let vts = video_timescale as u128;
        let ats = audio_timescale as u128;

        // FragmentReader needs the remainder of the stream AFTER init boxes.
        // AtomFramer's internal buffer holds any bytes already read past moov.
        // Pass the framer's remaining stream into FragmentReader.
        let mut aligner = FragmentAligner::new(
            v_framer.into_remaining_stream(),
            a_framer.into_remaining_stream(),
        );

        let mut seq = 1u32;
        while let Some(result) = aligner.next_merged(seq, vts, ats).await {
            yield result?;
            seq += 1;
        }
    })
}
```

Note: `AtomFramer` must expose `into_remaining_stream()` that returns a stream combining the internal unconsumed buffer bytes + the original stream. Implement as a wrapper stream type or by converting `BytesMut` remainder into a `futures::stream::once(Ok(buf.freeze()))` chained with the original stream via `futures::stream::chain`.

### Step 4 — Update `lib.rs`

Add to `crates/muxer/src/lib.rs`:
```rust
pub mod atom_framer;
pub mod fragment_aligner;
```

### Step 5 — Run tests and verify compile

```bash
cd /home/khoa2807/working-sources/downloadtool
cargo test -p muxer 2>&1
cargo clippy -p muxer -- -D warnings 2>&1
```

All existing tests must pass. Add unit tests for:
- `AtomFramer::read_box` with extended size box
- `AtomFramer::collect_init_segment` skipping a `styp` box
- `FragmentAligner::next_merged` with 2 video frags + 3 audio frags

## Todo List

- [ ] Create `crates/muxer/src/atom_framer.rs` with `AtomFramer` struct
- [ ] Implement `fill_to`, `read_box` (including extended size), `collect_init_segment`
- [ ] Implement `into_remaining_stream()` on `AtomFramer` (chain buf remainder + stream)
- [ ] Create `crates/muxer/src/fragment_aligner.rs` with `FragmentAligner` struct
- [ ] Implement video-led alignment loop + EOS flush in `next_merged()`
- [ ] Move `patch_tfhd_track_id` to `pub(crate)` or inline in `fragment_aligner.rs`
- [ ] Rewrite `fmp4_remuxer.rs` body using `AtomFramer` + `FragmentAligner`
- [ ] Remove `collect_stream()` and `skip_init_bytes()` from `fmp4_remuxer.rs`
- [ ] Add `pub mod atom_framer; pub mod fragment_aligner;` to `lib.rs`
- [ ] Run `cargo test -p muxer` — all tests pass
- [ ] Run `cargo clippy -p muxer` — no warnings
- [ ] Manually verify RAM usage profile with a test stream

## Success Criteria

- `cargo test -p muxer` passes (all existing + new tests)
- `cargo clippy -p muxer -- -D warnings` clean
- No `collect_stream()` call exists in `fmp4_remuxer.rs` (no full-stream buffering)
- Peak RSS per concurrent muxer connection ≤ 10MB (verify with `valgrind --tool=massif` or `heaptrack` on a test binary)
- Output format identical to current: ftyp → moov → [moof+mdat]* sequence
- QuickTime/Safari compatibility preserved (dual-traf structure, ftyp isom brand)

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|---|---|---|---|
| `into_remaining_stream()` drops bytes already in `AtomFramer` buf | Medium | Silent data corruption | Unit test with exact byte boundaries; assert first moof offset |
| `tokio::join!` on init collection — error in one doesn't cancel other | Low | Hang | Use `tokio::try_join!` or manually `select!` with cancellation |
| Audio arrives much faster than video → large audio_window | Low | Moderate RAM use | Cap `audio_window` at 30 frags, log warning; unlikely for YouTube DASH |
| Extended-size box (length field == 1) in YouTube streams | Very Low | Parse failure | Handle in `AtomFramer::read_box`; YouTube DASH uses standard 32-bit sizes |

## Security Considerations

- No new network surface — same input streams, same output
- `BytesMut` internal buffer is bounded by one box at a time; no unbounded growth unless a malformed box declares enormous size → add sanity cap: `if total_size > 256 * 1024 * 1024 { return Err(...) }` for non-mdat boxes

## Next Steps

- After this phase, update `docs/system-architecture.md` to reflect streaming pipeline
- Phase 2 (SOCKS5) is independent and can be done in parallel
