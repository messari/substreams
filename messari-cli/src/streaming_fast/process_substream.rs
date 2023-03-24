use std::env;
use std::path::PathBuf;
use futures::StreamExt;
use prost::Message;
use tonic::metadata::MetadataValue;
use tonic::Status;
use tonic::transport::Channel;

use crate::streaming_fast::streamingfast_dtos;
use crate::streaming_fast::file::LocationType;
use crate::streaming_fast::sink::Sink;
use crate::streaming_fast::streamingfast_dtos::ForkStep::StepIrreversible;
use crate::streaming_fast::streamingfast_dtos::{Package, Request, Response};
use crate::streaming_fast::streamingfast_dtos::module_output::Data;
use crate::streaming_fast::proto_structure_info::get_output_type_info;

pub(crate) async fn process_substream(spkg: Vec<u8>, module_name: String, encoding_type: EncodingType, location_type: LocationType, data_location_path: Option<PathBuf>, start_block_arg: Option<i64>, stop_block_arg: Option<u64>) {
    let package = Package::decode(spkg.as_ref()).unwrap();

    let start_block = start_block_arg.unwrap_or_default();
    // TODO: If the stop block number is not found then we should default to the latest block
    let stop_block = stop_block_arg.unwrap();

    let mut sink_output_path = if let Some(data_location_path) = data_location_path {
        data_location_path
    } else {
        match location_type {
            LocationType::Local => std::env::current_dir().unwrap().join("data"),
            LocationType::DataWarehouse => PathBuf::from("substreams")
        }
    };

    // TODO: data_location_path should be checked here to make sure something weird isn't happening here...

    let (output_type_info, proto_type_name) = get_output_type_info(&package, &module_name);
    sink_output_path = add_package_partitions_to_output_folder_path(sink_output_path, &proto_type_name, &output_type_info.type_name);

    let mut sink = Sink::new(output_type_info, encoding_type, location_type, sink_output_path, start_block);

    let request = Request {
        start_block_num: start_block,
        start_cursor: "".to_string(),
        stop_block_num: stop_block,
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
        Channel::from_static("https://api.streamingfast.io").connect_lazy(),
        move |mut r: tonic::Request<()>| {
            r.metadata_mut().insert("authorization", token_metadata.clone());
            Ok(r)
        },
    );

    let response_stream = client.blocks(request).await.unwrap();
    let mut block_stream = response_stream.into_inner();

    // TODO: Change the logic below into buffered streams in a select to prevent
    // TODO: downloading data and writing files blocking one another
    let mut last_seen_block_number = start_block;
    while let Some(block) = block_stream.next().await {
        if let Some((output_data, block_number)) = get_output_data(block).unwrap() {
            match sink.process(output_data, block_number) {
                Ok(files) => {
                    futures::future::join_all(files.into_iter().map(|file| file.save())).await;
                }
                Err(error) => {
                    // TODO: Flesh the error out and return it rather than panicking
                    panic!("{}", error);
                }
            }
            last_seen_block_number = block_number;
        }
    }

    futures::future::join_all(sink.flush_leftovers(last_seen_block_number).into_iter().map(|file| file.save())).await;
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