use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreSet};
use substreams::store::{StoreAddBigInt, StoreSetBigInt};

use crate::store_key::StoreKey;
use crate::utils;

pub(crate) struct Aggregator {
    set_store: Option<StoreSetBigInt>,
    add_store: Option<StoreAddBigInt>,
    day_timestamp: i64,
    hour_timestamp: i64,
}

impl Aggregator {
    pub(crate) fn new(
        set_store: Option<StoreSetBigInt>,
        add_store: Option<StoreAddBigInt>,
        timestamp: i64,
    ) -> Self {
        Aggregator {
            set_store,
            add_store,
            day_timestamp: utils::get_latest_day(Some(timestamp)).unwrap(),
            hour_timestamp: utils::get_latest_hour(Some(timestamp)).unwrap(),
        }
    }

    pub(crate) fn set_latest_daily_and_hourly_timestamp(&mut self) {
        if self.set_store.as_ref().is_none() {
            return;
        }

        self.set_store.as_ref().unwrap().set(
            0,
            "latest_hour_timestamp",
            &BigInt::from(self.hour_timestamp),
        );
        self.set_store.as_ref().unwrap().set(
            0,
            "latest_day_timestamp",
            &BigInt::from(self.day_timestamp),
        );
    }

    pub(crate) fn set_daily_and_hourly_active_user(&mut self, user: &String) {
        if self.set_store.as_ref().is_none() {
            return;
        }

        self.set_store.as_ref().unwrap().set(
            0,
            StoreKey::ActiveUser.get_hourly_user_key(user, &self.hour_timestamp.to_string()),
            &BigInt::one(),
        );
        self.set_store.as_ref().unwrap().set(
            0,
            StoreKey::ActiveUser.get_daily_user_key(user, &self.day_timestamp.to_string()),
            &BigInt::one(),
        );
    }

    pub(crate) fn set_cumulative_field(&mut self, key: StoreKey, unique_key: &String) {
        if self.set_store.as_ref().is_none() {
            return;
        }

        self.set_store.as_ref().unwrap().set(
            0,
            key.get_cumulative_field_key(unique_key),
            &BigInt::one(),
        );
    }

    pub(crate) fn add_cumulative_field_stats(&mut self, key: StoreKey) {
        if self.add_store.as_ref().is_none() {
            return;
        }

        self.add_store
            .as_ref()
            .unwrap()
            .add(0, key.get_cumulative_stats_key(), &BigInt::one());
    }

    pub(crate) fn add_daily_field_stats(&mut self, key: StoreKey) {
        if self.add_store.as_ref().is_none() {
            return;
        }

        self.add_store.as_ref().unwrap().add(
            0,
            key.get_daily_stats_key(&self.day_timestamp.to_string()),
            &BigInt::one(),
        );
    }

    pub(crate) fn add_hourly_field_stats(&mut self, key: StoreKey) {
        if self.add_store.as_ref().is_none() {
            return;
        }

        self.add_store.as_ref().unwrap().add(
            0,
            key.get_hourly_stats_key(&self.hour_timestamp.to_string()),
            &BigInt::one(),
        );
    }

    pub(crate) fn add_daily_and_hourly_field_stats(&mut self, key: StoreKey) {
        if self.add_store.as_ref().is_none() {
            return;
        }

        self.add_store.as_ref().unwrap().add(
            0,
            key.get_daily_stats_key(&self.day_timestamp.to_string()),
            &BigInt::one(),
        );

        self.add_store.as_ref().unwrap().add(
            0,
            key.get_hourly_stats_key(&self.hour_timestamp.to_string()),
            &BigInt::one(),
        );
    }
}
