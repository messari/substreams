use std::process::{Command, Stdio};
use clap::Parser;

use crate::commands::update_versions::get_modified_substream_projects;
use crate::commands::upload_config_and_spkg_to_bucket::upload_config_and_spkg_file;
use crate::file_modification::cargo_toml::CargoToml;
use crate::file_modification::makefile::MakeFile;
use crate::file_modification::substreams_yaml::SubstreamsYaml;
use crate::utils::get_relative_path_from_root_folder;

#[derive(Parser)]
pub(crate) struct BuildAndUploadNewOrModifiedSubstreams {}

impl BuildAndUploadNewOrModifiedSubstreams {
    pub(crate) async fn execute(&self) {
        let modified_substream_projects = get_modified_substream_projects();

        for modified_substream_project in modified_substream_projects.iter() {
            if modified_substream_project.version_change.is_none() {
                panic!("There is a modified substream project without a version change! Project name: {}, relative path: {}", modified_substream_project.substream_name, get_relative_path_from_root_folder(&modified_substream_project.project_path));
            }
        }

        // Now we can go through, build and upload all the substream project that have been modified
        for modified_substream_project in modified_substream_projects.into_iter() {
            let project_path_str = modified_substream_project.project_path.to_str().unwrap();
            let makefile = MakeFile::load_from_file(&modified_substream_project.project_path.join("Makefile"));
            let spkg_path = makefile.get_spkg_target_path().expect(&format!("Unable to retrieve spkg target path for substream project: {}, Path: {}", modified_substream_project.substream_name, get_relative_path_from_root_folder(&modified_substream_project.project_path)));
            let config_file_path = modified_substream_project.project_path.join("spkg_config.json");
            if !config_file_path.exists() {
                panic!("The config file path: {}, you gave here does not exist! Please add a spkg_config.json file to your substreams project!", config_file_path.to_string_lossy());
            }

            println!("Building project: {}", modified_substream_project.substream_name);

            let build_output = Command::new("make")
                .args(&["-C", project_path_str, "build"])
                .output()
                .expect(&format!("Error building substream {}! Path: {}", modified_substream_project.substream_name, get_relative_path_from_root_folder(&modified_substream_project.project_path)));

            if !build_output.status.success() {
                panic!("Error building substream {}! Path: {}", modified_substream_project.substream_name, get_relative_path_from_root_folder(&modified_substream_project.project_path));
            }

            println!("Packaging project: {}", modified_substream_project.substream_name);

            let pack_output = Command::new("make")
                .args(&["-C", project_path_str, "pack"])
                .output()
                .expect(&format!("Error packaging substream {}! Path: {}", modified_substream_project.substream_name, get_relative_path_from_root_folder(&modified_substream_project.project_path)));

            if !pack_output.status.success() {
                panic!("Error packaging substream {}! Path: {}", modified_substream_project.substream_name, get_relative_path_from_root_folder(&modified_substream_project.project_path));
            }

            println!("Uploading spkg for substream: {}", modified_substream_project.substream_name);

            upload_config_and_spkg_file(&config_file_path, &spkg_path).await;
        }
    }
}