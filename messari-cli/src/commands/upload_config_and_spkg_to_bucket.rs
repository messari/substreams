use std::{env, fs};
use std::path::PathBuf;
use s3::Bucket;
use s3::creds::Credentials;
use clap::Parser;
use prost::Message;

use crate::streaming_fast::streaming_config::{StreamingConfig, ToJsonL};
use crate::streaming_fast::streaming_fast_utils::get_file_size_string;
use crate::streaming_fast::streamingfast_dtos::{Module, Package};
use crate::streaming_fast::streamingfast_dtos::module::input::Input;

#[derive(Parser)]
pub(crate) struct UploadConfigAndSpkgToAws {
    project_folder_path: Option<String>,
    #[arg(short, long, value_name = "Spkg path", help="Defaults to determining path from checking the target folder. If more than one spkg in the target folder than the default retrieval will fail.")]
    spkg_path: Option<String>
}

impl UploadConfigAndSpkgToAws {
    /// Builds a debian binary of the messari cli and then uploads it to aws
    pub(crate) async fn execute(&self) {
        let project_folder_path = if let Some(project_folder_path) = self.project_folder_path.as_ref() {
            PathBuf::from(project_folder_path)
        } else {
            env::current_dir().unwrap()
        };

        let config_file_path = project_folder_path.join("spkg_config.json");
        if !config_file_path.exists() {
            panic!("The config file path: {}, you gave here does not exist! Please add a spkg_config.json file to your substreams project!", config_file_path.to_string_lossy());
        }

        let target_folder = project_folder_path.join("target");
        let spkg_file_path = if let Some(spkg_file) = self.spkg_path.as_ref() {
            let spkg_file_path = PathBuf::from(spkg_file);
            if !spkg_file_path.exists() {
                panic!("The spkg file path: {}, you gave here does not exist! Please specify a correct location for the spkg file you want to upload!", spkg_file);
            }
            spkg_file_path
        } else {
            let target_spkg_paths = target_folder.read_dir().unwrap().into_iter().filter_map(|entry| {
                let filepath = entry.unwrap().path();
                if filepath.ends_with(".spkg") {
                    Some(filepath)
                } else {
                    None
                }
            }).collect::<Vec<_>>();

            if target_spkg_paths.len() == 1 {
                panic!("No spkg file in target folder: {} - If not specifying spkg you need to ensure only one spkg is in the target folder!", target_folder.to_string_lossy());
            } else if target_spkg_paths.len() > 2 {
                panic!("More than one spkg file in target folder: {} - If not specifying spkg you need to ensure only one spkg is in the target folder!", target_folder.to_string_lossy());
            }

            target_spkg_paths.into_iter().next().unwrap()
        };

        upload_config_and_spkg_file(&config_file_path, &spkg_file_path).await;
    }
}

pub(crate) async fn upload_config_and_spkg_file(config_file_path: &PathBuf, spkg_file_path: &PathBuf) {
    let config_contents = fs::read_to_string(config_file_path).unwrap();
    let config_profiles = StreamingConfig::read_from_file_contents(&config_contents);

    let spkg_data = fs::read(spkg_file_path).unwrap();
    let spkg_file_stem = spkg_file_path.file_stem().unwrap().to_str().unwrap();
    assert_config_compatible_with_spkg(&config_profiles, spkg_data.as_slice(), spkg_file_stem);

    let bucket_name = "spkg-bucket";
    let region = "us-west-2".parse().unwrap();
    let credentials = Credentials::default().unwrap();
    let bucket = Bucket::new(bucket_name, region, credentials).unwrap();

    let response_data = bucket.put_object(format!("/config-files/{}.json", spkg_file_stem), config_profiles.to_jsonl().as_bytes()).await.unwrap();
    assert_eq!(response_data.status_code(), 200, "Response was unsuccessful!");
    println!("Config file: {}.json, has now been uploaded!\nFilesize: {}", spkg_file_stem, get_file_size_string(config_contents.as_bytes().len()));

    let response_data = bucket.put_object(format!("/spkg-files/{}.spkg", spkg_file_stem), spkg_data.as_slice()).await.unwrap();
    assert_eq!(response_data.status_code(), 200, "Response was unsuccessful!");
    println!("Spkg file: {}.spkg, has now been uploaded!\nFilesize: {}", spkg_file_stem, get_file_size_string(spkg_data.len()));
}

fn assert_config_compatible_with_spkg(config_profiles: &Vec<StreamingConfig>, spkg: &[u8], spkg_file_stem: &str) {
    let package = Package::decode(spkg).unwrap();
    let modules = package.modules.as_ref().unwrap();

    // To make sure a given config is compatible with a given spkg we need to make sure that the following are satisfied:
    //   - The output module exists
    //   - The module specified for a start block override exist
    //   - module has a param input if one is specified

    for config_profile in config_profiles {
        assert!(module_exists(config_profile.output_module.as_str(), modules.modules.as_ref()), "Output module: {} does not exist - config profile for {}.spkg is invalid!", config_profile.output_module.as_str(), spkg_file_stem);

        for start_block_override in config_profile.start_block_overrides.iter() {
            assert!(module_exists(start_block_override.module.as_str(), modules.modules.as_ref()), "Module: {} specified for start_block_override does not exist - config profile for {}.spkg is invalid!", start_block_override.module.as_str(), spkg_file_stem);
        }

        'a: for param_override in config_profile.param_overrides.iter() {
            for module in modules.modules.iter() {
                if module.name.as_str() == param_override.module.as_str() {
                    for input in module.inputs.iter() {
                        if let Input::Params(_) = input.input.as_ref().unwrap() {
                            continue 'a; // As long as there's a params field as an input that's good enough for us
                        }
                    }
                    panic!("Module: {} specified for param_override does not contain a param field - config profile for {}.spkg is invalid!", param_override.module.as_str(), spkg_file_stem);
                }
            }
            panic!("Module: {} specified for param_override does not exist - config profile for {}.spkg is invalid!", param_override.module.as_str(), spkg_file_stem);
        }
    }
}

fn module_exists(module_name: &str, modules: &Vec<Module>) -> bool {
    for module in modules.iter() {
        if module.name.as_str() == module_name {
            return true
        }
    }

    false
}