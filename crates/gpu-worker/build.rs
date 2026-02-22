//! Build script for GPU worker - compiles protobuf files

use std::io::Result;

fn main() -> Result<()> {
    // Path to the proto file
    let proto_file = "../../proto/transcode.proto";

    // Rerun if proto file changes
    println!("cargo:rerun-if-changed={}", proto_file);

    // Compile proto file using tonic-build
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&[proto_file], &["../../proto"])?;

    Ok(())
}
