use std::ops::Deref;

use substreams::scalar::BigInt;
use substreams::store::{DeltaBigInt, Deltas, StoreAdd, StoreAddBigInt};
use substreams::Hex;
use substreams_ethereum::NULL_ADDRESS;

use crate::store_key::StoreKey;

pub(crate) struct PoolBalanceRetriever<'a> {
    pool: String,
    store: &'a StoreAddBigInt,
}

impl<'a> PoolBalanceRetriever<'a> {
    pub fn new(pool: String, store: &'a StoreAddBigInt) -> Self {
        PoolBalanceRetriever { pool, store }
    }

    pub fn is_null_address(&self, user: &Vec<u8>) -> bool {
        *user == NULL_ADDRESS
    }

    pub fn store_user_balance(&self, user: Vec<u8>, value: &BigInt) {
        if self.is_null_address(&user) {
            return;
        }

        self.store.add(
            0,
            StoreKey::UserBalance.get_user_balance_key(&self.pool, &Hex(&user).to_string()),
            value.deref(),
        )
    }
}

pub fn get_user_balance_diff(
    balance_deltas: &Deltas<DeltaBigInt>,
    pool_address: &String,
    user_address: &String,
) -> Option<u64> {
    let mut balance_diff = 0;

    for delta in balance_deltas.deltas.iter() {
        if delta.key == StoreKey::UserBalance.get_user_balance_key(pool_address, user_address) {
            balance_diff = delta.new_value.to_u64() - delta.old_value.to_u64();
        }
    }

    Some(balance_diff)
}
