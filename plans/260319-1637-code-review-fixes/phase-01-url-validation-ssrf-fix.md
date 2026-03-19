# Phase 01 — URL Validation SSRF Fix

## Context Links
- Source: `crates/api/src/routes/extract.rs:210-213`
- Caller: `extract_handler()` in same file
- Downstream: `extractor::extract()` → yt-dlp subprocess
- **SSRF vector 2:** `crates/api/src/routes/playlist_jobs.rs:109-148` — `create_playlist_job_handler()` only checks `url.trim().is_empty()`, no host validation

## Overview
- **Priority:** P0 Critical (security)
- **Status:** completed
- **Effort:** 1h
- Current `is_valid_video_url()` uses `contains("youtube.com")` substring match. Attacker can craft URL like `https://evil.tld/?next=youtube.com/watch?v=abc` to pass validation and trigger yt-dlp against arbitrary hosts = SSRF.

## Key Insights
- yt-dlp supports many extractors; passing arbitrary URLs lets it fetch from any site
- `reqwest` already in `Cargo.toml:16` — re-exports `url::Url` as `reqwest::Url`, **no new dependency needed**
- `urlencoding = "2"` exists but only does encode/decode, NOT URL parsing
- Crate is **bin-only** (`main.rs` at `crates/api/src/main.rs`), no `lib.rs` — `mod validation;` goes in `main.rs`
- Only YouTube domains should be allowed (youtube.com variants + youtu.be)
- **Two endpoints need validation**: extract endpoint AND playlist job creation endpoint
- `create_playlist_job_handler()` at `playlist_jobs.rs:117` only checks `is_empty()` — no URL host validation at all, this is SSRF vector #2

## Requirements
- Parse URL with `reqwest::Url` (re-exported from `url` crate, already available)
- Validate host is exactly one of: `youtube.com`, `www.youtube.com`, `m.youtube.com`, `music.youtube.com`, `youtu.be`
- Reject all other hosts
- Apply same validation in playlist job creation path

## Architecture
No architectural change. Replace substring check with proper URL parsing.

## Related Code Files
- **Create:** `crates/api/src/validation.rs` — shared `is_valid_youtube_url()` using `reqwest::Url`
- **Modify:** `crates/api/src/main.rs` — add `mod validation;` (bin-only crate, no lib.rs)
- **Modify:** `crates/api/src/routes/extract.rs` — replace inline `is_valid_video_url()` with `crate::validation::is_valid_youtube_url()`
- **Modify:** `crates/api/src/routes/playlist_jobs.rs` — add URL validation in `create_playlist_job_handler()` after empty check at line 117
- **No new dependency:** `reqwest` already provides `reqwest::Url` (re-export of `url::Url`)
- **No change:** `crates/api/src/services/playlist_processor.rs:279` — internal URL construction (safe, hardcoded youtube.com format)

## Implementation Steps

1. **Create `crates/api/src/validation.rs`** with shared URL validation:
   ```rust
   /// Validate that a URL points to a legitimate YouTube domain.
   /// Prevents SSRF by parsing URL and checking host, not substring.
   pub fn is_valid_youtube_url(input: &str) -> bool {
       let Ok(parsed) = reqwest::Url::parse(input) else {
           return false;
       };
       let Some(host) = parsed.host_str() else {
           return false;
       };
       let host_lower = host.to_lowercase();
       matches!(
           host_lower.as_str(),
           "youtube.com" | "www.youtube.com" | "m.youtube.com"
               | "music.youtube.com" | "youtu.be"
       )
   }
   ```

2. **Register module** in `crates/api/src/main.rs` — add `mod validation;` alongside existing `mod auth;` etc. at line 33

3. **Replace** inline `is_valid_video_url()` in `extract.rs` with `crate::validation::is_valid_youtube_url()`

4. **Add URL validation to `create_playlist_job_handler()`** in `playlist_jobs.rs`:
   - After the empty check at line 117, add:
     ```rust
     if !crate::validation::is_valid_youtube_url(&payload.url) {
         return Err(JobsApiError {
             message: "Invalid YouTube URL".to_string(),
             status: StatusCode::BAD_REQUEST,
             retry_after_secs: None,
         });
     }
     ```
   - This is SSRF vector #2 — currently passes URL directly to extractor with only empty check

5. Add unit tests for `is_valid_youtube_url()` in `validation.rs`:
   - Valid: `https://www.youtube.com/watch?v=abc`, `https://youtu.be/abc`, `https://music.youtube.com/watch?v=abc`
   - Invalid: `https://evil.tld/?next=youtube.com`, `https://youtube.com.evil.tld/`, `https://notyoutube.com/`
   - Edge: empty string, no scheme, `javascript:` scheme

6. Run `cargo build --workspace` and `cargo test --workspace`

## Todo List
- [x] Create `crates/api/src/validation.rs` with `is_valid_youtube_url()` using `reqwest::Url` (no new dep)
- [x] Add `mod validation;` in `main.rs` (bin-only crate)
- [x] Replace inline `is_valid_video_url()` in `extract.rs` with shared validation
- [x] Add URL validation to `create_playlist_job_handler()` in `playlist_jobs.rs` (SSRF vector #2)
- [x] Add unit tests
- [x] Compile and test

## Success Criteria
- `is_valid_video_url("https://evil.tld/?next=youtube.com")` returns `false`
- All legitimate YouTube URL formats still accepted
- No regression in extract or playlist endpoints
- `cargo test` passes

## Risk Assessment
- **Low risk**: additive change, only makes validation stricter
- **Rollback**: revert to substring check if legitimate URLs rejected (unlikely)

## Security Considerations
- This IS the security fix — eliminates SSRF vector
- Consider rate limiting on extract endpoint as defense-in-depth (already exists via governor)
- yt-dlp itself has some URL validation but should not be relied upon

## Next Steps
- After merging, consider adding allowed-host allowlist as config (YAGNI for now — only YouTube supported)
- Monitor logs for rejected URLs to catch false positives
