use std::ops::Deref;

use substreams::scalar::BigInt;
use substreams::store::{DeltaBigInt, Deltas, StoreAdd, StoreAddBigInt};
use substreams::Hex;
use substreams_ethereum::NULL_ADDRESS;

use crate::store_key::StoreKey;

pub(crate) struct PoolBalanceUpdater<'a> {
    pool: String,
    store: &'a StoreAddBigInt,
}

impl<'a> PoolBalanceUpdater<'a> {
    pub fn new(pool: String, store: &'a StoreAddBigInt) -> Self {
        PoolBalanceUpdater { pool, store }
    }

    pub fn update_pool_token_supply(&self, from: &Vec<u8>, to: &Vec<u8>, value: &BigInt) {
        if *from == NULL_ADDRESS {
            self.store.add(
                0,
                StoreKey::PoolTokenSupply.get_unique_pool_key(&self.pool),
                value.deref(),
            )
        }

        if *to == NULL_ADDRESS {
            self.store.add(
                0,
                StoreKey::PoolTokenSupply.get_unique_pool_key(&self.pool),
                value.neg(),
            )
        }
    }

    pub fn update_user_balance(&self, user: &Vec<u8>, value: &BigInt) {
        if *user == NULL_ADDRESS {
            return;
        }

        self.store.add(
            0,
            StoreKey::UserBalance.get_user_balance_key(&self.pool, &Hex(user).to_string()),
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
