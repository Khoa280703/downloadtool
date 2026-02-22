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
    println!("cargo:rerun-if-changed=../../extractors/tiktok.ts");
    println!("cargo:rerun-if-changed=../../extractors/types.ts");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let extractors_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("..")
        .join("..")
        .join("extractors");

    // Create output directory
    let dist_dir = out_dir.join("extractors_dist");
    fs::create_dir_all(&dist_dir).expect("Failed to create dist directory");

    // Check if esbuild is available
    let esbuild_available = Command::new("npx")
        .args(["esbuild", "--version"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    let bundle_content = if esbuild_available {
        // Bundle with esbuild
        bundle_with_esbuild(&extractors_dir, &dist_dir)
    } else {
        // Use pre-bundled files or inline fallback
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

    // Add extractor registry
    bundle.push_str(
        r#"
// Extractor registry
const extractors = {
    youtube: { extract: extract$1 },
    tiktok: { extract: extract$2 }
};
"#,
    );

    bundle
}

/// Run esbuild on a TypeScript file
fn run_esbuild(extractors_dir: &Path, file: &str, dist_dir: &Path) -> String {
    let input = extractors_dir.join(file);
    let output = dist_dir.join(file.replace(".ts", ".js"));

    let result = Command::new("npx")
        .args([
            "esbuild",
            input.to_str().unwrap(),
            "--bundle",
            "--format=esm",
            "--platform=neutral",
            "--target=es2020",
            "--outfile",
            output.to_str().unwrap(),
        ])
        .output();

    match result {
        Ok(output) if output.status.success() => {
            fs::read_to_string(&output).unwrap_or_else(|_| create_inline_fallback(file))
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
        // Return inline fallback
        create_inline_fallback_bundle()
    } else {
        bundle.push_str("\nconst extractors = { youtube, tiktok };\n");
        bundle
    }
}

/// Create inline fallback for a specific file
fn create_inline_fallback(file: &str) -> String {
    match file {
        "types.ts" => r#"
export class ExtractionError extends Error {
    constructor(message, platform, cause) {
        super(message);
        this.platform = platform;
        this.cause = cause;
    }
}
"#
        .to_string(),
        "youtube.ts" => r#"
export async function extract(url, cookies) {
    throw new ExtractionError("YouTube extractor not bundled", "youtube");
}
"#
        .to_string(),
        "tiktok.ts" => r#"
export async function extract(url, cookies) {
    throw new ExtractionError("TikTok extractor not bundled", "tiktok");
}
"#
        .to_string(),
        _ => "// Unknown file\n".to_string(),
    }
}

/// Create complete inline fallback bundle
fn create_inline_fallback_bundle() -> String {
    r#"
// Fallback bundle - esbuild not available
export class ExtractionError extends Error {
    constructor(message, platform, cause) {
        super(message);
        this.platform = platform;
        this.cause = cause;
    }
}

export async function extract$1(url, cookies) {
    throw new ExtractionError("YouTube extractor not bundled - run esbuild", "youtube");
}

export async function extract$2(url, cookies) {
    throw new ExtractionError("TikTok extractor not bundled - run esbuild", "tiktok");
}

const extractors = {
    youtube: { extract: extract$1 },
    tiktok: { extract: extract$2 }
};
"#
    .to_string()
}
