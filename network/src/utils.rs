use crate::pb::aggregate_data;
use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::scalar::BigIntSign;

/// Returns the timestamp for the start of the most recent day
pub(crate) fn get_latest_day(timestamp: i64) -> i64 {
    const SECONDS_IN_DAY: i64 = 86400_i64;
    timestamp / SECONDS_IN_DAY
}

/// Returns the timestamp for the start of the most recent hour
pub(crate) fn get_latest_hour(timestamp: i64) -> i64 {
    const SECONDS_IN_HOUR: i64 = 3600_i64;
    timestamp / SECONDS_IN_HOUR
}

impl Into<BigInt> for aggregate_data::BigInt {
    fn into(self) -> BigInt {
        BigInt::from_bytes_le(BigIntSign::Plus, self.bytes.as_slice())
    }
}

impl From<BigInt> for aggregate_data::BigInt {
    fn from(big_int: BigInt) -> Self {
        aggregate_data::BigInt { bytes: big_int.to_bytes_le().1 }
    }
}

pub(crate) fn i64_to_str(num: i64) -> String {
    // TODO: Optimise this to shrink down number of chars needed to represent the i64 value
    num.to_string()
}

// The following had to be done rather than using an Into<T> impl due to both types coming from outside this crate
// TODO: Create a PR to add this into the substreams-rs crate

pub(crate) trait BigIntDeserializeExt {
    fn deserialize(&self) -> BigInt;
}

impl BigIntDeserializeExt for eth::BigInt {
    fn deserialize(&self) -> BigInt {
        BigInt::from_bytes_le(BigIntSign::Plus, self.bytes.as_slice())
    }
}

pub(crate) trait BigIntSerializeExt {
    fn serialize(&self) -> eth::BigInt;
}

impl BigIntSerializeExt for BigInt {
    fn serialize(&self) -> eth::BigInt {
        eth::BigInt { bytes: self.to_bytes_le().1 }
    }
}
