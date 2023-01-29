use substreams::scalar::BigInt;

use crate::common::traits::StoreGetter;
use crate::store_key::StoreKey;
use crate::utils::{get_latest_day, get_latest_hour};

pub(crate) struct StoreRetriever<'a, T> {
    store: &'a T,
    pub day_timestamp: Option<i64>,
    pub hour_timestamp: Option<i64>,
}

impl<'a, T: StoreGetter> StoreRetriever<'a, T> {
    pub(crate) fn new(store: &'a T, timestamp: Option<i64>) -> Self {
        StoreRetriever {
            store,
            day_timestamp: get_latest_day(timestamp),
            hour_timestamp: get_latest_hour(timestamp),
        }
    }

    pub(crate) fn set_store_retriever_timestamp(&mut self, timestamp: Option<BigInt>) {
        self.hour_timestamp = get_latest_hour(Some(timestamp.clone().unwrap().to_u64() as i64));
        self.day_timestamp = get_latest_day(Some(timestamp.unwrap().to_u64() as i64));
    }

    pub(crate) fn get_pool_specific_unique_field(
        &self,
        key: StoreKey,
        pool: &String,
    ) -> <T as StoreGetter>::Output {
        self.store.get(key.get_unique_pool_key(pool))
    }

    pub(crate) fn get_pool_specific_daily_field(
        &self,
        key: StoreKey,
        pool: &String,
    ) -> <T as StoreGetter>::Output {
        self.store
            .get(key.get_pool_specific_daily_key(&self.day_timestamp.unwrap(), pool))
    }

    pub(crate) fn get_pool_specific_cumulative_field(
        &self,
        key: StoreKey,
        pool: &String,
    ) -> <T as StoreGetter>::Output {
        self.store.get(key.get_pool_specific_cumulative_key(pool))
    }

    pub(crate) fn get_protocol_specific_hourly_field(
        &self,
        key: StoreKey,
    ) -> <T as StoreGetter>::Output {
        self.store
            .get(key.get_protocol_specific_hourly_key(&self.hour_timestamp.unwrap()))
    }

    pub(crate) fn get_protocol_specific_daily_field(
        &self,
        key: StoreKey,
    ) -> <T as StoreGetter>::Output {
        self.store
            .get(key.get_protocol_specific_daily_key(&self.day_timestamp.unwrap()))
    }

    pub(crate) fn get_protocol_specific_cumulative_field(
        &self,
        key: StoreKey,
    ) -> <T as StoreGetter>::Output {
        self.store.get(key.get_protocol_specific_cumulative_key())
    }
}
