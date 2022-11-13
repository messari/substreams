use clap::{Parser, ValueEnum};
use std::mem;
use std::path::PathBuf;
use strum_macros::{EnumIter, EnumVariantNames};

use crate::abi::{add_abis, AbisArgs};
use crate::file_modification::cargo_toml::CargoToml;
use crate::file_modification::file_contents_modifier::{
    create_dir_all, safely_modify_file_contents, File, FileContentsModification,
};
use crate::file_modification::makefile::MakeFile;
use crate::file_modification::substreams_yaml::{Input, InputType, Module, SubstreamsYaml, UpdatePolicy};
use crate::protocols::ProtocolAndNetworkArgs;
use crate::terminal_interface::{get_input, select_from_enum};
use crate::utils::{get_current_directory, get_repo_root_folder};

#[derive(Parser)]
pub(crate) struct Init {
    #[arg(short = 'n', long, value_name = "Project Name")]
    pub(crate) project_name: Option<String>,
    #[arg(
        short = 'd',
        long,
        value_name = "Base Directory",
        help = "Specify a directory for the project directory to be created in. Relative paths should start with \"./\" or \"../\". Leave blank for project directory to be created in current directory."
    )]
    pub(crate) base_dir: Option<String>,
    #[arg(short = 'i', long, value_name = "Project Description")]
    pub(crate) project_description: Option<String>,
    #[arg(short = 't', long, value_name = "Project Type")]
    #[clap(value_enum)]
    pub(crate) project_type: Option<ProjectType>,
    #[clap(flatten)]
    pub(crate) protocol_and_network_args: ProtocolAndNetworkArgs,
    #[clap(flatten)]
    pub(crate) abis_arg: AbisArgs,
}

#[derive(ValueEnum, Clone, EnumIter, EnumVariantNames, PartialEq)]
pub(crate) enum ProjectType {
    SubstreamsProject,
    LibraryProject,
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
            get_input(
                "Base Directory (Leave blank for current directory)",
                Some("Base Directory"),
                true,
            )
        };

        let base_dir = if base_dir_string.is_empty() {
            get_current_directory()
        } else {
            let mut base_dir = PathBuf::from(base_dir_string);
            if base_dir.is_relative() {
                // If relative it will always be treated as relative to the current directory
                base_dir = get_current_directory().join(base_dir);
            } else {
                if !base_dir.exists() {
                    // Absolute paths have to already exist although relative paths are allowed to be created
                    panic!("Directory: {} does not exist!", base_dir.to_string_lossy());
                }
            }
            base_dir
        };

        let project_dir = base_dir.join(&project_name);

        if project_dir.exists() {
            panic!(
                "Project you are trying to create already exists!! Project filepath: {}",
                project_dir.to_string_lossy()
            );
        }

        let project_description =
            if let Some(project_description) = mem::take(&mut self.project_description) {
                Some(project_description)
            } else {
                let project_description_input = get_input(
                    "Project Description (Leave blank to skip)",
                    Some("Project Description"),
                    true,
                );
                if project_description_input.is_empty() {
                    None
                } else {
                    Some(project_description_input)
                }
            };

        if !(self.project_type == Some(ProjectType::SubstreamsProject)
            || self.protocol_and_network_args.protocol_type.is_some()
            || self.protocol_and_network_args.network.is_some()
            || self.abis_arg.abis.is_some())
        {
            // User has not given enough information for us to determine what sort of project they want to build yet so we need to find out
            let project_type: ProjectType = select_from_enum("Project type", Some(0));

            if project_type == ProjectType::LibraryProject {
                create_library_project(project_name, project_description, project_dir);
                return;
            }
        }

        create_substreams_project(project_name, project_description, &project_dir);

        let protocol_and_network_info = self.protocol_and_network_args.get_info();
        add_abis(
            protocol_and_network_info,
            &self.abis_arg,
            &project_dir,
            false,
        );
    }
}

fn create_library_project(
    project_name: String,
    project_description: Option<String>,
    project_dir: PathBuf,
) {
    // Root files
    let root_folder = get_repo_root_folder();
    let root_cargo_toml = root_folder.join("Cargo.toml");

    // Project files
    let src_dir = project_dir.join("src");
    let lib_file = src_dir.join("lib.rs");
    let project_cargo_toml = project_dir.join("Cargo.toml");

    let mut root_cargo_toml_contents = CargoToml::load_from_file(&root_cargo_toml);
    root_cargo_toml_contents.add_project_to_workspace(&project_dir);

    let mut operations = create_dir_all(project_dir);
    operations.extend(vec![
        FileContentsModification::CreateFolder(src_dir),
        FileContentsModification::UpdateFile(File {
            file_contents: root_cargo_toml_contents.get_file_contents(),
            filepath: root_cargo_toml,
        }),
        FileContentsModification::CreateFile(File {
            filepath: lib_file,
            file_contents: "".to_string(),
        }),
        FileContentsModification::CreateFile(File {
            file_contents: CargoToml::new(
                project_name,
                project_description,
                ProjectType::LibraryProject,
                &project_cargo_toml,
            )
            .get_file_contents(),
            filepath: project_cargo_toml,
        }),
    ]);

    safely_modify_file_contents(operations);
}

fn create_substreams_project(
    project_name: String,
    project_description: Option<String>,
    project_dir: &PathBuf,
) {
    const TEMP_LIB_FILE_SCAFFOLDING: bool = true;

    // Root files
    let root_folder = get_repo_root_folder();
    let root_cargo_toml = root_folder.join("Cargo.toml");
    let root_makefile = root_folder.join("Makefile");

    // Project files
    let src_dir = project_dir.join("src");
    let lib_file = src_dir.join("lib.rs");
    let project_cargo_toml = project_dir.join("Cargo.toml");
    let project_makefile = project_dir.join("Makefile");
    let substreams_yaml = project_dir.join("substreams.yaml");

    let mut root_cargo_toml_contents = CargoToml::load_from_file(&root_cargo_toml);
    root_cargo_toml_contents.add_project_to_workspace(&project_dir);

    let mut root_makefile_contents = MakeFile::load_from_file(&root_makefile);
    root_makefile_contents.add_project_to_build_all_command(project_dir);
    if TEMP_LIB_FILE_SCAFFOLDING {
        root_makefile_contents.add_project_to_run_all_command(project_dir);
    }

    let mut project_makefile_contents = MakeFile::new(&project_makefile);
    project_makefile_contents.add_build_operation();
    if TEMP_LIB_FILE_SCAFFOLDING {
        project_makefile_contents.add_example_run_operation();
    }

    let mut yaml_contents = SubstreamsYaml::new(project_name.as_str(), &substreams_yaml);
    if TEMP_LIB_FILE_SCAFFOLDING {
        yaml_contents.add_module(Module::map("map_example".to_string(), Some(14690152), vec![Input { input_type: InputType::Source, input_value: "sf.ethereum.type.v2.Block".to_string() }], "proto:messari.erc20.v1.TransferEvents".to_string()));
        yaml_contents.add_module(Module::store("store_example".to_string(), None, UpdatePolicy::Set, "proto:messari.erc20.v1.TransferEvents".to_string(), vec![Input { input_type: InputType::Map, input_value: "map_example".to_string() }]));
        yaml_contents.add_protobuf_files(vec![get_repo_root_folder().join("common").join("proto").join("erc20.proto")]);
    }

    let mut operations = create_dir_all(project_dir.clone());
    operations.extend(vec![
        FileContentsModification::CreateFolder(src_dir),
        FileContentsModification::UpdateFile(File {
            filepath: root_cargo_toml,
            file_contents: root_cargo_toml_contents.get_file_contents(),
        }),
        FileContentsModification::UpdateFile(File {
            filepath: root_makefile,
            file_contents: root_makefile_contents.get_file_contents(),
        }),
        FileContentsModification::CreateFile(File {
            filepath: lib_file,
            file_contents: "".to_string(),
        }),
        FileContentsModification::CreateFile(File {
            file_contents: SubstreamsYaml::new(project_name.as_str(), &substreams_yaml)
                .get_file_contents(),
            filepath: substreams_yaml,
        }),
        FileContentsModification::CreateFile(File {
            file_contents: CargoToml::new(
                project_name,
                project_description,
                ProjectType::SubstreamsProject,
                &project_cargo_toml,
            )
            .get_file_contents(),
            filepath: project_cargo_toml,
        }),
        FileContentsModification::CreateFile(File {
            filepath: project_makefile,
            file_contents: project_makefile_contents.get_file_contents(),
        }),
    ]);

    safely_modify_file_contents(operations);
}
