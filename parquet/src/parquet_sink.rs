use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use parquet::data_type::Int64Type;
use parquet::file::properties::WriterPropertiesPtr;
use parquet::file::writer::SerializedFileWriter;
use prost_types::{DescriptorProto, FileDescriptorProto};
use prost_types::field_descriptor_proto;
use parquet::schema::types::{BasicTypeInfo, PrimitiveTypeBuilder, TypePtr};
use prost::Message;
use crate::decoder::{FieldSpecification, ProtoFieldExt};

use crate::file::{File, Location, LocationType};
use crate::file_buffer::FileBuffer;
use crate::parquet_schema_builder::ParquetSchemaBuilder;
use crate::process_substream::EncodingType;
use crate::sink::Sink;
use crate::struct_decoder::StructDecoder;

const UNCOMPRESSED_FILE_SIZE_THRESHOLD: usize = 500 * 1024 * 1024; // 500MB

pub(crate) struct ParquetSink {
    // TODO: We can write to multiple different outputs by wrapping innards in a struct and having a separate
    // TODO: instance of each of this struct for each sub field that wants it's own output file type
    items_field_number: Option<u64>,
    uncompressed_file_size: usize,
    decoder: StructDecoder,
    block_numbers: Vec<i64>,
    parquet_schema: TypePtr,
    writer_properties: WriterPropertiesPtr,
    sink_output_location: Location
}

impl ParquetSink {
    /// data_location_root allows one to point out where the "data lake" starts. (If left blank for a local output it will default to the current directory)
    pub(crate) fn new(proto_descriptors: &Vec<FileDescriptorProto>, proto_type_name: &str, location_type: LocationType, data_location_path: Option<PathBuf>) -> Self {
        let mut sink_output_path = if let Some(data_location_path) = data_location_path {
            data_location_path
        } else {
            match location_type {
                LocationType::Local => {
                    std::env::current_dir().unwrap()
                },
                LocationType::DataWarehouse => PathBuf::new()
            }
        };

        let mut proto_type = get_proto_type(proto_descriptors, proto_type_name);
        if proto_type.is_collection_of_items() {
            let inner_type_name = proto_type.field[0].type_name.as_ref().unwrap();
            let items_field_number = proto_type.field[0].get_field_number();
            proto_type = get_proto_type(proto_descriptors, inner_type_name);

            let mut parquet_schema_builder = ParquetSchemaBuilder::new(proto_type.name());
            let decoder = StructDecoder::new(proto_descriptors, proto_type, &mut parquet_schema_builder, false, false, FieldSpecification::Required);

            let (parquet_schema, writer_properties) = parquet_schema_builder.compile();

            add_partitions_to_sink_output_folder(&mut sink_output_path, proto_type_name);
            fs::create_dir_all(&sink_output_path).unwrap();

            let sink_output_location = Location::new(location_type, sink_output_path);

            ParquetSink {
                items_field_number: Some(items_field_number),
                uncompressed_file_size: 0,
                decoder,
                block_numbers: vec![],
                parquet_schema,
                writer_properties,
                sink_output_location
            }
        } else {
            let mut parquet_schema_builder = ParquetSchemaBuilder::new(proto_type.name());
            let decoder = StructDecoder::new(proto_descriptors, proto_type, &mut parquet_schema_builder, false, false, FieldSpecification::Required);

            let (parquet_schema, writer_properties) = parquet_schema_builder.compile();

            add_partitions_to_sink_output_folder(&mut sink_output_path, proto_type_name);
            fs::create_dir_all(&sink_output_path).unwrap();

            let sink_output_location = Location::new(location_type, sink_output_path);

            ParquetSink {
                items_field_number: None,
                uncompressed_file_size: 0,
                decoder,
                block_numbers: vec![],
                parquet_schema,
                writer_properties,
                sink_output_location
            }
        }
    }
}

fn add_partitions_to_sink_output_folder(sink_output_path: &mut PathBuf, proto_type_name: &str) {
    let proto_type = proto_type_name.replace("proto:", "");

    if proto_type.starts_with('.') {
        for proto_type_part in proto_type.split('.').skip(1) {
            sink_output_path.push(proto_type_part);
        }
    } else {
        for proto_type_part in proto_type.split('.') {
            sink_output_path.push(proto_type_part);
        }
    }
    sink_output_path.push("parquet");
}

impl Sink for ParquetSink {
    fn process(&mut self, proto_data: Vec<u8>, block_number: i64) -> Result<Option<File>, String> {
        if proto_data.is_empty() {
            return Ok(None);
        }

        if let Some(items_field_number) = self.items_field_number {
            let mut data_slice = proto_data.as_slice();

            loop {
                if proto_data.is_empty() {
                    println!("Straight out!! - {}", block_number);
                    break;
                }

                let tag = match u64::from_unsigned_varint(&mut data_slice) {
                    Some(tag) => tag,
                    None => {
                        println!("Straight out!!");
                        // return Err("TODO: Write error for this!1".to_string());
                        break;
                    }
                };

                let field_number = tag >> 3;
                let wire_type = (tag & 0x07) as u8;

                assert_eq!(items_field_number, field_number, "TODO: Write error message!!");

                println!("Fieldno: {}, wiretype: {}", field_number, wire_type);

                let struct_data_length = usize::from_unsigned_varint(&mut data_slice).unwrap();
                if data_slice.len() < struct_data_length {
                    return Err("TODO: Write error for this!3".to_string());
                }
                let (mut consumed, remainder) = data_slice.split_at(struct_data_length);
                data_slice = remainder;

                self.decoder.decode(&mut consumed, wire_type, &mut self.uncompressed_file_size, 0, &mut 0)?;
                self.block_numbers.push(block_number);
            }
        } else {
            // TODO: Figure out which wire_type is for struct types here
            println!("woifoifnwe");
            self.decoder.decode(&mut proto_data.as_slice(), 2, &mut self.uncompressed_file_size, 0, &mut 0)?;
            self.block_numbers.push(block_number);
        }

        if self.uncompressed_file_size > UNCOMPRESSED_FILE_SIZE_THRESHOLD {
            Ok(Some(self.make_file()))
        } else {
            Ok(None)
        }
    }

    /// Return a parquet file with an associated target location attached
    fn make_file(&mut self) -> File {
        // Write and return parquet file here
        let mut file_buffer = FileBuffer::new();
        let mut file_writer = SerializedFileWriter::new(file_buffer.clone(), self.parquet_schema.clone(), self.writer_properties.clone()).unwrap();
        let mut row_group_writer = file_writer.next_row_group().unwrap();

        // We need to add the block_numbers to the first column before adding the rest of the data from the proto decoding
        let mut serialized_column_writer = row_group_writer.next_column().unwrap().unwrap();
        serialized_column_writer.typed::<Int64Type>().write_batch(self.block_numbers.as_slice(), None, None).unwrap();
        serialized_column_writer.close().unwrap();

        println!("Block Numbers: {:?}", self.block_numbers);

        self.decoder.write_data_to_parquet(&mut row_group_writer);

        row_group_writer.close().unwrap();
        file_writer.close().unwrap();

        let file_output_location = self.sink_output_location.get_file_location(*self.block_numbers.first().unwrap(), *self.block_numbers.last().unwrap(), EncodingType::Parquet);

        self.block_numbers = Vec::new();

        File::new(file_buffer.get_data(), file_output_location)
    }
}

trait DescriptorProtoExt {
    fn is_collection_of_items(&self) -> bool;
}

impl DescriptorProtoExt for DescriptorProto {
    fn is_collection_of_items(&self) -> bool {
        self.field.len()==1 && self.field[0].is_collection_of_items_field()
    }
}

pub(crate) fn get_proto_type<'a>(proto_files: &'a Vec<FileDescriptorProto>, proto_type: &str) -> &'a DescriptorProto {
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

#[cfg(test)]
mod tests {
    use prost_types::FileDescriptorProto;
    use crate::parquet_sink::ParquetSink;
    use crate::test_helpers;
    use crate::test_helpers::{assert_data_sinks_correctly_from_proto_to_parquet, FlatSimple, get_parquet_sink, TestSinkType};

    #[test]
    fn test_flat_simple() {
        let flat_simple_samples = FlatSimple::generate_data_samples(3);
        let mut sink = get_parquet_sink::<FlatSimple>();

        assert_data_sinks_correctly_from_proto_to_parquet(flat_simple_samples, &mut sink);
    }
}