use std::borrow::BorrowMut;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::Arc;
use parquet::basic::Repetition;
use parquet::file::writer::SerializedRowGroupWriter;
use prost_types::field_descriptor_proto::{Label, Type};
use prost_types::{DescriptorProto, field_descriptor_proto, FieldDescriptorProto, FileDescriptorProto};

use crate::field_decoder::FieldDecoder;
use crate::struct_decoder;
use crate::file_buffer::FileBuffer;
use crate::parquet_schema_builder::ParquetSchemaBuilder;
use crate::parquet_sink::get_proto_type;
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

    pub(crate) fn decode(&mut self, data: &mut &[u8], wire_type: u8, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        match self {
            Decoder::FieldDecoder(field_decoder) => {
                field_decoder.decode(data, wire_type, uncompressed_file_size, current_definition_lvl, last_repetition_lvl)
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
    pub(crate) fn push_null_or_default_values(&mut self, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        match self {
            Decoder::FieldDecoder(field_decoder) => {
                field_decoder.push_null_or_default_value(uncompressed_file_size, current_definition_lvl, last_repetition_lvl)
            }
            Decoder::StructDecoder(struct_decoder) => struct_decoder.push_null_or_default_values(uncompressed_file_size, current_definition_lvl, last_repetition_lvl),
        }
    }

    pub(crate) fn push_nulls(&mut self, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        match self {
            Decoder::FieldDecoder(field_decoder) => {
                field_decoder.push_null(uncompressed_file_size, current_definition_lvl, last_repetition_lvl)
            }
            Decoder::StructDecoder(struct_decoder) => struct_decoder.push_nulls(uncompressed_file_size, current_definition_lvl, last_repetition_lvl),
        }
    }

    pub(crate) fn new(proto_descriptors: &Vec<FileDescriptorProto>, field_descriptor: &FieldDescriptorProto, parquet_schema_builder: &mut ParquetSchemaBuilder, track_definition_lvls: bool, track_repetition_lvls: bool) -> Decoder {
        let field_type = field_descriptor.get_field_type();

        macro_rules! make_decoder {
            ($field_type:ident) => {{
                let field_specification = field_descriptor.get_field_specification();
                let flattened_field_name = parquet_schema_builder.add_column_info(field_descriptor.name(), field_type, &field_specification);
                Decoder::FieldDecoder(FieldDecoder::new(ValueStore::$field_type(Vec::new()),
                                                        track_definition_lvls,
                                                        track_repetition_lvls,
                                                        field_specification,
                                                        flattened_field_name))
            }}
        }

        match field_type {
            Type::Double => make_decoder!(Double),
            Type::Float => make_decoder!(Float),
            Type::Int64 => make_decoder!(Int64),
            Type::Uint64 => make_decoder!(UInt64),
            Type::Int32 => make_decoder!(Int32),
            Type::Fixed64 => make_decoder!(Fixed64),
            Type::Fixed32 => make_decoder!(Fixed32),
            Type::Bool => make_decoder!(Bool),
            Type::String => make_decoder!(String),
            Type::Group => unimplemented!("Group type has been deprecated and is no longer supported by proto3!"),
            Type::Message => {
                parquet_schema_builder.start_building_sub_group(field_descriptor.name.as_ref().unwrap());

                let proto_type = get_proto_type(proto_descriptors, field_descriptor.type_name());
                let struct_decoder = Decoder::StructDecoder(StructDecoder::new(proto_descriptors,
                                                          &proto_type,
                                                          parquet_schema_builder,
                                                          track_definition_lvls,
                                                          track_repetition_lvls,field_descriptor.get_field_specification()));

                parquet_schema_builder.finish_building_sub_group();

                struct_decoder
            },
            Type::Bytes => make_decoder!(Bytes),
            Type::Uint32 => make_decoder!(UInt32),
            Type::Enum => make_decoder!(Enum),
            Type::Sfixed32 => make_decoder!(SFixed32),
            Type::Sfixed64 => make_decoder!(SFixed64),
            Type::Sint32 => make_decoder!(SInt32),
            Type::Sint64 => make_decoder!(SInt64),
        }
    }
}

pub(crate) trait ProtoFieldExt {
    fn get_field_specification(&self) -> FieldSpecification;
    fn is_collection_of_items_field(&self) -> bool;
    fn get_field_type(&self) -> Type;
    fn get_field_number(&self) -> u64;
    fn get_label(&self) -> Label;
}

#[derive(PartialEq)]
pub(crate) enum FieldSpecification {
    Required,
    Optional,
    Repeated,
    Packed
}

impl FieldSpecification {
    pub(crate) fn get_repetition(&self) -> Repetition {
        match self {
            FieldSpecification::Required => Repetition::REQUIRED,
            FieldSpecification::Optional => Repetition::OPTIONAL,
            FieldSpecification::Repeated => Repetition::REPEATED,
            FieldSpecification::Packed => Repetition::REPEATED
        }
    }
}

impl ProtoFieldExt for FieldDescriptorProto {
    fn get_field_specification(&self) -> FieldSpecification {
        // TODO: Check to see if Label::Optional field is reliable. If so then we can remove this check
        if self.proto3_optional == Some(true) {
            return FieldSpecification::Optional;
        }

        match self.get_label() {
            Label::Optional => FieldSpecification::Optional,
            Label::Required => FieldSpecification::Required,
            Label::Repeated => {
                const SUPPORTED_REPEATED_TYPES: [Type; 13] = [Type::Double, Type::Float, Type::Int64, Type::Uint64, Type::Int32,
                    Type::Fixed64, Type::Fixed32, Type::Bool, Type::Uint32, Type::Sfixed32, Type::Sfixed64, Type::Sint32, Type::Sint64];

                // TODO: Here we want to check the self.options field to make sure there isn't a manual override for setting the packed option
                // TODO: For now we will just assume that the override option is never set so we can just the following to determine a packed field:
                if SUPPORTED_REPEATED_TYPES.contains(&self.get_field_type()) {
                    FieldSpecification::Packed
                } else {
                    FieldSpecification::Repeated
                }
            }
            _ => unreachable!()
        }
    }

    fn is_collection_of_items_field(&self) -> bool {
        self.get_field_type() == Type::Message &&
            self.get_label() == Label::Repeated &&
            self.name.as_ref().unwrap() == &::prost::alloc::string::String::from("items")
    }

    fn get_field_type(&self) -> Type {
        match self.r#type.unwrap() {
            x if x == (Type::Double as i32) => Type::Double,
            x if x == (Type::Float as i32) => Type::Float,
            x if x == (Type::Int64 as i32) => Type::Int64,
            x if x == (Type::Uint64 as i32) => Type::Uint64,
            x if x == (Type::Int32 as i32) => Type::Int32,
            x if x == (Type::Fixed64 as i32) => Type::Fixed64,
            x if x == (Type::Fixed32 as i32) => Type::Fixed32,
            x if x == (Type::Bool as i32) => Type::Bool,
            x if x == (Type::String as i32) => Type::String,
            x if x == (Type::Group as i32) => Type::Group,
            x if x == (Type::Message as i32) => Type::Message,
            x if x == (Type::Bytes as i32) => Type::Bytes,
            x if x == (Type::Uint32 as i32) => Type::Uint32,
            x if x == (Type::Enum as i32) => Type::Enum,
            x if x == (Type::Sfixed32 as i32) => Type::Sfixed32,
            x if x == (Type::Sfixed64 as i32) => Type::Sfixed64,
            x if x == (Type::Sint32 as i32) => Type::Sint32,
            x if x == (Type::Sint64 as i32) => Type::Sint64,
            _ => unreachable!()
        }
    }

    fn get_field_number(&self) -> u64 {
        // I don't know why but for sure reason the field number here is one less than the index for the
        // field when decoding the proto data so in order to correct for this we have to add 1 here!
        println!("Field number: {}", (self.number.unwrap()) as u64);
        (self.number.unwrap()) as u64
    }

    fn get_label(&self) -> Label {
        match self.label.unwrap() {
            x if x==(Label::Optional as i32) => Label::Optional,
            x if x==(Label::Required as i32) => Label::Required,
            x if x==(Label::Repeated as i32) => Label::Repeated,
            _ => unreachable!()
        }
    }
}


trait FromUnsignedVarint: Sized
{
    fn from_unsigned_varint(data: &mut &[u8]) -> Option<Self>;
}

impl<T: Default + TryFrom<u64>> FromUnsignedVarint for T
    where
        T::Error: Debug,
{
    fn from_unsigned_varint(data: &mut &[u8]) -> Option<Self>
    {
        let mut result = 0u64;
        let mut idx = 0;
        loop {
            if idx >= data.len() {
                return None;
            }

            let b = data[idx];
            let value = (b & 0x7f) as u64;
            result += value << (idx * 7);

            idx += 1;
            if b & 0x80 == 0 {
                break;
            }
        }

        let result = T::try_from(result).expect("Out of range");
        *data = &data[idx..];
        Some(result)
    }
}