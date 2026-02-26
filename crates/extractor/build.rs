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
    println!("cargo:rerun-if-changed=../../extractors/types.ts");
    println!("cargo:rerun-if-changed=../../extractors/dist/youtube.js");
    println!("cargo:rerun-if-changed=../../extractors/dist/types.js");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let extractors_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("..")
        .join("..")
        .join("extractors");

    // Create output directory
    let dist_dir = out_dir.join("extractors_dist");
    fs::create_dir_all(&dist_dir).expect("Failed to create dist directory");

    // Priority: 1) esbuild live (IIFE format, globals-compatible), 2) inline fallback
    // Note: pre-built dist/*.js use ESM format (not suitable for V8 execute_script)
    // Try common npx locations since cargo build may not have full PATH
    let npx_candidates = ["/usr/bin/npx", "/usr/local/bin/npx", "npx"];
    let npx_bin = npx_candidates.iter()
        .find(|p| {
            let result = Command::new(p).args(["esbuild", "--version"]).output();
            match &result {
                Ok(o) => eprintln!("build.rs: npx {} esbuild --version -> status={}", p, o.status),
                Err(e) => eprintln!("build.rs: npx {} failed: {}", p, e),
            }
            result.map(|o| o.status.success()).unwrap_or(false)
        })
        .copied();
    eprintln!("build.rs: npx_bin={:?}, extractors_dir={:?}", npx_bin, extractors_dir);
    let bundle_content = if let Some(npx) = npx_bin {
        bundle_with_esbuild_cmd(&extractors_dir, &dist_dir, npx)
    } else {
        // npx not available (e.g. Docker build without Node.js): fall back to pre-built IIFE dist
        create_fallback_bundle(&extractors_dir)
    };

    // Write the combined bundle
    let bundle_path = out_dir.join("extractors_bundle.js");
    fs::write(&bundle_path, bundle_content).expect("Failed to write bundle");

    println!(
        "cargo:rustc-env=EXTRACTORS_BUNDLE={}",
        bundle_path.display()
    );
}

/// Bundle TypeScript files using esbuild (with specified npx binary)
fn bundle_with_esbuild_cmd(extractors_dir: &Path, dist_dir: &Path, npx: &str) -> String {
    let mut bundle = String::new();

    // Bundle types first
    let types_output = run_esbuild(extractors_dir, "types.ts", dist_dir, npx);
    bundle.push_str(&types_output);
    bundle.push('\n');

    // Bundle YouTube extractor
    let youtube_output = run_esbuild(extractors_dir, "youtube.ts", dist_dir, npx);
    bundle.push_str(&youtube_output);
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
    youtube: { extract: typeof youtube !== "undefined" && youtube && youtube.extract ? youtube.extract : async function(u,c){ throw new Error("YouTube extractor not bundled - run esbuild"); } }
};
"#,
    );

    bundle
}

/// Run esbuild on a TypeScript file
fn run_esbuild(extractors_dir: &Path, file: &str, dist_dir: &Path, npx: &str) -> String {
    let input = extractors_dir.join(file);
    let output = dist_dir.join(file.replace(".ts", ".js"));
    // Use iife format so output is plain JS compatible with execute_script
    // global-name exposes the module as a variable on globalThis
    let global_name = file.replace(".ts", "").replace(".", "_");

    let result = Command::new(npx)
        .args([
            "esbuild",
            input.to_str().unwrap(),
            "--bundle",
            "--format=iife",
            &format!("--global-name={}", global_name),
            "--platform=neutral",
            "--target=es2020",
            &format!("--outfile={}", output.display()),
        ])
        .output();

    match result {
        Ok(output) if output.status.success() => {
            fs::read_to_string(dist_dir.join(file.replace(".ts", ".js"))).unwrap_or_else(|_| create_inline_fallback(file))
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

/// Strip ESM export blocks from pre-built dist files (built with --format=esm by Makefile)
/// Deno V8 execute_script doesn't support ES module syntax
fn strip_esm_exports(content: &str) -> String {
    let mut result = String::new();
    let mut in_export_block = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("export {") {
            in_export_block = true;
        }
        if !in_export_block {
            result.push_str(line);
            result.push('\n');
        }
        if in_export_block && trimmed.ends_with("};") {
            in_export_block = false;
        }
    }
    result
}

/// Create a fallback bundle when esbuild is not available
fn create_fallback_bundle(extractors_dir: &Path) -> String {
    let mut bundle = String::new();

    // Try to read pre-bundled files, stripping ESM export syntax
    for file in ["types.js", "youtube.js"] {
        let path = extractors_dir.join("dist").join(file);
        if let Ok(content) = fs::read_to_string(path) {
            bundle.push_str(&strip_esm_exports(&content));
            bundle.push('\n');
        }
    }

    if bundle.is_empty() {
        create_inline_fallback_bundle()
    } else {
        bundle.push_str("\nvar extractors = { youtube: { extract: typeof youtube !== \"undefined\" && youtube.extract ? youtube.extract : null } };\n");
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
    }
};
"#
    .to_string()
}
