use substreams::Hex;
use substreams_ethereum::{pb::eth::v2::{self as eth}};
use substreams::scalar;

use crate::pb::common::v1 as common;
use crate::pb::store::v1 as store;

fn create_store_instruction<K, F>(ordinal: u64, key: K, f: F) -> store::StoreInstruction
where
    K: AsRef<str>,
    F: FnOnce(String) -> store::store_instruction::Type,
{
    store::StoreInstruction {
        r#type: Some(f(key.as_ref().to_string())),
    }
}

pub fn add_int_64<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: i64
) -> store::StoreInstruction {
    create_store_instruction(
        ordinal, 
        key, 
        |key| {
            store::store_instruction::Type::AddInt64(store::AddInt64 { ordinal, key, value })
        }
    )
}

pub fn set_int_64<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: i64
) -> store::StoreInstruction {
    create_store_instruction(
        ordinal, 
        key, 
        |key| {
            store::store_instruction::Type::SetInt64(store::SetInt64 { ordinal, key, value })
        }
    )
}

pub fn add_bigint<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: scalar::BigInt
) -> store::StoreInstruction {
    create_store_instruction(
        ordinal, 
        key, 
        |key| {
            store::store_instruction::Type::AddBigInt(store::AddBigInt { ordinal, key, value: Some(value.into()) })
        }
    )
}

pub fn set_bigint<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: scalar::BigInt
) -> store::StoreInstruction {
    create_store_instruction(
        ordinal, 
        key, 
        |key| {
            store::store_instruction::Type::SetBigInt(store::SetBigInt { ordinal, key, value: Some(value.into()) })
        }
    )
}

pub fn add_bigdecimal<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: scalar::BigDecimal
) -> store::StoreInstruction {
    create_store_instruction(
        ordinal, 
        key,
        |key| {
            store::store_instruction::Type::AddBigDecimal(store::AddBigDecimal { ordinal, key, value: Some(value.into()) })
        }
    )
}

pub fn set_bigdecimal<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: scalar::BigDecimal
) -> store::StoreInstruction {
    create_store_instruction(
        ordinal, 
        key,
        |key| {
            store::store_instruction::Type::SetBigDecimal(store::SetBigDecimal { ordinal, key, value: Some(value.into()) })
        }
    )
}

pub fn set_bytes<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: Vec<u8>
) -> store::StoreInstruction {
    create_store_instruction(
        ordinal, 
        key,
        |key| {
            store::store_instruction::Type::SetBytes(store::SetBytes { ordinal, key, value: value })
        }
    )
}

pub fn append_bytes<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: Vec<u8>
) -> store::StoreInstruction {
    create_store_instruction(
        ordinal, 
        key,
        |key| {
            store::store_instruction::Type::AppendBytes(store::AppendBytes { ordinal, key, value: value })
        }
    )
}

pub fn set_string<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: String
) -> store::StoreInstruction {
    create_store_instruction(
        ordinal, 
        key,
        |key| {
            store::store_instruction::Type::SetString(store::SetString { ordinal, key, value: value })
        }
    )
}

pub fn append_string<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: String
) -> store::StoreInstruction {
    create_store_instruction(
        ordinal, 
        key,
        |key| {
            store::store_instruction::Type::AppendString(store::AppendString { ordinal, key, value: value })
        }
    )
}
