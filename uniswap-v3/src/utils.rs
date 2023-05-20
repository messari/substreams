use std::ops::Sub;

use substreams::hex;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{DeltaBigDecimal};

pub const UNISWAP_V3_FACTORY: &str = "1f98431c8ad98523631ae4a59f267346ea31f984";
pub const UNISWAP_V3_FACTORY_SLICE: [u8; 20] = hex!("1f98431c8ad98523631ae4a59f267346ea31f984");

pub fn get_day_id(timestamp: i64) -> BigInt {
    const SECONDS_IN_DAY: i64 = 86400_i64;
    BigInt::from(timestamp / SECONDS_IN_DAY)
}

pub fn get_hour_id(timestamp: i64) -> BigInt {
    const SECONDS_IN_HOUR: i64 = 3600_i64;
    BigInt::from(timestamp / SECONDS_IN_HOUR)
}

pub fn delta_value(delta: &DeltaBigDecimal) -> BigDecimal {
    let old_value = delta.old_value.clone();
    let new_value = delta.new_value.clone();

    return new_value.clone().sub(old_value);
}

pub fn abs_bigint(value: &BigInt) -> BigInt {
    if value.lt(&BigInt::from(0)) {
        return value.neg().clone();
    }
    value.clone()
}

pub fn bigint_to_bigdecimal(value: &BigInt) -> BigDecimal {
    BigDecimal::from(value.to_u64())
}

pub fn bigdecimal_to_bigint(value: &BigDecimal) -> BigInt {
    let str_value = value.to_string();
    let vec_string: Vec<&str> = str_value.split('.').collect();
    BigInt::try_from(vec_string[0].to_string()).unwrap()
}