# Phase 03 — Clean crates/api

## Overview
- **Priority:** P1
- **Status:** pending
- **ETA:** 20m
- **Depends on:** Phase 01, Phase 02

## Files to Modify

| File | Change |
|------|--------|
| `crates/api/Cargo.toml` | Remove `gpu-pipeline` optional dep, `tonic` optional dep, `[features] gpu` block |
| `crates/api/src/routes/transcode.rs` | Delete entire file |
| `crates/api/src/routes/mod.rs` | Remove `pub mod transcode` + re-exports |
| `crates/api/src/main.rs` | Remove two transcode routes from router |
| `crates/api/src/config.rs` | Remove `gpu_worker_addr` and `gpu_enabled` fields |

---

## 1. `crates/api/Cargo.toml`

Remove the entire optional GPU block:

```toml
# GPU support (optional)          ← remove
gpu-pipeline = { path = "../gpu-pipeline", optional = true }   ← remove
tonic = { workspace = true, optional = true }                  ← remove

[features]                        ← keep section only if other features exist
default = []                      ← keep
gpu = ["dep:gpu-pipeline", "dep:tonic"]   ← remove
```

Result — `[features]` section becomes:

```toml
[features]
default = []
```

Or remove the `[features]` section entirely if `default = []` adds no value.

---

## 2. Delete `crates/api/src/routes/transcode.rs`

```bash
rm crates/api/src/routes/transcode.rs
```

---

## 3. `crates/api/src/routes/mod.rs`

Remove lines:

```rust
pub mod transcode;                                              // remove
pub use transcode::{transcode_handler, transcode_health_check}; // remove
```

Final file after removal:

```rust
//! HTTP route handlers for the API server

pub mod batch;
pub mod extract;
pub mod openapi;
pub mod static_files;
pub mod stream;

pub use batch::batch_handler;
pub use extract::extract_handler;
pub use openapi::openapi_handler;
pub use static_files::{bm_js_handler, userscript_handler};
pub use stream::{muxed_stream_handler, stream_handler};

use axum::http::StatusCode;

/// Health check endpoint.
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let status = health_check().await;
        assert_eq!(status, StatusCode::OK);
    }
}
```

---

## 4. `crates/api/src/main.rs`

Remove the two transcode route registrations:

```rust
.route("/api/transcode", post(routes::transcode_handler))          // remove
.route("/api/transcode/health", get(routes::transcode_health_check)) // remove
```

Also check if `post` import is still needed (used by `/api/extract`). It is — keep it.

---

## 5. `crates/api/src/config.rs`

Remove GPU fields from the `Config` struct and `from_env()`:

**Struct** — remove:
```rust
/// GPU worker gRPC address (e.g., "10.0.0.2:50051")
pub gpu_worker_addr: String,
/// Whether GPU transcoding is enabled
pub gpu_enabled: bool,
```

**`from_env()`** — remove:
```rust
let gpu_worker_addr = env::var("GPU_WORKER_ADDR")
    .unwrap_or_else(|_| "10.0.0.2:50051".to_string());

let gpu_enabled = env::var("GPU_ENABLED")
    .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
    .unwrap_or(false);
```

And remove from the `Ok(Self { ... })` constructor:
```rust
gpu_worker_addr,   // remove
gpu_enabled,       // remove
```

**Tests** — update `test_default_config` to remove GPU field assertions.

## Success Criteria
- `cargo check -p api` with no features passes with zero errors
- No reference to `transcode`, `gpu_worker_addr`, or `gpu_enabled` remains in `crates/api/`
