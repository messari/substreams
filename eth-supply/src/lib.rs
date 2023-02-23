#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;
#[rustfmt::skip]
pub mod helpers;

use std::ops::Add;
use std::ops::Sub;
use std::str::FromStr;

use pb::eth_supply::v1 as eth_supply;
use pb::eth_supply::v1::EthSupply;

use substreams::scalar::BigInt;
use substreams::store;
use substreams::store::StoreAdd;
use substreams::store::StoreAddBigInt;
use substreams::store::StoreGet;
use substreams::store::StoreGetBigInt;
use substreams::store::StoreNew;
use substreams_ethereum::pb::eth as pbeth;

use helpers::{get_delta, get_eip1559_burn, get_genesis_value};

#[substreams::handlers::map]
fn map_supply_delta(
    block: pbeth::v2::Block,
) -> Result<eth_supply::EthSupply, substreams::errors::Error> {
    let mut block_rewards: BigInt = BigInt::from(0);
    let mut uncle_rewards: BigInt = BigInt::from(0);
    let genesis: BigInt = get_genesis_value(&block);

    for change in block.balance_changes {
        match pbeth::v2::balance_change::Reason::from_i32(change.reason).unwrap_or_default() {
            pbeth::v2::balance_change::Reason::RewardMineUncle => {
                uncle_rewards = uncle_rewards.add(get_delta(change));
            }
            pbeth::v2::balance_change::Reason::RewardMineBlock => {
                block_rewards = block_rewards.add(get_delta(change));
            }
            _ => {}
        }
    }

    let burned = get_eip1559_burn(&block.header.unwrap());
    let deltas = EthSupply {
        genesis: genesis.to_string(),
        block_rewards: block_rewards.to_string(),
        uncle_rewards: uncle_rewards.to_string(),
        burned: burned.to_string(),
        total: "0".to_string(),
    };
    Ok(deltas)
}

const GENESIS_BALANCE_STORE_KEY: &str = "genesis";
const MINT_BALANCE_STORE_KEY: &str = "mint";
const UNCLE_BALANCE_STORE_KEY: &str = "uncles";
const BURNED_BALANCE_STORE_KEY: &str = "burned";

#[substreams::handlers::store]
fn store_supply(deltas: eth_supply::EthSupply, output: store::StoreAddBigInt) {
    output.add(
        0,
        GENESIS_BALANCE_STORE_KEY,
        BigInt::from_str(deltas.genesis.as_str()).unwrap(),
    );
    output.add(
        0,
        MINT_BALANCE_STORE_KEY,
        BigInt::from_str(deltas.block_rewards.as_str()).unwrap(),
    );
    output.add(
        0,
        UNCLE_BALANCE_STORE_KEY,
        BigInt::from_str(deltas.uncle_rewards.as_str()).unwrap(),
    );
    output.add(
        0,
        BURNED_BALANCE_STORE_KEY,
        BigInt::from_str(deltas.burned.as_str()).unwrap(),
    );
}

#[substreams::handlers::map]
fn map_output(
    store_supply: store::StoreGetBigInt,
) -> Result<eth_supply::EthSupply, substreams::errors::Error> {
    let genesis: BigInt = store_supply.get_last(GENESIS_BALANCE_STORE_KEY).unwrap();
    let mint: BigInt = store_supply.get_last(MINT_BALANCE_STORE_KEY).unwrap();
    let uncles: BigInt = store_supply.get_last(UNCLE_BALANCE_STORE_KEY).unwrap();
    let burned: BigInt = store_supply.get_last(BURNED_BALANCE_STORE_KEY).unwrap();

    let res = EthSupply {
        genesis: genesis.to_string(),
        block_rewards: mint.to_string(),
        uncle_rewards: uncles.to_string(),
        burned: burned.to_string(),
        total: genesis.add(mint).add(uncles).sub(burned).to_string(),
    };
    Ok(res)
}
