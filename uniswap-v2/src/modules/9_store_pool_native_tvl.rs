use substreams::scalar::BigInt;
use substreams::store::StoreNew;
use substreams::store::{StoreGet, StoreGetProto, StoreSet, StoreSetBigInt};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::abi::pair;
use crate::pb::uniswap::v2::Pool;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_pool_native_tvl(
    block: eth::Block,
    pool_store: StoreGetProto<Pool>,
    output: StoreSetBigInt,
) {
    for log in block.logs() {
        if let Some(event) = pair::events::Sync::match_and_decode(log) {
            let pool_address = Hex(log.address()).to_string();
            let amount0: BigInt = event.reserve0.into();
            let amount1: BigInt = event.reserve1.into();

            if let Some(pool) =
                pool_store.get_last(StoreKey::Pool.get_unique_pool_key(&pool_address))
            {
                output.set(
                    log.ordinal(),
                    StoreKey::InputTokenBalance
                        .get_pool_token_balance_key(&pool_address, pool.token0_address()),
                    &amount0,
                );
                output.set(
                    log.ordinal(),
                    StoreKey::InputTokenBalance
                        .get_pool_token_balance_key(&pool_address, pool.token1_address()),
                    &amount1,
                );
            }
        }
    }
}
