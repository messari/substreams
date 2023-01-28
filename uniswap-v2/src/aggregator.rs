use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{DeltaBigDecimal, StoreAdd, StoreAddBigDecimal, StoreAddBigInt};
use substreams::store::{StoreSet, StoreSetBigDecimal, StoreSetBigInt};

use crate::pb::uniswap::v2::Pool;
use crate::store_key::StoreKey;
use crate::utils;

pub(crate) struct Aggregator<T> {
    store: T,
    day_timestamp: Option<i64>,
    hour_timestamp: Option<i64>,
}

impl Aggregator<StoreSetBigInt> {
    pub(crate) fn new(store: StoreSetBigInt, timestamp: Option<i64>) -> Self {
        Aggregator {
            store,
            day_timestamp: utils::get_latest_day(timestamp),
            hour_timestamp: utils::get_latest_hour(timestamp),
        }
    }

    pub(crate) fn set_day_and_hour_timestamp(&mut self, timestamp: i64) {
        self.day_timestamp = utils::get_latest_day(Some(timestamp));
        self.hour_timestamp = utils::get_latest_hour(Some(timestamp));
    }

    pub(crate) fn set_latest_timestamp(&mut self, timestamp: i64) {
        self.store
            .set(0, "latest_timestamp", &BigInt::from(timestamp));
    }

    pub(crate) fn set_latest_block_number(&mut self, block_number: i64) {
        self.store
            .set(0, "latest_block_number", &BigInt::from(block_number));
    }

    pub(crate) fn set_daily_and_hourly_active_user(&mut self, user: &String) {
        if self.hour_timestamp.is_some() {
            self.store.set(
                0,
                StoreKey::ActiveUser
                    .get_hourly_user_key(user, &self.hour_timestamp.unwrap().to_string()),
                &BigInt::one(),
            );
        }

        if self.day_timestamp.is_some() {
            self.store.set(
                0,
                StoreKey::ActiveUser
                    .get_daily_user_key(user, &self.day_timestamp.unwrap().to_string()),
                &BigInt::one(),
            );
        }
    }

    pub(crate) fn set_cumulative_field(&mut self, key: StoreKey, unique_key: &String) {
        self.store
            .set(0, key.get_cumulative_field_key(unique_key), &BigInt::one());
    }

    pub(crate) fn set_pool_active(&mut self, key: StoreKey, pool_address: &String) {
        self.store
            .set(0, key.get_unique_pool_key(pool_address), &BigInt::one());
    }

    pub(crate) fn set_pool_balance_field(
        &mut self,
        key: StoreKey,
        pool: Pool,
        amount0: &BigInt,
        amount1: &BigInt,
    ) {
        self.store.set(
            0,
            key.get_pool_token_balance_key(&pool.address, pool.token0_address()),
            amount0,
        );

        self.store.set(
            0,
            key.get_pool_token_balance_key(&pool.address, pool.token1_address()),
            amount1,
        );
    }
}

impl Aggregator<StoreAddBigInt> {
    pub(crate) fn new(store: StoreAddBigInt, timestamp: Option<i64>) -> Self {
        Aggregator {
            store,
            day_timestamp: utils::get_latest_day(timestamp),
            hour_timestamp: utils::get_latest_hour(timestamp),
        }
    }

    pub(crate) fn add_cumulative_field_stats(&mut self, key: StoreKey) {
        self.store
            .add(0, key.get_cumulative_stats_key(), &BigInt::one());
    }

    pub(crate) fn add_daily_field_stats(&mut self, key: StoreKey) {
        if self.day_timestamp.is_some() {
            self.store.add(
                0,
                key.get_daily_stats_key(&self.day_timestamp.unwrap().to_string()),
                &BigInt::one(),
            );
        }
    }

    pub(crate) fn add_hourly_field_stats(&mut self, key: StoreKey) {
        if self.hour_timestamp.is_some() {
            self.store.add(
                0,
                key.get_hourly_stats_key(&self.hour_timestamp.unwrap().to_string()),
                &BigInt::one(),
            );
        }
    }

    pub(crate) fn add_daily_and_hourly_field_stats(&mut self, key: StoreKey) {
        if self.day_timestamp.is_some() {
            self.store.add(
                0,
                key.get_daily_stats_key(&self.day_timestamp.unwrap().to_string()),
                &BigInt::one(),
            );
        }

        if self.hour_timestamp.is_some() {
            self.store.add(
                0,
                key.get_hourly_stats_key(&self.hour_timestamp.unwrap().to_string()),
                &BigInt::one(),
            );
        }
    }
}

impl Aggregator<StoreSetBigDecimal> {
    pub(crate) fn new(store: StoreSetBigDecimal, timestamp: Option<i64>) -> Self {
        Aggregator {
            store,
            day_timestamp: utils::get_latest_day(timestamp),
            hour_timestamp: utils::get_latest_hour(timestamp),
        }
    }

    pub(crate) fn set_cumulative_field(
        &mut self,
        key: StoreKey,
        pool: &String,
        value: &BigDecimal,
    ) {
        self.store.set(0, key.get_cumulative_field_key(pool), value)
    }
}

impl Aggregator<StoreAddBigDecimal> {
    pub(crate) fn new(store: StoreAddBigDecimal, timestamp: Option<i64>) -> Self {
        Aggregator {
            store,
            day_timestamp: utils::get_latest_day(timestamp),
            hour_timestamp: utils::get_latest_hour(timestamp),
        }
    }

    pub(crate) fn set_day_and_hour_timestamp(&mut self, timestamp: i64) {
        self.day_timestamp = utils::get_latest_day(Some(timestamp));
        self.hour_timestamp = utils::get_latest_hour(Some(timestamp));
    }

    pub(crate) fn add_cumulative_field(
        &mut self,
        key: StoreKey,
        pool: &String,
        value: &BigDecimal,
    ) {
        self.store.add(0, key.get_cumulative_field_key(pool), value);
    }

    pub(crate) fn add_daily_and_hourly_field(
        &mut self,
        key: StoreKey,
        pool: &String,
        value: &BigDecimal,
    ) {
        if self.day_timestamp.is_some() {
            self.store.add(
                0,
                key.get_daily_field_key(&self.day_timestamp.unwrap().to_string(), pool),
                value,
            );
        }

        if self.hour_timestamp.is_some() {
            self.store.add(
                0,
                key.get_hourly_field_key(&self.hour_timestamp.unwrap().to_string(), pool),
                value,
            );
        }
    }

    pub(crate) fn add_protocol_cumulative_delta(&mut self, key: StoreKey, delta: &DeltaBigDecimal) {
        self.store.add(
            0,
            key.get_cumulative_stats_key(),
            delta.new_value.clone() - delta.old_value.clone(),
        );
    }

    pub(crate) fn add_protocol_daily_delta(&mut self, key: StoreKey, delta: &DeltaBigDecimal) {
        if self.day_timestamp.is_some() {
            self.store.add(
                0,
                key.get_daily_stats_key(&self.day_timestamp.unwrap().to_string()),
                delta.new_value.clone() - delta.old_value.clone(),
            );
        }
    }

    pub(crate) fn add_protocol_hourly_delta(&mut self, key: StoreKey, delta: &DeltaBigDecimal) {
        if self.hour_timestamp.is_some() {
            self.store.add(
                0,
                key.get_hourly_stats_key(&self.day_timestamp.unwrap().to_string()),
                delta.new_value.clone() - delta.old_value.clone(),
            );
        }
    }
}
