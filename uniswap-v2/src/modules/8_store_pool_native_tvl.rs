use substreams::scalar::BigInt;
use substreams::store::StoreNew;
use substreams::store::{StoreGet, StoreGetProto, StoreSetBigInt};
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::abi::pair;
use crate::aggregator::Aggregator;
use crate::pb::uniswap::v2::Pool;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_pool_native_tvl(
    block: eth::Block,
    pool_store: StoreGetProto<Pool>,
    native_tvl_store: StoreSetBigInt,
) {
    let mut aggregator = Aggregator::<StoreSetBigInt>::new(
        &native_tvl_store,
        Some(block.timestamp_seconds() as i64),
    );

    for log in block.logs() {
        if let Some(event) = pair::events::Sync::match_and_decode(log) {
            let pool_address = Hex(log.address()).to_string();
            let amount0: BigInt = event.reserve0.into();
            let amount1: BigInt = event.reserve1.into();

            if let Some(pool) =
                pool_store.get_last(StoreKey::Pool.get_unique_pool_key(&pool_address))
            {
                aggregator.set_pool_active(StoreKey::Pool, &pool_address, &BigInt::one());

                aggregator.set_latest_block_number(&BigInt::from(block.number));
                aggregator.set_latest_timestamp(&BigInt::from(block.timestamp_seconds()));

                log::info!(
                    "Set Token Balance: {}, {}: {}",
                    &pool_address,
                    pool.token0_address(),
                    &amount0
                );

                aggregator.set_pool_token_balance_field(
                    StoreKey::InputTokenBalance,
                    &pool_address,
                    pool.token0_address(),
                    &amount0,
                );

                aggregator.set_pool_token_balance_field(
                    StoreKey::InputTokenBalance,
                    &pool_address,
                    pool.token1_address(),
                    &amount1,
                );
            }
        }
    }
}