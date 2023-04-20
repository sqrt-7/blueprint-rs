use std::io::Result;

fn main() -> Result<()> {
    // Build script to compile .proto file(s)
    let mut cfg = prost_build::Config::new();
    cfg.out_dir("proto");
    cfg.compile_protos(&["proto/blueprint.proto"], &["proto/"])?;
    Ok(())
}
