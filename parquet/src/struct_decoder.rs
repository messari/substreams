use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::Arc;
use parquet::file::writer::SerializedRowGroupWriter;
use prost_types::{DescriptorProto, FileDescriptorProto};

use crate::decoder::{Decoder, FieldSpecification, ProtoFieldExt};
use crate::file_buffer::FileBuffer;
use crate::parquet_schema_builder::ParquetSchemaBuilder;

pub(crate) struct StructDecoder {
    field_decoders: BTreeMap<u64, Decoder>,
    field_specification: FieldSpecification,
    non_repeated_fields: Vec<u64>
}

impl StructDecoder {
    pub(crate) fn new(proto_descriptors: &Vec<FileDescriptorProto>, proto_type: &DescriptorProto, parquet_schema_builder: &mut ParquetSchemaBuilder, mut track_definition_lvls: bool, mut track_repetition_lvls: bool, field_specification: FieldSpecification) -> Self {
        match field_specification {
            FieldSpecification::Required => {}
            FieldSpecification::Optional => track_definition_lvls = true,
            FieldSpecification::Repeated => track_repetition_lvls = true,
            FieldSpecification::Packed => track_repetition_lvls = true,
        };

        let mut field_decoders = BTreeMap::new();
        let mut non_repeated_fields = Vec::new();
        for field in proto_type.field.iter() {
            let field_number = field.get_field_number();
            match field.get_field_specification() {
                FieldSpecification::Required | FieldSpecification::Optional => non_repeated_fields.push(field_number),
                _ => {}
            }
            field_decoders.insert(field_number, Decoder::new(proto_descriptors, field, parquet_schema_builder, track_definition_lvls, track_repetition_lvls));
        }

        StructDecoder {
            field_decoders,
            field_specification,
            non_repeated_fields
        }
    }

    pub(crate) fn write_data_to_parquet(&mut self, row_group_writer: &mut SerializedRowGroupWriter<FileBuffer>) {
        for field_decoder in self.field_decoders.values_mut() {
            field_decoder.write_data_to_parquet(row_group_writer);
        }
    }

    /// This is triggered when the proto data does not contain a value for a given field.
    pub(crate) fn push_null_or_default_values(&mut self, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        match self.field_specification {
            FieldSpecification::Required => {
                // TODO: Should check if we should always get some data returned for a struct - if this is the case then we should just
                // TODO: return an error and stop processing rather than propagating null or default values
                for field_number in self.non_repeated_fields.iter() {
                    self.field_decoders.get_mut(field_number).unwrap().push_null_or_default_values(uncompressed_file_size, current_definition_lvl, last_repetition_lvl)?;
                }
                Ok(())
            }
            FieldSpecification::Optional => {
                self.push_nulls(uncompressed_file_size, current_definition_lvl, last_repetition_lvl)
            }
            _ => Ok(())
        }
    }

    pub(crate) fn push_nulls(&mut self, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        for field_number in self.non_repeated_fields.iter() {
            self.field_decoders.get_mut(field_number).unwrap().push_nulls(uncompressed_file_size, current_definition_lvl, last_repetition_lvl)?;
        }

        Ok(())
    }

    pub(crate) fn decode(&mut self, mut data: &mut &[u8], wire_type: u8, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
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
                    // TODO: should we throw an error here if the tag is none?!
                    break;
                }
            };

            let field_number = tag >> 3;
            let wire_type = (tag & 0x07) as u8;

            println!("Tag: {}, field_number: {}, wiretype: {}", tag, field_number, wire_type);

            match self.field_decoders.get_mut(&field_number) {
                Some(field) => {
                    field.decode(data, wire_type, uncompressed_file_size, current_definition_lvl, last_repetition_lvl)?;
                    decoded_fields.push(field_number);
                }
                _ => {
                    // We should just return an error here for an unknown field..
                    println!("Field number11: {}", field_number);

                    // TODO: Throw error here for the unknown field


                    // fn try_read_8_bytes(data: &mut &[u8]) -> Option<[u8; 8]>
                    // {
                    //     if data.len() < 8 {
                    //         return None;
                    //     }
                    //
                    //     match (data[..8]).try_into() {
                    //         Ok(v) => {
                    //             *data = &data[8..];
                    //             Some(v)
                    //         }
                    //         Err(_) => None,
                    //     }
                    // }
                    //
                    // fn try_read_4_bytes(data: &mut &[u8]) -> Option<[u8; 4]>
                    // {
                    //     if data.len() < 4 {
                    //         return None;
                    //     }
                    //
                    //     match (data[..4]).try_into() {
                    //         Ok(v) => {
                    //             *data = &data[4..];
                    //             Some(v)
                    //         }
                    //         Err(_) => None,
                    //     }
                    // }

                    // For now we will just skip the bytes for this mysterious field... (This is only for debugging - remember to delete this!)
                    // let original = *data;
                    // match wire_type {
                    //     0 => {
                    //         u128::from_unsigned_varint(data).unwrap();
                    //     },
                    //     1 => {
                    //         try_read_8_bytes(data);
                    //     },
                    //     2 => {
                    //         usize::from_unsigned_varint(data).map(|length| {
                    //             if length > data.len() {
                    //                 *data = original;
                    //                 return;
                    //             }
                    //             let (consumed, remainder) = data.split_at(length);
                    //             *data = remainder;
                    //         }).unwrap();
                    //     },
                    //     5 => {
                    //         try_read_4_bytes(data).unwrap();
                    //     },
                    //     _ => {
                    //         *data = &[];
                    //     }
                    // }
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