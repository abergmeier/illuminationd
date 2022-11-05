/*
use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["api/org/freedesktop/illumination/v1/entity.proto"], &["api/"])?;
    Ok(())
}
*/
fn main () -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("api/org/freedesktop/illumination/v1/entity.proto")?;
    Ok(())
  }
