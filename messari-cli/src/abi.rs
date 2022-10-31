use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use serde_json::Value;

use crate::automapper::add_block_to_object_mapping_code;
use crate::file_modification::cargo_toml::CargoToml;
use crate::terminal_interface::{get_input, get_success_message, Spinner};
use crate::file_modification::file_contents_modifier::{safely_modify_file_contents, File, FileContentsModification};
use crate::protocols::{
    Protocol, ProtocolAndNetworkInfo, ProtocolType, SupportedAbiAdditionMethods,
};
use crate::utils::{StaticStrExt, StrExt};

#[derive(Parser)]
pub(crate) struct AbisArg {
    #[arg(
        short,
        long,
        value_name = "ABIs",
        help = "ABIs can be specified as local file paths or as contract addresses. Multiple can be specified at once with comma separation"
    )]
    pub(crate) abis: Option<String>,
}

impl AbisArg {
    pub(crate) fn get_abi_infos(&self, protocol: &Protocol) -> Vec<AbiInfo> {
        if let Some(abis) = &self.abis {
            abis.split(",")
                .into_iter()
                .map(|abi_string| {
                    let abi_info: AbiInfo = abi_string.into();
                    abi_info.assert_compatible_with_protocol(protocol);
                    abi_info
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

pub(crate) enum AbiInfo {
    LocalFilePath(PathBuf),
    ContractAddress(String),
}

impl AbiInfo {
    pub(crate) fn assert_compatible_with_protocol(&self, protocol: &Protocol) {
        match (self, &protocol.supported_abi_addition_methods) {
            (AbiInfo::LocalFilePath(local_file_path), SupportedAbiAdditionMethods::DownloadFromContractAddress) => panic!("Local file path: {}, supplied although protocol: {} only supports download from contract address!", local_file_path.to_string_lossy(), protocol),
            (AbiInfo::ContractAddress(contract_address), SupportedAbiAdditionMethods::CopyFromLocalFilePath) => panic!("Contract address: {}, supplied although protocol: {} only supports copying from a local file path!", contract_address, protocol),
            _ => {}
        }
    }
}

impl From<&str> for AbiInfo {
    fn from(abi_arg: &str) -> Self {
        if abi_arg.is_valid_abi_address() {
            AbiInfo::ContractAddress(abi_arg.to_string())
        } else {
            let abi_path = Path::new(abi_arg);
            if abi_path.exists() {
                AbiInfo::LocalFilePath(abi_path.to_path_buf())
            } else {
                panic!(
                    "Abi arg supplied: {} is neither a contract address nor a local file path!",
                    abi_arg
                )
            }
        }
    }
}

pub(crate) fn add_abis(
    protocol_and_network_info: ProtocolAndNetworkInfo,
    abis_arg: &AbisArg,
    project_dir: &PathBuf,
    is_add_operation: bool,
) {
    let abi_infos = abis_arg.get_abi_infos(&protocol_and_network_info.protocol);

    if abi_infos.is_empty() {
        let abi_string = if is_add_operation {
            let abi_string = get_input("Abi (Leave blank to skip)", Some("Abi"), true);
            if abi_string.is_empty() {
                println!("Skipping to next step");
                return;
            }
            abi_string
        } else {
            get_input("Abi", None, false)
        };

        let abi_info: AbiInfo = abi_string.as_str().into();
        abi_info.assert_compatible_with_protocol(&protocol_and_network_info.protocol);
        let abi_file_contents = get_abi_file_contents(
            abi_info,
            &protocol_and_network_info.protocol.protocol_type,
            &protocol_and_network_info.network,
        );

        let contract_name = get_input("Contract Name", None, false);
        add_abi_to_project(
            abi_file_contents,
            &contract_name,
            project_dir,
        );
        add_block_to_object_mapping_code(contract_name);
    } else {
        for abi_info in abi_infos {
            let abi_file_contents = get_abi_file_contents(
                abi_info,
                &protocol_and_network_info.protocol.protocol_type,
                &protocol_and_network_info.network,
            );

            let contract_name = get_input("Contract Name", None, false);
            add_abi_to_project(
                abi_file_contents,
                &contract_name,
                project_dir,
            );
            add_block_to_object_mapping_code(contract_name);
        }
    }

    loop {
        let abi_string = if is_add_operation {
            get_input(
                "Add another Abi Address (Leave blank to end program)",
                Some("Abi"),
                true,
            )
        } else {
            get_input(
                "Add another Abi Address (Leave blank to move on to next step)",
                Some("Abi"),
                true,
            )
        };
        if abi_string.is_empty() {
            return;
        }
        let abi_info: AbiInfo = abi_string.as_str().into();
        abi_info.assert_compatible_with_protocol(&protocol_and_network_info.protocol);

        let abi_file_contents = get_abi_file_contents(
            abi_info,
            &protocol_and_network_info.protocol.protocol_type,
            &protocol_and_network_info.network,
        );

        let contract_name = get_input("Contract Name", None, false);
        add_abi_to_project(
            abi_file_contents,
            &contract_name,
            project_dir,
        );
        add_block_to_object_mapping_code(contract_name);
    }
}

fn get_abi_file_contents(
    abi_info: AbiInfo,
    protocol_type: &ProtocolType,
    network: &String,
) -> String {
    match abi_info {
        AbiInfo::LocalFilePath(local_file_path) => {
            println!(
                "{}",
                get_success_message("Abi retrieved!")
            );
            fs::read_to_string(local_file_path).unwrap()
        }
        AbiInfo::ContractAddress(contract_address) => {
            println!(
                "{}",
                get_success_message("Abi retrieved!")
            );
            download_abi(contract_address, protocol_type, network)
        }
    }
}

fn download_abi(
    contract_address: String,
    protocol_type: &ProtocolType,
    network: &String,
) -> String {
    let url = match protocol_type {
        ProtocolType::Ethereum => {
            let base_url = match network.as_str() {
                "mainnet" => "https://api.etherscan.io/api".to_string(),
                "arbitrum-one" => "https://api.arbiscan.io/api".to_string(),
                "bsc" => "https://api.bscscan.com/api".to_string(),
                "matic" => "https://api.polygonscan.com/api".to_string(),
                "mumbai" => "https://api-testnet.polygonscan.com/api".to_string(),
                "aurora" => "https://api.aurorascan.dev/api".to_string(),
                "aurora-testnet" => "https://api-testnet.aurorascan.dev/api".to_string(),
                "optimism-kovan" => "https://api-kovan-optimistic.etherscan.io/api".to_string(),
                "optimism" => "https://api-optimistic.etherscan.io/api".to_string(),
                "moonbeam" => "https://api-moonbeam.moonscan.io/api".to_string(),
                "moonriver" => "https://api-moonriver.moonscan.io/api".to_string(),
                "mbase" => "https://api-moonbase.moonscan.io/api".to_string(),
                "avalanche" => "https://api.snowtrace.io/api".to_string(),
                "fuji" => "https://api-testnet.snowtrace.io/api".to_string(),
                "gnosis" => "https://api.gnosisscan.io/api".to_string(),
                "poa-core" => "`https://blockscout.com/poa/core/api".to_string(),
                _ => format!("https://api-{}.etherscan.io/api", network),
            };

            format!(
                "{}?module=contract&action=getabi&address={}",
                base_url, contract_address
            )
        }
        _ => unreachable!(),
    };

    let spinner = Spinner::new("Downloading abi..".to_string());

    let response_text = match reqwest::blocking::get(url) {
        Ok(response) => match response.text() {
            Ok(response_text) => response_text,
            Err(error) => {
                spinner.end_with_error_message("Download failed!".to_string());
                panic!("Failed to download ABI response text with error: {}", error);
            }
        },
        Err(error) => {
            spinner.end_with_error_message("Download failed!".to_string());
            panic!("ABI download failed with response error: {}", error);
        }
    };

    let contract_download_response =
        match serde_json::from_str::<ContractDownloadResponse>(&response_text) {
            Ok(response) => response,
            Err(error) => {
                spinner.end_with_error_message("Download failed!".to_string());
                panic!(
                    "Issue deserializing ABI download response!\nDownload response: {}\nError: {}",
                    response_text, error
                )
            }
        };

    // According to graph-cli this is a necessary check for validity of the ABI contract
    if contract_download_response.status != "1" {
        spinner.end_with_error_message("Download failed!".to_string());
        panic!("ABI response status not equal to 1!\nABI response message: {}\nABI response result: {}", contract_download_response.message, contract_download_response.result);
    }

    spinner.end_with_success_message("Download completed!".to_string());

    // Lets make the json look nice
    let result_json: Value = serde_json::from_str(&contract_download_response.result).expect(&format!("Downloaded contract is not valid json! Contract contents: {}", contract_download_response.result));
    serde_json::to_string_pretty(&result_json).unwrap()
}

fn add_abi_to_project(
    abi_file_contents: String,
    contract_name: &String,
    project_dir: &PathBuf,
) {
    let abi_dir = project_dir.join("abi");
    let contract_filepath = if contract_name.ends_with(".json") {
        abi_dir.join(contract_name)
    } else {
        abi_dir.join(format!("{}.json", contract_name))
    };
    let build_rs_filepath = project_dir.join("build.rs");
    let cargo_toml_filepath = project_dir.join("Cargo.toml");
    let mut cargo_toml_contents = CargoToml::load_from_file(&cargo_toml_filepath);

    let spinner = Spinner::new(format!("Adding abi boilerplate for {}", contract_name));

    let mut operations = Vec::new();

    if !abi_dir.exists() {
        operations.push(FileContentsModification::CreateFolder(abi_dir));
    }

    if !build_rs_filepath.exists() {
        operations.push(FileContentsModification::CreateFile(File {
            filepath: build_rs_filepath,
            file_contents: get_build_rs_default_file_contents(),
        }));
        if cargo_toml_contents.add_build_dependencies(vec!["anyhow".into_dep(), "substreams-common".dep_with_local_path("common")]) {
            operations.push(FileContentsModification::UpdateFile(File {
                filepath: cargo_toml_filepath,
                file_contents: cargo_toml_contents.get_file_contents(),
            }))
        }
    }

    operations.push(FileContentsModification::CreateFile(File {
        filepath: contract_filepath,
        file_contents: abi_file_contents,
    }));

    safely_modify_file_contents(operations);

    spinner.end_with_success_message(format!("Abi boilerplate added for {}", contract_name));
}

fn get_build_rs_default_file_contents() -> String {
    "use anyhow::{Ok, Result};
use substreams_common::codegen;

fn main() -> Result<(), anyhow::Error> {
    println!(\"cargo:rerun-if-changed=proto\");
    println!(\"cargo:rerun-if-changed=abi\");
    codegen::generate(None)?;

    Ok(())
}"
    .to_string()
}

#[derive(Deserialize)]
struct ContractDownloadResponse {
    status: String,
    message: String,
    result: String,
}
