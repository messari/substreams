use std::collections::{BTreeMap, HashMap};
use parquet::file::writer::SerializedRowGroupWriter;
use derives::proto_structure_info::{FieldSpecification, MessageInfo};

use crate::streaming_fast::file_sinks::helpers::parquet::decoder::{Decoder};
use crate::streaming_fast::file_sinks::helpers::parquet::parquet_schema_builder::ParquetSchemaBuilder;
use crate::streaming_fast::file_sinks::helpers::parquet::file_buffer::FileBuffer;
use crate::streaming_fast::file_sinks::helpers::parquet::repetition_and_definition::{RepetitionAndDefinitionLvls, RepetitionAndDefinitionLvlStoreBuilder};
use crate::streaming_fast::streaming_fast_utils::FromUnsignedVarint;

pub(in crate::streaming_fast::file_sinks) struct StructDecoder {
    field_decoders: BTreeMap<u64, Decoder>,
    field_specification: FieldSpecification,
    fields: Vec<u64>,
    repeated_fields: Vec<u64>,
    max_repetition_lvl_for_repeated_fields: i16,
    flattened_field_name: String,
    oneof_group_tracker: OneofGroupTracker
}

impl StructDecoder {
    pub(in crate::streaming_fast::file_sinks) fn new(field_name: &str, message_info: MessageInfo, parquet_schema_builder: &mut ParquetSchemaBuilder, lvls_store_builder: &mut RepetitionAndDefinitionLvlStoreBuilder) -> Self {
        let repetition = message_info.field_specification.get_repetition();
        lvls_store_builder.modify_lvls_for_struct_repetition(&repetition);

        let mut field_decoders = BTreeMap::new();
        let mut fields = Vec::new();
        let mut repeated_fields = Vec::new();
        for field_info in message_info.fields {
            fields.push(field_info.field_number);
            if field_info.field_specification == FieldSpecification::Repeated {
                repeated_fields.push(field_info.field_number);
            }
            field_decoders.insert(field_info.field_number, Decoder::new(field_info, parquet_schema_builder, lvls_store_builder));
        }

        let max_repetition_lvl = lvls_store_builder.get_max_repetition_lvl();
        lvls_store_builder.revert_lvls_for_struct_repetition(&repetition);

        StructDecoder {
            field_decoders,
            field_specification: message_info.field_specification,
            fields,
            repeated_fields,
            max_repetition_lvl_for_repeated_fields: max_repetition_lvl + 1,
            flattened_field_name: parquet_schema_builder.get_flattened_field_name(field_name),
            oneof_group_tracker: OneofGroupTracker::new(message_info.oneof_groups)
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn write_data_to_parquet(&mut self, row_group_writer: &mut SerializedRowGroupWriter<FileBuffer>) {
        for field_decoder in self.field_decoders.values_mut() {
            field_decoder.write_data_to_parquet(row_group_writer);
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn decode(&mut self, mut data: &mut &[u8], wire_type: u8, uncompressed_file_size: &mut usize, mut lvls: RepetitionAndDefinitionLvls) -> Result<(), String> {
        // TODO: Validate wire_type before proceeding to make sure proto data was properly encoded

        if self.field_specification == FieldSpecification::Optional {
            lvls.optional_item_seen();
        }

        let mut fields_seen = Vec::new();

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
                    if fields_seen.contains(&field_number) {
                        if self.repeated_fields.contains(&field_number) {
                            field.decode(data, wire_type, uncompressed_file_size, lvls.repeated_item_previously_seen(self.max_repetition_lvl_for_repeated_fields))?;
                        } else {
                            panic!("Non repeated field seen more than once... TODO: flesh out error");
                        }
                    } else {
                        if self.repeated_fields.contains(&field_number) {
                            field.decode(data, wire_type, uncompressed_file_size, lvls.repeated_item_newly_seen())?;
                        } else {
                            field.decode(data, wire_type, uncompressed_file_size, lvls.clone())?;
                        }

                        fields_seen.push(field_number);
                    }

                    self.oneof_group_tracker.notify_field_seen(field_number)?;
                }
                _ => {
                        panic!("TODO! - struct path: {}, field_number: {}", self.flattened_field_name, field_number);
                },
            };
        }

        for field in self.fields.iter() {
            if !fields_seen.contains(field) {
                self.field_decoders.get_mut(field).unwrap().push_null_or_default_values(uncompressed_file_size, lvls.clone())?;
            }
        }

        self.oneof_group_tracker.assert_oneof_groups_all_seen()?;
        self.oneof_group_tracker.reset();

        Ok(())
    }

    /// This is triggered when the proto data does not contain a value for a given field. Returns true if data is pushed to value store and false if no data is added
    pub(in crate::streaming_fast::file_sinks) fn push_null_or_default_values(&mut self, uncompressed_file_size: &mut usize, lvls: RepetitionAndDefinitionLvls) -> Result<(), String> {
        if self.field_specification == FieldSpecification::Required {
            assert!(self.oneof_group_tracker.is_unpopulated(), "Null response given for struct with oneOf fields. This can't happen! TODO: Flesh out error");
            for field_number in self.fields.iter() {
                self.field_decoders.get_mut(field_number).unwrap().push_null_or_default_values(uncompressed_file_size, lvls.clone())?;
            }
            Ok(())
        } else {
            self.push_nulls(uncompressed_file_size, lvls);
            Ok(())
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn push_nulls(&mut self, uncompressed_file_size: &mut usize, lvls: RepetitionAndDefinitionLvls) {
        for field_number in self.fields.iter() {
            self.field_decoders.get_mut(field_number).unwrap().push_nulls(uncompressed_file_size, lvls.clone());
        }
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
            oneof_group_tracker.insert(oneof_group_number, None);
        }

        OneofGroupTracker {
            field_to_oneof_group_mappings,
            oneof_group_tracker,
        }
    }

    fn is_unpopulated(&self) -> bool {
        self.oneof_group_tracker.is_empty()
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