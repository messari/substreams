use std::borrow::BorrowMut;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::Arc;
use parquet::file::writer::SerializedRowGroupWriter;
use prost_types::field_descriptor_proto::{Label, Type};
use prost_types::{DescriptorProto, FieldDescriptorProto, FileDescriptorProto};

use crate::field_decoder::FieldDecoder;
use crate::{get_proto_type, struct_decoder};
use crate::file_buffer::FileBuffer;
use crate::parquet_schema_builder::ParquetSchemaBuilder;
use crate::parquet_sink::ValueStore;
use crate::struct_decoder::StructDecoder;
use crate::value_store::ValueStore;

pub(crate) enum Decoder {
    FieldDecoder(FieldDecoder),
    StructDecoder(StructDecoder),
}

impl Decoder {
    pub(crate) fn write_data_to_parquet(&mut self, row_group_writer: &mut SerializedRowGroupWriter<FileBuffer>) {
        match self.borrow_mut() {
            Decoder::FieldDecoder(field_decoder) => field_decoder.write_data_to_parquet(row_group_writer),
            Decoder::StructDecoder(struct_decoder) => struct_decoder.write_data_to_parquet(row_group_writer)
        }
    }

    pub(crate) fn decode(&mut self, data: &mut &[u8], wire_type: u8, uncompressed_file_size: &mut usize, current_definition_lvl: u8, last_repetition_lvl: &mut u8) -> Result<(), String> {
        match self {
            Decoder::FieldDecoder(field_decoder) => {
                field_decoder.decode(data, wire_type, uncompressed_file_size, current_definition_lvl, last_repetition_lvl)
            }
            Decoder::StructDecoder(struct_decoder) => struct_decoder.decode(data, wire_type, uncompressed_file_size, current_definition_lvl, last_repetition_lvl),
        }
    }

    pub(crate) fn new(proto_descriptors: &Vec<FileDescriptorProto>, field_descriptor: &FieldDescriptorProto, parquet_schema_builder: &mut ParquetSchemaBuilder, track_definition_lvls: bool, track_repetition_lvls: bool) -> Decoder {
        let field_type = field_descriptor.r#type.unwrap() as Type;

        match field_type {
            Type::Double => {
                parquet_schema_builder.add_column_info(field_descriptor.name(), field_type);
                Decoder::FieldDecoder(FieldDecoder::new(ValueStore::Double(Vec::new()),
                                                        track_definition_lvls,
                                                        track_repetition_lvls,
                                                        field_descriptor.is_repeated(),
                                                        field_descriptor.is_packed(),
                                                        field_descriptor.is_optional(),
                                                        parquet_schema_builder.get_flattened_field_name(field_descriptor.name())))
            },
            Type::Float => {
                parquet_schema_builder.add_column_info(field_descriptor.name(), field_type);
                Decoder::FieldDecoder(FieldDecoder::new(ValueStore::Float(Vec::new()),
                                                        track_definition_lvls,
                                                        track_repetition_lvls,
                                                        field_descriptor.is_repeated(),
                                                        field_descriptor.is_packed(),
                                                        field_descriptor.is_optional(),
                                                        parquet_schema_builder.get_flattened_field_name(field_descriptor.name())))
            },
            Type::Int64 => {
                parquet_schema_builder.add_column_info(field_descriptor.name(), field_type);
                Decoder::FieldDecoder(FieldDecoder::new(ValueStore::Int64(Vec::new()),
                                                        track_definition_lvls,
                                                        track_repetition_lvls,
                                                        field_descriptor.is_repeated(),
                                                        field_descriptor.is_packed(),
                                                        field_descriptor.is_optional(),
                                                        parquet_schema_builder.get_flattened_field_name(field_descriptor.name())))
            },
            Type::Uint64 => {
                parquet_schema_builder.add_column_info(field_descriptor.name(), field_type);
                Decoder::FieldDecoder(FieldDecoder::new(ValueStore::UInt64(Vec::new()),
                                                        track_definition_lvls,
                                                        track_repetition_lvls,
                                                        field_descriptor.is_repeated(),
                                                        field_descriptor.is_packed(),
                                                        field_descriptor.is_optional(),
                                                        parquet_schema_builder.get_flattened_field_name(field_descriptor.name())))
            },
            Type::Int32 => {
                parquet_schema_builder.add_column_info(field_descriptor.name(), field_type);
                Decoder::FieldDecoder(FieldDecoder::new(ValueStore::Int32(Vec::new()),
                                                        track_definition_lvls,
                                                        track_repetition_lvls,
                                                        field_descriptor.is_repeated(),
                                                        field_descriptor.is_packed(),
                                                        field_descriptor.is_optional(),
                                                        parquet_schema_builder.get_flattened_field_name(field_descriptor.name())))
            },
            Type::Fixed64 => {
                parquet_schema_builder.add_column_info(field_descriptor.name(), field_type);
                Decoder::FieldDecoder(FieldDecoder::new(ValueStore::Fixed64(Vec::new()),
                                                        track_definition_lvls,
                                                        track_repetition_lvls,
                                                        field_descriptor.is_repeated(),
                                                        field_descriptor.is_packed(),
                                                        field_descriptor.is_optional(),
                                                        parquet_schema_builder.get_flattened_field_name(field_descriptor.name())))
            },
            Type::Fixed32 => {
                parquet_schema_builder.add_column_info(field_descriptor.name(), field_type);
                Decoder::FieldDecoder(FieldDecoder::new(ValueStore::Fixed32(Vec::new()),
                                                        track_definition_lvls,
                                                        track_repetition_lvls,
                                                        field_descriptor.is_repeated(),
                                                        field_descriptor.is_packed(),
                                                        field_descriptor.is_optional(),
                                                        parquet_schema_builder.get_flattened_field_name(field_descriptor.name())))
            },
            Type::Bool => {
                parquet_schema_builder.add_column_info(field_descriptor.name(), field_type);
                Decoder::FieldDecoder(FieldDecoder::new(ValueStore::Bool(Vec::new()),
                                                        track_definition_lvls,
                                                        track_repetition_lvls,
                                                        field_descriptor.is_repeated(),
                                                        field_descriptor.is_packed(),
                                                        field_descriptor.is_optional(),
                                                        parquet_schema_builder.get_flattened_field_name(field_descriptor.name())))
            },
            Type::String => {
                parquet_schema_builder.add_column_info(field_descriptor.name(), field_type);
                Decoder::FieldDecoder(FieldDecoder::new(ValueStore::String(Vec::new()),
                                                        track_definition_lvls,
                                                        track_repetition_lvls,
                                                        field_descriptor.is_repeated(),
                                                        field_descriptor.is_packed(),
                                                        field_descriptor.is_optional(),
                                                        parquet_schema_builder.get_flattened_field_name(field_descriptor.name())))
            },
            Type::Group => unimplemented!("Group type has been deprecated and is no longer supported by proto3!"),
            Type::Message => {
                parquet_schema_builder.start_building_sub_group(field_descriptor.name());

                let proto_type = get_proto_type(proto_descriptors, field_descriptor.type_name());
                let struct_decoder = Decoder::StructDecoder(StructDecoder::new(proto_descriptors,
                                                          &proto_type,
                                                          parquet_schema_builder,
                                                          track_definition_lvls,
                                                          track_repetition_lvls,
                                                          field_descriptor.is_repeated(),
                                                          field_descriptor.is_optional()));

                parquet_schema_builder.finish_building_sub_group();

                struct_decoder
            },
            Type::Bytes => {
                parquet_schema_builder.add_column_info(field_descriptor.name(), field_type);
                Decoder::FieldDecoder(FieldDecoder::new(ValueStore::Bytes(Vec::new()),
                                                        track_definition_lvls,
                                                        track_repetition_lvls,
                                                        field_descriptor.is_repeated(),
                                                        field_descriptor.is_packed(),
                                                        field_descriptor.is_optional(),
                                                        parquet_schema_builder.get_flattened_field_name(field_descriptor.name())))
            },
            Type::Uint32 => {
                parquet_schema_builder.add_column_info(field_descriptor.name(), field_type);
                Decoder::FieldDecoder(FieldDecoder::new(ValueStore::UInt32(Vec::new()),
                                                        track_definition_lvls,
                                                        track_repetition_lvls,
                                                        field_descriptor.is_repeated(),
                                                        field_descriptor.is_packed(),
                                                        field_descriptor.is_optional(),
                                                        parquet_schema_builder.get_flattened_field_name(field_descriptor.name())))
            },
            Type::Enum => {
                parquet_schema_builder.add_column_info(field_descriptor.name(), field_type);
                Decoder::FieldDecoder(FieldDecoder::new(ValueStore::Enum(Vec::new()),
                                                        track_definition_lvls,
                                                        track_repetition_lvls,
                                                        field_descriptor.is_repeated(),
                                                        field_descriptor.is_packed(),
                                                        field_descriptor.is_optional(),
                                                        parquet_schema_builder.get_flattened_field_name(field_descriptor.name())))
            },
            Type::Sfixed32 => {
                parquet_schema_builder.add_column_info(field_descriptor.name(), field_type);
                Decoder::FieldDecoder(FieldDecoder::new(ValueStore::SFixed32(Vec::new()),
                                                        track_definition_lvls,
                                                        track_repetition_lvls,
                                                        field_descriptor.is_repeated(),
                                                        field_descriptor.is_packed(),
                                                        field_descriptor.is_optional(),
                                                        parquet_schema_builder.get_flattened_field_name(field_descriptor.name())))
            },
            Type::Sfixed64 => {
                parquet_schema_builder.add_column_info(field_descriptor.name(), field_type);
                Decoder::FieldDecoder(FieldDecoder::new(ValueStore::SFixed64(Vec::new()),
                                                        track_definition_lvls,
                                                        track_repetition_lvls,
                                                        field_descriptor.is_repeated(),
                                                        field_descriptor.is_packed(),
                                                        field_descriptor.is_optional(),
                                                        parquet_schema_builder.get_flattened_field_name(field_descriptor.name())))
            },
            Type::Sint32 => {
                parquet_schema_builder.add_column_info(field_descriptor.name(), field_type);
                Decoder::FieldDecoder(FieldDecoder::new(ValueStore::SInt32(Vec::new()),
                                                        track_definition_lvls,
                                                        track_repetition_lvls,
                                                        field_descriptor.is_repeated(),
                                                        field_descriptor.is_packed(),
                                                        field_descriptor.is_optional(),
                                                        parquet_schema_builder.get_flattened_field_name(field_descriptor.name())))
            },
            Type::Sint64 => {
                parquet_schema_builder.add_column_info(field_descriptor.name(), field_type);
                Decoder::FieldDecoder(FieldDecoder::new(ValueStore::SInt64(Vec::new()),
                                                        track_definition_lvls,
                                                        track_repetition_lvls,
                                                        field_descriptor.is_repeated(),
                                                        field_descriptor.is_packed(),
                                                        field_descriptor.is_optional(),
                                                        parquet_schema_builder.get_flattened_field_name(field_descriptor.name())))
            },
        }
    }
}

trait ProtoFieldExt {
    fn is_optional(&self) -> bool;
    fn is_repeated(&self) -> bool;
    fn is_packed(&self) -> bool;
}

impl ProtoFieldExt for FieldDescriptorProto {
    fn is_optional(&self) -> bool {
        self.proto3_optional == Some(true)
    }

    fn is_repeated(&self) -> bool {
        // TODO: Make sure the field is repeated and not actually packed (you'll need to
        // TODO: check all options and metadata to ensure this is the case
        self.label == Some(Label::Repeated as i32);
        todo!()
    }

    fn is_packed(&self) -> bool {
        todo!()
    }
}
