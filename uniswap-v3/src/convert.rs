use crate::pb;
use substreams::scalar::{BigDecimal, BigInt};

impl From<BigInt> for pb::common::v1::BigInt {
    fn from(bi: BigInt) -> Self {
        pb::common::v1::BigInt {
            value: bi.to_string(),
        }
    }
}

impl From<pb::common::v1::BigInt> for BigInt {
    fn from(bi: pb::common::v1::BigInt) -> Self {
        BigInt::try_from(bi.value).unwrap()
    }
}


impl From<u32> for pb::common::v1::BigInt {
    fn from(u32: u32) -> Self {
        pb::common::v1::BigInt {
            value: u32.to_string(),
        }
    }
}

impl From<u64> for pb::common::v1::BigInt {
    fn from(u64: u64) -> Self {
        pb::common::v1::BigInt {
            value: u64.to_string(),
        }
    }
}

impl From<BigDecimal> for pb::common::v1::BigDecimal {
    fn from(bi: BigDecimal) -> Self {
        pb::common::v1::BigDecimal {
            value: bi.to_string(),
        }
    }
}

impl From<pb::common::v1::BigDecimal> for BigDecimal {
    fn from(bi: pb::common::v1::BigDecimal) -> Self {
        BigDecimal::try_from(bi.value).unwrap()
    }
}


impl From<u32> for pb::common::v1::BigDecimal {
    fn from(u32: u32) -> Self {
        pb::common::v1::BigDecimal {
            value: u32.to_string(),
        }
    }
}

impl From<u64> for pb::common::v1::BigDecimal {
    fn from(u64: u64) -> Self {
        pb::common::v1::BigDecimal {
            value: u64.to_string(),
        }
    }
}


use substreams_entity_change::pb::entity::value::Typed;
use substreams_entity_change::pb::entity::{Array, Value};
use crate::tables::ToValue;

impl ToValue for &pb::common::v1::BigDecimal {
    fn to_value(&self) -> Value {
        Value {
            typed: Some(Typed::Bigdecimal(self.value.to_string())),
        }
    }
}

impl ToValue for &Vec<pb::common::v1::BigDecimal> {
    fn to_value(&self) -> Value {
        let mut list: Vec<Value> = vec![];
        for item in self.iter() {
            list.push(Value {
                typed: Some(Typed::Bigdecimal(item.value.to_string())),
            });
        }

        Value {
            typed: Some(Typed::Array(Array { value: list })),
        }
    }
}


impl ToValue for &pb::common::v1::BigInt {
    fn to_value(&self) -> Value {
        Value {
            typed: Some(Typed::Bigint(self.value.to_string())),
        }
    }
}


impl ToValue for &Vec<pb::common::v1::BigInt> {
    fn to_value(&self) -> Value {
        let mut list: Vec<Value> = vec![];
        for item in self.iter() {
            list.push(Value {
                typed: Some(Typed::Bigint(item.value.to_string())),
            });
        }

        Value {
            typed: Some(Typed::Array(Array { value: list })),
        }
    }
}

