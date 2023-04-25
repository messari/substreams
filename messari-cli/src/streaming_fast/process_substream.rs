use std::{env, fs};
use std::path::PathBuf;
use futures::StreamExt;
use prost::Message;
use s3::Bucket;
use s3::creds::Credentials;
use tonic::metadata::MetadataValue;
use tonic::Status;
use tonic::transport::Channel;

use crate::streaming_fast::block_client::get_latest_block_number;
use crate::streaming_fast::streamingfast_dtos;
use crate::streaming_fast::file::{Location, LocationType};
use crate::streaming_fast::sink::Sink;
use crate::streaming_fast::streamingfast_dtos::ForkStep::StepIrreversible;
use crate::streaming_fast::streamingfast_dtos::{Package, Request, Response};
use crate::streaming_fast::streamingfast_dtos::module_output::Data;
use crate::streaming_fast::proto_structure_info::get_output_type_info;

pub(crate) async fn process_substream(spkg: Vec<u8>, module_name: String, encoding_type: EncodingType, location_type: LocationType, data_location_path: Option<PathBuf>, start_block_arg: Option<i64>, stop_block_arg: Option<u64>) {
    let package = Package::decode(spkg).unwrap();
    let mut sink = get_sink(&package, module_name, encoding_type, location_type, data_location_path);

    let (start_block, stop_block) = get_block_range(&sink, start_block_arg, stop_block_arg).await;

    sink.set_starting_block_number(start_block);

    let request = Request {
        start_block_num: start_block, // TODO: Should check whether negative values actually correspond to "x behind end block num"
        start_cursor: "".to_string(),
        stop_block_num: stop_block as u64,
        fork_steps: vec![StepIrreversible as i32],
        irreversibility_condition: "".to_string(),
        modules: package.modules,
        output_modules: vec![module_name],
        production_mode: true,
        ..Default::default()
    };

    let streamingfast_token = env::var("SUBSTREAMS_API_TOKEN").unwrap();
    let token_metadata = MetadataValue::try_from(streamingfast_token.as_str()).unwrap();

    let mut client = streamingfast_dtos::stream_client::StreamClient::with_interceptor(
        Channel::from_static("https://mainnet.eth.streamingfast.io:443").connect_lazy(),
        move |mut r: tonic::Request<()>| {
            r.metadata_mut().insert("authorization", token_metadata.clone());
            Ok(r)
        },
    );

    let response_stream = client.blocks(request).await.unwrap();
    let mut block_stream = response_stream.into_inner();

    // TODO: Change the logic below into buffered streams in a select to prevent
    // TODO: downloading data and writing files blocking one another
    while let Some(block) = block_stream.next().await {
        println!("Result: {:?}", block);
        if let Some((output_data, block_number)) = get_output_data(block).unwrap() {
            println!("Processing block: {}", block_number);
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
async fn get_block_range(sink: &Sink, start_block_arg: Option<i64>, stop_block_arg: Option<u64>) -> (i64, i64) {
    let mut stop_block= get_latest_block_number().await;
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
        get_start_block_num(sink.get_an_output_folder_path(), &package, &proto_type_name).await
    };

    (start_block, stop_block)
}

fn get_sink(package: &Package, module_name: String, encoding_type: EncodingType, location_type: LocationType, data_location_path: Option<PathBuf>) -> Sink {
    let mut sink_output_path = if let Some(data_location_path) = data_location_path {
        data_location_path
    } else {
        match location_type {
            LocationType::Local => std::env::current_dir().unwrap().join("data"),
            LocationType::DataWarehouse => PathBuf::from("substreams")
        }
    };

    let (output_type_info, proto_type_name) = get_output_type_info(package, &module_name);
    sink_output_path = add_package_partitions_to_output_folder_path(sink_output_path, &proto_type_name, &output_type_info.type_name);

    Sink::new(output_type_info, encoding_type, location_type, sink_output_path)
}

/// Returns block range info in the form -> (start_block_num, stop_block_num)
pub(crate) async fn get_block_range_info(spkg: Vec<u8>, module_name: String, location_type: LocationType, data_location_path: Option<PathBuf>) -> (i64, i64) {
    let package = Package::decode(spkg).unwrap();
    let sink = get_sink(&package, module_name, EncodingType::Parquet, location_type, data_location_path);

    get_block_range(&sink, start_block_arg, stop_block_arg).await
}

async fn get_start_block_num(location: Location, package: &Package, proto_type_name: &str) -> i64 {
    // First we check to see if we have processed data for this substream/module combination before.
    // If we have we will take this as the starting block_number
    let processed_block_files = match location {
        Location::DataWarehouse(path) => {
            let bucket_name = "data-warehouse-load-427049689281-dev";
            let region = "us-west-2".parse().unwrap();
            let credentials = Credentials::default().unwrap();
            let bucket = Bucket::new(bucket_name, region, credentials).unwrap();
            let list_response = bucket.list(path.to_string_lossy().to_string(), None).await.unwrap();
            list_response.into_iter().map(|x| x.name).collect::<Vec<_>>()
        }
        Location::Local(path) => {
            fs::read_dir(path).unwrap().into_iter().map(|path| path.unwrap().path().display().to_string()).collect::<Vec<_>>()
        }
    };

    if processed_block_files.len() > 0 {
        // For now we will just assume all files will be in form -> startBlock_stopBlock.fileExtension
        let mut last_block_num_iterator = processed_block_files.into_iter().map(|file| file.split('.').next().unwrap().split('_').skip(1).next().unwrap().parse::<i64>().unwrap());
        let mut latest_block_num = last_block_num_iterator.next().unwrap();
        for block_num in last_block_num_iterator {
            if block_num > latest_block_num {
                latest_block_num = block_num;
            }
        }
        return latest_block_num;
    }

    // Otherwise we will fall back to taking the initial block number specified for the given module
    for module in package.modules.as_ref().unwrap().modules.iter() {
        if module.output.as_ref().unwrap().r#type.as_str() == proto_type_name {
            return module.initial_block as i64;
        }
    }

    panic!("Unable to match the module output: {} to a given module!", proto_type_name);
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
                    streamingfast_dtos::response::Message::Data(block_scoped_data) => {
                        let block_number = block_scoped_data.clock.unwrap().number as i64;
                        for module_output in block_scoped_data.outputs {
                            match module_output.data {
                                None => {}
                                Some(Data::MapOutput(data)) => {
                                    return Ok(Some((data.value, block_number)));
                                }
                                _ => {}
                            }
                        }
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