use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use derives::proto_structure_info::FieldInfo;
use async_trait::async_trait;

use crate::streaming_fast::file::{File, Location, LocationType};
use crate::streaming_fast::multiple_files_sink::MultipleFilesSink;
use crate::streaming_fast::process_substream::EncodingType;
use crate::streaming_fast::streaming_fast_utils::{FromUnsignedVarint, get_start_block_numbers};
use crate::streaming_fast::single_file_sink::SingleFileSink;

pub(crate) struct SplitFilesSink {
    file_sinks: HashMap<u64, SingleFileSink>,
    pending_sinks: HashMap<u64, i64> // i64 is the starting block corresponding to the sink
}

impl SplitFilesSink {
    pub(crate) fn new(oneof_fields: Vec<FieldInfo>, encoding_type: EncodingType, location_type: LocationType, sink_output_path: PathBuf) -> Self {
        SplitFilesSink {
            file_sinks: oneof_fields.into_iter().map(|field| {
                (field.field_number, SingleFileSink::new(field.get_struct_info().0, encoding_type.clone(), location_type.clone(), sink_output_path.clone()))
            }).collect(),
            pending_sinks: Default::default(),
        }
    }
}

#[async_trait]
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

            if wire_type != 2 {
                return Err(format!("Wire type read: {}, expected wire type: 2! Proto data of entire result, type: Struct, data: {:?}", wire_type, proto_data));
            }

            if self.file_sinks.contains_key(&field_number) {
                return Err(format!("FieldNumber: {}, is not a valid field! Error found in split file sink!", field_number));
            }

            if let Some(field_seen) = field_seen {
                if field_number != field_seen {
                    return Err(format!("Split file sink only deals with OneOf fields! More than one oneOf field has been detected in proto response. Fields: [{}, {}]", field_seen, field_number));
                }
            } else {
                field_seen = Some(field_number);
            }

            let struct_data_length = usize::from_unsigned_varint(proto_data).unwrap();
            if proto_data.len() < struct_data_length {
                return Err(format!("Data to deserialize: {:?} is smaller than expected struct data size: {}!", proto_data, struct_data_length));
            }
            let (mut consumed, remainder) = proto_data.split_at(struct_data_length);
            *proto_data = remainder;

            if !self.pending_sinks.contains_key(&field_number) {
                output_files.extend(self.file_sinks.get_mut(&field_number).unwrap().process(&mut consumed, block_number)?);
            }
        }

        if field_seen.is_some() {
            Err("No fields seen when deserializing proto response!".to_string())
        } else {
            Ok(output_files)
        }
    }

    fn flush_leftovers(&mut self, block_number: i64) -> Vec<File> {
        let mut output_files = Vec::new();

        for filesink in self.file_sinks.values_mut() {
            output_files.extend(filesink.flush_leftovers(block_number));
        }

        output_files
    }

    fn get_output_folder_locations(&self) -> Vec<Location> {
        self.file_sinks.values().flat_map(|file_sink| file_sink.get_output_folder_locations()).collect()
    }

    fn notify_new_block(&mut self, block_number: i64) {
        let mut keys_to_remove = HashSet::new();
        for (key, val) in self.pending_sinks.clone() {
            if block_number >= val {
                keys_to_remove.insert(key);
            }
        }

        for key in keys_to_remove.into_iter() {
            self.pending_sinks.remove(&key);
        }
    }

    async fn set_starting_block_number(&mut self, starting_block_number: i64) {
        let starting_block_numbers = get_start_block_numbers(self.get_output_folder_locations(), starting_block_number).await;
        let min_starting_block = starting_block_numbers.iter().min().unwrap().clone();

        for ((sink_id, sink), starting_block_number) in self.file_sinks.iter_mut().zip(starting_block_numbers.into_iter()) {
            sink.set_starting_block_number(starting_block_number).await;
            if starting_block_number > min_starting_block {
                self.pending_sinks.insert(sink_id.clone(), starting_block_number);
            }
        }
    }
}