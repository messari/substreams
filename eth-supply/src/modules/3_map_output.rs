use std::ops::Add;
use std::ops::Sub;

use substreams::scalar::BigInt;
use substreams::store;
use substreams::store::StoreGet;
use substreams::store::StoreGetBigInt;

use crate::constants::*;
use crate::helpers::BigIntPbSerialize;
use crate::pb::eth_supply::v1::EthSupply;

#[substreams::handlers::map]
fn map_output(
    supply_delta: EthSupply,
    store_supply: store::StoreGetBigInt,
) -> Result<EthSupply, substreams::errors::Error> {
    let genesis: BigInt = store_supply.get_last(GENESIS_BALANCE_STORE_KEY).unwrap();
    let mint: BigInt = store_supply.get_last(MINT_BALANCE_STORE_KEY).unwrap();
    let uncles: BigInt = store_supply.get_last(UNCLE_BALANCE_STORE_KEY).unwrap();
    let burned: BigInt = store_supply.get_last(BURNED_BALANCE_STORE_KEY).unwrap();

    let res = EthSupply {
        block_hash: supply_delta.block_hash,
        block_number: supply_delta.block_number,
        genesis: genesis.serialize().into(),
        block_rewards: mint.serialize().into(),
        uncle_rewards: uncles.serialize().into(),
        burned: burned.serialize().into(),
        total: genesis.add(mint).add(uncles).sub(burned).serialize().into(),
    };
    Ok(res)
}
