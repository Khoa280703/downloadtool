//! Build script for the extractor crate
//!
//! This script bundles the TypeScript extractor files using esbuild
//! at compile time, embedding them into the binary.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../../extractors/youtube.ts");
    println!("cargo:rerun-if-changed=../../extractors/youtube-innertube.ts");
    println!("cargo:rerun-if-changed=../../extractors/tiktok.ts");
    println!("cargo:rerun-if-changed=../../extractors/types.ts");
    println!("cargo:rerun-if-changed=../../extractors/dist/youtube.js");
    println!("cargo:rerun-if-changed=../../extractors/dist/tiktok.js");
    println!("cargo:rerun-if-changed=../../extractors/dist/types.js");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let extractors_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("..")
        .join("..")
        .join("extractors");

    // Create output directory
    let dist_dir = out_dir.join("extractors_dist");
    fs::create_dir_all(&dist_dir).expect("Failed to create dist directory");

    // Priority: 1) pre-built dist/*.js (run `make extractors` to update), 2) esbuild live, 3) inline fallback
    let dist_youtube = extractors_dir.join("dist").join("youtube.js");
    let bundle_content = if dist_youtube.exists() {
        create_fallback_bundle(&extractors_dir)
    } else {
        let esbuild_ok = Command::new("npx")
            .args(["esbuild", "--version"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        if esbuild_ok {
            bundle_with_esbuild(&extractors_dir, &dist_dir)
        } else {
            create_inline_fallback_bundle()
        }
    };

    // Write the combined bundle
    let bundle_path = out_dir.join("extractors_bundle.js");
    fs::write(&bundle_path, bundle_content).expect("Failed to write bundle");

    println!(
        "cargo:rustc-env=EXTRACTORS_BUNDLE={}",
        bundle_path.display()
    );
}

/// Bundle TypeScript files using esbuild
fn bundle_with_esbuild(extractors_dir: &Path, dist_dir: &Path) -> String {
    let mut bundle = String::new();

    // Bundle types first
    let types_output = run_esbuild(extractors_dir, "types.ts", dist_dir);
    bundle.push_str(&types_output);
    bundle.push('\n');

    // Bundle YouTube extractor
    let youtube_output = run_esbuild(extractors_dir, "youtube.ts", dist_dir);
    bundle.push_str(&youtube_output);
    bundle.push('\n');

    // Bundle TikTok extractor
    let tiktok_output = run_esbuild(extractors_dir, "tiktok.ts", dist_dir);
    bundle.push_str(&tiktok_output);
    bundle.push('\n');

    // If all individual bundles are empty (esbuild failed / no TS files), use full fallback
    if bundle.trim().is_empty() {
        return create_inline_fallback_bundle();
    }

    // Add extractor registry — use typeof to safely check IIFE global vars
    bundle.push_str(
        r#"
// Extractor registry - access exports via IIFE global vars (typeof to avoid ReferenceError)
var extractors = {
    youtube: { extract: typeof youtube !== "undefined" && youtube && youtube.extract ? youtube.extract : async function(u,c){ throw new Error("YouTube extractor not bundled - run esbuild"); } },
    tiktok:  { extract: typeof tiktok  !== "undefined" && tiktok  && tiktok.extract  ? tiktok.extract  : async function(u,c){ throw new Error("TikTok extractor not bundled - run esbuild"); } }
};
"#,
    );

    bundle
}

/// Run esbuild on a TypeScript file
fn run_esbuild(extractors_dir: &Path, file: &str, dist_dir: &Path) -> String {
    let input = extractors_dir.join(file);
    let output = dist_dir.join(file.replace(".ts", ".js"));
    // Use iife format so output is plain JS compatible with execute_script
    // global-name exposes the module as a variable on globalThis
    let global_name = file.replace(".ts", "").replace(".", "_");

    let result = Command::new("npx")
        .args([
            "esbuild",
            input.to_str().unwrap(),
            "--bundle",
            "--format=iife",
            &format!("--global-name={}", global_name),
            "--platform=neutral",
            "--target=es2020",
            "--outfile",
            output.to_str().unwrap(),
        ])
        .output();

    match result {
        Ok(output) if output.status.success() => {
            fs::read_to_string(&dist_dir.join(file.replace(".ts", ".js"))).unwrap_or_else(|_| create_inline_fallback(file))
        }
        Ok(output) => {
            eprintln!(
                "esbuild warning: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            create_inline_fallback(file)
        }
        Err(e) => {
            eprintln!("esbuild error: {}", e);
            create_inline_fallback(file)
        }
    }
}

/// Create a fallback bundle when esbuild is not available
fn create_fallback_bundle(extractors_dir: &Path) -> String {
    let mut bundle = String::new();

    // Try to read pre-bundled files
    for file in ["types.js", "youtube.js", "tiktok.js"] {
        let path = extractors_dir.join("dist").join(file);
        if let Ok(content) = fs::read_to_string(path) {
            bundle.push_str(&content);
            bundle.push('\n');
        }
    }

    if bundle.is_empty() {
        create_inline_fallback_bundle()
    } else {
        bundle.push_str("\nvar extractors = { youtube: { extract: typeof youtube !== \"undefined\" && youtube.extract ? youtube.extract : null }, tiktok: { extract: typeof tiktok !== \"undefined\" && tiktok.extract ? tiktok.extract : null } };\n");
        bundle
    }
}

/// Create inline fallback for a specific file (no export - plain JS)
fn create_inline_fallback(_file: &str) -> String {
    // Return empty string; the full fallback bundle handles everything
    String::new()
}

/// Create complete inline fallback bundle (plain JS, no ES module syntax)
fn create_inline_fallback_bundle() -> String {
    r#"
// Fallback bundle - esbuild not available
// Plain JS compatible with deno_core execute_script (no ES module syntax)
var ExtractionError = (function() {
    function ExtractionError(message, platform, cause) {
        this.message = message;
        this.platform = platform;
        this.cause = cause;
        this.name = "ExtractionError";
    }
    ExtractionError.prototype = Object.create(Error.prototype);
    return ExtractionError;
})();

var extractors = {
    youtube: {
        extract: async function(url, cookies) {
            throw new ExtractionError("YouTube extractor not bundled - build with esbuild", "youtube");
        }
    },
    tiktok: {
        extract: async function(url, cookies) {
            throw new ExtractionError("TikTok extractor not bundled - build with esbuild", "tiktok");
        }
    }
};
"#
    .to_string()
}
