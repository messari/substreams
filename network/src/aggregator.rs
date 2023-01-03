use substreams::scalar::BigInt;
use substreams::store;
use substreams::store::StoreAdd;

use crate::store_key::StoreKey;
use crate::utils::i64_to_str;

pub(crate) struct Aggregator {
    store: store::StoreAddBigInt,
    day_timestamp: String,
    hour_timestamp: String,
}

impl Aggregator {
    pub(crate) fn new(store: store::StoreAddBigInt, day_timestamp: i64, hour_timestamp: i64) -> Self {
        Aggregator {
            store,
            day_timestamp: i64_to_str(day_timestamp),
            hour_timestamp: i64_to_str(hour_timestamp),
        }
    }

    pub(crate) fn store_total_sum_contribution(&mut self, key: StoreKey, value: BigInt) {
        self.store.add(0, key.get_total_sum_key(), value);
    }

    pub(crate) fn store_total_stats_contribution(&mut self, key: StoreKey, value: BigInt) {
        self.store.add(0, key.get_total_sum_key(), value.clone());
        self.store.add(0, key.get_total_sum_squares_key(), value.pow(2));
    }

    pub(crate) fn store_day_and_hour_stats_contributions(&mut self, key: StoreKey, value: BigInt) {
        let value_squared = value.clone().pow(2);

        self.store.add(0, key.get_day_sum_key(&self.day_timestamp), value.clone());
        self.store.add(0, key.get_day_sum_squares_key(&self.day_timestamp), value_squared.clone());
        self.store.add(0, key.get_hour_sum_key(&self.hour_timestamp), value);
        self.store.add(0, key.get_hour_sum_squares_key(&self.hour_timestamp), value_squared);
    }

    pub(crate) fn store_day_and_hour_sum_contributions(&mut self, key: StoreKey, value: BigInt) {
        self.store.add(0, key.get_day_sum_key(&self.day_timestamp), value.clone());
        self.store.add(0, key.get_hour_sum_key(&self.hour_timestamp), value);
    }

    pub(crate) fn store_day_sum_contribution(&mut self, key: StoreKey, value: BigInt) {
        self.store.add(0, key.get_day_sum_key(&self.day_timestamp), value);
    }

    pub(crate) fn store_hour_sum_contribution(&mut self, key: StoreKey, value: BigInt) {
        self.store.add(0, key.get_hour_sum_key(&self.hour_timestamp), value);
    }
}
