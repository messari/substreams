use std::collections::BTreeMap;
use std::sync::Arc;
use parquet::file::writer::SerializedRowGroupWriter;
use prost_types::{DescriptorProto, FileDescriptorProto};

use crate::decoder::Decoder;
use crate::file_buffer::FileBuffer;
use crate::parquet_schema_builder::ParquetSchemaBuilder;

pub(crate) struct StructDecoder {
    field_decoders: BTreeMap<u64, Decoder>,
    is_repeated: bool,
    is_optional: bool
}

impl StructDecoder {
    pub(crate) fn new(proto_descriptors: &Vec<FileDescriptorProto>, proto_type: &DescriptorProto, parquet_schema_builder: &mut ParquetSchemaBuilder, mut track_definition_lvls: bool, mut track_repetition_lvls: bool, is_repeated: bool, is_optional: bool) -> Self {
        if is_repeated {
            track_repetition_lvls = true;
        }

        if is_optional {
            track_definition_lvls = true;
        }

        let mut field_decoders = BTreeMap::new();
        for field in proto_type.field.iter() {
            field_decoders.insert(field.number.unwrap() as u64, Decoder::new(proto_descriptors, field, parquet_schema_builder, track_definition_lvls, track_repetition_lvls));
        }

        StructDecoder {
            field_decoders,
            is_repeated,
            is_optional,
        }
    }

    pub(crate) fn write_data_to_parquet(&mut self, row_group_writer: &mut SerializedRowGroupWriter<FileBuffer>) {
        for field_decoder in self.field_decoders.values_mut() {
            field_decoder.write_data_to_parquet(row_group_writer);
        }
    }

    pub(crate) fn decode(&mut self, mut data: &mut &[u8], wire_type: u8, uncompressed_file_size: &mut usize, current_definition_lvl: u8, last_repetition_lvl: &mut u8) -> Result<(), String> {
        // TODO: Validate wire_type before proceeding to make sure proto data was properly encoded

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

            match self.field_decoders.get_mut(&field_number) {
                Some(field) => {
                    field.decode(data, wire_type, uncompressed_file_size, current_definition_lvl, last_repetition_lvl)?
                }
                _ => {
                    // We should just return an error here for an unknown field..
                    todo!()
                },
            };
        }

        Ok(())
    }
}