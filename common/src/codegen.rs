use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::string::ToString;
use substreams_ethereum::Abigen;

/// Default output directory for generated code.
pub const DEFAULT_OUTPUT_DIR: &str = "target";
pub const DEFAULT_PROTO_DIR: &str = "proto";

#[derive(thiserror::Error, Debug)]
pub enum Error {}

pub fn generate_abi(out_dir: Option<&str>) -> Result<(), Error> {
    let out_dir = out_dir.unwrap_or(DEFAULT_OUTPUT_DIR);
    let abi_filenames = self::dir_filenames("./abi");
    let abi_dir = Path::new("src").join("abi");
    let target_abi_dir = Path::new(out_dir).join("abi");
    fs::remove_dir_all(&target_abi_dir).ok();
    fs::create_dir_all(&target_abi_dir).ok();
    fs::rename(&abi_dir, &target_abi_dir).ok();

    // generate abi files under src/abi
    abi_filenames.iter().for_each(|contract| {
        Abigen::new(contract, &format!("abi/{}.json", contract))
            .unwrap()
            .generate()
            .unwrap()
            .write_to_file(format!("{}/abi/{}.rs", out_dir, contract))
            .unwrap()
    });

    // generate src/abi.rs module
    fs::write(
        "src/abi.rs",
        abi_filenames
            .iter()
            .map(|contract| {
                format!(
                    "#[rustfmt::skip]\n#[path = \"../{}/abi/{}.rs\"]\npub mod {};\n",
                    out_dir, contract, contract
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
    )
    .unwrap();

    Ok(())
}

pub fn generate_pb(out_dir: Option<&str>) -> Result<(), Error> {
    let out_dir = out_dir.unwrap_or(DEFAULT_OUTPUT_DIR);
    let pb_dir = Path::new("src").join("pb");
    let target_pb_dir = Path::new(out_dir).join("pb");

    // Remove previous generated files
    fs::remove_dir_all(&target_pb_dir).ok();
    fs::remove_dir_all(&pb_dir).ok();

    // generate pb files under src/pb
    Command::new("make")
        .args(&["codegen"])
        .status()
        .expect("failed to codegen");

    // Create target directories
    fs::create_dir_all(&target_pb_dir).ok();
    fs::create_dir("./src/pb/").ok();

    let mut pb_files = HashMap::new();
    let pb_filenames = dir_filenames(&pb_dir);
    for file in pb_filenames.iter() {
        // parse version from file name
        let filename = file.split('.').collect::<Vec<&str>>();
        // let package_name = filename[0];
        let name = filename[1];
        let version = filename[2];
        pb_files
            .entry(name)
            .or_insert(HashSet::new())
            .insert(version.to_owned());
    }

    let pb_mod_content = pb_files
        .iter()
        .map(|(pb, _)| format!("pub mod {};", pb))
        .collect::<Vec<_>>()
        .join("\n");

    fs::write("src/pb.rs", pb_mod_content).unwrap();

    // Create target directories
    fs::create_dir_all(&target_pb_dir).ok();

    // Move to target folders
    fs::rename(&pb_dir, &target_pb_dir).ok();

    fs::create_dir(pb_dir.clone()).ok();

    for (filename, versions) in pb_files.iter() {
        let pb_file = format!("src/pb/{}.rs", filename);
        let content = versions
            .iter()
            .map(|v| {
                format!(
                    "#[rustfmt::skip]\n#[path = \"../../{}/pb/messari.{}.{}.rs\"]\npub mod {};\n",
                    out_dir, filename, v, v
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(pb_file, content).unwrap();
    }

    Ok(())
}

pub fn generate(out_dir: Option<&str>) -> Result<(), Error> {
    // generate protobuf files
    generate_pb(out_dir)?;
    // generate ABI bindings
    generate_abi(out_dir)?;

    Ok(())
}

/// Get filenames without file type suffix
pub fn dir_filenames(path: impl AsRef<OsStr>) -> Vec<String> {
    println!("Searching for files in {}", path.as_ref().to_str().unwrap());
    fs::read_dir(&path.as_ref())
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