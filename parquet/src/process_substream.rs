use std::env;
use std::path::PathBuf;
use std::time::Duration;
use anyhow::Context;
use futures::StreamExt;
use http::Uri;
use http::uri::Scheme;
use prost::Message;
use prost_types::{DescriptorProto, FileDescriptorProto};
use tonic::metadata::MetadataValue;
use tonic::Status;
use tonic::transport::{Channel, ClientTlsConfig};
use crate::streamingfast_dtos;
use crate::file::{File, LocationType};
use crate::parquet_sink::ParquetSink;
use crate::sink::Sink;
use crate::streamingfast_dtos::ForkStep::StepIrreversible;
use crate::streamingfast_dtos::{module_output, ModuleOutput, Package, Request, Response};
use crate::streamingfast_dtos::module_output::Data;

pub(crate) async fn process_substream(spkg: Vec<u8>, module_name: String, encoding_type: EncodingType, location_type: LocationType, data_location_path: Option<PathBuf>, start_block: i64, stop_block: u64) {
    let mut package = Package::decode(spkg.as_ref()).unwrap();

    let output_type = get_output_type(&package, &module_name);
    let mut sink = encoding_type.get_sink(&package.proto_files, &output_type, location_type, data_location_path);

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
    while let Some(block) = block_stream.next().await {
        if let Some((output_data, block_number)) = get_output_data(block).unwrap() {
            match sink.process(output_data, block_number) {
                Ok(Some(file)) => {
                    file.save().await;
                }
                Err(error) => {
                    // TODO: Flesh the error out and return it rather than panicking
                    panic!("{}", error);
                }
                _ => {}
            }
        }
    }

    // For testing purposes we will create a file from the remaining data, however post-testing
    // this line should either be deleted or there should be a flag to enable it
    sink.make_file().save().await;
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

fn get_output_type(package: &Package, module_name: &String) -> String {
    for module in package.modules.as_ref().unwrap().modules.iter() {
        if &module.name == module_name {
            let output_type = module.output.as_ref().unwrap().r#type.to_string();

            if !output_type.starts_with("proto:") {
                panic!("TODO!");
            }

            return output_type;
        }
    }

    panic!("Couldn't find output type!!")
}

pub(crate) enum EncodingType {
    // JsonL,
    Parquet
}

impl EncodingType {
    pub(crate) fn get_sink(&self, proto_descriptors: &Vec<FileDescriptorProto>, proto_type: &str, location_type: LocationType, data_location_path: Option<PathBuf>) -> Box<dyn Sink> {
        match self {
            EncodingType::Parquet => Box::new(ParquetSink::new(proto_descriptors, proto_type, location_type, data_location_path))
        }
    }
}

pub(crate) enum StorageLocation {
    Local(PathBuf),
    // DWH(PathBuf)
}
