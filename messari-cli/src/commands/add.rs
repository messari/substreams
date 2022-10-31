use std::mem;
use std::path::PathBuf;
use clap::{Parser, ValueEnum};
use strum_macros::{EnumIter, EnumVariantNames};

use crate::abi::{add_abis, AbisArg};
use crate::protocols::ProtocolAndNetworkArgs;
use crate::utils::{get_current_directory, PathBufExt};
use crate::terminal_interface::select_from_enum;

#[derive(Parser)]
pub(crate) struct Add {
    pub(crate) add_operation_type: Option<AddOperationType>,
    #[arg(short = 'd', long, value_name = "Project Directory", help="Specify where the project is that you want to ABI to. Leave blank to use the current directory.")]
    pub(crate) project_dir: Option<String>,
    #[clap(flatten)]
    pub(crate) protocol_and_network_args: ProtocolAndNetworkArgs,
    #[clap(flatten)]
    pub(crate) abis_arg: AbisArg,
}

impl Add {
    pub(crate) fn execute(&mut self) {
        let operation_type = if let Some(operation_type) = mem::take(&mut self.add_operation_type) {
            operation_type
        } else {
            select_from_enum("Operation Type", Some(0))
        };

        let project_dir = if let Some(project_dir_string) = mem::take(&mut self.project_dir) {
            let mut project_dir = PathBuf::from(project_dir_string);
            if project_dir.is_relative() {
                // If relative it will always be treated as relative to the current directory
                project_dir = get_current_directory().join(project_dir);
                project_dir.clean_path(); // Would only fail here if the project does not exist
            } else {
                if !project_dir.exists() {
                    // Absolute paths have to already exist although relative paths are allowed to be created
                    panic!("Directory: {} does not exist!", project_dir.to_string_lossy());
                }
            }
            if !project_dir.is_dir() {
                panic!("Input: {}, is not a directory!", project_dir.to_string_lossy());
            }
            project_dir
        } else {
            // For add commands we don't ask you for the project directory if you haven't put is as a cmd line arg.
            // Instead we just assume that you want to add something to the project that is your current directory.
            get_current_directory()
        };

        // We need to make sure that the project selected is actually a substreams project
        if !project_dir.join("Cargo.toml").exists() {
            panic!("Project supplied: {}, is not a valid project. It contains not Cargo.toml file!", project_dir.to_string_lossy());
        }

        match operation_type {
            AddOperationType::Abi => {
                execute_add_abi(project_dir, &self.protocol_and_network_args, &self.abis_arg);
            }
        }
    }
}

#[derive(ValueEnum, EnumIter, EnumVariantNames, Clone)]
pub(crate) enum AddOperationType {
    Abi
}

fn execute_add_abi(project_dir: PathBuf, protocol_and_network_args: &ProtocolAndNetworkArgs, abis_arg: &AbisArg) {
    let protocol_and_network_info = protocol_and_network_args.get_info();
    add_abis(
        protocol_and_network_info,
        abis_arg,
        &project_dir,
        true,
    );
}