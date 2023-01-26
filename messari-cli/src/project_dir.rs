use std::mem;
use std::path::PathBuf;
use clap::Parser;

use crate::utils::get_current_directory;

#[derive(Parser)]
pub(crate) struct ProjectDirArg {
    #[arg(
    short = 'd',
    long,
    value_name = "Project Directory",
    help = "Specify where the project is that you want to add ABI to. Relative paths should start with \"./\" or \"../\". Leave blank to use the current directory."
    )]
    pub(crate) project_dir: Option<String>,
}

impl ProjectDirArg {
    pub(crate) fn get_project_dir(&mut self, has_to_be_substreams_project: bool) -> PathBuf {
        let project_dir = if let Some(project_dir_string) = mem::take(&mut self.project_dir) {
            let mut project_dir = PathBuf::from(project_dir_string);
            if project_dir.is_relative() {
                // If relative it will always be treated as relative to the current directory
                project_dir = get_current_directory().join(project_dir);
            } else {
                if !project_dir.exists() {
                    // Absolute paths have to already exist although relative paths are allowed to be created
                    panic!(
                        "Directory: {} does not exist!",
                        project_dir.to_string_lossy()
                    );
                }
            }
            project_dir
        } else {
            // For add commands we don't ask you for the project directory if you haven't put is as a cmd line arg.
            // Instead we just assume that you want to add something to the project that is your current directory.
            get_current_directory()
        };

        // We need to make sure that the project selected is actually a substreams project
        if has_to_be_substreams_project && !project_dir.join("Cargo.toml").exists() {
            panic!(
                "Project supplied: {}, is not a valid project. It contains not Cargo.toml file!",
                project_dir.to_string_lossy()
            );
        }

        project_dir
    }
}