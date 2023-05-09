// use std::fmt::Debug;
// use parquet::record::{Field, Row};
// use prost::Message;
// use rand::distributions::{Distribution, Standard};
// use rand::Rng;
// use rand_derive2::RandGen;
//
// use crate::streaming_fast::file_sinks::file_sink::FileSink;
// use crate::streaming_fast::file_sinks::parquet::ParquetFileSink;
// use crate::streaming_fast::proto_structure_info::{FieldInfo, FieldSpecification, FieldType, MessageInfo};
//
// pub(in crate::streaming_fast::file_sinks) fn get_parquet_sink<T: TestSinkType>() -> ParquetFileSink {
//     ParquetFileSink::new(T::get_proto_structure_info())
// }
//
// pub(in crate::streaming_fast::file_sinks) trait TestSinkType: PartialEq + Debug + GenRandSamples {
//     fn encode_to_proto(&self) -> Vec<u8>;
//     fn get_from_parquet_row(row: Row) -> (Self, u64) where Self: Sized;
//     fn get_proto_structure_info() -> MessageInfo;
// }
//
// pub(in crate::streaming_fast::file_sinks) trait GenRandSamples {
//     /// Will panic if you ask if to generate you anything less than 10 samples. Also when generating samples
//     /// 3 of the samples are guaranteed to be default value samples
//     fn get_samples<T: Rng>(num_samples: usize, rng: &mut T) -> Vec<Self> where Self: Sized;
// }
//
// impl Clone for u64 {
//     fn clone(&self) -> Self {
//         todo!()
//     }
// }
//
// impl<T: Default + Clone> GenRandSamples for T where Standard: Distribution<T> {
//     fn get_samples<R: Rng>(num_samples: usize, rng: &mut R) -> Vec<Self> {
//         assert!(num_samples >= 10);
//
//         let mut samples = Vec::with_capacity(num_samples);
//         for _ in 0..num_samples {
//             samples.push(rng.gen());
//         }
//
//         let default_value = T::default();
//         for _ in 0..3 {
//             let insertion_index = rng.gen_range(0..samples.len());
//             samples.insert(insertion_index, default_value.clone());
//         }
//
//         samples
//     }
// }
//
// #[derive(Default, PartialEq, Debug, RandGen, Clone)]
// pub(in crate::streaming_fast::file_sinks) struct FlatSimple {
//     field1: u32,
//     field2: u64,
//     field3: i32,
//     field4: i64,
//     field5: String,
//     // TODO: Put all types here for testing
// }
//
// impl TestSinkType for FlatSimple {
//     fn encode_to_proto(&self) -> Vec<u8> {
//         #[derive(Clone, PartialEq, ::prost::Message)]
//         pub struct FlatSimpleProtoDto {
//             #[prost(uint32, tag="1")]
//             pub field1: u32,
//             #[prost(uint64, tag="2")]
//             pub field2: u64,
//             #[prost(int32, tag="3")]
//             pub field3: i32,
//             #[prost(int64, tag="4")]
//             pub field4: i64,
//             #[prost(string, tag="5")]
//             pub field5: ::prost::alloc::string::String,
//         }
//
//         let proto_dto = FlatSimpleProtoDto {
//             field1: self.field1,
//             field2: self.field2,
//             field3: self.field3,
//             field4: self.field4,
//             field5: self.field5.clone(),
//         };
//
//         proto_dto.encode_to_vec()
//     }
//
//     fn get_from_parquet_row(row: Row) -> (Self, u64) {
//         const REQUIRED_FIELDS: [&str; 5] = ["field1", "field2", "field3", "field4", "field5"];
//
//         let mut block_number = 0_u64;
//         let mut flat_simple: FlatSimple = Default::default();
//         let mut fields_seen = Vec::new();
//         for (field_name, field_value) in row.get_column_iter() {
//             match (field_name.as_str(), field_value) {
//                 ("field1", Field::UInt(val)) => {
//                     flat_simple.field1 = *val;
//                 }
//                 ("field2", Field::ULong(val)) => {
//                     flat_simple.field2 = *val;
//                 }
//                 ("field3", Field::Int(val)) => {
//                     flat_simple.field3 = *val;
//                 }
//                 ("field4", Field::Long(val)) => {
//                     flat_simple.field4 = *val;
//                 }
//                 ("field5", Field::Str(val)) => {
//                     flat_simple.field5 = val.clone();
//                 }
//                 ("block_number", Field::ULong(block_num)) => {
//                     block_number = *block_num;
//                 }
//                 _ => {
//                     assert!(REQUIRED_FIELDS.contains(&field_name.as_str()), "{} is not a valid field name for this struct. Accepted field names = {:?}", field_name, REQUIRED_FIELDS);
//                     panic!("field name: {}, does not match type of field value: {:?}!", field_name, field_value);
//                 }
//             }
//             fields_seen.push(field_name.clone());
//         }
//
//         if fields_seen.len() != 5 {
//             for field in REQUIRED_FIELDS {
//                 if !fields_seen.contains(&field.to_string()) {
//                     panic!("{} was not seen in parquet row data. This is a required field - something has gone wrong!!", field);
//                 }
//             }
//         }
//
//         (flat_simple, block_number)
//     }
//
//     fn get_proto_structure_info() -> MessageInfo {
//         get_message_info(vec![
//             ("field1".to_string(), FieldType::Uint32),
//             ("field2".to_string(), FieldType::Uint64),
//             ("field3".to_string(), FieldType::Int32),
//             ("field4".to_string(), FieldType::Int64),
//             ("field5".to_string(), FieldType::String)
//         ])
//     }
// }
//
// fn get_message_info(field_info: Vec<(String, FieldType)>) -> MessageInfo {
//     MessageInfo {
//         type_name: Default::default(),
//         field_specification: FieldSpecification::Required,
//         fields: field_info.into_iter().enumerate().map(|(field_number, (field_name, field_type))| FieldInfo {
//             field_name,
//             field_type,
//             field_specification: FieldSpecification::Required,
//             field_number: (field_number+1) as u64,
//         }).collect::<Vec<_>>(),
//         oneof_groups: vec![],
//     }
// }
//
// #[allow(unused)]
// pub(in crate::streaming_fast::file_sinks) struct FlatDremel {
//     // TODO: Put all types but with optional and vec wraps over each type
// }
//
// // TODO: impl TestSinkType for FlatDremel {}
//
// #[allow(unused)]
// pub(in crate::streaming_fast::file_sinks) struct HierarchicalSimple {
//     // TODO
// }
//
// // TODO: impl TestSinkType for HierarchicalSimple {}
//
// #[allow(unused)]
// pub(in crate::streaming_fast::file_sinks) struct HierarchicalDremel {
//     // TODO
// }
//
// // TODO: impl TestSinkType for HierarchicalDremel {}
