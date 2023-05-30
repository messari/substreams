use std::borrow::BorrowMut;
use parquet::file::writer::SerializedRowGroupWriter;
use derives::proto_structure_info::FieldInfo;

use crate::streaming_fast::file_sinks::helpers::parquet::enum_decoder::EnumDecoder;
use crate::streaming_fast::file_sinks::helpers::parquet::field_decoder::FieldDecoder;
use crate::streaming_fast::file_sinks::helpers::parquet::file_buffer::FileBuffer;
use crate::streaming_fast::file_sinks::helpers::parquet::parquet_schema_builder::ParquetSchemaBuilder;
use crate::streaming_fast::file_sinks::helpers::parquet::struct_decoder::StructDecoder;
use crate::streaming_fast::streaming_fast_utils::FromUnsignedVarint;

pub(in crate::streaming_fast::file_sinks) enum Decoder {
    FieldDecoder(FieldDecoder),
    StructDecoder(StructDecoder),
    EnumDecoder(EnumDecoder)
}

impl Decoder {
    pub(in crate::streaming_fast::file_sinks) fn new(field_info: FieldInfo, parquet_schema_builder: &mut ParquetSchemaBuilder, track_definition_lvls: bool, track_repetition_lvls: bool) -> Decoder {
        if field_info.is_struct_field() {
            let repetition = field_info.field_specification.get_repetition();
            parquet_schema_builder.start_building_sub_group(field_info.field_name.clone());
            let (message_info, field_name) = field_info.get_struct_info();

            let decoder = Decoder::StructDecoder(StructDecoder::new(&field_name,
                                                        message_info,
                                                      parquet_schema_builder,
                                                      track_definition_lvls,
                                                      track_repetition_lvls));

            parquet_schema_builder.finish_building_sub_group(repetition);

            decoder
        } else if field_info.is_enum_field() {
            Decoder::EnumDecoder(EnumDecoder::new(field_info, parquet_schema_builder, track_definition_lvls, track_repetition_lvls))
        } else {
            Decoder::FieldDecoder(FieldDecoder::new(field_info, parquet_schema_builder, track_definition_lvls, track_repetition_lvls))
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn write_data_to_parquet(&mut self, row_group_writer: &mut SerializedRowGroupWriter<FileBuffer>) {
        match self.borrow_mut() {
            Decoder::FieldDecoder(field_decoder) => field_decoder.write_data_to_parquet(row_group_writer),
            Decoder::StructDecoder(struct_decoder) => struct_decoder.write_data_to_parquet(row_group_writer),
            Decoder::EnumDecoder(enum_decoder) => enum_decoder.write_data_to_parquet(row_group_writer),
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn decode(&mut self, data: &mut &[u8], wire_type: u8, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        match self {
            Decoder::FieldDecoder(field_decoder) => {
                field_decoder.decode(data, wire_type, uncompressed_file_size, current_definition_lvl, last_repetition_lvl)
            }
            Decoder::EnumDecoder(enum_decoder) => {
                enum_decoder.decode(data, wire_type, uncompressed_file_size, current_definition_lvl, last_repetition_lvl)
            }
            Decoder::StructDecoder(struct_decoder) => {
                let struct_data_length = usize::from_unsigned_varint(data).unwrap();
                if data.len() < struct_data_length {
                    return Err("TODO: Write error for this!3".to_string());
                }
                let (mut consumed, remainder) = data.split_at(struct_data_length);
                *data = remainder;
                struct_decoder.decode(&mut consumed, wire_type, uncompressed_file_size, current_definition_lvl, last_repetition_lvl)
            },
        }
    }

    /// This is triggered when the proto data does not contain a value for a given field.
    pub(in crate::streaming_fast::file_sinks) fn push_null_or_default_values(&mut self, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        match self {
            Decoder::FieldDecoder(field_decoder) => {
                field_decoder.push_null_or_default_value(uncompressed_file_size, current_definition_lvl, last_repetition_lvl)
            }
            Decoder::EnumDecoder(enum_decoder) => {
                enum_decoder.push_null_or_default_value(uncompressed_file_size, current_definition_lvl, last_repetition_lvl)
            }
            Decoder::StructDecoder(struct_decoder) => struct_decoder.push_null_or_default_values(uncompressed_file_size, current_definition_lvl, last_repetition_lvl).map(|_| ()),
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn push_nulls(&mut self, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        match self {
            Decoder::FieldDecoder(field_decoder) => {
                field_decoder.push_null(uncompressed_file_size, current_definition_lvl, last_repetition_lvl)
            }
            Decoder::EnumDecoder(enum_decoder) => {
                enum_decoder.push_null(uncompressed_file_size, current_definition_lvl, last_repetition_lvl)
            }
            Decoder::StructDecoder(struct_decoder) => struct_decoder.push_nulls(uncompressed_file_size, current_definition_lvl, last_repetition_lvl),
        }
    }
}
