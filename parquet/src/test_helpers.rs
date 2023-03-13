use std::any::Any;
use std::default;
use std::fmt::Debug;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::{Field, Row};
use prost::Message;
use prost_types::{DescriptorProto, FieldDescriptorProto, FileDescriptorProto};
use prost_types::field_descriptor_proto::{Label, Type};
use crate::file::LocationType;

use crate::parquet_sink::ParquetSink;
use crate::sink::Sink;

pub(crate) fn assert_data_sinks_correctly_from_proto_to_parquet<T: TestSinkType>(test_data: Vec<T>, sink: &mut ParquetSink) {
    const DUMMY_BLOCK_NUMBER: i64 = 1;

    for test_datum in test_data.iter() {
        let bytes: Vec<u8> = test_datum.encode_to_proto();

        sink.process(bytes, DUMMY_BLOCK_NUMBER).unwrap();
    }

    let parquet_file_data = sink.make_file().get_data();
    let reader = SerializedFileReader::new(bytes::Bytes::from(parquet_file_data)).unwrap();

    for (parquet_row, test_datum) in reader.get_row_iter(None).unwrap().zip(test_data) {
        let parsed_data = T::get_from_parquet_row(parquet_row).0;
        assert_eq!(parsed_data, test_datum);
    }
}

pub(crate) fn get_parquet_sink<T: TestSinkType>() -> ParquetSink {
    let mut descriptor_proto = T::get_descriptor_proto();
    descriptor_proto.name = Some(::prost::alloc::string::String::from("Only1Message"));

    let mut file_descriptor_proto = FileDescriptorProto::default();
    file_descriptor_proto.package = Some(::prost::alloc::string::String::from("Only1File"));
    file_descriptor_proto.message_type = vec![descriptor_proto];

    ParquetSink::new(&vec![file_descriptor_proto], "Only1File.Only1Message", LocationType::Local, None)
}

pub(crate) trait TestSinkType: PartialEq + Debug {
    fn encode_to_proto(&self) -> Vec<u8>;
    fn get_from_parquet_row(row: Row) -> (Self, u64) where Self: Sized;
    // TODO: Give an rng object here to seed the random values instead of the current method of stubbing them
    fn generate_data_samples(num_samples: usize) -> Vec<Self> where Self: Sized;
    fn get_descriptor_proto() -> DescriptorProto;
}

#[derive(Default, PartialEq, Debug)]
pub(crate) struct FlatSimple {
    field1: u32,
    field2: u64,
    field3: i32,
    field4: i64,
    field5: String,
    // TODO: Put all types here for testing
}

impl TestSinkType for FlatSimple {
    fn encode_to_proto(&self) -> Vec<u8> {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FlatSimpleProtoDto {
            #[prost(uint32, tag="1")]
            pub field1: u32,
            #[prost(uint64, tag="2")]
            pub field2: u64,
            #[prost(int32, tag="3")]
            pub field3: i32,
            #[prost(int64, tag="4")]
            pub field4: i64,
            #[prost(string, tag="5")]
            pub field5: ::prost::alloc::string::String,
        }

        let proto_dto = FlatSimpleProtoDto {
            field1: self.field1,
            field2: self.field2,
            field3: self.field3,
            field4: self.field4,
            field5: self.field5.clone(),
        };

        proto_dto.encode_to_vec()
    }

    fn get_from_parquet_row(row: Row) -> (Self, u64) {
        const REQUIRED_FIELDS: [&str; 5] = ["field1", "field2", "field3", "field4", "field5"];

        let mut block_number = 0_u64;
        let mut flat_simple: FlatSimple = Default::default();
        let mut fields_seen = Vec::new();
        for (field_name, field_value) in row.get_column_iter() {
            match (field_name.as_str(), field_value) {
                ("field1", Field::UInt(val)) => {
                    flat_simple.field1 = *val;
                }
                ("field2", Field::ULong(val)) => {
                    flat_simple.field2 = *val;
                }
                ("field3", Field::Int(val)) => {
                    flat_simple.field3 = *val;
                }
                ("field4", Field::Long(val)) => {
                    flat_simple.field4 = *val;
                }
                ("field5", Field::Str(val)) => {
                    flat_simple.field5 = val.clone();
                }
                ("block_number", Field::ULong(block_num)) => {
                    block_number = *block_num;
                }
                _ => {
                    assert!(REQUIRED_FIELDS.contains(&field_name.as_str()), "{} is not a valid field name for this struct. Accepted field names = {:?}", field_name, REQUIRED_FIELDS);
                    panic!("field name: {}, does not match type of field value: {:?}!", field_name, field_value);
                }
            }
            fields_seen.push(field_name.clone());
        }

        if fields_seen.len() != 5 {
            for field in REQUIRED_FIELDS {
                if !fields_seen.contains(&field.to_string()) {
                    panic!("{} was not seen in parquet row data. This is a required field - something has gone wrong!!", field);
                }
            }
        }

        (flat_simple, block_number)
    }

    fn generate_data_samples(num_samples: usize) -> Vec<Self> {
        // TODO: Generate these samples automatically using rng
        vec![
            FlatSimple {
                field1: 42,
                field2: 123,
                field3: -21,
                field4: -643345324,
                field5: "1234wdc11111w".to_string(),
            },
            FlatSimple {
                field1: 0,
                field2: 0,
                field3: 0,
                field4: 0,
                field5: "".to_string(),
            },
            FlatSimple {
                field1: 21,
                field2: 1243321,
                field3: 32,
                field4: 89,
                field5: "TESTING_123".to_string(),
            }
        ]
    }

    fn get_descriptor_proto() -> DescriptorProto {
        get_descriptor_proto(vec![
            ("field1".to_string(), Type::Uint32),
            ("field2".to_string(), Type::Uint64),
            ("field3".to_string(), Type::Int32),
            ("field4".to_string(), Type::Int64),
            ("field5".to_string(), Type::String)
        ])
    }
}

fn get_descriptor_proto(field_info: Vec<(String, Type)>) -> DescriptorProto {
    let field_descriptors = field_info.into_iter().enumerate().map(|(field_number, (field_name, field_type))| FieldDescriptorProto {
        name: Some(field_name),
        label: Some(Label::Required as i32),
        number: Some(field_number as i32),
        r#type: Some(field_type as i32),
        ..Default::default()
    }).collect::<Vec<_>>();

    DescriptorProto {
        field: field_descriptors,
        ..Default::default()
    }
}

struct FlatDremel {
    // TODO: Put all types but with optional and vec wraps over each type
}

// TODO: impl TestSinkType for FlatDremel {}

struct HierarchicalSimple {
    // TODO
}

// TODO: impl TestSinkType for HierarchicalSimple {}

struct HierarchicalDremel {
    // TODO
}

// TODO: impl TestSinkType for HierarchicalDremel {}
