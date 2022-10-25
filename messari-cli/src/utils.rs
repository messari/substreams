use std::path::PathBuf;
use std::process::{Command, Stdio};
use regex::Regex;

pub(crate) trait StrExt {
    fn is_valid_abi_address(&self) -> bool;
}

impl StrExt for &str {
    fn is_valid_abi_address(&self) -> bool {
        let re_contract_address = Regex::new(r"^0x[A-Za-z0-9]{40}$").unwrap();
        re_contract_address.is_match(self)
    }
}

pub(crate) fn get_repo_root_folder() -> PathBuf {
    let repo_root_folder_string = match Command::new("git").args(&["rev-parse", "--show-toplevel"]).stdout(Stdio::piped()).output() {
        Ok(repo_root_folder_string) => repo_root_folder_string,
        Err(error) => panic!("Error finding the repo root folder. Error:\n{}", error)
    };

    let repo_root_folder = PathBuf::from(repo_root_folder_string);
    if !repo_root_folder.exists() {
        panic!("Error getting the repo root folder. Error:\nValue returned from running \"git rev-parse --show-toplevel\":\n{} - This folder does not exist!", repo_root_folder.to_string_lossy());
    }

    repo_root_folder
}