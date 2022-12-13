use substreams::scalar::BigInt;
use substreams::store;
use substreams::store::{StoreMax, StoreMin};

use crate::store_key::StoreKey;
use crate::utils::{get_latest_day, get_latest_hour, i64_to_str};

pub(crate) struct MinMaxUpdater<'a, T: MinMaxStore> {
    store: &'a mut T,
    day_timestamp: String,
    hour_timestamp: String,
}

impl<'a, T: MinMaxStore> MinMaxUpdater<'a, T> {
    pub(crate) fn new(store: &mut T, timestamp: i64) -> Self {
        MinMaxUpdater {
            store,
            day_timestamp: i64_to_str(get_latest_day(timestamp)),
            hour_timestamp: i64_to_str(get_latest_hour(timestamp)),
        }
    }

    pub(crate) fn update_total_value(&mut self, key: StoreKey, value: &BigInt) {
        self.store.update(key.get_unique_id(), value);
    }

    pub(crate) fn update_hourly_and_daily_values(&mut self, key: StoreKey, value: &BigInt) {
        self.store.update(key.get_unique_day_key(&self.day_timestamp), value);
        self.store.update(key.get_unique_hour_key(&self.hour_timestamp), value);
    }
}

pub(crate) trait MinMaxStore {
    fn update<K: AsRef<str>>(&self, key: K, value: &BigInt);
}

impl MinMaxStore for store::StoreMaxBigInt {
    fn update<K: AsRef<str>>(&self, key: K, value: &BigInt) {
        self.max(0, key, value);
    }
}

impl MinMaxStore for store::StoreMinBigInt {
    fn update<K: AsRef<str>>(&self, key: K, value: &BigInt) {
        self.min(0, key, value);
    }
}
