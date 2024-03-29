use std::env;
use std::collections::HashSet;
use std::path::PathBuf;
use futures::StreamExt;
use prost::Message;
use regex::Regex;
use tonic::metadata::MetadataValue;
use tonic::Status;
use tonic::transport::Channel;

use crate::streaming_fast::block_client::get_latest_block_number;
use crate::streaming_fast::streamingfast_dtos;
use crate::streaming_fast::file::LocationType;
use crate::streaming_fast::sink::Sink;
use crate::streaming_fast::streamingfast_dtos::{Package, Request, Response};
use crate::streaming_fast::proto_structure_info::get_output_type_info;
use crate::streaming_fast::streaming_config::{Chain, StreamingConfig};
use crate::streaming_fast::streaming_fast_utils::{get_initial_block_for_module, get_start_block_num};
use crate::streaming_fast::streamingfast_dtos::module::input::Input;

pub(crate) async fn process_substream(spkg: Vec<u8>, config: StreamingConfig, encoding_type: EncodingType, location_type: LocationType, data_location_path: Option<PathBuf>, bucket_name: Option<String>, start_block_arg: Option<i64>, stop_block_arg: Option<u64>) {
    let mut package = Package::decode(spkg.as_slice()).unwrap();

    let chain = if let Some(chain_override) = config.chain_override {
        for module in package.modules.as_mut().unwrap().modules.iter_mut() {
            for input in module.inputs.iter_mut() {
                if let Input::Source(source) = input.input.as_mut().unwrap() {
                    if source.r#type.ends_with(".Block") {
                        source.r#type = chain_override.get_proto_block_type();
                    }
                }
            }
        }
        chain_override
    } else {
        get_chain_info(&package)
    };

    let (mut sink, proto_type_name) = get_sink_and_proto_type_name(&package, config.output_module.as_str(), encoding_type, location_type, data_location_path, bucket_name, &chain);

    if let Some(substream_name) = config.substream_name_override {
        package.package_meta.iter_mut().next().unwrap().name = substream_name;
    }
    for module in package.modules.as_mut().unwrap().modules.iter_mut() {
        for param_override in config.param_overrides.iter() {
            if param_override.module == module.name {
                for input in module.inputs.iter_mut() {
                    if let Input::Params(param) = input.input.as_mut().unwrap() {
                        param.value = param_override.value.clone();
                    }
                }
            }
        }
        for start_block_override in config.start_block_overrides.iter() {
            if start_block_override.module == module.name {
                module.initial_block = start_block_override.block_number;
            }
        }
    }

    let (start_block, stop_block) = get_block_range(&sink, &package, &proto_type_name, &chain, start_block_arg, stop_block_arg).await;

    sink.set_starting_block_number(start_block).await;

    let request = Request {
        start_block_num: start_block,
        start_cursor: "".to_string(),
        stop_block_num: stop_block as u64,
        modules: package.modules,
        production_mode: true,
        final_blocks_only: true,
        output_module: config.output_module,
        debug_initial_store_snapshot_for_modules: vec![],
    };

    let streamingfast_token = env::var("SUBSTREAMS_API_TOKEN").unwrap();
    let token_metadata = MetadataValue::try_from(streamingfast_token.as_str()).unwrap();

    let mut client = streamingfast_dtos::stream_client::StreamClient::with_interceptor(
        Channel::builder(chain.get_endpoint()).connect_lazy(),
        move |mut r: tonic::Request<()>| {
            r.metadata_mut().insert("authorization", token_metadata.clone());
            Ok(r)
        },
    );

    let response_stream = client.blocks(request).await.unwrap();
    let mut block_stream = response_stream.into_inner();

    let mut num_block = 1;

    // TODO: Change the logic below into buffered streams in a select to prevent
    // TODO: downloading data and writing files blocking one another
    while let Some(block) = block_stream.next().await {
        if let Some((output_data, block_number)) = get_output_data(block).unwrap() {
            num_block += 1;
            if output_data.len() > 0 {
                println!("Num block: {}, Block process: {}, data size: {}", num_block,block_number, output_data.len());
            }
            match sink.process(output_data, block_number) {
                Ok(files) => {
                    futures::future::join_all(files.into_iter().map(|file| file.save())).await;
                }
                Err(error) => {
                    // TODO: Flesh the error out and return it rather than panicking
                    panic!("{}", error);
                }
            }
        }
    }

    futures::future::join_all(sink.flush_leftovers(stop_block).into_iter().map(|file| file.save())).await;
}

/// Returns block range info in the form -> (start_block_num, stop_block_num)
async fn get_block_range(sink: &Sink, package: &Package, proto_type_name: &str, chain: &Chain, start_block_arg: Option<i64>, stop_block_arg: Option<u64>) -> (i64, i64) {
    let mut stop_block= get_latest_block_number(chain).await;
    if let Some(stop_block_unwrapped) = stop_block_arg {
        let stop_block_i64 = stop_block_unwrapped as i64;
        if stop_block_i64 < stop_block {
            stop_block = stop_block_i64;
        } else {
            panic!("Stop block_num: {} specified when last block number is {} - can't specify a stop block number that's larger than the latest block number!", stop_block_i64, stop_block);
        }
    }
    let start_block = if let Some(start_block) = start_block_arg {
        if start_block < 0 {
            let new_start_block = stop_block + start_block;
            if new_start_block < 0 {
                panic!("Offset: {} given for start block is larger than the stop block number: {} - this would make the starting block number <0 which is invalid!", start_block.abs(), stop_block);
            }
            new_start_block
        } else {
            start_block
        }
    } else {
        let fallback_starting_block = get_initial_block_for_module(package, proto_type_name);
        get_start_block_num(sink.get_output_folder_locations(), fallback_starting_block).await
    };

    (start_block, stop_block)
}

fn get_sink_and_proto_type_name(package: &Package, module_name: &str, encoding_type: EncodingType, location_type: LocationType, data_location_path: Option<PathBuf>, bucket_name: Option<String>, chain: &Chain) -> (Sink, String) {
    let mut sink_output_path = if let Some(data_location_path) = data_location_path {
        data_location_path
    } else {
        match location_type {
            LocationType::Local => std::env::current_dir().unwrap().join("data"),
            LocationType::DataWarehouse => PathBuf::from("substreams")
        }
    };

    sink_output_path = chain.add_chain_folders_to_path(sink_output_path);

    let substream_name = package.package_meta.iter().next().unwrap().name.as_str(); // Assumes the first package specified is always the main spkg rather than a sub-spkg (haven't check this though)
    sink_output_path = sink_output_path.join(substream_name);

    let (output_type_info, proto_type_name) = get_output_type_info(package, module_name);
    sink_output_path = add_package_partitions_to_output_folder_path(sink_output_path, &proto_type_name, &output_type_info.type_name);

    let package_version = get_package_version(package);
    sink_output_path = sink_output_path.join(package_version);

    let sink = Sink::new(output_type_info, encoding_type, location_type, sink_output_path, bucket_name);

    (sink, proto_type_name)
}

/// Gets package version and reforms it into semver form but with underscores instead of full-stops (eg. X_Y_Z)
fn get_package_version(package: &Package) -> String {
    let spkg_version = package.package_meta.first().unwrap().version.as_str(); // Assumes the first package specified is always the main spkg rather than a sub-spkg (haven't check this though)

    // Only expecting spkg_version to be in forms: either vX.Y.Z or X.Y.Z
    let v_semver = Regex::new(r"^v\d+.\d+.\d+$").unwrap();
    let semver = Regex::new(r"^\d+.\d+.\d+$").unwrap();
    let semver_str = if v_semver.is_match(spkg_version) {
        spkg_version[1..].to_string()
    } else if semver.is_match(spkg_version) {
        spkg_version.to_string()
    } else {
        panic!("Couldn't extract proper versioning from spkg! Expecting version to be either in form: vX.Y.Z or X.Y.Z - actual version given: {}", spkg_version);
    };

    semver_str.replace(".", "_")
}

fn get_chain_info(package: &Package) -> Chain {
    let mut block_containing_inputs = HashSet::new();
    for module in package.modules.as_ref().unwrap().modules.iter() {
        for input in module.inputs.iter() {
            if let Input::Source(input_type) = input.input.as_ref().unwrap() {
                if input_type.r#type.ends_with(".Block") {
                    block_containing_inputs.insert(input_type.r#type.clone());
                }
            }
        }
    }

    let block_input_types =  block_containing_inputs.into_iter().collect::<Vec<_>>();
    if block_input_types.len() == 0 {
        panic!("Couldn't determine default chain from block type! Either specify a block input in one of your substream modules or specify a block override in your config file corresponding to this substream!");
    } else if block_input_types.len() > 1 {
        panic!("Couldn't determine default chain from block type! More than one module input type ending in \".Block\" was specified for this substream leading to too much ambiguity for deciding which chain to pick!");
    }

    let block_type = block_input_types.into_iter().next().unwrap();

    Chain::default_for_block_type(&block_type)
}

/// Returns block range info in the form -> (start_block_num, stop_block_num)
pub(crate) async fn get_block_range_info(spkg: Vec<u8>, module_name: &str, location_type: LocationType, data_location_path: Option<PathBuf>, bucket_name: Option<String>, start_block_override: Option<i64>, chain_override: Option<Chain>) -> (i64, i64) {
    let package = Package::decode(spkg.as_slice()).unwrap();
    let chain = chain_override.unwrap_or(get_chain_info(&package));
    let (sink, proto_type_name) = get_sink_and_proto_type_name(&package, module_name, EncodingType::Parquet, location_type, data_location_path, bucket_name, &chain);

    get_block_range(&sink, &package, &proto_type_name, &chain, start_block_override, None).await
}

fn add_package_partitions_to_output_folder_path(mut sink_output_path: PathBuf, proto_type_name: &str, entity_name: &str) -> PathBuf {
    let proto_type = proto_type_name.replace("proto:", "");

    for proto_type_part in proto_type.split('.') {
        if proto_type_part!="messari" && proto_type_part!="" && proto_type_part!=entity_name { // We will add the entity name to the path later on :)
            sink_output_path = sink_output_path.join(proto_type_part);
        }
    }

    sink_output_path
}

/// Returns both the output data and also it's corresponding block_number
fn get_output_data(block: Result<Response, Status>) -> Result<Option<(Vec<u8>, i64)>, String> {
    match block {
        Ok(response) => {
            if let Some(message) = response.message {
                match message {
                    streamingfast_dtos::response::Message::BlockScopedData(block_scoped_data) => {
                        let block_number = block_scoped_data.clock.unwrap().number as i64;
                        let output = block_scoped_data.output.unwrap().map_output.unwrap();
                        return Ok(Some((output.value, block_number)));
                    },
                    _ => {}
                }
            }

            Ok(None)
        }
        Err(error) => {
            Err(format!("Error!: {} - TODO: Give proper error message here..", error.message()))
        }
    }
}

#[derive(Clone)]
pub(crate) enum EncodingType {
    // JsonL,
    Parquet
}