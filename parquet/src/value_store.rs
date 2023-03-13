use std::borrow::BorrowMut;
use bytes::Bytes;
use parquet::data_type::ByteArray;

#[derive(Debug, PartialEq, Clone)]
pub enum ValueStore
{
    Double(Vec<f64>),
    Float(Vec<f32>),
    Int32(Vec<i32>),
    Int64(Vec<i64>),
    UInt32(Vec<i32>),
    UInt64(Vec<i64>),
    SInt32(Vec<i32>),
    SInt64(Vec<i64>),
    Fixed32(Vec<i32>),
    Fixed64(Vec<i64>),
    SFixed32(Vec<i32>),
    SFixed64(Vec<i64>),
    Bool(Vec<bool>),
    String(Vec<ByteArray>),
    Bytes(Vec<ByteArray>),
    Enum(Vec<i64>),
}

impl ValueStore {
    pub(crate) fn push_default_value(&mut self) {
        match self.borrow_mut() {
            ValueStore::Double(values) => values.push(0_f64),
            ValueStore::Float(values) => values.push(0_f32),
            ValueStore::Int32(values) => values.push(0),
            ValueStore::Int64(values) => values.push(0),
            ValueStore::UInt32(values) => values.push(0),
            ValueStore::UInt64(values) => values.push(0),
            ValueStore::SInt32(values) => values.push(0),
            ValueStore::SInt64(values) => values.push(0),
            ValueStore::Fixed32(values) => values.push(0),
            ValueStore::Fixed64(values) => values.push(0),
            ValueStore::SFixed32(values) => values.push(0),
            ValueStore::SFixed64(values) => values.push(0),
            ValueStore::Bool(values) => values.push(false),
            ValueStore::String(values) => values.push(ByteArray::from("")),
            ValueStore::Bytes(values) => values.push(ByteArray::from("")),
            ValueStore::Enum(values) => values.push(0),
        }
    }
}