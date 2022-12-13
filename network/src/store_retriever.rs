use substreams::pb::substreams::module::input::Input::Store;
use substreams::scalar::BigInt;
use substreams::store;
use substreams::store::StoreGet;
use substreams::store::StoreSetI64;

use crate::store_key::StoreKey;
use crate::utils::i64_to_str;

pub(crate) struct StoreRetriever<'a> {
    store: &'a store::StoreGetBigInt,
    day_timestamp: Option<String>,
    hour_timestamp: Option<String>,
}

impl<'a> StoreRetriever<'a> {
    pub(crate) fn new(store: &store::StoreGetBigInt, day_timestamp: Option<i64>, hour_timestamp: Option<i64>) -> Self {
        StoreRetriever {
            store,
            day_timestamp: day_timestamp.map(|x| i64_to_str(x)),
            hour_timestamp: hour_timestamp.map(|x| i64_to_str(x)),
        }
    }

    pub(crate) fn day_timestamp_is_not_set(&self) -> bool {
        self.day_timestamp.is_none()
    }

    pub(crate) fn hour_timestamp_is_not_set(&self) -> bool {
        self.hour_timestamp.is_none()
    }

    pub(crate) fn set_day_timestamp(&mut self, day_timestamp: i64) {
        self.day_timestamp = Some(i64_to_str(day_timestamp));
    }

    pub(crate) fn set_hour_timestamp(&mut self, hour_timestamp: i64) {
        self.hour_timestamp = Some(i64_to_str(hour_timestamp));
    }

    pub(crate) fn get_total_sum(&self, key: StoreKey) -> BigInt {
        self.store.get_at(0, key.get_total_sum_key()).unwrap()
    }

    pub(crate) fn get_day_sum(&self, key: StoreKey) -> BigInt {
        self.store.get_at(0, key.get_day_sum_key(self.day_timestamp.as_ref().unwrap())).unwrap()
    }

    pub(crate) fn get_hour_sum(&self, key: StoreKey) -> BigInt {
        self.store.get_at(0, key.get_hour_sum_key(self.hour_timestamp.as_ref().unwrap())).unwrap()
    }

    /// Returns the sum of the variable and the sum of the variable squared across the entire network up to the current block being indexed. Return type -> (sum{x},sum{x^2})
    pub(crate) fn get_total_stats_values(&self, key: StoreKey) -> (BigInt, BigInt) {
        let sum = self.store.get_at(0, key.get_total_sum_key()).unwrap();
        let sum_squares = self.store.get_at(0, key.get_total_sum_squares_key()).unwrap();
        (sum, sum_squares)
    }

    /// Returns the sum of the variable and the sum of the variable squared across all blocks that have already been indexed for the the given day. Return type -> (sum{x},sum{x^2})
    pub(crate) fn get_day_stats_values(&self, key: StoreKey) -> (BigInt, BigInt) {
        let sum = self.store.get_at(0, key.get_day_sum_key(self.day_timestamp.as_ref().unwrap())).unwrap();
        let sum_squares = self.store.get_at(0, key.get_day_sum_squares_key(self.day_timestamp.as_ref().unwrap())).unwrap();
        (sum, sum_squares)
    }

    /// Returns the sum of the variable and the sum of the variable squared across all blocks that have already been indexed for the the given hour. Return type -> (sum{x},sum{x^2})
    pub(crate) fn get_hour_stats_values(&self, key: StoreKey) -> (BigInt, BigInt) {
        let sum = self.store.get_at(0, key.get_hour_sum_key(self.hour_timestamp.as_ref().unwrap())).unwrap();
        let sum_squares = self.store.get_at(0, key.get_hour_sum_squares_key(self.hour_timestamp.as_ref().unwrap())).unwrap();
        (sum, sum_squares)
    }

    pub(crate) fn get_total_min_or_max_value(&self, key: StoreKey) -> BigInt {
        self.store.get_at(0, key.get_unique_id()).unwrap()
    }

    pub(crate) fn get_day_min_or_max_value(&self, key: StoreKey) -> BigInt {
        self.store.get_at(0, key.get_unique_day_key(self.day_timestamp.as_ref().unwrap())).unwrap()
    }

    pub(crate) fn get_hour_min_or_max_value(&self, key: StoreKey) -> BigInt {
        self.store.get_at(0, key.get_unique_hour_key(self.hour_timestamp.as_ref().unwrap())).unwrap()
    }
}
