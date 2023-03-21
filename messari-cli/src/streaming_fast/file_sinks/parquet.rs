use parquet::data_type::Int64Type;
use parquet::file::properties::WriterPropertiesPtr;
use parquet::file::writer::SerializedFileWriter;
use parquet::schema::types::TypePtr;

use crate::streaming_fast::file_sinks::file_sink::FileSink;
use crate::streaming_fast::file_sinks::helpers::parquet::file_buffer::FileBuffer;
use crate::streaming_fast::file_sinks::helpers::parquet::parquet_schema_builder::ParquetSchemaBuilder;
use crate::streaming_fast::file_sinks::helpers::parquet::struct_decoder::StructDecoder;
use crate::streaming_fast::proto_structure_info::MessageInfo;

const UNCOMPRESSED_FILE_SIZE_THRESHOLD: usize = 500 * 1024 * 1024; // 500MB

pub(crate) struct ParquetFileSink {
    decoder: StructDecoder,
    uncompressed_file_size: usize,
    block_numbers: Vec<i64>,
    parquet_schema: TypePtr,
    writer_properties: WriterPropertiesPtr,
}

impl FileSink for ParquetFileSink {
    fn new(output_type_info: MessageInfo) -> Self {
        let mut parquet_schema_builder = ParquetSchemaBuilder::new(output_type_info.type_name.clone());
        let decoder = StructDecoder::new(output_type_info, &mut parquet_schema_builder, false, false);

        let (parquet_schema, writer_properties) = parquet_schema_builder.compile();

        ParquetFileSink {
            decoder,
            uncompressed_file_size: 0,
            block_numbers: vec![],
            parquet_schema,
            writer_properties,
        }
    }

    fn process(&mut self, proto_data: &mut &[u8], block_number: i64) -> Result<Option<Vec<u8>>, String> {
        if proto_data.is_empty() {
            if self.decoder.push_null_or_default_values(&mut self.uncompressed_file_size, 0, &mut 0)? {
                self.block_numbers.push(block_number);
            }
        } else {
            self.decoder.decode(proto_data, 2, &mut self.uncompressed_file_size, 0, &mut 0)?;
            self.block_numbers.push(block_number);
        }

        if self.uncompressed_file_size > UNCOMPRESSED_FILE_SIZE_THRESHOLD {
            self.uncompressed_file_size = 0;
            Ok(Some(self.make_file()))
        } else {
            Ok(None)
        }
    }

    fn make_file(&mut self) -> Vec<u8> {
        if self.block_numbers.is_empty() {
            return Vec::new();
        }

        let file_buffer = FileBuffer::new();
        let mut file_writer = SerializedFileWriter::new(file_buffer.clone(), self.parquet_schema.clone(), self.writer_properties.clone()).unwrap();
        let mut row_group_writer = file_writer.next_row_group().unwrap();

        // We need to add the block_numbers to the first column before adding the rest of the data from the proto decoding (block_number is the primary key for our data!)
        let mut serialized_column_writer = row_group_writer.next_column().unwrap().unwrap();
        serialized_column_writer.typed::<Int64Type>().write_batch(self.block_numbers.as_slice(), None, None).unwrap();
        serialized_column_writer.close().unwrap();
        self.block_numbers = Vec::new();

        self.decoder.write_data_to_parquet(&mut row_group_writer);

        row_group_writer.close().unwrap();
        file_writer.close().unwrap();

        file_buffer.get_data()
    }
}

#[cfg(test)]
mod tests {
    use parquet::file::reader::{FileReader, SerializedFileReader};
    use prost_types::FileDescriptorProto;
    use crate::streaming_fast::file_sinks::file_sink::FileSink;
    use crate::streaming_fast::file_sinks::helpers::parquet::test_helpers::{FlatSimple, get_parquet_sink, TestSinkType};
    use crate::streaming_fast::file_sinks::parquet::ParquetFileSink;
    use crate::streaming_fast::sink::Sink;

    const DUMMY_BLOCK_NUMBER: i64 = 1;

    #[test]
    fn test_flat_simple() {
        let flat_simple_samples = FlatSimple::generate_data_samples(3);
        let mut sink = get_parquet_sink::<FlatSimple>();

        assert_data_sinks_correctly_from_proto_to_parquet(flat_simple_samples, &mut sink);
    }

    fn assert_data_sinks_correctly_from_proto_to_parquet<T: TestSinkType>(test_data: Vec<T>, sink: &mut ParquetFileSink) {
        for test_datum in test_data.iter() {
            let bytes: Vec<u8> = test_datum.encode_to_proto();

            sink.process(&mut bytes.as_slice(), DUMMY_BLOCK_NUMBER).unwrap();
        }

        let parquet_file_data = sink.make_file();
        let reader = SerializedFileReader::new(bytes::Bytes::from(parquet_file_data)).unwrap();

        for (parquet_row, test_datum) in reader.get_row_iter(None).unwrap().zip(test_data) {
            let parsed_data = T::get_from_parquet_row(parquet_row).0;
            assert_eq!(parsed_data, test_datum);
        }
    }
}