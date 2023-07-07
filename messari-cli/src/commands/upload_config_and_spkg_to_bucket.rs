use std::{env, fs};
use std::io::Read;
use std::path::PathBuf;
use std::collections::HashMap;
use s3::Bucket;
use s3::creds::Credentials;
use clap::Parser;
use prost::Message;
use serde::{Deserialize};

use crate::streaming_fast::streaming_config::{Chain, ParamOverride, StartBlockOverride, StreamingConfig, ToJsonL};
use crate::streaming_fast::streaming_fast_utils::get_file_size_string;
use crate::streaming_fast::streamingfast_dtos::{Module, Package};
use crate::streaming_fast::streamingfast_dtos::module::input::Input;

#[derive(Parser)]
pub(crate) struct UploadConfigAndSpkgToAws {
    substream: String
}

#[derive(Deserialize, Clone)]
pub(crate) struct SubstreamDeployment {
    name: String,
    network: String,
    params: HashMap<String, String>,
    startBlocks: HashMap<String, u64>,
}

#[derive(Deserialize, Clone)]
pub(crate) struct SubstreamConfig {
    name: String,
    path: String,
    outputModules: Vec<String>,
    subgraphModule: String,
    deployments: Vec<SubstreamDeployment>,
}

impl SubstreamConfig {
    pub(crate) fn into_streaming_configs(&self) -> Vec<StreamingConfig> {
        let mut confs = vec![];
        for deployment in &self.deployments {
            let mut start_block_overrides = vec![];
            for (module_name, start_block) in &deployment.startBlocks {
                start_block_overrides.push(StartBlockOverride {
                    module: module_name.to_owned(),
                    block_number: start_block.to_owned(),
                });
            }

            let mut param_overrides = vec![];
            for (module_name, value) in &deployment.params {
                param_overrides.push(ParamOverride {
                    module: module_name.to_owned(),
                    value: value.to_owned(),
                });
            }

            let chain = match deployment.network.as_str() {
                "mainnet" => Chain::EthereumMainnet,
                "polygon" => Chain::Polygon,
                _ => panic!("Unknown chain: {}", deployment.network),
            };
            confs.push(StreamingConfig {
                name: self.name.clone(),
                output_module: self.outputModules[0].clone(),
                chain_override: Some(chain),
                substream_name_override: None,
                start_block_overrides,
                param_overrides,
            });

        }
        confs
    }
}

impl UploadConfigAndSpkgToAws {
    /// Builds a debian binary of the messari cli and then uploads it to aws
    pub(crate) async fn execute(&self) {
        let config = find_substream_config(self.substream.clone());

        let mut project_folder_path = PathBuf::from(env::current_dir().unwrap());
        project_folder_path.push("config");
        project_folder_path.push(config.path.clone());
        project_folder_path.push("target");

        let spkg_file_path = {
            let target_spkg_paths = project_folder_path.read_dir().unwrap().into_iter().filter_map(|entry| {
                let filepath = entry.unwrap().path();
                if filepath.to_string_lossy().ends_with(".spkg") {
                    Some(filepath)
                } else {
                    None
                }
            }).collect::<Vec<_>>();

            if target_spkg_paths.len() == 0 {
                panic!("No spkg file in target folder: {} - If not specifying spkg you need to ensure only one spkg is in the target folder!", project_folder_path.to_string_lossy());
            } else if target_spkg_paths.len() > 2 {
                panic!("More than one spkg file in target folder: {} - If not specifying spkg you need to ensure only one spkg is in the target folder!", project_folder_path.to_string_lossy());
            }

            target_spkg_paths.into_iter().next().unwrap()
        };

        upload_config_and_spkg_file(&config, &spkg_file_path).await;
    }
}

pub(crate) async fn upload_config_and_spkg_file(config: &SubstreamConfig, spkg_file_path: &PathBuf) {
    let spkg_data = fs::read(spkg_file_path).unwrap();
    let spkg_file_stem = spkg_file_path.file_stem().unwrap().to_str().unwrap();

    let config_profiles = config.into_streaming_configs();
    assert_config_compatible_with_spkg(&config_profiles, spkg_data.as_slice(), spkg_file_stem);

    let bucket_name = "data-warehouse-load-427049689281-dev";
    let region = "us-west-2".parse().unwrap();
    let credentials = Credentials::default().unwrap();
    let bucket = Bucket::new(bucket_name, region, credentials).unwrap();

    let response_data = bucket.put_object(format!("/configs/{}.json", spkg_file_stem), config_profiles.to_jsonl().as_bytes()).await.unwrap();
    assert_eq!(response_data.status_code(), 200, "Response was unsuccessful!");
    println!("Config file: {}.json, has now been uploaded!\n", spkg_file_stem);

    let response_data = bucket.put_object(format!("/spkgs/{}.spkg", spkg_file_stem), spkg_data.as_slice()).await.unwrap();
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

fn find_substream_config(substream: String) -> SubstreamConfig {
    // Read the JSON file
    let mut file = fs::File::open("./config/params.json").expect("Unable to open the file. Are you at the root of the repo?");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse the JSON
    let confs: Vec<SubstreamConfig> = serde_json::from_str(&contents).expect("Unable to parse JSON");
    let config = confs.iter().find(|conf| conf.name == substream).expect("Unable to find given substream in params.json");
    config.clone()
}
