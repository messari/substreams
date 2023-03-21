use std::collections::BTreeMap;
use parquet::file::writer::SerializedRowGroupWriter;

use crate::streaming_fast::file_sinks::helpers::parquet::decoder::{Decoder};
use crate::streaming_fast::file_sinks::helpers::parquet::parquet_schema_builder::ParquetSchemaBuilder;
use crate::streaming_fast::file_sinks::helpers::parquet::file_buffer::FileBuffer;
use crate::streaming_fast::proto_structure_info::{FieldSpecification, MessageInfo};
use crate::streaming_fast::proto_utils::FromUnsignedVarint;

pub(in crate::streaming_fast::file_sinks) struct StructDecoder {
    field_decoders: BTreeMap<u64, Decoder>,
    field_specification: FieldSpecification,
    non_repeated_fields: Vec<u64>,
    flattened_field_name: String
}

impl StructDecoder {
    pub(in crate::streaming_fast::file_sinks) fn new(message_info: MessageInfo, parquet_schema_builder: &mut ParquetSchemaBuilder, mut track_definition_lvls: bool, mut track_repetition_lvls: bool) -> Self {
        match message_info.field_specification {
            FieldSpecification::Required => {}
            FieldSpecification::Optional => track_definition_lvls = true,
            FieldSpecification::Repeated => track_repetition_lvls = true,
            FieldSpecification::Packed => track_repetition_lvls = true,
        };

        let mut field_decoders = BTreeMap::new();
        let mut non_repeated_fields = Vec::new();
        for field_info in message_info.fields {
            match field_info.field_specification {
                FieldSpecification::Required | FieldSpecification::Optional => non_repeated_fields.push(field_info.field_number),
                _ => {}
            }
            field_decoders.insert(field_info.field_number, Decoder::new(field_info, parquet_schema_builder, track_definition_lvls, track_repetition_lvls));
        }

        StructDecoder {
            field_decoders,
            field_specification: message_info.field_specification,
            non_repeated_fields,
            flattened_field_name: parquet_schema_builder.get_flattened_field_name(&message_info.type_name)
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
                    decoded_fields.push(field_number);
                }
                _ => {
                    panic!("TODO! - struct path: {}, field_number: {}", self.flattened_field_name, field_number);
                },
            };
        }

        for field in self.non_repeated_fields.iter() {
            if !decoded_fields.contains(field) {
                self.field_decoders.get_mut(field).unwrap().push_null_or_default_values(uncompressed_file_size, current_definition_lvl, last_repetition_lvl)?;
            }
        }

        Ok(())
    }

    /// This is triggered when the proto data does not contain a value for a given field. Returns true if data is pushed to value store and false if no data is added
    pub(in crate::streaming_fast::file_sinks) fn push_null_or_default_values(&mut self, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<bool, String> {
        match self.field_specification {
            FieldSpecification::Required => {
                // TODO: Should check if we should always get some data returned for a struct - if this is the case then we should just
                // TODO: return an error and stop processing rather than propagating null or default values
                for field_number in self.non_repeated_fields.iter() {
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
        for field_number in self.non_repeated_fields.iter() {
            self.field_decoders.get_mut(field_number).unwrap().push_nulls(uncompressed_file_size, current_definition_lvl, last_repetition_lvl)?;
        }

        Ok(())
    }
}