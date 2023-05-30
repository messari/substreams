use std::collections::{BTreeMap, HashMap};
use parquet::file::writer::SerializedRowGroupWriter;
use derives::proto_structure_info::{FieldSpecification, MessageInfo};

use crate::streaming_fast::file_sinks::helpers::parquet::decoder::{Decoder};
use crate::streaming_fast::file_sinks::helpers::parquet::parquet_schema_builder::ParquetSchemaBuilder;
use crate::streaming_fast::file_sinks::helpers::parquet::file_buffer::FileBuffer;
use crate::streaming_fast::streaming_fast_utils::FromUnsignedVarint;

pub(in crate::streaming_fast::file_sinks) struct StructDecoder {
    field_decoders: BTreeMap<u64, Decoder>,
    field_specification: FieldSpecification,
    optional_and_required_fields: Vec<u64>,
    flattened_field_name: String,
    oneof_group_tracker: OneofGroupTracker
}

impl StructDecoder {
    pub(in crate::streaming_fast::file_sinks) fn new(field_name: &str, message_info: MessageInfo, parquet_schema_builder: &mut ParquetSchemaBuilder, mut track_definition_lvls: bool, mut track_repetition_lvls: bool) -> Self {
        match message_info.field_specification {
            FieldSpecification::Required => {}
            FieldSpecification::Optional => track_definition_lvls = true,
            FieldSpecification::Repeated => track_repetition_lvls = true,
            FieldSpecification::Packed => track_repetition_lvls = true,
        };

        let mut field_decoders = BTreeMap::new();
        let mut optional_and_required_fields = Vec::new();
        let oneof_field_numbers = message_info.oneof_groups.clone().into_iter().flat_map(|x| x).collect::<Vec<_>>();
        for field_info in message_info.fields {
            match field_info.field_specification {
                FieldSpecification::Required | FieldSpecification::Optional => {
                    if !oneof_field_numbers.contains(&field_info.field_number) {
                        optional_and_required_fields.push(field_info.field_number);
                    }
                },
                _ => {}
            }
            field_decoders.insert(field_info.field_number, Decoder::new(field_info, parquet_schema_builder, track_definition_lvls, track_repetition_lvls));
        }

        StructDecoder {
            field_decoders,
            field_specification: message_info.field_specification,
            optional_and_required_fields,
            flattened_field_name: parquet_schema_builder.get_flattened_field_name(field_name),
            oneof_group_tracker: OneofGroupTracker::new(message_info.oneof_groups)
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn write_data_to_parquet(&mut self, row_group_writer: &mut SerializedRowGroupWriter<FileBuffer>) {
        for field_decoder in self.field_decoders.values_mut() {
            field_decoder.write_data_to_parquet(row_group_writer);
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn decode(&mut self, mut data: &mut &[u8], wire_type: u8, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        // TODO: Validate wire_type before proceeding to make sure proto data was properly encoded

        // TODO: Deal with field specification when working out how to change current_definition_lvl and last_repetition_lvl

        let mut decoded_fields = Vec::new();

        loop {
            if data.is_empty() {
                break;
            }

            let tag = match u64::from_unsigned_varint(&mut data) {
                Some(tag) => tag,
                None => {
                    break;
                }
            };

            let field_number = tag >> 3;
            let wire_type = (tag & 0x07) as u8;

            match self.field_decoders.get_mut(&field_number) {
                Some(field) => {
                    field.decode(data, wire_type, uncompressed_file_size, current_definition_lvl, last_repetition_lvl)?;
                    self.oneof_group_tracker.notify_field_seen(field_number)?;
                    decoded_fields.push(field_number);
                }
                _ => {
                    panic!("TODO! - struct path: {}, field_number: {}", self.flattened_field_name, field_number);
                },
            };
        }

        for field in self.optional_and_required_fields.iter() {
            if !decoded_fields.contains(field) {
                self.field_decoders.get_mut(field).unwrap().push_null_or_default_values(uncompressed_file_size, current_definition_lvl, last_repetition_lvl)?;
            }
        }

        self.oneof_group_tracker.assert_oneof_groups_all_seen()?;
        self.oneof_group_tracker.reset();

        Ok(())
    }

    /// This is triggered when the proto data does not contain a value for a given field. Returns true if data is pushed to value store and false if no data is added
    pub(in crate::streaming_fast::file_sinks) fn push_null_or_default_values(&mut self, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<bool, String> {
        match self.field_specification {
            FieldSpecification::Required => {
                for field_number in self.optional_and_required_fields.iter() {
                    self.field_decoders.get_mut(field_number).unwrap().push_null_or_default_values(uncompressed_file_size, current_definition_lvl, last_repetition_lvl)?;
                }
                Ok(true)
            }
            FieldSpecification::Optional => {
                self.push_nulls(uncompressed_file_size, current_definition_lvl, last_repetition_lvl)?;
                Ok(true)
            }
            _ => Ok(false)
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn push_nulls(&mut self, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        for field_number in self.optional_and_required_fields.iter() {
            self.field_decoders.get_mut(field_number).unwrap().push_nulls(uncompressed_file_size, current_definition_lvl, last_repetition_lvl)?;
        }

        Ok(())
    }
}

struct OneofGroupTracker {
    field_to_oneof_group_mappings: HashMap<u64, u8>,
    oneof_group_tracker: HashMap<u8, Option<u64>>
}

impl OneofGroupTracker {
    fn new(oneof_groups: Vec<Vec<u64>>) -> Self {
        let mut field_to_oneof_group_mappings = HashMap::new();
        let mut oneof_group_tracker = HashMap::new();
        for (group_index, oneof_group_fields) in oneof_groups.into_iter().enumerate() {
            let oneof_group_number = group_index as u8;
            for field in oneof_group_fields {
                field_to_oneof_group_mappings.insert(field, oneof_group_number);
            }
            oneof_group_tracker.insert(oneof_group_number, -1);
        }

        OneofGroupTracker {
            field_to_oneof_group_mappings: Default::default(),
            oneof_group_tracker: Default::default(),
        }
    }

    /// Needs to get reset after each deserialization of a struct
    fn reset(&mut self) {
        self.oneof_group_tracker.values_mut().for_each(|val| *val=None)
    }

    fn notify_field_seen(&mut self, field_number: u64) -> Result<(), String> {
        if let Some(group_index) = self.field_to_oneof_group_mappings.get(&field_number) {
            let field = self.oneof_group_tracker.get_mut(group_index).unwrap();
            if let Some(field_seen) = field.as_ref() {
                if field_number != *field_seen {
                    return Err("TODO".to_string());
                }
            } else {
                *field = Some(field_number);
            }
        }

        Ok(())
    }

    fn assert_oneof_groups_all_seen(&self) -> Result<(), String> {
        for (_oneof_group_number, field) in self.oneof_group_tracker.iter() {
            if field.is_none() {
                return Err("TODO".to_string());
            }
        }
        Ok(())
    }
}