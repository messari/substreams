use std::collections::HashMap;
use std::path::PathBuf;

use crate::streaming_fast::file::{File, Location, LocationType};
use crate::streaming_fast::multiple_files_sink::MultipleFilesSink;
use crate::streaming_fast::process_substream::EncodingType;
use crate::streaming_fast::proto_structure_info::FieldInfo;
use crate::streaming_fast::proto_utils::FromUnsignedVarint;
use crate::streaming_fast::single_file_sink::SingleFileSink;

pub(crate) struct SplitFilesSink {
    file_sinks: HashMap<u64, SingleFileSink>,
}

impl SplitFilesSink {
    pub(crate) fn new(oneof_fields: Vec<FieldInfo>, encoding_type: EncodingType, location_type: LocationType, sink_output_path: PathBuf) -> Self {
        SplitFilesSink {
            file_sinks: oneof_fields.into_iter().map(|field| {
                (field.field_number, SingleFileSink::new(field.get_struct_info(), encoding_type.clone(), location_type.clone(), sink_output_path.clone()))
            }).collect(),
        }
    }
}

impl MultipleFilesSink for SplitFilesSink {
    fn process(&mut self, proto_data: &mut &[u8], block_number: i64) -> Result<Vec<File>, String> {
        let mut output_files = Vec::new();
        let mut field_seen = None;
        loop {
            if proto_data.is_empty() {
                break;
            }

            let tag = match u64::from_unsigned_varint(proto_data) {
                Some(tag) => tag,
                None => {
                    break;
                }
            };

            let field_number = tag >> 3;
            let wire_type = (tag & 0x07) as u8;

            assert!(self.file_sinks.contains_key(&field_number), "TODO: Write error message!!");

            if let Some(field_seen) = field_seen {
                assert_eq!(field_number, field_seen, "TODO: Write error message!!");
            } else {
                field_seen = Some(field_number);
            }

            let struct_data_length = usize::from_unsigned_varint(proto_data).unwrap();
            if proto_data.len() < struct_data_length {
                return Err("TODO: Write error for this!3".to_string());
            }
            let (mut consumed, remainder) = proto_data.split_at(struct_data_length);
            *proto_data = remainder;

            output_files.extend(self.file_sinks.get_mut(&field_number).unwrap().process(&mut consumed, block_number)?);
        }

        assert!(field_seen.is_some(), "TODO: Write error message!!");
        Ok(output_files)
    }

    fn flush_leftovers(&mut self, block_number: i64) -> Vec<File> {
        let mut output_files = Vec::new();

        for filesink in self.file_sinks.values_mut() {
            output_files.extend(filesink.flush_leftovers(block_number));
        }

        output_files
    }

    fn get_an_output_folder_location(&self) -> Location {
        self.file_sinks.values().next().unwrap().get_an_output_folder_location()
    }

    fn set_starting_block_number(&mut self, starting_block_number: i64) {
        for sink in self.file_sinks.values_mut() {
            sink.set_starting_block_number(starting_block_number);
        }
    }
}