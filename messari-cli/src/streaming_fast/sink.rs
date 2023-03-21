use std::path::PathBuf;

use crate::streaming_fast::file::{File, Location, LocationType};
use crate::streaming_fast::file_sinks::file_sink::FileSink;
use crate::streaming_fast::file_sinks::parquet::ParquetFileSink;
use crate::streaming_fast::proto_structure_info::MessageInfo;
use crate::streaming_fast::process_substream::EncodingType;
use crate::streaming_fast::proto_utils::FromUnsignedVarint;

pub(crate) struct Sink {
    items_field_number: Option<u64>,
    file_sink: Box<dyn FileSink>,
    sink_output_location: Location,
    starting_block_number: i64,
    encoding_type: EncodingType
}

impl Sink {
    pub(crate) fn new(output_type_info: MessageInfo, encoding_type: EncodingType, location_type: LocationType, mut sink_output_path: PathBuf, starting_block_number: i64) -> Self {
        if output_type_info.is_collection_of_items() {
            let (inner_type_info, items_field_number) = output_type_info.get_item_type_info();

            sink_output_path = sink_output_path.join(&inner_type_info.type_name);

            let file_sink = match encoding_type {
                EncodingType::Parquet => {
                    sink_output_path = sink_output_path.join("parquet");
                    Box::new(ParquetFileSink::new(inner_type_info))
                }
            };

            let sink_output_location = Location::new(location_type, sink_output_path);

            Sink {
                items_field_number: Some(items_field_number),
                file_sink,
                sink_output_location,
                starting_block_number,
                encoding_type
            }
        } else {
            sink_output_path = sink_output_path.join(&output_type_info.type_name.clone());

            let file_sink = match encoding_type {
                EncodingType::Parquet => {
                    sink_output_path = sink_output_path.join("parquet");
                    Box::new(ParquetFileSink::new(output_type_info))
                }
            };

            let sink_output_location = Location::new(location_type, sink_output_path);

            Sink {
                items_field_number: None,
                file_sink,
                sink_output_location,
                starting_block_number,
                encoding_type
            }
        }
    }

    pub(crate) fn process(&mut self, proto_data: Vec<u8>, block_number: i64) -> Result<Vec<File>, String> {
        if let Some(items_field_number) = self.items_field_number {
            let mut data_slice = proto_data.as_slice();
            let mut output_files = Vec::new();
            loop {
                if data_slice.is_empty() {
                    break;
                }

                let tag = match u64::from_unsigned_varint(&mut data_slice) {
                    Some(tag) => tag,
                    None => {
                        break;
                    }
                };

                let field_number = tag >> 3;
                let wire_type = (tag & 0x07) as u8;

                assert_eq!(items_field_number, field_number, "TODO: Write error message!!");

                let struct_data_length = usize::from_unsigned_varint(&mut data_slice).unwrap();
                if data_slice.len() < struct_data_length {
                    return Err("TODO: Write error for this!3".to_string());
                }
                let (mut consumed, remainder) = data_slice.split_at(struct_data_length);
                data_slice = remainder;

                if let Some(file_data) = self.file_sink.process(&mut consumed, block_number)? {
                    output_files.push(File::new(file_data, self.sink_output_location.get_file_location(self.starting_block_number, block_number, &self.encoding_type)));
                    self.starting_block_number = block_number + 1;
                }
            }
            Ok(output_files)
        } else {
            // TODO: Figure out which wire_type is for struct types here
            if let Some(file_data) = self.file_sink.process(&mut proto_data.as_slice(), block_number)? {
                let output_files = vec![File::new(file_data, self.sink_output_location.get_file_location(self.starting_block_number, block_number, &self.encoding_type))];
                self.starting_block_number = block_number + 1;
                Ok(output_files)
            } else {
                Ok(Vec::new())
            }
        }
    }

    /// Instead of waiting for file to "fill" to required size, instead you can call this method to
    /// make a file out of the data you have parsed so far
    pub(crate) fn flush_leftovers_to_file(&mut self, block_number: i64) -> Option<File> {
        let file_data = self.file_sink.make_file();

        if file_data.is_empty() {
            None
        } else {
            let file = File::new(file_data, self.sink_output_location.get_file_location(self.starting_block_number, block_number, &self.encoding_type));
            self.starting_block_number = block_number + 1;
            Some(file)
        }
    }
}