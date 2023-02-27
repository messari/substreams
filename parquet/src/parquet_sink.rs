use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use bytes::Bytes;
use parquet::basic::LogicalType;
use parquet::column::writer::ColumnWriter;
use parquet::file::writer::SerializedFileWriter;
use prost_types::{DescriptorProto, FieldDescriptorProto, FileDescriptorProto};
use prost_types::field_descriptor_proto;
use parquet::schema::types::{BasicTypeInfo, PrimitiveTypeBuilder};

use crate::decoder::StructDecoder;
use crate::file::{File, Location, LocationType};
use crate::file_buffer::FileBuffer;
use crate::parquet_schema_builder::ParquetSchemaBuilder;
use crate::sink::Sink;

const UNCOMPRESSED_FILE_SIZE_THRESHOLD: usize = 500 * 1024 * 1024; // 500MB

pub(crate) struct ParquetSink {
    // TODO: We can write to multiple different outputs by wrapping innards in a struct and having a separate
    // TODO: instance of each of this struct for each sub field that wants it's own output file type
    items_field_number: Option<i32>,
    uncompressed_file_size: usize,
    decoder: StructDecoder,
    block_numbers: Vec<i64>,
    file_writer: SerializedFileWriter<FileBuffer>,
    file_buffer: FileBuffer,
    sink_output_location: Location
}

impl ParquetSink {
    /// data_location_root allows one to point out where the "data lake" starts. (If left blank for a local output it will default to the current directory)
    pub(crate) fn new(proto_descriptors: &Vec<FileDescriptorProto>, proto_type: &str, location_type: LocationType, data_location_path: Option<PathBuf>) -> Self {
        let mut sink_output_path = if let Some(data_location_path) = data_location_path {
            data_location_path
        } else {
            match LocationType {
                LocationType::Local => {
                    std::env::current_dir().unwrap()
                },
                LocationType::DataWarehouse => PathBuf::new()
            }
        };
        // Perhaps we should make a folder from a cleaned version of the proto type rather than the raw version to keep things cleaner here?
        sink_output_path.push(proto_type);
        sink_output_path.push("parquet");

        let sink_output_location = Location::new(location_type, sink_output_path);

        let mut proto_type = get_proto_type(proto_descriptors, proto_type);
        if proto_type.is_collection_of_items() {
            let inner_type_name = proto_type.field[0].type_name.as_ref().unwrap();
            let items_field_number = proto_type.field[0].number.unwrap();
            proto_type = get_proto_type(proto_descriptors, inner_type_name);

            let mut parquet_schema_builder = ParquetSchemaBuilder::new(proto_type.name());
            let decoder = StructDecoder::new(proto_descriptors, proto_type, &mut parquet_schema_builder, false, false, false, false);

            let (file_writer, file_data) = parquet_schema_builder.compile();

            ParquetSink {
                items_field_number: Some(items_field_number),
                uncompressed_file_size: 0,
                decoder,
                block_numbers: vec![],
                file_writer,
                file_buffer: file_data,
                sink_output_location
            }
        } else {
            let mut parquet_schema_builder = ParquetSchemaBuilder::new(proto_type.name());
            let decoder = StructDecoder::new(proto_descriptors, proto_type, &mut parquet_schema_builder, false, false, false, false);

            let (file_writer, file_data) = parquet_schema_builder.compile();

            ParquetSink {
                items_field_number: None,
                uncompressed_file_size: 0,
                decoder,
                block_numbers: vec![],
                file_writer,
                file_buffer: file_data,
                sink_output_location
            }
        }
    }
}

impl Sink for ParquetSink {
    fn process(&mut self, proto_data: Vec<u8>, block_number: i64) -> Result<Option<File>, String> {
        self.block_numbers.push(block_number);

        if let Some(items_field_number) = self.items_field_number {
            let mut data_slice = proto_data.as_slice();

            loop {
                if data.is_empty() {
                    break;
                }

                let tag = match u64::from_unsigned_varint(&mut data) {
                    Some(tag) => tag,
                    None => {
                        return Err("TODO: Write error for this!".to_string());
                    }
                };

                let field_number = tag >> 3;
                let wire_type = (tag & 0x07) as u8;

                assert_eq!(items_field_number, field_number, "TODO: Write error message!!");

                self.decoder.decode(&mut data_slice, wire_type, &mut self.uncompressed_file_size, 0, &mut 0)?;

                self.block_numbers.push(block_number);
            }
        } else {
            // TODO: Figure out which wire_type is for struct types here
            self.decoder.decode(&mut proto_data.as_slice(), 2, &mut self.uncompressed_file_size, 0, &mut 0)?;
        }

        if self.uncompressed_file_size > UNCOMPRESSED_FILE_SIZE_THRESHOLD {
            // Write and return parquet file here
            let mut row_group_writer = self.file_writer.next_row_group().unwrap();

            // We need to add the block_numbers to the first column before adding the rest of the data from the proto decoding
            let serialized_column_writer = row_group_writer.next_column().unwrap().unwrap();
            match serialized_column_writer {
                ColumnWriter::Int64ColumnWriter(ref mut column_writer) => column_writer.write_batch(self.block_numbers.as_slice(), None, None).unwrap()
            }
            serialized_column_writer.close().unwrap();

            self.decoder.write_data_to_parquet(&mut row_group_writer);

            let file_output_location = self.sink_output_location.get_file_location(*self.block_numbers.first().unwrap(), self.block_numbers.last().unwrap());

            self.block_numbers = Vec::new();

            Ok(Some(File::new(self.file_buffer.get_data(), file_output_location)))
        } else {
            Ok(None)
        }
    }
}

trait DescriptorProtoExt {
    fn is_collection_of_items(&self) -> bool;
}

impl DescriptorProtoExt for DescriptorProto {
    fn is_collection_of_items(&self) -> bool {
        self.field.len()==1 &&
            self.field[0].r#type==Some(field_descriptor_proto::Type::Message as i32) &&
            self.field[0].is_repeated()
    }
}

fn get_proto_type(proto_files: &Vec<FileDescriptorProto>, proto_type: &str) -> &DescriptorProto {
    for proto in proto_files.iter() {
        if proto_type.contains(proto.package.as_ref().unwrap()) {
            let message_type = proto_type.split('.').last().unwrap().to_string();
            for proto_message in proto.message_type.iter() {
                if proto_message.name.as_ref().unwrap() == &message_type {
                    return proto_message;
                }
            }
        }
    }

    panic!("TODO: Something like: Unable to find proto type!!");
}