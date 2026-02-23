# Phase 1: Fix Download Timeout Bug

## Context Links

- [plan.md](plan.md) -- overview
- [anti_bot.rs](../../crates/proxy/src/anti_bot.rs) -- file to modify

## Overview

- **Priority**: P1 (critical -- downloads fail completely for large files)
- **Status**: completed
- **Effort**: 30min

reqwest's `.timeout()` sets a total request timeout covering the entire transfer. For streaming video downloads that take minutes, 30s kills the connection mid-transfer. Fix: use `.connect_timeout()` which only limits TCP handshake + TLS negotiation.

## Key Insights

- `reqwest::Client::builder().timeout(Duration)` = total request duration limit (connect + transfer)
- `reqwest::Client::builder().connect_timeout(Duration)` = only TCP connection establishment timeout
- Large video files (500MB+) easily exceed 30s transfer time even at high speeds
- The same timeout pattern exists in `runtime.rs` `op_fetch` (line 325-326) but that's for short metadata fetches, not streaming -- leave it as-is

## Requirements

### Functional
- Downloads must not timeout during active data transfer
- TCP connection must still timeout after 30s if server unreachable

### Non-functional
- No change to retry logic or error handling behavior
- Backward compatible with existing AntiBotClient consumers

## Architecture

No architectural changes. Single line fix in `build_client()`.

## Related Code Files

### Files to Modify
- `crates/proxy/src/anti_bot.rs` -- line 99: change `.timeout()` to `.connect_timeout()`

### Files NOT to Modify
- `crates/extractor/src/runtime.rs` line 326 -- `op_fetch` timeout is for metadata fetches (player JS, InnerTube API), 30s is appropriate there

## Implementation Steps

1. Open `crates/proxy/src/anti_bot.rs`
2. Line 99: change `.timeout(Duration::from_secs(30))` to `.connect_timeout(Duration::from_secs(30))`
3. Run `~/.cargo/bin/cargo b` to verify compilation
4. Run `~/.cargo/bin/cargo test -p proxy` to verify existing tests pass
5. Manual test: start a large video download and confirm it completes past 30s

## Todo List

- [x] Change `.timeout()` to `.connect_timeout()` in `build_client()`
- [x] Verify Rust compilation
- [x] Run proxy crate tests
- [x] Manual test with large file download

## Success Criteria

- `cargo b` compiles without errors
- Existing tests pass
- Download of a 100MB+ video completes without timeout error

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| No total timeout = hung connections | Low | reqwest has default idle timeouts; server-side timeouts also apply |
| Connection to unreachable proxy hangs | Low | `connect_timeout(30s)` still covers this case |

## Security Considerations

No security impact. Connect timeout still prevents indefinite connection attempts.

## Next Steps

After Phase 1, proceed to Phase 2 (n-param transform) for full speed improvement.
