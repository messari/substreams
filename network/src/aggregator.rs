use substreams::scalar::BigInt;
use substreams::store;
use substreams::store::StoreAdd;
use substreams_ethereum::scalar::BigIntSign;

use crate::store_key::StoreKey;
use crate::utils::{BigIntSerializeExt, i64_to_str};

pub(crate) struct Aggregator<'a> {
    store: &'a mut store::StoreAddBigInt,
    day_timestamp: String,
    hour_timestamp: String
}

impl Aggregator {
    pub(crate) fn new(store: &mut store::StoreAddBigInt, day_timestamp: i64, hour_timestamp: i64) -> Self {
        Aggregator {
            store,
            day_timestamp: i64_to_str(day_timestamp),
            hour_timestamp: i64_to_str(hour_timestamp)
        }
    }

    pub(crate) fn store_total_sum_contribution(&mut self, key: StoreKey, value: &BigInt) {
        self.store.add(0, key.get_total_sum_key(), value);
    }

    pub(crate) fn store_total_stats_contribution(&mut self, key: StoreKey, value: &BigInt) {
        self.store.add(0, key.get_total_sum_key(), value.clone());
        self.store.add(0, key.get_total_sum_squares_key(), value.pow(2));
    }

    pub(crate) fn store_day_and_hour_stats_contributions(&mut self, key: StoreKey, value: &BigInt) {
        let value_squared = value.clone().pow(2);

        self.store.add(0, key.get_day_sum_key(self.day_timestamp.as_ref().unwrap()), value.clone());
        self.store.add(0, key.get_day_sum_squares_key(self.day_timestamp.as_ref().unwrap()), value_squared.clone());
        self.store.add(0, key.get_hour_sum_key(self.hour_timestamp.as_ref().unwrap()), value);
        self.store.add(0, key.get_hour_sum_squares_key(self.hour_timestamp.as_ref().unwrap()), value_squared);
    }

    pub(crate) fn store_day_and_hour_sum_contributions(&mut self, key: StoreKey, value: &BigInt) {
        self.store.add(0, key.get_day_sum_key(self.day_timestamp.as_ref().unwrap()), value.clone());
        self.store.add(0, key.get_hour_sum_key(self.hour_timestamp.as_ref().unwrap()), value);
    }

    pub(crate) fn store_day_sum_contribution(&mut self, key: StoreKey, value: &BigInt) {
        self.store.add(0, key.get_day_sum_key(self.day_timestamp.as_ref().unwrap()), value.serialize());
    }

    pub(crate) fn store_hour_sum_contribution(&mut self, key: StoreKey, value: &BigInt) {
        self.store.add(0, key.get_hour_sum_key(self.hour_timestamp.as_ref().unwrap()), value.serialize());
    }
}