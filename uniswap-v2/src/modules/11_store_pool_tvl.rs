use substreams::store::{DeltaBigInt, Deltas, StoreGetBigDecimal, StoreSetBigDecimal};
use substreams::store::{StoreGet, StoreGetProto};
use substreams::store::{StoreGetBigInt, StoreNew};

use crate::aggregator::Aggregator;
use crate::pb::uniswap::v2::Pool;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_pool_tvl(
    pool_store: StoreGetProto<Pool>,
    native_tvl_store: StoreGetBigInt,
    native_tvl_deltas: Deltas<DeltaBigInt>,
    usd_price_store: StoreGetBigDecimal,
    tvl_store: StoreSetBigDecimal,
) {
    let mut aggregator = Aggregator::<StoreSetBigDecimal>::new(tvl_store, None);

    for native_tvl_delta in native_tvl_deltas.deltas {
        if let Some((pool_address, token_address)) =
            StoreKey::InputTokenBalance.get_pool_and_token_from_key(&native_tvl_delta.key)
        {
            let pool = pool_store.must_get_last(StoreKey::Pool.get_unique_pool_key(&pool_address));

            if pool.token0_address() != &token_address {
                continue;
            }

            let token0_usd_price = pool.token0_price(&usd_price_store);
            let token0_native_tvl = pool
                .token0_balance(&native_tvl_store)
                .to_decimal(pool.token0_decimals());

            let token1_usd_price = pool.token1_price(&usd_price_store);
            let token1_native_tvl = pool
                .token1_balance(&native_tvl_store)
                .to_decimal(pool.token1_decimals());

            let pool_tvl =
                (token0_native_tvl * token0_usd_price) + (token1_native_tvl * token1_usd_price);

            aggregator.set_cumulative_field(StoreKey::PoolTVL, &pool.address, &pool_tvl);
        }
    }
}
