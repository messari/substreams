use substreams_entity_change::pb::entity::value::Typed;
use substreams_entity_change::pb::entity::{Array, Value};

use crate::tables::ToValue;
use crate::pb;


impl ToValue for &pb::dex_amm::v3_0_3::BigDecimal {
    fn to_value(&self) -> Value {
        Value {
            typed: Some(Typed::Bigdecimal(self.value.to_string())),
        }
    }
}

impl ToValue for &Vec<pb::dex_amm::v3_0_3::BigDecimal> {
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


impl ToValue for &pb::dex_amm::v3_0_3::BigInt {
    fn to_value(&self) -> Value {
        Value {
            typed: Some(Typed::Bigint(self.value.to_string())),
        }
    }
}


impl ToValue for &Vec<pb::dex_amm::v3_0_3::BigInt> {
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
