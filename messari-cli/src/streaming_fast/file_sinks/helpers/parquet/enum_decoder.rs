use std::collections::HashMap;
use parquet::data_type::{ByteArray, ByteArrayType};
use parquet::file::writer::SerializedRowGroupWriter;
use derives::proto_structure_info::{FieldInfo, FieldSpecification};

use crate::streaming_fast::file_sinks::helpers::parquet::file_buffer::FileBuffer;
use crate::streaming_fast::file_sinks::helpers::parquet::parquet_schema_builder::ParquetSchemaBuilder;
use crate::streaming_fast::file_sinks::helpers::parquet::repetition_and_definition::{RepetitionAndDefinitionLvls, RepetitionAndDefinitionLvlStore, RepetitionAndDefinitionLvlStoreBuilder};
use crate::streaming_fast::streaming_fast_utils::FromSignedVarint;

pub(in crate::streaming_fast::file_sinks) struct EnumDecoder {
    values: Vec<ByteArray>,
    repetition_and_definition_lvl_store: Option<RepetitionAndDefinitionLvlStore>,
    field_specification: FieldSpecification,
    flattened_field_name: String,
    enum_mappings: HashMap<i64, ByteArray>
}

impl EnumDecoder {
    pub(in crate::streaming_fast::file_sinks) fn new(field_info: FieldInfo, parquet_schema_builder: &mut ParquetSchemaBuilder, lvls_store_builder: &RepetitionAndDefinitionLvlStoreBuilder) -> Self {
        let repetition = field_info.field_specification.get_repetition();
        let lvls_store = lvls_store_builder.get_store(&repetition);

        parquet_schema_builder.add_column_info(&field_info.field_name, field_info.field_type.clone(), repetition);
        let flattened_field_name = parquet_schema_builder.get_flattened_field_name(&field_info.field_name);

        let enum_mappings = field_info.get_enum_mappings().into_iter().map(|(field_number, enum_value)| {
            (*field_number as i64, ByteArray::from(enum_value.as_str()))
        }).collect();

        EnumDecoder {
            values: Vec::new(),
            repetition_and_definition_lvl_store: lvls_store,
            field_specification: field_info.field_specification,
            flattened_field_name,
            enum_mappings
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn write_data_to_parquet(&mut self, row_group_writer: &mut SerializedRowGroupWriter<FileBuffer>) {
        let mut serialized_column_writer = row_group_writer.next_column().unwrap().unwrap();

        let (repetition_lvls, definition_lvls) = if let Some(lvls_store) = self.repetition_and_definition_lvl_store.as_ref() {
            (lvls_store.get_repetition_lvls(), lvls_store.get_definition_lvls())
        } else {
            (None, None)
        };

        serialized_column_writer.typed::<ByteArrayType>().write_batch(self.values.as_slice(), definition_lvls, repetition_lvls).unwrap();
        self.values.clear();

        serialized_column_writer.close().unwrap();
    }

    pub(in crate::streaming_fast::file_sinks) fn decode(&mut self, data: &mut &[u8], wire_type: u8, uncompressed_file_size: &mut usize, lvls: RepetitionAndDefinitionLvls) -> Result<(), String> {
        match self.field_specification {
            FieldSpecification::Required => {
                self.decode_value(data, wire_type, uncompressed_file_size)?;
                if let Some(lvl_store) = self.repetition_and_definition_lvl_store.as_mut() {
                    lvl_store.add_lvls(lvls);
                    *uncompressed_file_size += 32;
                }
            }
            FieldSpecification::Optional => {
                self.decode_value(data, wire_type, uncompressed_file_size)?;
                self.repetition_and_definition_lvl_store.as_mut().unwrap().add_lvls_for_optional_field(lvls);
                *uncompressed_file_size += 32;
            }
            FieldSpecification::Repeated => {
                self.decode_value(data, wire_type, uncompressed_file_size)?;
                self.repetition_and_definition_lvl_store.as_mut().unwrap().add_lvls(lvls);
                *uncompressed_file_size += 32;
            }
            _ => unreachable!()
        }

        Ok(())
    }

    pub(in crate::streaming_fast::file_sinks) fn push_null_or_default_value(&mut self, uncompressed_file_size: &mut usize, lvls: RepetitionAndDefinitionLvls) -> Result<(), String> {
        match self.field_specification {
            FieldSpecification::Required => {
                // Assuming 0 to be the default enum variant value
                if let Some(default_enum_value) = self.enum_mappings.get(&0) {
                    self.values.push(default_enum_value.clone());
                    *uncompressed_file_size += 64;
                } else {
                    return Err("TODO: no default enum value set! (Assuming a default value to be the 0 variant!)".to_string());
                }
                if let Some(lvls_store) = self.repetition_and_definition_lvl_store.as_mut() {
                    lvls_store.add_lvls(lvls);
                    *uncompressed_file_size += 32;
                }
            },
            _ => {
                self.push_null(uncompressed_file_size, lvls);
            },
        }

        Ok(())
    }

    pub(in crate::streaming_fast::file_sinks) fn push_null(&mut self, uncompressed_file_size: &mut usize, lvls: RepetitionAndDefinitionLvls) {
        self.repetition_and_definition_lvl_store.as_mut().unwrap().add_lvls(lvls);
        *uncompressed_file_size += 32;
    }

    fn decode_value(&mut self, data: &mut &[u8], wire_type: u8, uncompressed_file_size: &mut usize) -> Result<(), String> {
        // TODO: Add wire_type checks for single value decoding

        match i64::from_signed_varint(data) {
            Some(enum_key) => {
                if let Some(enum_value) = self.enum_mappings.get(&enum_key) {
                    // Reason for using 64 bytes for size increment rather than the string size is that almost all
                    // values get converted to integer version and mappings are stored in the dictionary page
                    *uncompressed_file_size += 64;
                    self.values.push(enum_value.clone());
                } else {
                    return Err("TODO..".to_string());
                }
            }
            None => return Err(format!("Error reading proto data for column: {}! Field Type: Enum, data: {:?}", self.flattened_field_name, data))
        }


        Ok(())
    }
}