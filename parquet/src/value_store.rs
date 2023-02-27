use bytes::Bytes;

#[derive(Debug, PartialEq, Clone)]
pub enum ValueStore
{
    Double(Vec<f64>),
    Float(Vec<f32>),
    Int32(Vec<i32>),
    Int64(Vec<i64>),
    UInt32(Vec<u32>),
    UInt64(Vec<u64>),
    SInt32(Vec<i32>),
    SInt64(Vec<i64>),
    Fixed32(Vec<u32>),
    Fixed64(Vec<u64>),
    SFixed32(Vec<i32>),
    SFixed64(Vec<i64>),
    Bool(Vec<bool>),
    String(Vec<String>),
    Bytes(Vec<Bytes>),
    Enum(Vec<i64>),
}
