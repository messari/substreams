use std::ops::Add;

use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth as pbeth;

use substreams_helper::convert::BigIntDeserializeExt;
use substreams_helper::math::get_balance_gain;

use crate::pb::eth_supply::v1::BigInt as pbBigInt;

// If passed the genesis block it will calculate all value minted from the crowdsale
// on that block (it doesn't come as a block BalanceChange, so we inspect the calls inside)
// the only transaction in that block.
pub fn get_genesis_value(block: &pbeth::v2::Block) -> BigInt {
    let mut genesis = BigInt::from(0);
    if block.number != 0 {
        return genesis;
    }

    for tx in &block.transaction_traces {
        for call in &tx.calls {
            for change in &call.balance_changes {
                genesis = genesis.add(get_balance_gain(change));
            }
        }
    }
    genesis
}

impl BigIntDeserializeExt for pbBigInt {
    fn deserialize(&self) -> BigInt {
        BigInt::from_signed_bytes_le(self.bytes.as_slice())
    }
}

pub trait BigIntPbSerialize {
    fn serialize(&self) -> pbBigInt;
}

impl BigIntPbSerialize for BigInt {
    fn serialize(&self) -> pbBigInt {
        let bytes = self.to_signed_bytes_le();
        pbBigInt { bytes }
    }
}
