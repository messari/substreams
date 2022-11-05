use substreams::scalar::{BigDecimal, BigInt};
use std::str::FromStr;

// Convert String to BigInt type
pub fn convert_string_to_bigint(string: &String) -> BigInt {
    BigInt::from_str(string).unwrap()
}