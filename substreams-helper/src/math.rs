use bigdecimal::{BigDecimal, One, Zero};
use num_bigint::{BigInt, BigUint};
use pad::PadStr;
use std::ops::{Add, Div, Mul};
use std::str::FromStr;

pub fn safe_div(amount0: &BigDecimal, amount1: &BigDecimal) -> BigDecimal {
    let big_decimal_zero: &BigDecimal = &BigDecimal::zero();
    return if amount1.eq(big_decimal_zero) {
        BigDecimal::zero()
    } else {
        amount0.div(amount1)
    };
}

// converts the string representation (in bytes) of a decimal
pub fn decimal_from_bytes(price_bytes: &Vec<u8>) -> BigDecimal {
    decimal_from_str(std::str::from_utf8(price_bytes.as_slice()).unwrap())
}

pub fn decimal_from_str(price_str: &str) -> BigDecimal {
    return BigDecimal::from_str(price_str).unwrap().with_prec(100);
}

pub fn decimal_from_hex_be_bytes(price_bytes: &Vec<u8>) -> BigDecimal {
    let big_uint_amount = BigUint::from_bytes_be(price_bytes.as_slice());
    return BigDecimal::from_str(big_uint_amount.to_string().as_str())
        .unwrap()
        .with_prec(100);
}

pub fn exponent_to_big_decimal(decimals: u8) -> BigDecimal {
    let mut result = BigDecimal::one();
    let big_decimal_ten: &BigDecimal = &BigDecimal::from(10);
    for _i in 0..decimals {
        result = result.mul(big_decimal_ten);
    }
    return result;
}

pub fn divide_by_decimals(big_float_amount: BigDecimal, decimals: u64) -> BigDecimal {
    let bd = BigDecimal::from_str(
        "1".pad_to_width_with_char((decimals + 1) as usize, '0')
            .as_str(),
    )
    .unwrap()
    .with_prec(100);
    return big_float_amount.div(bd).with_prec(100);
}
