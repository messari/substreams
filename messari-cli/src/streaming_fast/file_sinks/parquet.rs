use parquet::data_type::Int64Type;
use parquet::file::properties::WriterPropertiesPtr;
use parquet::file::writer::SerializedFileWriter;
use parquet::schema::types::TypePtr;
use derives::proto_structure_info::{FieldSpecification, MessageInfo};

use crate::streaming_fast::file_sinks::file_sink::FileSink;
use crate::streaming_fast::file_sinks::helpers::parquet::file_buffer::FileBuffer;
use crate::streaming_fast::file_sinks::helpers::parquet::parquet_schema_builder::ParquetSchemaBuilder;
use crate::streaming_fast::file_sinks::helpers::parquet::repetition_and_definition::{RepetitionAndDefinitionLvls, RepetitionAndDefinitionLvlStoreBuilder};
use crate::streaming_fast::file_sinks::helpers::parquet::struct_decoder::StructDecoder;

const UNCOMPRESSED_FILE_SIZE_THRESHOLD: usize = 500 * 1024 * 1024; // 500MB

pub(crate) struct ParquetFileSink {
    decoder: StructDecoder,
    struct_is_required: bool,
    uncompressed_file_size: usize,
    block_numbers: Vec<i64>,
    parquet_schema: TypePtr,
    writer_properties: WriterPropertiesPtr,
}

impl FileSink for ParquetFileSink {
    fn new(output_type_info: MessageInfo) -> Self {
        let mut parquet_schema_builder = ParquetSchemaBuilder::new(output_type_info.type_name.clone());

        let struct_is_required = output_type_info.field_specification == FieldSpecification::Required;

        let decoder = StructDecoder::new("", output_type_info, &mut parquet_schema_builder, &mut RepetitionAndDefinitionLvlStoreBuilder::new());

        let (parquet_schema, writer_properties) = parquet_schema_builder.compile();

        ParquetFileSink {
            decoder,
            struct_is_required,
            uncompressed_file_size: 0,
            block_numbers: vec![],
            parquet_schema,
            writer_properties,
        }
    }

    fn process(&mut self, proto_data: &mut &[u8], block_number: i64) -> Result<Option<Vec<u8>>, String> {
        if proto_data.is_empty() {
            if self.struct_is_required {
                self.decoder.push_null_or_default_values(&mut self.uncompressed_file_size, RepetitionAndDefinitionLvls::new())?;
                self.block_numbers.push(block_number);
            }
        } else {
            self.decoder.decode(proto_data, 2, &mut self.uncompressed_file_size, RepetitionAndDefinitionLvls::new())?;
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
        self.block_numbers.clear();

        self.decoder.write_data_to_parquet(&mut row_group_writer);

        row_group_writer.close().unwrap();
        file_writer.close().unwrap();

        file_buffer.get_data()
    }
}

#[cfg(test)]
mod tests {
    use derives::{ProtoInfo, TestData};
    use parquet::file::footer;
    use parquet::file::reader::{FileReader, SerializedFileReader};
    use tonic::metadata;
    use crate::streaming_fast::file_sinks::file_sink::FileSink;
    use crate::streaming_fast::file_sinks::parquet::ParquetFileSink;

    use crate::streaming_fast::streaming_fast_utils::assert_data_sinks_to_parquet_correctly;

    #[test]
    fn test_oneof() {
        #[derive(TestData)]
        pub struct AnotherStruct {
            field1: Vec<u64>,
            field2: Option<String>,
            field3: u32
        }

        #[derive(TestData)]
        pub enum ExampleEnum {
            Variant1(u64),
            Variant2(String),
            Variant3(AnotherStruct)
        }

        #[derive(TestData)]
        pub struct EnumTest {
            #[proto_type(Oneof[(Variant1,u64), (Variant2,String), (Variant3,AnotherStruct)])]
            field1: ExampleEnum,
            field2: Vec<String>
        }

        assert_data_sinks_to_parquet_correctly::<EnumTest>();
    }

    #[test]
    fn test_optional_and_repeated_fields() {
        #[derive(TestData)]
        pub struct OptionalAndRepeatedFields {
            field1: String,
            field2: Option<u64>,
            field3: Vec<String>,
            field4: Vec<u64>,
        }

        #[derive(TestData)]
        pub struct TwoLayeredOptionalAndRepeatedFields {
            field1: String,
            field2: Option<u64>,
            field3: Vec<String>,
            field4: Vec<u64>,
            field5: OptionalAndRepeatedFields,
            field6: Option<OptionalAndRepeatedFields>,
            field7: Vec<OptionalAndRepeatedFields>
        }

        assert_data_sinks_to_parquet_correctly::<TwoLayeredOptionalAndRepeatedFields>();
    }

    #[test]
    fn test_escrow_reward() {
        #[derive(TestData)]
        pub struct Timestamp {
            timestamp: u64
        }

        #[derive(TestData)]
        pub struct BigInt {
            val: String
        }

        #[derive(TestData)]
        pub enum BalanceType {
            ESCROWED,
            VESTED,
        }

        #[derive(TestData)]
        pub enum EscrowContractVersion {
            V1,
            V2,
        }

        #[derive(TestData)]
        pub struct EscrowReward {
            #[proto_type(Enum)]
            balance_type: BalanceType,
            #[proto_type(Enum)]
            escrow_contract_version: EscrowContractVersion,
            balance: BigInt,
            holder: String,
            timestamp: Timestamp
        }

        assert_data_sinks_to_parquet_correctly::<EscrowReward>();
    }

    #[test]
    fn test_token_balance() {
        #[derive(TestData)]
        pub struct Timestamp {
            timestamp: u64
        }

        #[derive(TestData)]
        pub struct BigInt {
            val: String
        }

        #[derive(TestData)]
        pub struct TokenBalance {
            token: String,
            holder: String,
            balance: BigInt,
            timestamp: Timestamp
        }

        assert_data_sinks_to_parquet_correctly::<TokenBalance>();
    }

    #[test]
    fn test_flat_simple() {
        #[derive(TestData)]
        pub enum TestEnum {
            Field1,
            Field2,
            Field3
        }

        #[derive(TestData)]
        pub struct FlatSimple {
            field1: u32,
            field2: u64,
            field3: i32,
            field4: i64,
            #[proto_type(Enum)]
            field5: TestEnum,
            field6: String
            // TODO: Put all types here for testing
        }

        assert_data_sinks_to_parquet_correctly::<FlatSimple>();
    }
}