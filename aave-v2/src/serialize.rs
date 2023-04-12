use ethabi::ethereum_types::U256;
use substreams::scalar::BigInt;

use crate::pb::aave_v2::v1::BigInt as pbBigInt;

impl From<BigInt> for pbBigInt {
    fn from(bi: BigInt) -> Self {
        pbBigInt {
            value: bi.to_string(),
        }
    }
}

impl From<U256> for pbBigInt {
    fn from(u: U256) -> Self {
        pbBigInt {
            value: u.to_string(),
        }
    }
}
