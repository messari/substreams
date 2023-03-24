use std::path::PathBuf;

use crate::streaming_fast::file::{File, LocationType};
use crate::streaming_fast::multiple_files_sink::MultipleFilesSink;
use crate::streaming_fast::proto_structure_info::MessageInfo;
use crate::streaming_fast::process_substream::EncodingType;
use crate::streaming_fast::proto_utils::FromUnsignedVarint;
use crate::streaming_fast::single_file_sink::SingleFileSink;
use crate::streaming_fast::split_files_sink::SplitFilesSink;

pub(crate) struct Sink {
    items_field_number: Option<u64>,
    multiple_files_sink: Box<dyn MultipleFilesSink>
}

impl Sink {
    pub(crate) fn new(output_type_info: MessageInfo, encoding_type: EncodingType, location_type: LocationType, sink_output_path: PathBuf, starting_block_number: i64) -> Self {
        if output_type_info.is_collection_of_items() {
            let (inner_type_info, items_field_number) = output_type_info.get_item_type_info();

            if inner_type_info.is_oneof_type() {
                Sink {
                    items_field_number: Some(items_field_number),
                    multiple_files_sink: Box::new(SplitFilesSink::new(inner_type_info.fields, encoding_type, location_type, sink_output_path, starting_block_number)),
                }
            } else {
                Sink {
                    items_field_number: Some(items_field_number),
                    multiple_files_sink: Box::new(SingleFileSink::new(inner_type_info, encoding_type, location_type, sink_output_path, starting_block_number)),
                }
            }
        } else {
            Sink {
                items_field_number: None,
                multiple_files_sink: Box::new(SingleFileSink::new(output_type_info, encoding_type, location_type, sink_output_path, starting_block_number)),
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

                output_files.extend(self.multiple_files_sink.process(&mut consumed, block_number)?);
            }
            Ok(output_files)
        } else {
            self.multiple_files_sink.process(&mut proto_data.as_slice(), block_number)
        }
    }

    /// Instead of waiting for file to "fill" to required size, instead you can call this method to
    /// make a file out of the data you have parsed so far
    pub(crate) fn flush_leftovers(&mut self, block_number: i64) -> Vec<File> {
        self.multiple_files_sink.flush_leftovers(block_number)
    }
}