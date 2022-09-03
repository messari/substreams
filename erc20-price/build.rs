use std::process::Command;
use std::string::ToString;
use std::{env, fs, path::Path};

use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    // generate pb files under src/pb
    Command::new("make")
        .args(&["codegen"])
        .status()
        .expect("failed to codegen");

    // generate src/pb/mod.rs
    fs::write(
        "src/pb/mod.rs",
        dir_filenames("./proto")
            .iter()
            .map(|pb| format!("#[path = \"messari.{}.rs\"]\npub mod {};\n", pb, pb))
            .collect::<Vec<_>>()
            .join("\n"),
    )
    .unwrap();

    let abi_filenames = dir_filenames("./abi");

    // generate abi files under src/abi
    abi_filenames.iter().for_each(|contract| {
        Abigen::new(contract, &format!("abi/{}.json", contract))
            .unwrap()
            .generate()
            .unwrap()
            .write_to_file(format!("src/abi/{}.rs", contract))
            .unwrap()
    });

    // generate src/abi/mod.rs
    fs::write(
        "src/abi/mod.rs",
        abi_filenames
            .iter()
            .map(|contract| format!("pub mod {};", contract))
            .collect::<Vec<_>>()
            .join("\n"),
    )
    .unwrap();

    Ok(())
}

/// Get filenames without file type suffix
fn dir_filenames(path: &str) -> Vec<String> {
    fs::read_dir(path)
        .unwrap()
        .map(|x| {
            x.unwrap()
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>()
}
