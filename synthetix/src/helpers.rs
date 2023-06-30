use std::str::FromStr;

use substreams::{pb::substreams::Clock, scalar::BigInt};

use substreams_ethereum::pb::eth::v2::Block;
use substreams_helper::block::BlockHandler;
use substreams_helper::convert::BigIntDeserializeExt;

use crate::pb::synthetix::v1::BigInt as pbBigInt;
use crate::pb::synthetix::v1::Timestamp as pbTimestamp;

impl BigIntDeserializeExt for pbBigInt {
    fn deserialize(&self) -> BigInt {
        BigInt::from_str(self.val.as_str()).unwrap()
    }
}

pub trait BigIntPbSerialize {
    fn serialize(&self) -> pbBigInt;
}

impl BigIntPbSerialize for BigInt {
    fn serialize(&self) -> pbBigInt {
        pbBigInt {
            val: self.to_string(),
        }
    }
}

impl From<BigInt> for pbBigInt {
    fn from(value: BigInt) -> Self {
        value.serialize()
    }
}

impl From<Clock> for pbTimestamp {
    fn from(clock: Clock) -> Self {
        pbTimestamp {
            block_number: clock.number,
            timestamp: clock.timestamp.unwrap().seconds as u64,
        }
    }
}

impl From<&Block> for pbTimestamp {
    fn from(block: &Block) -> Self {
        let bh = BlockHandler::new(block);

        pbTimestamp {
            block_number: bh.block_number(),
            timestamp: bh.timestamp() as u64,
        }
    }
}
