use std::collections::{HashMap, HashSet};
use std::env::current_dir;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::string::ToString;
use substreams_ethereum::Abigen;

/// Default output directory for generated code.
pub const DEFAULT_OUTPUT_DIR: &str = "target";
pub const DEFAULT_PROTO_DIR: &str = "proto";

const NO_VERSION_SET: &str = "NO_VERSION_SET";

#[derive(thiserror::Error, Debug)]
pub enum Error {}

fn generate_abi(out_dir: Option<&str>) -> Result<(), Error> {
    let mut abi_filenames = dir_filenames("./abi");
    if abi_filenames.len() == 0 {
        return Ok(());
    }
    abi_filenames.sort();

    let out_dir = out_dir.unwrap_or(DEFAULT_OUTPUT_DIR);
    let target_abi_dir = Path::new(out_dir).join("abi");
    fs::remove_dir_all(&target_abi_dir).ok();
    fs::create_dir_all(&target_abi_dir).ok();

    // generate abi structs under target/abi based on abi json under abi/
    abi_filenames.iter().for_each(|contract| {
        Abigen::new(contract, &format!("abi/{}.json", contract))
            .unwrap()
            .generate()
            .unwrap()
            .write_to_file(format!("{}/abi/{}.rs", out_dir, contract))
            .unwrap()
    });

    let mut abi_file_content = abi_filenames
        .iter()
        .map(|contract| {
            format!(
                "#[rustfmt::skip]\n\
                #[allow(unused_imports)]\n\
                #[path = \"../{}/abi/{}.rs\"]\n\
                pub mod {};\n",
                out_dir, contract, contract
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    abi_file_content.push('\n');

    // generate src/abi.rs module
    write_or_replace_if_different(Path::new("src").join("abi.rs"), abi_file_content);

    Ok(())
}

fn generate_pb(out_dir: Option<&str>) -> Result<(), Error> {
    let out_dir = out_dir.unwrap_or(DEFAULT_OUTPUT_DIR);
    let pb_file = current_dir().unwrap().join("src").join("pb.rs");
    let tmp_dir = current_dir().unwrap().join("target").join("tmp");
    let target_pb_dir = current_dir().unwrap().join(out_dir).join("pb");
    let substreams_yaml = current_dir().unwrap().join("substreams.yaml");

    if tmp_dir.exists() {
        fs::remove_dir_all(&tmp_dir).unwrap();
    }

    // generate pb files under src/pb
    Command::new("substreams")
        .args(&["protogen", substreams_yaml.to_string_lossy().as_ref(), "--output-path=target/tmp"])
        .status()
        .expect("failed to run substreams protogen");

    // Cleanup unwanted .proto bindings
    if let Ok(read_dir) = fs::read_dir(&tmp_dir) {
        read_dir
            .map(|x| {
                let dir_entry = x.unwrap();
                (dir_entry.path(), dir_entry.file_name().to_string_lossy().to_string())
            })
            .for_each(|(filepath, filename)| {
                if filename.starts_with("sf.ethereum") || filename.starts_with("sf.substreams") || filename.starts_with("google") {
                    fs::remove_file(filepath).unwrap();
                }
            });
    }

    let pb_files = {
        let mut pb_files_hash = HashMap::new();
        let pb_filenames = dir_filenames(&tmp_dir);

        for file in pb_filenames.iter() {
            let filename_parts = file.split('.').collect::<Vec<&str>>();
            let (name, version) = match filename_parts.len() {
                1 => (filename_parts[0], NO_VERSION_SET),
                3 => {
                    // First part is assumed to be the namespace (eg messari); second part to be the filename and the third part is assumed to be the version
                    (filename_parts[1], filename_parts[2])
                }
                _ => panic!(
                    "A proto package should have either 1 or 3 parts to it whereas this package has {} parts specified. Package name: {}",
                    filename_parts.len(),
                    file
                ),
            };

            pb_files_hash.entry(name.to_owned()).or_insert(HashSet::new()).insert(version.to_owned());
        }
        let mut pb_files_vec = pb_files_hash
            .into_iter()
            .map(|(filename, versions_hash)| {
                let mut versions = versions_hash.into_iter().collect::<Vec<_>>();
                versions.sort();
                (filename, versions)
            })
            .collect::<Vec<_>>();

        pb_files_vec.sort_by(|(filename1, _), (filename2, _)| filename1.cmp(filename2));
        pb_files_vec
    };

    if pb_files.is_empty() {
        return Ok(());
    }

    if !target_pb_dir.exists() {
        // We use create_dir rather than create_dir_all as the substreams protogen cmd above always creates the
        // target/tmp folder if successful so we only need to create the pb folder itself. Failure to create this
        // folder would imply a failure with the substreams protogen cmd.
        fs::create_dir(&target_pb_dir).unwrap();
    }

    // Move all pb files to target folder
    if let Ok(read_dir) = fs::read_dir(&tmp_dir) {
        for file in read_dir.into_iter() {
            let current_filepath = file.unwrap().path();
            let target_filepath = target_pb_dir.join(current_filepath.file_name().unwrap());
            let mut file_contents = fs::read_to_string(current_filepath).unwrap();
            file_contents = file_contents.replace("super::super::", "super::"); // Path directions need to be changed now we are collating bindings in the same file
            fs::write(&target_filepath, file_contents).unwrap();
        }
    }

    // Cleanup
    fs::remove_dir_all(&tmp_dir).unwrap();

    let pb_file_content = pb_files
        .into_iter()
        .map(|(filename, versions)| {
            if versions.len() == 1 && versions[0] == NO_VERSION_SET {
                format!(
                    "#[rustfmt::skip]\n\
                    #[allow(unused_imports, dead_code)]\n\
                    #[path = \"../{}/pb/{}.rs\"]\n\
                    pub mod {1};\n",
                    out_dir, filename
                )
            } else {
                let (mod_content, registration_content): (Vec<String>, Vec<String>) = versions
                    .into_iter()
                    .filter_map(|version| if version == NO_VERSION_SET { None } else { Some(version) })
                    .map(|version| {
                        (
                            format!(
                                "#[rustfmt::skip]\n\
                                #[allow(unused_imports, dead_code)]\n\
                                #[path = \"../{}/pb/messari.{}.{}.rs\"]\n\
                                pub(self) mod {1}_{2};\n",
                                out_dir, filename, version
                            ),
                            format!(
                                "    pub mod {} {{\n        \
                                         pub use super::super::{}_{0}::*;\n    \
                                     }}\n",
                                version, filename
                            ),
                        )
                    })
                    .unzip();

                format!(
                    "{}\n\
                    pub mod {} {{\n\
                        {}\
                    }}\n",
                    mod_content.join("\n"),
                    filename,
                    registration_content.join("\n")
                )
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    write_or_replace_if_different(pb_file, pb_file_content);

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
    if let Ok(read_dir) = fs::read_dir(&path.as_ref()) {
        read_dir.map(|x| x.unwrap().path().file_stem().unwrap().to_str().unwrap().to_string()).collect::<Vec<String>>()
    } else {
        Vec::new()
    }
}

fn write_or_replace_if_different(filepath: PathBuf, content: String) {
    if filepath.exists() {
        let mut current_content = fs::read_to_string(&filepath).unwrap();
        // This is primarily for windows OS to make sure different newline declarations are treated equivalent when comparing file contents
        current_content = current_content.replace("\r\n", "\n");
        if content == current_content {
            return;
        }
    }

    fs::write(filepath, content).unwrap();
}
