use crate::file_modification::cargo_toml::{Dependency, Location};
use regex::Regex;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str;

pub(crate) trait StrExt {
    fn is_valid_abi_address(&self) -> bool;
}

impl StrExt for &str {
    fn is_valid_abi_address(&self) -> bool {
        let re_contract_address = Regex::new(r"^0x[A-Za-z0-9]{40}$").unwrap();
        re_contract_address.is_match(self)
    }
}

pub(crate) trait StaticStrExt {
    fn dep_with_major_version(self, major_version_requirement: u64) -> Dependency;
    fn dep_with_local_path<T: Into<PathBuf>>(self, local_path: T) -> Dependency;
    fn dep_from_workspace(self) -> Dependency;
    fn into_dep(self) -> Dependency;
}

impl StaticStrExt for &'static str {
    fn dep_with_major_version(self, major_version_requirement: u64) -> Dependency {
        Dependency {
            crate_name: self,
            location: Location::Remote {
                major_version_requirement: Some(major_version_requirement),
            },
        }
    }

    fn dep_with_local_path<T: Into<PathBuf>>(self, local_path: T) -> Dependency {
        Dependency {
            crate_name: self,
            location: Location::Local {
                local_path: local_path.into(),
            },
        }
    }

    fn dep_from_workspace(self) -> Dependency {
        Dependency {
            crate_name: self,
            location: Location::Workspace,
        }
    }

    fn into_dep(self) -> Dependency {
        Dependency {
            crate_name: self,
            location: Location::Remote {
                major_version_requirement: None,
            },
        }
    }
}

pub(crate) fn get_repo_root_folder() -> PathBuf {
    let repo_root_folder_bytes = Command::new("git")
        .args(&["rev-parse", "--show-toplevel"])
        .stdout(Stdio::piped())
        .output()
        .expect("Error getting the repo root folder")
        .stdout;
    let repo_root_folder_str = str::from_utf8(&repo_root_folder_bytes)
        .expect("Failed to read output from \"git rev-parse --show-toplevel\" command! Make sure you are running commands from inside the messari/substreams repo on your machine!");

    let repo_root_folder = PathBuf::from(repo_root_folder_str.replace("\n", ""));
    if !repo_root_folder.exists() {
        panic!("Error getting the repo root folder. Error:\nValue returned from running \"git rev-parse --show-toplevel\":\n{} - This folder does not exist!", repo_root_folder.to_string_lossy());
    }

    repo_root_folder
}

pub(crate) fn get_current_directory() -> PathBuf {
    env::current_dir().expect("Failed to get current directory!")
}

pub(crate) fn get_relative_path_from_root_folder(folder_path: &PathBuf) -> String {
    get_relative_path(&get_repo_root_folder(), folder_path)
}

pub(crate) fn get_relative_path(starting_path: &PathBuf, target_path: &PathBuf) -> String {
    pathdiff::diff_paths(target_path, starting_path)
        .unwrap()
        .to_string_lossy()
        .to_string()
        .replace("\\", "/")
}
