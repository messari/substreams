use std::{env, fs, mem};
use std::path::PathBuf;
use clap::{Parser, ValueEnum};
use dialoguer::{Input, Select};
use strum_macros::{EnumIter, EnumVariantNames};

use crate::abi::{AbisArg, add_abis, download_abi};
use crate::automapper::add_block_to_object_mapping_code;
use crate::cargo::add_member_to_workspace;
use crate::protocols::{Protocol, ProtocolAndNetworkArgs, ProtocolType};
use crate::cmd_helper::{get_input, select_from_enum, select_from_values};
use crate::file_contents_modifier::{File, FileContentsModification, safely_modify_file_contents};
use crate::utils::get_repo_root_folder;

#[derive(Parser)]
pub(crate) struct Init {
    #[arg(short='n', long, value_name = "Project Name")]
    pub(crate) project_name: Option<String>,
    #[arg(short='d', long, value_name = "Base Directory", help="Specify a directory for project to get built. Defaults to current directory. (Base directory needs to already exist)")]
    pub(crate) base_dir: Option<String>,
    #[arg(short='t', long, value_name = "Project Type")]
    #[clap(value_enum)]
    pub(crate) project_type: Option<ProjectType>,
    #[clap(flatten)]
    pub(crate) protocol_and_network_args: ProtocolAndNetworkArgs,
    #[clap(flatten)]
    pub(crate) abis_arg: AbisArg
}

#[derive(ValueEnum, Clone, EnumIter, EnumVariantNames, PartialEq)]
pub(crate) enum ProjectType {
    SubstreamsProject,
    LibraryProject
}

impl Init {
    pub(crate) fn execute(&mut self) {
        let project_name = if let Some(project_name) = mem::take(&mut self.project_name) {
            project_name
        } else {
            get_input("Project Name", None, false)
        };

        let base_dir_string = if let Some(base_dir_string) = mem::take(&mut self.base_dir) {
            base_dir_string
        } else {
            get_input("Base Directory (Leave blank for current directory)", Some("Base Directory"), true);
        };

        let base_dir = if base_dir_string.is_empty() {
            env::current_dir().unwrap()
        } else {
            let base_dir = PathBuf::from(base_dir_string);
            if !base_dir.exists() {
                panic!("Directory: {} does not exist!", base_dir.to_string_lossy());
            }
            if !base_dir.is_dir() {
                panic!("Input: {}, is not a directory!", base_dir.to_string_lossy());
            }
            base_dir
        };

        let project_dir = base_dir.join(&project_name);

        if self.project_type==Some(ProjectType::SubstreamsProject) || self.protocol_and_network_args.protocol_type.is_some() || self.protocol_and_network_args.network.is_some() || self.abis_arg.abis.is_some() {
            // User has not given enough information for us to determine what sort of project they want to build yet so we need to find out
            let project_type: ProjectType = select_from_enum("Project type", Some(0));

            if project_type == ProjectType::LibraryProject {
                create_library_project(project_name, project_dir);
                return;
            }
        }

        create_substreams_project(project_name, &project_dir);

        let protocol_and_network_info = self.protocol_and_network_args.get_info();
        add_abis(protocol_and_network_info, &self.abis_arg, &project_dir);
    }
}

fn create_library_project(project_name: String, project_dir: PathBuf) {
    let src_dir = project_dir.join("src");
    let lib_file = src_dir.join("lib.rs");
    let cargo_toml_file = project_dir.join("Cargo.toml");
    let root_folder = get_repo_root_folder();
    let root_cargo_toml = root_folder.join("Cargo.toml");

    let mut operations = vec![
        FileContentsModification::CreateFolder(project_dir),
        FileContentsModification::CreateFolder(src_dir),
        FileContentsModification::UpdateFile(File {
            filepath: root_cargo_toml,
            file_contents: add_member_to_workspace(&project_name, &root_cargo_toml)
        }),
        FileContentsModification::CreateFile(File {
            filepath: lib_file,
            file_contents: "".to_string(),
        }),
        FileContentsModification::CreateFile(File {
            filepath: cargo_toml_file,
            file_contents: format!("[package]\nname = \"{0}\"\nversion = "0.1.0"\nedition = \"2021\"\nrepository = \"https://github.com/messari/substreams/{0}\"\n", project_name),
        }),
    ];

    safely_modify_file_contents(operations);
}

fn create_substreams_project(project_name: String, project_dir: &PathBuf) {
    let src_dir = project_dir.join("src");
    let lib_file = src_dir.join("lib.rs");
    let cargo_toml_file = project_dir.join("Cargo.toml");
    let root_folder = get_repo_root_folder();
    let root_cargo_toml = root_folder.join("Cargo.toml");

    let mut operations = vec![
        FileContentsModification::CreateFolder(project_dir.clone()),
        FileContentsModification::CreateFolder(src_dir),
        FileContentsModification::UpdateFile(File {
            filepath: root_cargo_toml,
            file_contents: add_member_to_workspace(&project_name, &root_cargo_toml)
        }),
        FileContentsModification::CreateFile(File {
            filepath: lib_file,
            file_contents: "".to_string(),
        }),
        FileContentsModification::CreateFile(File {
            filepath: cargo_toml_file,
            file_contents: format!("[package]\nname = \"{0}\"\nversion = "0.1.0"\nedition = \"2021\"\nrepository = \"https://github.com/messari/substreams/{0}\"\n\n[lib]\ncrate-type = [\"cdylib\"]\n", project_name),
        }),
    ];

    safely_modify_file_contents(operations);
}