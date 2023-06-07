use substreams::scalar;

use crate::pb::store::v1 as store;

fn create_store_operation<K, F>(key: K, f: F) -> store::StoreOperation
where
    K: AsRef<str>,
    F: FnOnce(String) -> store::store_operation::Type,
{
    store::StoreOperation {
        r#type: Some(f(key.as_ref().to_string())),
    }
}

pub fn add_int64<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: i64
) -> store::StoreOperation {
    create_store_operation(
        key, 
        |key| {
            store::store_operation::Type::AddInt64(store::AddInt64 { ordinal, key, value })
        }
    )
}

pub fn set_int64<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: i64
) -> store::StoreOperation {
    create_store_operation(
        key, 
        |key| {
            store::store_operation::Type::SetInt64(store::SetInt64 { ordinal, key, value })
        }
    )
}

pub fn add_bigint<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: scalar::BigInt
) -> store::StoreOperation {
    create_store_operation(
        key, 
        |key| {
            store::store_operation::Type::AddBigInt(store::AddBigInt { ordinal, key, value: Some(value.into()) })
        }
    )
}

pub fn set_bigint<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: scalar::BigInt
) -> store::StoreOperation {
    create_store_operation(
        key, 
        |key| {
            store::store_operation::Type::SetBigInt(store::SetBigInt { ordinal, key, value: Some(value.into()) })
        }
    )
}

pub fn add_bigdecimal<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: scalar::BigDecimal
) -> store::StoreOperation {
    create_store_operation(
        key,
        |key| {
            store::store_operation::Type::AddBigDecimal(store::AddBigDecimal { ordinal, key, value: Some(value.into()) })
        }
    )
}

pub fn set_bigdecimal<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: scalar::BigDecimal
) -> store::StoreOperation {
    create_store_operation(
        key,
        |key| {
            store::store_operation::Type::SetBigDecimal(store::SetBigDecimal { ordinal, key, value: Some(value.into()) })
        }
    )
}

pub fn set_bytes<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: Vec<u8>
) -> store::StoreOperation {
    create_store_operation(
        key,
        |key| {
            store::store_operation::Type::SetBytes(store::SetBytes { ordinal, key, value: value })
        }
    )
}

pub fn append_bytes<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: Vec<u8>
) -> store::StoreOperation {
    create_store_operation(
        key,
        |key| {
            store::store_operation::Type::AppendBytes(store::AppendBytes { ordinal, key, value: value })
        }
    )
}

pub fn set_string<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: String
) -> store::StoreOperation {
    create_store_operation(
        key,
        |key| {
            store::store_operation::Type::SetString(store::SetString { ordinal, key, value: value.to_string() })
        }
    )
}

pub fn append_string<K: AsRef<str>>(
    ordinal: u64, 
    key: K, 
    value: String
) -> store::StoreOperation {
    create_store_operation(
        key,
        |key| {
            store::store_operation::Type::AppendString(store::AppendString { ordinal, key, value: value.to_string() })
        }
    )
}
