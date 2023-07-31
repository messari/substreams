use std::path::PathBuf;
use derives::proto_structure_info::MessageInfo;

use crate::streaming_fast::file::{File, Location, LocationType};
use crate::streaming_fast::multiple_files_sink::MultipleFilesSink;
use crate::streaming_fast::process_substream::EncodingType;
use crate::streaming_fast::streaming_fast_utils::FromUnsignedVarint;
use crate::streaming_fast::single_file_sink::SingleFileSink;
use crate::streaming_fast::split_files_sink::SplitFilesSink;

pub(crate) struct Sink {
    items_field_number: Option<u64>,
    multiple_files_sink: Box<dyn MultipleFilesSink>,
}

impl Sink {
    pub(crate) fn new(output_type_info: MessageInfo, encoding_type: EncodingType, location_type: LocationType, sink_output_path: PathBuf, bucket_name: Option<String>) -> Self {
        if output_type_info.is_collection_of_items() {
            let (inner_type_info, items_field_number) = output_type_info.get_item_type_info();

            if inner_type_info.is_oneof_type() {
                Sink {
                    items_field_number: Some(items_field_number),
                    multiple_files_sink: Box::new(SplitFilesSink::new(inner_type_info.fields, encoding_type, location_type, sink_output_path, bucket_name)),
                }
            } else {
                Sink {
                    items_field_number: Some(items_field_number),
                    multiple_files_sink: Box::new(SingleFileSink::new(inner_type_info, encoding_type, location_type, sink_output_path, bucket_name)),
                }
            }
        } else if output_type_info.is_oneof_type() {
            Sink {
                items_field_number: None,
                multiple_files_sink: Box::new(SplitFilesSink::new(output_type_info.fields, encoding_type, location_type, sink_output_path, bucket_name)),
            }
        } else {
            Sink {
                items_field_number: None,
                multiple_files_sink: Box::new(SingleFileSink::new(output_type_info, encoding_type, location_type, sink_output_path, bucket_name)),
            }
        }
    }

    pub(crate) async fn set_starting_block_number(&mut self, starting_block_number: i64) {
        self.multiple_files_sink.set_starting_block_number(starting_block_number).await;
    }

    pub(crate) fn process(&mut self, proto_data: Vec<u8>, block_number: i64) -> Result<Vec<File>, String> {
        self.multiple_files_sink.notify_new_block(block_number);

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

                if wire_type != 2 {
                    return Err(format!("Wire type read: {}, expected wire type: 2! Proto data of entire result, type: Struct, data: {:?}", wire_type, proto_data));
                }

                if items_field_number != field_number {
                    return Err(format!("Sink is expecting a solo items field with number: {}. Instead a field number: {} was seen!", items_field_number, field_number));
                }

                let struct_data_length = usize::from_unsigned_varint(&mut data_slice).unwrap();
                if data_slice.len() < struct_data_length {
                    return Err(format!("Data to deserialize: {:?} is smaller than expected struct data size: {}!", data_slice, struct_data_length));
                }
                let (mut consumed, remainder) = data_slice.split_at(struct_data_length);
                data_slice = remainder;

                output_files.extend(self.multiple_files_sink.process(&mut consumed, block_number)?);
            }
            Ok(output_files)
        } else {
            self.multiple_files_sink.process(&mut proto_data.as_slice(), block_number)
        }
    }

    pub(crate) fn get_output_folder_locations(&self) -> Vec<Location> {
        self.multiple_files_sink.get_output_folder_locations()
    }

    /// Instead of waiting for file to "fill" to required size, instead you can call this method to
    /// make a file out of the data you have parsed so far
    pub(crate) fn flush_leftovers(&mut self, block_number: i64) -> Vec<File> {
        self.multiple_files_sink.flush_leftovers(block_number)
    }
}