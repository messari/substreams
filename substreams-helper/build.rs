use anyhow::{Ok, Result};
use substreams_common::codegen;

fn main() -> Result<(), anyhow::Error> {
    codegen::generate(None)?;

    Ok(())
}
