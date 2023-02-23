use num_bigint;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth as pbeth;
use substreams_ethereum::pb::eth::v2::BigInt as pbBigInt;
use substreams_ethereum::pb::eth::v2::BlockHeader;

// ETH burns the base fee. So burn per block = base fee = baseFeePerGas * gasUsed
pub fn get_eip1559_burn(header: &BlockHeader) -> BigInt {
    return pb_bigint_to_bigint(header.base_fee_per_gas.to_owned())
        .mul(BigInt::from(header.gas_used));
}

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
                genesis = genesis.add(get_delta(change.to_owned()));
            }
        }
    }
    genesis
}

// Aux function to calculate the actual change in balance.
pub fn get_delta(change: pbeth::v2::BalanceChange) -> BigInt {
    let old = pb_bigint_to_bigint(change.old_value);
    let new = pb_bigint_to_bigint(change.new_value);
    new.sub(old)
}

// Aux function to convert a protobuf BigInt into a useable BigInt.
fn pb_bigint_to_bigint(bi: Option<pbBigInt>) -> BigInt {
    bi.as_ref()
        .map(|value| num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &value.bytes).into())
        .unwrap_or(BigInt::zero())
}
