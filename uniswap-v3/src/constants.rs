use substreams::scalar::{BigDecimal, BigInt};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref BIGDECIMAL_ZERO: BigDecimal = BigDecimal::from(0);
    pub static ref BIGDECIMAL_100: BigDecimal = BigDecimal::from(100);

    pub static ref BIGINT_ZERO: BigInt = BigInt::from(0);
}