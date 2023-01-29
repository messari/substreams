use crate::common::traits::StoreSetter;
use crate::store_key::StoreKey;
use crate::utils::{get_latest_day, get_latest_hour};

pub(crate) struct Aggregator<'a, T> {
    store: &'a T,
    day_timestamp: Option<i64>,
    hour_timestamp: Option<i64>,
}

impl<'a, T: StoreSetter> Aggregator<'a, T> {
    pub(crate) fn new(store: &'a T, timestamp: Option<i64>) -> Self {
        Aggregator {
            store,
            day_timestamp: get_latest_day(timestamp),
            hour_timestamp: get_latest_hour(timestamp),
        }
    }

    pub(crate) fn set_day_and_hour_timestamp(&mut self, timestamp: i64) {
        self.hour_timestamp = get_latest_hour(Some(timestamp));
        self.day_timestamp = get_latest_day(Some(timestamp));
    }

    pub(crate) fn set_latest_timestamp(&mut self, timestamp: &<T as StoreSetter>::Input) {
        self.store.set_value("latest_timestamp", timestamp);
    }

    pub(crate) fn set_latest_block_number(&mut self, block_number: &<T as StoreSetter>::Input) {
        self.store.set_value("latest_block_number", block_number);
    }

    pub(crate) fn set_pool_active(
        &self,
        key: StoreKey,
        pool: &String,
        value: &<T as StoreSetter>::Input,
    ) {
        self.store.set_value(key.get_unique_pool_key(pool), value)
    }

    pub(crate) fn set_pool_token_balance_field(
        &self,
        key: StoreKey,
        pool: &String,
        token: &String,
        value: &<T as StoreSetter>::Input,
    ) {
        self.store
            .set_value(key.get_pool_token_balance_key(pool, token), &value)
    }

    pub(crate) fn set_global_hourly_unique_field(
        &self,
        key: StoreKey,
        unique_key: &String,
        value: &<T as StoreSetter>::Input,
    ) {
        self.store.set_value(
            key.get_pool_specific_hourly_key(&self.hour_timestamp.unwrap(), unique_key),
            value,
        )
    }

    pub(crate) fn set_global_daily_unique_field(
        &self,
        key: StoreKey,
        unique_key: &String,
        value: &<T as StoreSetter>::Input,
    ) {
        self.store.set_value(
            key.get_pool_specific_daily_key(&self.day_timestamp.unwrap(), unique_key),
            value,
        )
    }

    pub(crate) fn set_global_daily_and_hourly_unique_field(
        &self,
        key: StoreKey,
        unique_key: &String,
        value: &<T as StoreSetter>::Input,
    ) {
        self.set_global_hourly_unique_field(key.clone(), unique_key, value.clone());
        self.set_global_daily_unique_field(key, unique_key, value);
    }

    pub(crate) fn set_global_cumulative_unique_field(
        &self,
        key: StoreKey,
        unique_key: &String,
        value: &<T as StoreSetter>::Input,
    ) {
        self.store
            .set_value(key.get_pool_specific_cumulative_key(unique_key), value)
    }

    pub(crate) fn add_protocol_specific_hourly_field(
        &self,
        key: StoreKey,
        value: &<T as StoreSetter>::Input,
    ) {
        self.store.add_value(
            key.get_protocol_specific_hourly_key(&self.hour_timestamp.unwrap()),
            &value,
        )
    }

    pub(crate) fn add_protocol_specific_daily_field(
        &self,
        key: StoreKey,
        value: &<T as StoreSetter>::Input,
    ) {
        self.store.add_value(
            key.get_protocol_specific_daily_key(&self.day_timestamp.unwrap()),
            &value,
        )
    }

    pub(crate) fn add_protocol_specific_daily_and_hourly_field(
        &self,
        key: StoreKey,
        value: &<T as StoreSetter>::Input,
    ) {
        self.add_protocol_specific_daily_field(key.clone(), value.clone());
        self.add_protocol_specific_hourly_field(key, value);
    }

    pub(crate) fn add_protocol_specific_cumulative_field(
        &self,
        key: StoreKey,
        value: &<T as StoreSetter>::Input,
    ) {
        self.store
            .add_value(key.get_protocol_specific_cumulative_key(), &value)
    }

    pub(crate) fn add_pool_specific_hourly_field(
        &self,
        key: StoreKey,
        pool: &String,
        value: &<T as StoreSetter>::Input,
    ) {
        self.store.add_value(
            key.get_pool_specific_hourly_key(&self.hour_timestamp.unwrap(), pool),
            &value,
        )
    }

    pub(crate) fn add_pool_specific_daily_field(
        &self,
        key: StoreKey,
        pool: &String,
        value: &<T as StoreSetter>::Input,
    ) {
        self.store.add_value(
            key.get_pool_specific_daily_key(&self.day_timestamp.unwrap(), pool),
            &value,
        )
    }

    pub(crate) fn add_pool_specific_daily_and_hourly_field(
        &self,
        key: StoreKey,
        pool: &String,
        value: &<T as StoreSetter>::Input,
    ) {
        self.add_pool_specific_daily_field(key.clone(), pool, value.clone());
        self.add_pool_specific_hourly_field(key, pool, value);
    }

    pub(crate) fn add_pool_specific_cumulative_field(
        &self,
        key: StoreKey,
        pool: &String,
        value: &<T as StoreSetter>::Input,
    ) {
        self.store
            .add_value(key.get_pool_specific_cumulative_key(pool), &value)
    }
}
