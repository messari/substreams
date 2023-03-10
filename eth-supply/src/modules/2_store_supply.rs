use substreams::store;
use substreams::store::StoreAdd;
use substreams::store::StoreAddBigInt;
use substreams::store::StoreNew;

use substreams_helper::convert::BigIntDeserializeExt;

use crate::constants::*;
use crate::pb::eth_supply::v1::EthSupply;

#[substreams::handlers::store]
fn store_supply(deltas: EthSupply, output: store::StoreAddBigInt) {
    output.add(
        0,
        GENESIS_BALANCE_STORE_KEY,
        deltas.genesis.unwrap_or_default().deserialize(),
    );
    output.add(
        0,
        MINT_BALANCE_STORE_KEY,
        deltas.block_rewards.unwrap_or_default().deserialize(),
    );
    output.add(
        0,
        UNCLE_BALANCE_STORE_KEY,
        deltas.uncle_rewards.unwrap_or_default().deserialize(),
    );
    output.add(
        0,
        BURNED_BALANCE_STORE_KEY,
        deltas.burned.unwrap_or_default().deserialize(),
    );
}
