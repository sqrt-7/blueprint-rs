use std::io::Result;

fn main() -> Result<()> {
    // Build script to compile .proto file(s)
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .out_dir("proto")
        .compile(&["proto/blueprint.proto"], &["proto/"])?;

    Ok(())
}
