use std::{env, fs};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::process::Command;
use s3::Bucket;
use s3::creds::Credentials;
use clap::Parser;
use dialoguer::Editor;
use prost::Message;
use reqwest::StatusCode;

use crate::file_modification::cargo_toml::CargoToml;
use crate::file_modification::file_contents_modifier::{File, FileContentsModification, safely_modify_file_contents};
use crate::file_modification::substreams_yaml::{SubstreamsYaml, VersionType};

use crate::streaming_fast::streaming_config::{StreamingConfig, ToJsonL};
use crate::streaming_fast::streaming_fast_utils::get_file_size_string;
use crate::streaming_fast::streamingfast_dtos::{Module, Package};
use crate::streaming_fast::streamingfast_dtos::module::input::Input;
use crate::utils::{get_files_changed_from_master_branch, get_relative_path_from_root_folder, get_repo_root_folder};

#[derive(Parser)]
pub(crate) struct UpdateVersions {
    version_type: Option<VersionType>
}

impl UpdateVersions {
    pub(crate) fn execute(&self) {
        let default_version_type = if let Some(version_type) = self.version_type.as_ref() {
            version_type.clone()
        } else {
            VersionType::Patch
        };

        let version_changes = get_version_changes(default_version_type);

        let operations = version_changes.into_iter().map(|version_change| {
            let mut substreams_yaml = SubstreamsYaml::load_from_file(&version_change.substreams_yaml_path);
            substreams_yaml.modify_version(version_change.version_change_type, version_change.existing_version_increment);

            FileContentsModification::UpdateFile(File {
                file_contents: substreams_yaml.get_file_contents(),
                filepath: version_change.substreams_yaml_path,
            })
        }).collect::<Vec<_>>();

        safely_modify_file_contents(operations);
    }
}

struct VersionChange {
    version_change_type: VersionType,
    substreams_yaml_path: PathBuf,
    existing_version_increment: Option<VersionType>
}

fn get_version_changes(default_version_type: VersionType) -> Vec<VersionChange> {
    let modified_substream_projects = get_modified_substream_projects();

    const SPECIFY_VERSION_INFO_LINE: &str = "Specify version type to update using M,K,or P for the modified substream projects below. \
                                            (M represents a major update, K a minor update, and P for a patch update):";

    let (already_versioned_spkgs, unversioned_spkgs): (Vec<SubstreamProject>, Vec<SubstreamProject>) = modified_substream_projects.into_iter().partition(|substream_project| substream_project.version_change.is_some());

    let already_versioned_line_infos = already_versioned_spkgs.iter().map(|substream_project| format!("{} - {}", substream_project.substream_name, get_relative_path_from_root_folder(&substream_project.project_path))).collect::<Vec<_>>();
    let unversioned_line_infos = unversioned_spkgs.iter().map(|substream_project| format!("{} - {}", substream_project.substream_name, get_relative_path_from_root_folder(&substream_project.project_path))).collect::<Vec<_>>();

    let already_versioned_lines = already_versioned_line_infos.iter().map(|version_line_info| format!("{}:{}", default_version_type.get_version_char(), version_line_info)).collect::<Vec<_>>();
    let unversioned_lines = unversioned_line_infos.iter().zip(unversioned_spkgs.iter()).map(|(version_line_info, substream_project)| format!("{}:{}", substream_project.version_change.as_ref().unwrap().get_version_char(), version_line_info)).collect::<Vec<_>>();

    const VERSIONS_UPDATED_LINE: &str = "Spkgs that have already had their versions updated:";

    let mut edit_response = Editor::new().edit(&format!("{}\n\n{}\n\n{}\n\n{}", SPECIFY_VERSION_INFO_LINE, unversioned_lines.join("\n"), VERSIONS_UPDATED_LINE, already_versioned_lines.join("\n"))).expect("Failed to get versioning edits!").expect("Failed to get versioning edits!");

    edit_response = edit_response.replace("\n\n", "\n");

    const EDIT_RESPONSE_ERROR: &str = "Only the version update letters should be edited here (with M, K or P)! You edited something else other than the version update types in the edit!";

    let mut edit_line_iter = edit_response.split("\n").into_iter();

    assert_eq!(edit_line_iter.next().unwrap(), SPECIFY_VERSION_INFO_LINE);

    let mut version_changes = Vec::new();
    for (line, (unversioned_line, (unversioned_line_info, substreams_project))) in edit_line_iter.clone().zip(unversioned_lines.into_iter().zip(unversioned_line_infos.iter().zip(unversioned_spkgs.into_iter()))) {
        if line == unversioned_line {
            version_changes.push(VersionChange {
                version_change_type: default_version_type.clone(),
                substreams_yaml_path: substreams_project.substreams_yaml_file,
                existing_version_increment: None,
            });
        } else {
            assert!(line.ends_with(unversioned_line_info), "{}", EDIT_RESPONSE_ERROR);
            assert_eq!(line.len(), unversioned_line_info.len()+1, "{}", EDIT_RESPONSE_ERROR);
            version_changes.push(VersionChange {
                version_change_type: VersionType::from_char(line.chars().next().unwrap()),
                substreams_yaml_path: substreams_project.substreams_yaml_file,
                existing_version_increment: None,
            });
        }
    }

    assert_eq!(edit_line_iter.next().unwrap(), VERSIONS_UPDATED_LINE);

    for (line, (already_versioned_line, (already_versioned_line_info, substream_project))) in edit_line_iter.clone().skip(unversioned_line_infos.len()).zip(already_versioned_lines.iter().zip(already_versioned_line_infos.iter().zip(already_versioned_spkgs.into_iter()))) {
        if line != already_versioned_line {
            assert!(line.ends_with(already_versioned_line_info), "{}", EDIT_RESPONSE_ERROR);
            assert_eq!(line.len(), already_versioned_line_info.len()+1, "{}", EDIT_RESPONSE_ERROR);
            version_changes.push(VersionChange {
                version_change_type: VersionType::from_char(line.chars().next().unwrap()),
                substreams_yaml_path: substream_project.substreams_yaml_file,
                existing_version_increment: substream_project.version_change,
            });
        }
    }

    assert!(edit_line_iter.skip(unversioned_line_infos.len()+already_versioned_line_infos.len()).next().is_none(), "{}", EDIT_RESPONSE_ERROR);

    version_changes
}

fn get_version_change_type(master_version: &str, branch_version: &str) -> Option<VersionType> {
    // Assumes versions are in the form X.Y.Z:
    const VERSION_TYPES: [VersionType; 3] = [VersionType::Major, VersionType::Minor, VersionType::Patch];
    for (version_type, (master, branch)) in VERSION_TYPES.into_iter().zip(master_version.split(".").into_iter().zip(branch_version.split(".").into_iter())) {
        if master != branch {
            let master_num = master.parse::<u8>().unwrap();
            let branch_num = branch.parse::<u8>().unwrap();
            if master_num+1 == branch_num {
                return Some(version_type);
            } else {
                panic!("{} version has been modified between versions although it has not been incremented by one!!", version_type);
            }
        }
    }

    None
}

pub(crate) struct SubstreamProject {
    pub(crate) substream_name: String,
    pub(crate) substreams_yaml_file: PathBuf,
    pub(crate) project_path: PathBuf,
    branch_version: String,
    pub(crate) version_change: Option<VersionType>, // If the version has already been changed it will tell you which version type was changed
}

pub(crate) fn get_modified_substream_projects() -> Vec<SubstreamProject> {
    let all_substream_projects = get_substream_projects();

    let mut spkg_project_folders = Vec::new();
    let mut local_crate_folders = HashSet::new();
    for substream_project in all_substream_projects.iter() {
        spkg_project_folders.push(substream_project.project_path.clone());
        for local_crate_folder in substream_project.local_crate_dependencies.iter() {
            local_crate_folders.insert(local_crate_folder);
        }
    }

    let mut modified_substream_yamls = Vec::new();
    let mut modified_crates = HashSet::new();
    let mut directly_modified_substream_projects = HashSet::new();
    'a: for modified_file in get_files_changed_from_master_branch().into_iter() {
        if modified_file.ends_with("substreams.yaml") {
            if modified_file.exists() {
                // We will use these later on to see if the version of an spkg has already been modified
                modified_substream_yamls.push(modified_file);
            }
        } else {
            for spkg_project_folder in spkg_project_folders.iter() {
                if modified_file.starts_with(spkg_project_folder) {
                    directly_modified_substream_projects.insert(spkg_project_folder);
                    continue 'a;
                }
            }

            for local_crate_folder in local_crate_folders.iter() {
                if modified_file.starts_with(local_crate_folder) {
                    modified_crates.insert(local_crate_folder);
                    continue 'a;
                }
            }
        }
    }

    let mut spkg_dependents: HashMap<PathBuf, HashSet<PathBuf>> = HashMap::new();
    for substream_project in all_substream_projects.iter() {
        for spkg_dependency in substream_project.spkg_dependencies.iter() {
            if spkg_dependents.contains_key(spkg_dependency) {
                spkg_dependents.get_mut(spkg_dependency).unwrap().insert(substream_project.project_path.clone());
            } else {
                spkg_dependents.insert(spkg_dependency.clone(), HashSet::from([substream_project.project_path.clone()]));
            }
        }

        for local_crate_dependency in substream_project.local_crate_dependencies.iter() {
            if modified_crates.contains(&local_crate_dependency) {
                directly_modified_substream_projects.insert(&substream_project.project_path);
            }
        }
    }

    fn add_all_modified_substream_projects(modified_substream_project: &PathBuf, all_modified_substream_projects: &mut HashSet<PathBuf>, spkg_dependents: &HashMap<PathBuf, HashSet<PathBuf>>) {
        all_modified_substream_projects.insert(modified_substream_project.clone());
        if spkg_dependents.contains_key(modified_substream_project) {
            for substream_project in spkg_dependents.get(modified_substream_project).unwrap().iter() {
                add_all_modified_substream_projects(substream_project, all_modified_substream_projects, spkg_dependents);
            }
        }
    }

    let mut all_modified_substream_projects = HashSet::new();
    for directly_modified_substream_project in directly_modified_substream_projects.into_iter() {
        add_all_modified_substream_projects(directly_modified_substream_project, &mut all_modified_substream_projects, &spkg_dependents);
    }

    let root_repo_length = get_repo_root_folder().to_str().unwrap().len();
    all_substream_projects.into_iter().filter_map(|substream_project| {
        if all_modified_substream_projects.contains(&substream_project.project_path) {
            let version_change = if modified_substream_yamls.contains(&substream_project.substreams_yaml_file) {
               // We will check the substreams yaml file from main to see if there is a different version set in the master branch
               let file_path = substream_project.substreams_yaml_file.to_str().unwrap();
               let relative_file_path = &file_path[root_repo_length..file_path.len()];

               let master_yaml_url = format!("https://raw.githubusercontent.com/messari/substreams/master/{}", relative_file_path);

               let response  = reqwest::blocking::get(&master_yaml_url).expect(&format!("Unable to get response when retrieving master yaml file from: {}", master_yaml_url));

               if response.status() == StatusCode::NOT_FOUND {
                   Some(substream_project.get_starting_version_type().expect(&format!("New substreams project: {} is not starting out at an appropriate version number (version: {})! Only one of [v0.0.1, 0.0.1, v0.1.0, 0.1.0, v1.0.0, 1.0.0] should be used for a new substream project!", substream_project.substream_name, substream_project.version)))
               } else {
                   let response_text = response.text().expect(&format!("Unable to get response text when retrieving master yaml file from: {}", master_yaml_url));
                   let master_yaml: SubstreamsYaml = response_text.as_str().into();
                   get_version_change_type(&master_yaml.get_version(), &substream_project.version)
               }
            } else {
                Some(substream_project.get_starting_version_type().expect(&format!("New substreams project: {} is not starting out at an appropriate version number (version: {})! Only one of [v0.0.1, 0.0.1, v0.1.0, 0.1.0, v1.0.0, 1.0.0] should be used for a new substream project!", substream_project.substream_name, substream_project.version)))
            };

            Some(SubstreamProject {
                substream_name: substream_project.substream_name,
                substreams_yaml_file: substream_project.substreams_yaml_file,
                project_path: substream_project.project_path,
                version_change,
                branch_version: substream_project.version,
            })
        } else {
            None
        }
    }).collect()
}

struct SubstreamInfo {
    substream_name: String,
    project_path: PathBuf,
    substreams_yaml_file: PathBuf,
    version: String,
    spkg_dependencies: Vec<PathBuf>,
    local_crate_dependencies: Vec<PathBuf>
}

impl SubstreamInfo {
    /// If a new substream has been created we want to see what the starting version type is
    /// ie. if 1.0.0 then it would be a major version start, (etc for others..)
    fn get_starting_version_type(&self) -> Option<VersionType> {
        match self.version.as_str() {
            "v1.0.0" | "1.0.0" => Some(VersionType::Major),
            "v0.1.0" | "0.1.0" => Some(VersionType::Minor),
            "v0.0.1" | "0.0.1" => Some(VersionType::Patch),
            _ => None,
        }
    }
}

fn get_substream_projects() -> Vec<SubstreamInfo> {
    get_all_substream_yaml_files().into_iter().map(|substreams_yaml_file| {
        let project_path = substreams_yaml_file.parent().unwrap().to_path_buf();

        let cargo_file = CargoToml::load_from_file(&project_path.join("Cargo.toml"));
        let yaml_file = SubstreamsYaml::load_from_file(&substreams_yaml_file);

        SubstreamInfo {
            substream_name: yaml_file.get_substream_name(),
            project_path,
            substreams_yaml_file,
            version: yaml_file.get_version(),
            spkg_dependencies: yaml_file.get_local_spkg_dependencies(),
            local_crate_dependencies: cargo_file.get_local_dependencies(),
        }
    }).collect()
}

fn get_all_substream_yaml_files() -> Vec<PathBuf> {
    fn recursive_get_yaml_files(current_folder: PathBuf, yaml_files: &mut Vec<PathBuf>) {
        for result in current_folder.read_dir().unwrap() {
            let entry = result.unwrap();
            let file_type = entry.file_type().unwrap();
            if file_type.is_file() {
                if entry.file_name() == "substreams.yaml" {
                    yaml_files.push(entry.path());
                }
            } else if file_type.is_dir() {
                if entry.file_name() != "target" && !entry.file_name().to_str().unwrap().starts_with(".") {
                    recursive_get_yaml_files(entry.path(), yaml_files);
                }
            }
        }
    }

    let repo_root_folder = get_repo_root_folder();
    let mut yaml_files = Vec::new();

    recursive_get_yaml_files(repo_root_folder, &mut yaml_files);

    yaml_files
}