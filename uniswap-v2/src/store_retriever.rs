use substreams::scalar::BigInt;
use substreams::store::{StoreGet, StoreGetBigInt};

use crate::store_key::StoreKey;
use crate::utils;

pub(crate) struct StoreRetriever<'a> {
    store: &'a StoreGetBigInt,
    day_timestamp: Option<i64>,
    hour_timestamp: Option<i64>,
}

impl<'a> StoreRetriever<'a> {
    pub(crate) fn new(store: &'a StoreGetBigInt, timestamp: Option<i64>) -> Self {
        StoreRetriever {
            store,
            day_timestamp: utils::get_latest_day(timestamp),
            hour_timestamp: utils::get_latest_hour(timestamp),
        }
    }

    pub(crate) fn day_timestamp_is_not_set(&self) -> bool {
        self.day_timestamp.is_none()
    }

    pub(crate) fn hour_timestamp_is_not_set(&self) -> bool {
        self.hour_timestamp.is_none()
    }

    pub(crate) fn set_day_timestamp(&mut self, day_timestamp: BigInt) {
        self.day_timestamp = Some(day_timestamp.to_u64() as i64);
    }

    pub(crate) fn set_hour_timestamp(&mut self, hour_timestamp: BigInt) {
        self.hour_timestamp = Some(hour_timestamp.to_u64() as i64);
    }

    pub(crate) fn get_cumulative_value(&self, key: StoreKey) -> i64 {
        self.store
            .get_last(key.get_cumulative_stats_key())
            .unwrap_or(BigInt::zero())
            .to_u64() as i64
    }

    pub(crate) fn get_hourly_stats_value(&self, key: StoreKey) -> i64 {
        self.store
            .get_last(key.get_hourly_stats_key(&self.hour_timestamp.unwrap().to_string()))
            .unwrap_or(BigInt::zero())
            .to_u64() as i64
    }

    pub(crate) fn get_daily_stats_value(&self, key: StoreKey) -> i64 {
        self.store
            .get_last(key.get_daily_stats_key(&self.day_timestamp.unwrap().to_string()))
            .unwrap_or(BigInt::zero())
            .to_u64() as i64
    }
}
