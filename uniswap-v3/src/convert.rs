use crate::pb;
use substreams::scalar::{BigDecimal, BigInt};

impl From<BigInt> for pb::dex_amm::v3_0_3::BigInt {
    fn from(bi: BigInt) -> Self {
        pb::dex_amm::v3_0_3::BigInt {
            value: bi.to_string(),
        }
    }
}

impl From<pb::dex_amm::v3_0_3::BigInt> for BigInt {
    fn from(bi: pb::dex_amm::v3_0_3::BigInt) -> Self {
        BigInt::try_from(bi.value).unwrap()
    }
}


impl From<u32> for pb::dex_amm::v3_0_3::BigInt {
    fn from(u32: u32) -> Self {
        pb::dex_amm::v3_0_3::BigInt {
            value: u32.to_string(),
        }
    }
}

impl From<u64> for pb::dex_amm::v3_0_3::BigInt {
    fn from(u64: u64) -> Self {
        pb::dex_amm::v3_0_3::BigInt {
            value: u64.to_string(),
        }
    }
}

impl From<BigDecimal> for pb::dex_amm::v3_0_3::BigDecimal {
    fn from(bi: BigDecimal) -> Self {
        pb::dex_amm::v3_0_3::BigDecimal {
            value: bi.to_string(),
        }
    }
}

impl From<pb::dex_amm::v3_0_3::BigDecimal> for BigDecimal {
    fn from(bi: pb::dex_amm::v3_0_3::BigDecimal) -> Self {
        BigDecimal::try_from(bi.value).unwrap()
    }
}


impl From<u32> for pb::dex_amm::v3_0_3::BigDecimal {
    fn from(u32: u32) -> Self {
        pb::dex_amm::v3_0_3::BigDecimal {
            value: u32.to_string(),
        }
    }
}

impl From<u64> for pb::dex_amm::v3_0_3::BigDecimal {
    fn from(u64: u64) -> Self {
        pb::dex_amm::v3_0_3::BigDecimal {
            value: u64.to_string(),
        }
    }
}

