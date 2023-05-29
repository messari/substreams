use std::collections::HashMap;
use parquet::data_type::{BoolType, ByteArray, ByteArrayType, Int32Type};
use parquet::file::writer::SerializedRowGroupWriter;
use derives::proto_structure_info::{FieldInfo, FieldSpecification};

use crate::streaming_fast::file_sinks::helpers::parquet::file_buffer::FileBuffer;
use crate::streaming_fast::file_sinks::helpers::parquet::parquet_schema_builder::ParquetSchemaBuilder;
use crate::streaming_fast::streaming_fast_utils::FromSignedVarint;

pub(in crate::streaming_fast::file_sinks) struct EnumDecoder {
    values: Vec<ByteArray>,
    definition_lvls: Option<Vec<i16>>,
    repetition_lvls: Option<Vec<i16>>,
    field_specification: FieldSpecification,
    flattened_field_name: String,
    enum_mappings: HashMap<i64, ByteArray>
}

impl EnumDecoder {
    pub(in crate::streaming_fast::file_sinks) fn new(field_info: FieldInfo, parquet_schema_builder: &mut ParquetSchemaBuilder, mut track_definition_lvls: bool, mut track_repetition_lvls: bool) -> Self {
        match field_info.field_specification {
            FieldSpecification::Required => {}
            FieldSpecification::Optional => track_definition_lvls = true,
            FieldSpecification::Repeated => track_repetition_lvls = true,
            FieldSpecification::Packed => track_repetition_lvls = true,
        };

        let flattened_field_name = parquet_schema_builder.add_column_info(&field_info.field_name, field_info.field_type.clone(), &field_info.field_specification);

        let enum_mappings = field_info.get_enum_mappings().into_iter().map(|(field_number, enum_value)| {
            (*field_number as i64, ByteArray::from(enum_value.as_str()))
        }).collect();

        EnumDecoder {
            values: Vec::new(),
            definition_lvls: if track_definition_lvls { Some(Vec::new()) } else { None },
            repetition_lvls: if track_repetition_lvls { Some(Vec::new()) } else { None },
            field_specification: field_info.field_specification,
            flattened_field_name,
            enum_mappings
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn write_data_to_parquet(&mut self, row_group_writer: &mut SerializedRowGroupWriter<FileBuffer>) {
        let mut serialized_column_writer = row_group_writer.next_column().unwrap().unwrap();

        serialized_column_writer.typed::<ByteArrayType>().write_batch(self.values.as_slice(), self.definition_lvls.as_ref().map(|lvls| lvls.as_slice()), self.repetition_lvls.as_ref().map(|lvls| lvls.as_slice())).unwrap();

        serialized_column_writer.close().unwrap();
    }

    pub(in crate::streaming_fast::file_sinks) fn decode(&mut self, data: &mut &[u8], wire_type: u8, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        match self.field_specification {
            FieldSpecification::Required => {
                self.decode_value(data, wire_type, uncompressed_file_size)?;
                if let Some(repetition_lvls) = self.repetition_lvls.as_mut() {
                    repetition_lvls.push(*last_repetition_lvl);
                }

                if let Some(definition_lvls) = self.definition_lvls.as_mut() {
                    definition_lvls.push(current_definition_lvl);
                }
            }
            FieldSpecification::Optional => {
                self.decode_value(data, wire_type, uncompressed_file_size)?;
                if let Some(repetition_lvls) = self.repetition_lvls.as_mut() {
                    repetition_lvls.push(*last_repetition_lvl);
                }

                self.definition_lvls.as_mut().unwrap().push(current_definition_lvl+1);
            }
            FieldSpecification::Repeated => {
                self.decode_value(data, wire_type, uncompressed_file_size)?;
                self.repetition_lvls.as_mut().unwrap().push(*last_repetition_lvl);
                *last_repetition_lvl += 1;

                if let Some(definition_lvls) = self.definition_lvls.as_mut() {
                    definition_lvls.push(current_definition_lvl);
                }
            }
            _ => unreachable!()
        }

        Ok(())
    }

    pub(in crate::streaming_fast::file_sinks) fn push_null_or_default_value(&mut self, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        match self.field_specification {
            FieldSpecification::Required => {
                // Assuming 0 to be the default enum variant value
                if let Some(default_enum_value) = self.enum_mappings.get(&1) {
                    self.values.push(default_enum_value.clone());
                } else {
                    panic!("TODO: (no default enum value set!)");
                }
                // TODO: If tracking def and rep lvls these need to be updated here also
                Ok(())
            },
            FieldSpecification::Optional => self.push_null(uncompressed_file_size, current_definition_lvl, last_repetition_lvl),
            _ => Ok(())
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn push_null(&mut self, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        todo!()
    }

    fn decode_value(&mut self, data: &mut &[u8], wire_type: u8, uncompressed_file_size: &mut usize) -> Result<(), String> {
        match i64::from_signed_varint(data) {
            Some(enum_key) => {
                if let Some(enum_value) = self.enum_mappings.get(&enum_key) {
                    // Reason for using 64 bytes for size increment rather than the string size is that almost all
                    // values get converted to integer version and mappings are stored in the dictionary page
                    *uncompressed_file_size += 64;
                    self.values.push(enum_value.clone());
                } else {
                    panic!("TODO..")
                }
            }
            None => return Err(format!("Error reading proto data for column: {}! Field Type: Enum, data: {:?}", self.flattened_field_name, data))
        }


        Ok(())
    }
}