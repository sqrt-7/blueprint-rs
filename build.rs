use std::io::Result;

fn main() -> Result<()> {
    // Build script to compile .proto file(s)
    tonic_build::configure()
        .out_dir("proto")
        .compile(&["proto/blueprint.proto"], &["proto/"])?;

    Ok(())
}
