pub mod proto_structure_info;

pub use derive_macros::TestData;

use std::fmt::Debug;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use parquet::record::Row;
use parquet::record::Field;
use prost::Message;

use crate::proto_structure_info::{FieldInfo, FieldSpecification, FieldType, MessageInfo};

pub trait TestData: ProtoInfo + GenRandSamples + PartialEq + Debug {
    type ProtoType;

    fn to_proto_bytes(&self) -> Vec<u8>;
    fn get_proto_value(&self) -> Self::ProtoType;
    fn get_from_parquet_row<'a, T: Iterator<Item=(&'a String, &'a Field)>>(row: T) -> (Self, Option<u64>) where Self: Sized;
}

pub trait ProtoInfo {
    fn add_proto_field_info(field_name: String, field_number: &mut u8, fields: &mut Vec<FieldInfo>);
    fn get_proto_structure_info() -> MessageInfo;
}

pub trait GenRandSamples: Default + Clone {
    fn get_sample<T: Rng>(rng: &mut T) -> Self where Self: Sized;
    /// Will panic if you ask if to generate you anything less than 10 samples. Also when generating samples
    /// 3 of the samples are guaranteed to be default value samples
    fn get_samples<R: rand::Rng>(num_samples: usize, rng: &mut R) -> Vec<Self> {
        assert!(num_samples >= 10, "You should be using at least 10 samples for any given test!");

        let mut samples = Vec::with_capacity(num_samples);
        for _ in 0..(num_samples-3) {
            samples.push(Self::get_sample(rng));
        }

        let default_value = Self::default();
        for _ in 0..3 {
            let insertion_index = rng.gen_range(0..samples.len());
            samples.insert(insertion_index, default_value.clone());
        }

        samples
    }
}

impl<T: ProtoInfo> ProtoInfo for Option<T> {
    fn add_proto_field_info(field_name: String, field_number: &mut u8, fields: &mut Vec<FieldInfo>) {
        let mut field_infos = Vec::new();
        T::add_proto_field_info(field_name, field_number, &mut field_infos);
        assert_eq!(field_infos.len(), 1, "Option<Enum> not supported!");
        let mut field_info: FieldInfo = field_infos.into_iter().next().unwrap();
        field_info.field_specification = FieldSpecification::Optional;
        if let FieldType::Message(ref mut message_info) = field_info.field_type {
            message_info.field_specification = FieldSpecification::Optional;
        }
        fields.push(field_info);
    }

    fn get_proto_structure_info() -> MessageInfo {
        unreachable!()
    }
}

impl<T: ProtoInfo> ProtoInfo for Vec<T> {
    fn add_proto_field_info(field_name: String, field_number: &mut u8, fields: &mut Vec<FieldInfo>) {
        let mut field_infos = Vec::new();
        T::add_proto_field_info(field_name, field_number, &mut field_infos);
        assert_eq!(field_infos.len(), 1, "Vec<Enum> not supported!");
        let mut field_info: FieldInfo = field_infos.into_iter().next().unwrap();

        match field_info.field_type {
            FieldType::Double | FieldType::Float | FieldType::Int64 | FieldType::Uint64 |
            FieldType::Int32 | FieldType::Fixed64 | FieldType::Fixed32 | FieldType::Bool |
            FieldType::Uint32 | FieldType::Sfixed32 | FieldType::Sfixed64 | FieldType::Sint32 |
            FieldType::Sint64 => {
                field_info.field_specification = FieldSpecification::Packed;
            },
            FieldType::Message(ref mut message_info) => {
                message_info.field_specification = FieldSpecification::Repeated;
                field_info.field_specification = FieldSpecification::Repeated;
            }
            _ => {
                field_info.field_specification = FieldSpecification::Repeated;
            }
        }

        fields.push(field_info);
    }

    fn get_proto_structure_info() -> MessageInfo {
        unreachable!()
    }
}

macro_rules! impl_proto_info {
    ($type_ident:ty, $field_type_ident:ident) => {
        impl ProtoInfo for $type_ident {
            fn add_proto_field_info(field_name: String, field_number: &mut u8, fields: &mut Vec<FieldInfo>) {
                fields.push(FieldInfo {
                    field_name,
                    field_type: FieldType::$field_type_ident,
                    field_specification: FieldSpecification::Required,
                    field_number: *field_number as u64,
                });

                *field_number += 1;
            }

            fn get_proto_structure_info() -> MessageInfo {
                unreachable!()
            }
        }
    };
}

impl_proto_info!(bool, Bool);
impl_proto_info!(i32, Int32);
impl_proto_info!(i64, Int64);
impl_proto_info!(u32, Uint32);
impl_proto_info!(u64, Uint64);
impl_proto_info!(f32, Float);
impl_proto_info!(f64, Double);
impl_proto_info!(Vec<u8>, Bytes);
impl_proto_info!(String, String);

impl<O: TestData> TestData for Option<O> {
    type ProtoType = Option<O::ProtoType>;

    fn to_proto_bytes(&self) -> Vec<u8> {
        unreachable!()
    }

    fn get_proto_value(&self) -> Self::ProtoType {
        self.as_ref().map(|x| x.get_proto_value())
    }

    fn get_from_parquet_row<'a, T: Iterator<Item=(&'a String, &'a Field)>>(row: T) -> (Self, Option<u64>) where Self: Sized {
        unreachable!()
    }
}

impl<V: TestData> TestData for Vec<V> {
    type ProtoType = Vec<V::ProtoType>;

    fn to_proto_bytes(&self) -> Vec<u8> {
        unreachable!()
    }

    fn get_proto_value(&self) -> Self::ProtoType {
        self.iter().map(|x| x.get_proto_value()).collect()
    }

    fn get_from_parquet_row<'a, T: Iterator<Item=(&'a String, &'a Field)>>(row: T) -> (Self, Option<u64>) where Self: Sized {
        unreachable!()
    }
}

macro_rules! impl_test_data {
    ($type_ident:ty) => {
        impl TestData for $type_ident {
            type ProtoType = $type_ident;

            fn to_proto_bytes(&self) -> Vec<u8> {
                unreachable!()
            }

            fn get_proto_value(&self) -> Self::ProtoType {
                self.clone()
            }

            fn get_from_parquet_row<'a, T: Iterator<Item=(&'a String, &'a Field)>>(row: T) -> (Self, Option<u64>) where Self: Sized {
                unreachable!()
            }
        }
    };
}

impl_test_data!(bool);
impl_test_data!(i32);
impl_test_data!(i64);
impl_test_data!(u32);
impl_test_data!(u64);
impl_test_data!(f32);
impl_test_data!(f64);
impl_test_data!(Vec<u8>);
impl_test_data!(String);

impl<T: GenRandSamples> GenRandSamples for Option<T> {
    fn get_sample<R: Rng>(rng: &mut R) -> Self where Self: Sized {
        if rng.gen() {
            Some(T::get_sample(rng))
        } else {
            None
        }
    }
}

impl<T: GenRandSamples> GenRandSamples for Vec<T> {
    fn get_sample<R: Rng>(rng: &mut R) -> Self where Self: Sized {
        T::get_samples(rng.gen_range(10..15), rng)
    }
}

macro_rules! impl_gen_rand_samples {
    ($type_ident:ty) => {
        impl GenRandSamples for $type_ident {
            fn get_sample<R: Rng>(rng: &mut R) -> Self where Self: Sized {
                rng.gen()
            }
        }
    };
}

impl_gen_rand_samples!(bool);
impl_gen_rand_samples!(i32);
impl_gen_rand_samples!(i64);
impl_gen_rand_samples!(f32);
impl_gen_rand_samples!(f64);
impl_gen_rand_samples!(u8);

impl GenRandSamples for String {
    fn get_sample<T: Rng>(rng: &mut T) -> Self where Self: Sized {
        (0..rng.gen_range(5..15)).into_iter().map(|_| rng.gen::<char>()).collect()
    }
}

impl GenRandSamples for u32 {
    fn get_sample<T: Rng>(rng: &mut T) -> Self where Self: Sized {
        rng.gen_range(0..i32::MAX) as u32
    }
}

impl GenRandSamples for u64 {
    fn get_sample<T: Rng>(rng: &mut T) -> Self where Self: Sized {
        rng.gen_range(0..i64::MAX) as u64
    }
}