use substreams::scalar::{BigDecimal, BigInt};

// pub fn get_day_id(timestamp: i64) -> BigInt {
//     const SECONDS_IN_DAY: i64 = 86400_i64;
//     BigInt::from(timestamp / SECONDS_IN_DAY)
// }

// pub fn get_hour_id(timestamp: i64) -> BigInt {
//     const SECONDS_IN_HOUR: i64 = 3600_i64;
//     BigInt::from(timestamp / SECONDS_IN_HOUR)
// }

// pub fn delta_value(delta: &store::DeltaBigDecimal) -> BigDecimal {
//     let old_value = delta.old_value.clone();
//     let new_value = delta.new_value.clone();

//     return new_value.clone().sub(old_value);
// }

pub fn abs_bigint(value: &BigInt) -> BigInt {
    if value.lt(&BigInt::from(0)) {
        return value.neg().clone();
    }
    value.clone()
}

pub fn bigint_to_bigdecimal(value: &BigInt) -> BigDecimal {
    // let value = BigDecimal::from(value.to_u64());
    value.to_decimal(0)
}

pub fn bigdecimal_to_bigint(value: &BigDecimal) -> BigInt {
    let str_value = value.to_string();
    let vec_string: Vec<&str> = str_value.split('.').collect();
    BigInt::try_from(vec_string[0].to_string()).unwrap()
}
