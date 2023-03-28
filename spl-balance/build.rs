use anyhow::{Ok, Result};
use substreams_common::codegen;

fn main() -> Result<(), anyhow::Error> {
    println!("cargo:rerun-if-changed=proto");
    println!("cargo:rerun-if-changed=abi");
    codegen::generate(None)?;

    Ok(())
}
