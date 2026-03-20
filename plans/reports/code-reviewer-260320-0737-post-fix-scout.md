# Code Review: Post-Fix Scout

**Date:** 2026-03-20
**Reviewer:** code-reviewer
**Scope:** SSRF fix, playlist mode fix, CI frontend job, docs sync

---

## Files Checked

| File | LOC | Status |
|------|-----|--------|
| `crates/api/src/validation.rs` | 128 | New — SSRF validation |
| `crates/api/src/routes/extract.rs` | 373 | Uses `validation::is_valid_youtube_url` |
| `crates/api/src/routes/playlist_jobs.rs` | 563 | Uses `validation::is_valid_youtube_url` |
| `crates/api/src/services/playlist_processor.rs` | 695 | Mode-aware stream selection |
| `crates/api/src/main.rs` | 497 | `mod validation` declared |
| `crates/api/src/routes/batch.rs` | ~213 | **SSRF gap found** |
| `crates/api/src/routes/jobs.rs` | ~540 | Uses `proxy::client::validate_stream_url` |
| `crates/api/src/routes/stream.rs` | ~650 | Uses `proxy::client::validate_stream_url` |
| `crates/proxy/src/client.rs` | ~230 | `validate_stream_url` — host allowlist |
| `.github/workflows/ci.yml` | 212 | Frontend job added |
| `docs/` (4 files) | — | Phantom ref check |

---

## Critical Issues

### [CRITICAL] SSRF in `/api/batch` — `is_valid_batch_url()` still uses substring match

**File:** `crates/api/src/routes/batch.rs:189-192`

```rust
fn is_valid_batch_url(url: &str) -> bool {
    let lower = url.to_lowercase();
    (lower.contains("youtube.com") || lower.contains("youtu.be")) && lower.contains("list=")
}
```

This is the exact same vulnerability pattern the SSRF fix in `validation.rs` was meant to eliminate. An attacker can craft:
- `https://evil.tld/?list=abc&next=youtube.com` — passes both checks
- `https://youtube.com.evil.tld/playlist?list=abc` — passes both checks

**Impact:** The URL is passed to `extractor::extract_playlist()` which calls yt-dlp against arbitrary hosts.

**Mitigation:** Although `discover_playlist_items()` reconstructs a clean YouTube URL from the extracted playlist ID (`format!("https://www.youtube.com/playlist?list={playlist_id}")`), the `is_valid_batch_url` check is still misleading and inconsistent. The batch endpoint should use `crate::validation::is_valid_youtube_url` for consistency. The playlist ID extraction fallback also uses substring matching (`url.find("list=")`) which could be exploited independently.

**Fix:** Replace `is_valid_batch_url` with `crate::validation::is_valid_youtube_url` + check for `list=` param:
```rust
fn is_valid_batch_url(url: &str) -> bool {
    if !crate::validation::is_valid_youtube_url(url) {
        return false;
    }
    let Ok(parsed) = reqwest::Url::parse(url) else { return false };
    parsed.query_pairs().any(|(k, v)| k == "list" && !v.is_empty())
}
```

---

## High Priority

### [HIGH] Remaining phantom references in docs

Grep found 3 remaining phantom refs the docs sync missed:

1. **`docs/code-standards.md:167`** — `phase-06-gpu-pipeline.md` in example directory tree
2. **`docs/project-roadmap.md:172`** — `crates/gpu-pipeline/` expansion mentioned as future work
3. **`docs/codebase-summary.md:586`** — footnote mentions gpu-pipeline removal but the roadmap still references it

Items 1 and 3 are minor (example/footnote). Item 2 is misleading — references a crate that doesn't exist.

### [HIGH] `video-only` mode may return combined stream (has_audio=true)

**File:** `crates/api/src/services/playlist_processor.rs:478`

```rust
let all_videos: Vec<&extractor::VideoFormat> =
    formats.iter().filter(|f| !f.is_audio_only).collect();
```

Filter is `!f.is_audio_only` which includes both video-only AND combined (video+audio) streams. In `video-only` mode, if the highest-res stream happens to be a combined stream, it returns that — which is fine for direct download but semantically unexpected for "video-only" mode. The user likely wants a video-only stream (no audio track). Consider adding `&& !f.has_audio` to truly isolate video-only streams, with a fallback to combined if none exist.

---

## Medium Priority

### [MEDIUM] `extract_playlist_id()` duplicated in two files

The function exists in both:
- `crates/api/src/routes/batch.rs:194`
- `crates/api/src/services/playlist_processor.rs:560`

Same logic, same fallback substring parsing. Should be consolidated into `validation.rs` or a shared util.

### [MEDIUM] CI frontend job missing pnpm cache

The `frontend` job in CI installs pnpm but doesn't cache `node_modules` or pnpm store, unlike the Rust jobs which cache cargo. Adds ~30-60s to each run.

```yaml
- name: Get pnpm store directory
  id: pnpm-cache
  run: echo "STORE_PATH=$(pnpm store path)" >> $GITHUB_OUTPUT

- uses: actions/cache@v4
  with:
    path: ${{ steps.pnpm-cache.outputs.STORE_PATH }}
    key: ${{ runner.os }}-pnpm-store-${{ hashFiles('**/pnpm-lock.yaml') }}
```

### [MEDIUM] `build_stream_download_url` does not encode `ext` param

**File:** `crates/api/src/services/playlist_processor.rs:535`

```rust
format!("/api/stream?url={encoded_url}&title={encoded_title}&ext={ext}")
```

`ext` is not URL-encoded. While in practice ext values are safe strings like "mp4", "m4a", "webm", it's inconsistent that url and title are encoded but ext is not. Low practical risk but violates defense-in-depth.

---

## Low Priority

### [LOW] `process_single_item` step 4 logic could be clearer

Lines 308-321: the condition `v.has_audio || audio.is_none()` covers two different cases (combined stream direct download vs. video-only when no audio found). A comment clarifying this dual purpose would help readability.

### [LOW] `rand_jitter` uses `DefaultHasher` which is not cryptographically random

Used only for jitter timing between playlist items — no security impact. Fine as-is.

---

## Positive Observations

1. **SSRF fix in `validation.rs` is solid** — host-based allow-list with `reqwest::Url` parsing, case-insensitive, 14 well-targeted tests covering bypass vectors
2. **`pick_best_streams()` mode routing is well-structured** — clear comments explaining why WebM is filtered in default mode but allowed in audio/video-only modes
3. **`validate_stream_url` in proxy crate** uses proper host parsing with `ends_with(".{domain}")` pattern — not vulnerable to the substring SSRF
4. **Playlist processor recovery** handles orphaned jobs correctly across server restarts
5. **CI frontend job** correctly uses `--frozen-lockfile` and `pnpm --filter frontend`

---

## Overall Assessment

The SSRF fix in `validation.rs` and its integration into `extract.rs` and `playlist_jobs.rs` is correct and well-tested. However, **the `/api/batch` endpoint was missed** and still uses the vulnerable substring-match pattern (`contains("youtube.com")`). This is the most critical finding.

The playlist mode logic in `pick_best_streams()` works correctly for the three modes, with one semantic edge case in video-only mode that may return combined streams.

Docs sync removed most phantom refs but 3 remain (non-blocking).

**Verdict:** One critical SSRF gap in batch endpoint needs fixing before the SSRF remediation can be considered complete.

---

## Recommended Actions (Priority Order)

1. **[CRITICAL]** Replace `is_valid_batch_url()` in `batch.rs` with `validation::is_valid_youtube_url()` + parsed list param check
2. **[HIGH]** Remove `gpu-pipeline` reference from `docs/project-roadmap.md:172`
3. **[HIGH]** Consider filtering combined streams from `video-only` mode in `pick_best_streams()`
4. **[MEDIUM]** Consolidate `extract_playlist_id()` into shared module
5. **[MEDIUM]** Add pnpm store cache to CI frontend job
6. **[LOW]** URL-encode `ext` param in `build_stream_download_url`

---

## Metrics

- Type Coverage: N/A (Rust — compiler-enforced)
- Test Coverage: validation.rs 14 tests, playlist_processor 5 tests, main.rs 2 integration tests
- Linting: Not run (review-only mode)

## Unresolved Questions

1. Is the `/api/batch` endpoint actively used in production, or has it been superseded by the playlist-jobs flow? If superseded, consider deprecating it entirely.
2. Should `video-only` mode truly exclude combined streams, or is "best video regardless of audio presence" the intended behavior?
