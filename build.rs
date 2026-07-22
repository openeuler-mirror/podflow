use std::io::Result;

fn main() -> Result<()> {
    // Compile collector protobuf files
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&["proto/collector.proto"], &["proto"])?;

    // Compile NRI protobuf files (containerd official protocol)
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&["proto/nri.proto"], &["proto"])?;

    // Tell cargo to re-run when proto files change
    println!("cargo:rerun-if-changed=proto/collector.proto");
    println!("cargo:rerun-if-changed=proto/nri.proto");

    Ok(())
}