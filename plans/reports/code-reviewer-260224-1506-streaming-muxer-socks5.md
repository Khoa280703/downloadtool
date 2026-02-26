# Code Review: Streaming Muxer Refactor + SOCKS5 Routing

**Date:** 2026-02-24
**Scope:** atom_framer.rs, fragment_aligner.rs, fmp4_remuxer.rs, lib.rs, runtime.rs, Cargo.toml
**Build:** PASS (only pre-existing warnings, none in reviewed files)

---

## Overall Assessment

**Ready for testing** — no blocking bugs found. The refactor is sound and the streaming pipeline correctly prevents OOM. Several medium-priority issues need attention before production deployment.

---

## 1. AtomFramer (`atom_framer.rs`, 321 LOC)

### PASS — MP4 box header parsing
- 4-byte length + 4-byte type at lines 77-78: correct.
- `try_into().unwrap()` on slices already checked by `fill_to(8)` — no panic risk.

### PASS — Extended size boxes
- `size32 == 1` triggers `fill_to(16)` → reads `buf[8..16]` as u64. Correct per ISO 14496-12.
- `header_size` correctly set to 16 for extended boxes (line 96).

### PASS — Partial data / BytesMut buffering
- `fill_to(n)` pumps stream until `buf.len() >= n`. Correct sliding-buffer pattern.
- `split_to(total).freeze()` at line 143 yields exactly the right bytes and advances the buffer.

### PASS — Panic risks
- All `try_into().unwrap()` calls are guarded by prior `fill_to()` success checks.
- `unwrap_or("????")` on `from_utf8` is display-only in error messages — safe.

### WARN — `collect_init_segment` infinite loop on degenerate stream
- Line 154: the while loop exits only when both `ftyp` and `moov` are found. If the stream has 100k `sidx` boxes before `moov`, it silently buffers them all. Not a practical YouTube issue, but worth noting. The 256MB `MAX_NON_MDAT_BOX_SIZE` guard does not cover this case (the guard is per-box, not cumulative). **Low priority.**

### WARN — `size32 == 0` (EOF-terminated box) rejected for all box types
- Line 97-103: this returns `InvalidInput` immediately. Correct for streaming context. However if encountered in a non-`mdat` box (e.g. a `mdat` at end of a local file), it will break. Acceptable since this is a streaming-only path. Well-documented in the error message.

### WARN — `total_size as usize` on 64-bit extended boxes (line 125)
- A malformed box declaring `total_size = u64::MAX` passes the `MAX_NON_MDAT_BOX_SIZE` check only for `mdat`. For `mdat` boxes, no size guard is applied (by design to allow large media). `total_size as usize` would wrap on 32-bit platforms. Not an issue on 64-bit Linux, but defensively should be noted.

---

## 2. FragmentAligner (`fragment_aligner.rs`, 396 LOC)

### PASS — State machine phases
- No explicit state enum, but logical phases are clean: video-led loop (lines 80-188) transitions naturally to EOS flush (lines 191-221). Works correctly.

### PASS — moof+mdat pairing
- `FragmentReader::next_fragment()` enforces strict moof-then-mdat pairing (errors on mdat not immediately following moof). `FragmentAligner` relies on this guarantee correctly.

### PASS — EOS handling (audio flush after video ends)
- Lines 192-219: after `v_frag` is None, drains `audio_window` deque first, then drains remaining from `audio_reader`. Correct three-level flush.

### PASS — Memory: fragment drop after emit
- `drained: Vec<Fragment>` at line 173 is dropped at end of its scope after `merge_fragments`. No accumulation.
- `audio_window` is a bounded `VecDeque`; fragments are `drain`-ed on each emit.

### CRITICAL BUG — Timestamp normalization cross-multiply asymmetry

**Lines 91 and 121/137/166:**
```rust
// Video norm (for comparison purposes):
let v_tfdt_norm = v_frag.tfdt as u128 * self.ats;   // multiplied by ATS

// Audio norm (to compare against v_tfdt_norm):
let last_norm = last.tfdt as u128 * self.vts;         // multiplied by VTS
let a_norm   = a_frag.tfdt as u128 * self.vts;        // multiplied by VTS
```

The cross-multiply normalization is correct in principle (both values end up in units of `video_ticks * audio_ticks`). **However**, the drain predicate at line 166-168 uses the same asymmetric multiply which is mathematically equivalent. Let me verify:

- `v_tfdt_norm = v.tfdt * ats`  → unit: `v_ticks * ats`
- `a_norm = a.tfdt * vts`       → unit: `a_ticks * vts`

For these to be comparable: `v.tfdt / vts == a.tfdt / ats` iff `v.tfdt * ats == a.tfdt * vts`. Correct.

**Re-assessment: PASS** — the math is correct. Both sides are multiplied to common unit `ticks_V * ticks_A`. The asymmetry is intentional.

However there is a **real subtle bug**: the fill loop condition at line 123 uses `last_norm > v_tfdt_norm`, which means the loop breaks when the *last audio already in the window* exceeds the current video position. But when `audio_window` is **empty**, `self.audio_window.back()` returns `None` and the `if let` is skipped, causing an unconditional `audio_reader.next_fragment()` call. This is correct behavior — it will pull audio until it overshoots the current video tfdt. **No bug here.**

### HIGH — Lookahead comment noise is misleading
- Lines 93-111 contain a large comment block that describes abandoned approaches and ends with "For now, we'll use a simpler approach". This is dead comment, not dead code, but it creates confusion about the actual algorithm. Reader has to disregard ~20 lines of obsolete reasoning to understand what the code actually does.

### WARN — `MAX_AUDIO_WINDOW` warning is post-push
- Line 140: the warning fires *after* the fragment is already pushed. The window is not capped; it can grow unbounded if `MAX_AUDIO_WINDOW` is exceeded. This is intentional (warn but don't drop), but if audio is consistently 30+ fragments ahead of video, memory grows unbounded. Should be documented as a hard limit or handled with backpressure.

### WARN — Audio-only EOS fragments lose track_id patch
- Lines 194-199: flush from `audio_window` (already patched at push time) — OK.
- Lines 204-211: flush direct from `audio_reader` applies `patch_tfhd_track_id` but ignores error via `.ok()`. If patching fails, the fragment is emitted with **track_id=1** (original), which could confuse the player. Should propagate the error.

```rust
// Line 207 — current:
patch_tfhd_track_id(&mut moof, 2).ok();

// Should be:
if let Err(e) = patch_tfhd_track_id(&mut moof, 2) {
    return Some(Err(e));
}
```

### WARN — `patch_mfhd_sequence` error silently ignored in audio-only flush
- Line 195: `.ok()` on `patch_mfhd_sequence`. Player receives stale/wrong sequence numbers for tail audio fragments. Low impact (players tolerate this) but inconsistent with error handling elsewhere.

### INFO — LOC exceeds 200-line guideline
- `fragment_aligner.rs` is 396 lines, `atom_framer.rs` is 321 lines. Both exceed the project's 200-line guideline. The test code accounts for ~110 lines in each. Splitting tests into a `#[cfg(test)]` submodule file or using a test helper module could reduce main file size. Not a functional issue.

---

## 3. fmp4_remuxer.rs (102 LOC)

### PASS — Pipeline orchestration
- `remux_streams` is clean and correctly chains the two phases.
- `tokio::try_join!` for parallel init collection is a good choice.

### PASS — API surface unchanged
- `lib.rs` re-exports `remux_streams` and `MuxedStream` with same signatures. Callers are not broken.

### WARN — ftyp brand patch is unconditional
- Lines 71-77: patches brand `[8..12]` to `"isom"` regardless of original content. If brand is already `isom` or `mp42`, this is a no-op. If the `ftyp` box is exactly 8 bytes (no major brand payload), the `b.len() >= 12` check prevents a panic — good. But if a non-YouTube stream has a 12-byte ftyp with a meaningful brand (e.g. `avc1`), it silently overwrites it. Acceptable given the YouTube-only scope; would benefit from a comment stating the assumption.

### PASS — No full buffer accumulation
- `collect_stream` function no longer exists. Confirmed streaming-only path throughout.

---

## 4. SOCKS5 Routing (`runtime.rs`)

### PASS — Selective routing: youtube.com → SOCKS5, googlevideo.com → direct
- `should_use_socks5()` (lines 316-320): matches `youtube.com`, `*.youtube.com`, `youtu.be` only. `googlevideo.com` explicitly excluded (test at line 449 confirms).
- Comment at line 296 documents the CDN-direct rationale.

### PASS — `SOCKS5_PROXY_URL` is optional
- `build_fetch_client()` at line 302: uses `if let Ok(socks5_url) = std::env::var(...)`. If env var is absent, builder proceeds without proxy. Correct.

### PASS — No regression when env var unset
- Without `SOCKS5_PROXY_URL`, behavior is identical to pre-refactor. Build validated.

### PASS — Unit tests for `should_use_socks5`
- Lines 434-467: 7 test cases covering YouTube, www.YouTube, youtu.be, googlevideo (false), YouTube (false), unknown (false), invalid URL (false). Complete coverage.

### WARN — New `reqwest::Client` per fetch request
- `build_fetch_client()` is called on every `op_fetch` invocation (line 359). Each call creates a new `reqwest::Client`, which creates a new connection pool. For high-frequency extraction (multiple YouTube API calls per extraction), this prevents connection reuse (HTTP/1.1 keep-alive, HTTP/2 multiplexing). Not critical for a download tool but worth noting.

### WARN — SOCKS5 proxy receives auth credentials without TLS validation config
- `reqwest::Proxy::all()` at line 303 routes all traffic (including HTTPS) through the proxy. There's no `danger_accept_invalid_certs` or certificate pinning configured for the proxy connection itself. Acceptable for a home server setup but should be noted for production hardening.

### WARN — `body` field in `FetchResponse` is always `String`
- Line 388: `resp.text().await.unwrap_or_default()`. For binary responses, this corrupts the body. Acceptable because the extractor is only requesting HTML/JSON from YouTube, but could cause silent failures if an extractor endpoint returns binary data.

### INFO — Security domain whitelist includes `googlevideo.com`
- Line 269: `googlevideo.com` is in `ALLOWED_DOMAINS`. This allows JavaScript extractors to fetch CDN URLs directly. The comment explains this is for YouTube CDN. If an attacker controls the extractor JS bundle, they could exfiltrate data to `*.googlevideo.com`. Acceptable given trusted bundle, but worth documenting the trust boundary.

---

## 5. Cargo.toml (extractor)

### PASS — `socks` feature correctly added
- Line 21: `reqwest = { workspace = true, features = ["socks"] }`. Required for `reqwest::Proxy` SOCKS5 support. Correct.

### INFO — `notify = "6"` and `num_cpus = "1"` not in workspace
- These are defined locally, not in workspace `Cargo.toml`. Minor inconsistency with workspace dependency management pattern.

---

## Summary Table

| Area | Status | Notes |
|------|--------|-------|
| AtomFramer: box header parsing | PASS | |
| AtomFramer: extended size | PASS | |
| AtomFramer: partial data | PASS | |
| AtomFramer: panic risks | PASS | |
| FragmentAligner: state machine | PASS | |
| FragmentAligner: moof+mdat pairing | PASS | |
| FragmentAligner: timestamp normalization | PASS | Math is correct |
| FragmentAligner: EOS flush | PASS | |
| FragmentAligner: memory drop | PASS | |
| fmp4_remuxer: pipeline orchestration | PASS | |
| fmp4_remuxer: API unchanged | PASS | |
| SOCKS5: selective routing | PASS | |
| SOCKS5: env var optional | PASS | |
| SOCKS5: no regression | PASS | |
| Build | PASS | 1 pre-existing warning (dead_code in stream_fetcher) |

---

## Issues by Priority

### High
1. **Misleading lookahead comment block** (`fragment_aligner.rs` lines 93-111) — 20 lines of obsolete reasoning should be removed to prevent future misunderstanding of the algorithm.

### Medium
2. **Error silently ignored on audio-only EOS track_id patch** (`fragment_aligner.rs` line 207) — `patch_tfhd_track_id(...).ok()` should propagate errors, else tail audio fragments emit with wrong track_id.
3. **New HTTP client per fetch** (`runtime.rs` line 359) — defeats connection reuse; consider a cached/shared client per proxy configuration.

### Low
4. **`patch_mfhd_sequence` error silently swallowed** (`fragment_aligner.rs` line 195).
5. **`MAX_AUDIO_WINDOW` is a soft warn, not a hard cap** — can grow unbounded under pathological AV drift.
6. **`ftyp` brand patch is unconditional** — harmless in YouTube context but undocumented assumption.
7. **`notify` and `num_cpus` not in workspace deps** — minor Cargo inconsistency.

### Info / Style
8. Both new files exceed 200-line guideline. Tests could be moved to separate test files.
9. `ALLOWED_DOMAINS` security boundary trust model undocumented.

---

## Positive Observations

- `u128` for cross-multiply normalization prevents overflow for realistic timescales (90000 * 44100 * u64::MAX is safely within u128 range).
- `BytesMut` + `split_to().freeze()` pattern is zero-copy and correct.
- `into_remaining_stream()` correctly chains leftover buffer with original stream — no byte loss at phase boundary.
- `done` flag on both `AtomFramer` and `FragmentReader` prevents polling after error/EOS — correct fused-iterator behavior.
- `tokio::try_join!` for parallel init collection is a clean improvement.
- Test coverage is good: chunked delivery, extended size, multi-fragment alignment, video-only, EOS cases all covered.
- Domain whitelist in `op_fetch` is a solid security boundary for the JS sandbox.

---

## Unresolved Questions

1. Does `merge_fragments` handle the case where `v_moof` is shorter than 24 bytes (moof_hdr + mfhd)? Line 92 of `traf_merger.rs` does `v_moof[8..24]` without bounds check. If a malformed moof is shorter, this panics. Out of scope for this review but worth a follow-up.
2. `fragment_stream.rs` line 112-114: `moof_size` is read as `u32` with no 64-bit extended size support. Is a moof > 4GB possible in practice? Unlikely, but inconsistent with `AtomFramer`'s extended size support.
