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

pub(crate) fn get_files_changed_from_master_branch() -> Vec<PathBuf> {
    let repo_root_folder = get_repo_root_folder();

    let changed_files_output_bytes = Command::new("git")
        .args(&["diff", "--name-status", "master"])
        .stdout(Stdio::piped())
        .output()
        .expect("Error getting the modified files between the current branch and master branch!")
        .stdout;

    let changed_files_output = str::from_utf8(&changed_files_output_bytes)
        .expect("Failed to read output from \"git diff --name-status main\" command! Make sure you are running commands from inside the messari/substreams repo on your machine!");

    let mut relative_file_paths: Vec<String> = Vec::new();
    for line in changed_files_output.split("\n").into_iter() {
        let mut line_iter = line.chars().into_iter();

        let mut spaces_seen = false;
        let mut relative_file_path = String::new();
        'a: while let Some(char) = line_iter.next() {
            if char == ' ' {
                spaces_seen = true;
            } else {
                if spaces_seen {
                    relative_file_path.push(char);
                    break 'a;
                }
            }
        }
        relative_file_path.push_str(&line_iter.collect::<String>());
        if relative_file_path.is_empty() {
            panic!("Failed to parse relative file path from \"git diff --name-status main\" command!")
        }
        relative_file_paths.push(relative_file_path);
    }

    relative_file_paths.into_iter().map(|relative_file_path| repo_root_folder.join(relative_file_path)).collect()
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
