use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{StoreGet, StoreGetBigDecimal, StoreGetBigInt};

use crate::store_key::StoreKey;
use crate::utils;

pub(crate) struct StoreRetriever<'a, T> {
    store: &'a T,
    day_timestamp: Option<i64>,
    hour_timestamp: Option<i64>,
}

impl<'a> StoreRetriever<'a, StoreGetBigInt> {
    pub(crate) fn new(store: &'a StoreGetBigInt, timestamp: Option<i64>) -> Self {
        StoreRetriever {
            store,
            day_timestamp: utils::get_latest_day(timestamp),
            hour_timestamp: utils::get_latest_hour(timestamp),
        }
    }

    pub(crate) fn set_day_and_hour_timestamp(&mut self, timestamp: Option<BigInt>) {
        self.day_timestamp =
            utils::get_latest_day(Some(timestamp.clone().unwrap().to_u64() as i64));
        self.hour_timestamp = utils::get_latest_hour(Some(timestamp.unwrap().to_u64() as i64));
    }

    pub(crate) fn get_day_timestamp(&mut self) -> i64 {
        self.day_timestamp.unwrap()
    }

    pub(crate) fn get_hour_timestamp(&mut self) -> i64 {
        self.hour_timestamp.unwrap()
    }

    pub(crate) fn get_pool_non_static_field(&self, key: StoreKey, pool_address: &String) -> BigInt {
        self.store
            .get_last(key.get_unique_pool_key(pool_address))
            .unwrap_or(BigInt::zero())
    }

    pub(crate) fn get_cumulative_value(&self, key: StoreKey) -> BigInt {
        self.store
            .get_last(key.get_cumulative_stats_key())
            .unwrap_or(BigInt::zero())
    }

    pub(crate) fn get_hourly_stats_value(&self, key: StoreKey) -> BigInt {
        self.store
            .get_last(key.get_hourly_stats_key(&self.hour_timestamp.unwrap().to_string()))
            .unwrap_or(BigInt::zero())
    }

    pub(crate) fn get_daily_stats_value(&self, key: StoreKey) -> BigInt {
        self.store
            .get_last(key.get_daily_stats_key(&self.day_timestamp.unwrap().to_string()))
            .unwrap_or(BigInt::zero())
    }
}

impl<'a> StoreRetriever<'a, StoreGetBigDecimal> {
    pub(crate) fn new(store: &'a StoreGetBigDecimal, timestamp: Option<i64>) -> Self {
        StoreRetriever {
            store,
            day_timestamp: utils::get_latest_day(timestamp),
            hour_timestamp: utils::get_latest_hour(timestamp),
        }
    }

    pub(crate) fn set_day_and_hour_timestamp(&mut self, timestamp: Option<BigInt>) {
        self.day_timestamp =
            utils::get_latest_day(Some(timestamp.clone().unwrap().to_u64() as i64));
        self.hour_timestamp = utils::get_latest_hour(Some(timestamp.unwrap().to_u64() as i64));
    }

    pub(crate) fn get_day_timestamp(&mut self) -> i64 {
        self.day_timestamp.unwrap()
    }

    pub(crate) fn get_cumulative_pool_value(&self, key: StoreKey, pool: &String) -> BigDecimal {
        self.store
            .get_last(key.get_cumulative_field_key(pool))
            .unwrap_or(BigDecimal::zero())
    }

    pub(crate) fn get_hourly_pool_field_value(&self, key: StoreKey, pool: &String) -> BigDecimal {
        self.store
            .get_last(key.get_hourly_field_key(&self.hour_timestamp.unwrap().to_string(), pool))
            .unwrap_or(BigDecimal::zero())
    }

    pub(crate) fn get_daily_pool_field_value(&self, key: StoreKey, pool: &String) -> BigDecimal {
        self.store
            .get_last(key.get_daily_field_key(&self.day_timestamp.unwrap().to_string(), pool))
            .unwrap_or(BigDecimal::zero())
    }

    pub(crate) fn get_cumulative_protocol_value(&self, key: StoreKey) -> BigDecimal {
        self.store
            .get_last(key.get_cumulative_stats_key())
            .unwrap_or(BigDecimal::zero())
    }

    pub(crate) fn get_daily_protocol_field_value(&self, key: StoreKey) -> BigDecimal {
        self.store
            .get_last(key.get_daily_stats_key(&self.day_timestamp.unwrap().to_string()))
            .unwrap_or(BigDecimal::zero())
    }
}
