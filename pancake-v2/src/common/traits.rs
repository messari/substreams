use ethabi::ethereum_types::Address;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{StoreAdd, StoreAddBigDecimal, StoreAddBigInt};
use substreams::store::{StoreDelete, StoreGet, StoreGetProto};
use substreams_helper::common::HasAddresser;
use substreams_helper::hex::Hexable;

use crate::pb::pancake::v2::Pool;
use crate::store_key::StoreKey;

pub struct PoolAddresser<'a> {
    pub store: &'a StoreGetProto<Pool>,
}

impl<'a> PoolAddresser<'a> {
    fn has_address(&self, key: Address) -> bool {
        let pool = self
            .store
            .get_last(StoreKey::Pool.get_unique_pool_key(&key.to_hex()));

        pool.is_some()
    }
}

impl<'a> HasAddresser for PoolAddresser<'a> {
    fn has_address(&self, key: Address) -> bool {
        return self.has_address(key);
    }
}

pub trait StoreAddSnapshot<V> {
    fn add_snapshot<K: AsRef<str>>(&self, ord: u64, id: i64, k: StoreKey, keys: Vec<K>, value: V);
    fn add_protocol_snapshot(&self, ord: u64, id: i64, k: StoreKey, value: V);
}

impl<V: AsRef<BigDecimal>> StoreAddSnapshot<V> for StoreAddBigDecimal {
    fn add_snapshot<K: AsRef<str>>(&self, ord: u64, id: i64, k: StoreKey, keys: Vec<K>, value: V) {
        let keys: Vec<&str> = keys.iter().map(AsRef::as_ref).collect();

        self.delete_prefix(ord as i64, &format!("{}:{}", k.unique_id(), id - 1));
        self.add(ord, k.get_unique_snapshot_key(id, keys), value);
    }

    fn add_protocol_snapshot(&self, ord: u64, id: i64, k: StoreKey, value: V) {
        self.delete_prefix(
            ord as i64,
            &format!("[Protocol]:{}:{}", k.unique_id(), id - 1),
        );
        self.add(ord, k.get_unique_daily_protocol_key(id), value);
    }
}

impl<V: AsRef<BigInt>> StoreAddSnapshot<V> for StoreAddBigInt {
    fn add_snapshot<K: AsRef<str>>(&self, ord: u64, id: i64, k: StoreKey, keys: Vec<K>, value: V) {
        let keys: Vec<&str> = keys.iter().map(AsRef::as_ref).collect();

        self.delete_prefix(ord as i64, &format!("{}:{}", k.unique_id(), id - 1));
        self.add(ord, k.get_unique_snapshot_key(id, keys), value);
    }

    fn add_protocol_snapshot(&self, ord: u64, id: i64, k: StoreKey, value: V) {
        self.delete_prefix(
            ord as i64,
            &format!("[Protocol]:{}:{}", k.unique_id(), id - 1),
        );
        self.add(ord, k.get_unique_daily_protocol_key(id), value);
    }
}
