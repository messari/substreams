use std::ops::Sub;

use crate::common::constants;
use crate::store_key::StoreKey;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{DeltaBigDecimal, DeltaBigInt, Deltas};
use substreams::store::{StoreGet, StoreGetBigDecimal};

pub fn get_day_id(timestamp: i64) -> i64 {
    const SECONDS_IN_DAY: i64 = 86400_i64;
    timestamp / SECONDS_IN_DAY
}

pub fn get_hour_id(timestamp: i64) -> i64 {
    const SECONDS_IN_HOUR: i64 = 3600_i64;
    timestamp / SECONDS_IN_HOUR
}

pub fn is_pricing_asset(address: &String) -> bool {
    constants::STABLE_COINS.contains(&address.as_str())
        || constants::WHITELIST_TOKENS.contains(&address.as_str())
}

pub fn delta_value(delta: &DeltaBigDecimal) -> BigDecimal {
    let old_value = delta.old_value.clone();
    let new_value = delta.new_value.clone();

    return new_value.sub(old_value);
}

pub fn calculate_revenue(volume: BigDecimal) -> (BigDecimal, BigDecimal) {
    let supply_side_revenue =
        volume.clone() * BigDecimal::from(25_i32) / BigDecimal::from(10000_i32);
    let protocol_side_revenue =
        volume * BigDecimal::from(5_i32) / BigDecimal::from(10000_i32);

    (supply_side_revenue, protocol_side_revenue)
}

pub fn get_token_price(ordinal: u64, store: &StoreGetBigDecimal, address: &String) -> BigDecimal {
    store
        .get_at(ordinal, StoreKey::TokenPrice.get_unique_pool_key(address))
        .unwrap_or(BigDecimal::zero())
}

pub fn get_output_token_amount(
    balance_deltas: &Deltas<DeltaBigInt>,
    pool_address: &String,
) -> BigInt {
    let mut balance_diff = BigInt::zero();

    for delta in balance_deltas.deltas.iter() {
        if delta.key == StoreKey::OutputTokenBalance.get_unique_pool_key(pool_address) {
            balance_diff = delta.new_value.clone() - delta.old_value.clone();
        }
    }

    balance_diff
}
