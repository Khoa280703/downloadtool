//! Static file routes for client-side scripts
//!
//! GET /bm.js        - Bookmarklet script (embedded at compile time)
//! GET /userscript   - UserScript for Tampermonkey/Violentmonkey (embedded at compile time)
//!
//! Build apps/injector first: `pnpm --filter @downloadtool/injector build`
//! Then recompile Rust to embed the updated files.

use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};

// Embed at compile time — paths relative to this crate's Cargo.toml directory
const BM_JS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../apps/injector/dist/bm.js"
));

const USERSCRIPT_JS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../apps/injector/dist/youtube-downloader.user.js"
));

/// Serve the bookmarklet script.
///
/// GET /bm.js
pub async fn bm_js_handler() -> Response {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        BM_JS,
    )
        .into_response()
}

/// Serve the UserScript for Tampermonkey/Violentmonkey.
///
/// GET /userscript
pub async fn userscript_handler() -> Response {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        USERSCRIPT_JS,
    )
        .into_response()
}
