use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{DeltaBigInt, Deltas, StoreGetBigDecimal, StoreSetBigDecimal};
use substreams::store::{StoreGet, StoreGetProto, StoreSet};
use substreams::store::{StoreGetBigInt, StoreNew};

use crate::pb::uniswap::v2::Pool;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_pool_tvl(
    pool_store: StoreGetProto<Pool>,
    pool_native_tvl_store: StoreGetBigInt,
    pool_native_tvl_delta: Deltas<DeltaBigInt>,
    usd_price_store: StoreGetBigDecimal,
    output: StoreSetBigDecimal,
) {
    for pool_native_tvl in pool_native_tvl_delta.deltas {
        if let Some((pool_address, token_address)) =
            StoreKey::InputTokenBalance.get_pool_and_token_from_key(&pool_native_tvl.key)
        {
            let pool = pool_store.must_get_last(StoreKey::Pool.get_unique_pool_key(&pool_address));

            if pool.token0_address() != &token_address {
                continue;
            }

            let token0_native_tvl = pool_native_tvl_store
                .get_last(
                    StoreKey::InputTokenBalance
                        .get_pool_token_balance_key(&pool_address, pool.token0_address()),
                )
                .unwrap_or(BigInt::zero());
            let token0_usd_price = usd_price_store
                .get_last(StoreKey::TokenPrice.get_unique_token_key(&pool.token0_address()))
                .unwrap_or(BigDecimal::zero());

            let token1_native_tvl = pool_native_tvl_store
                .get_last(
                    StoreKey::InputTokenBalance
                        .get_pool_token_balance_key(&pool_address, pool.token1_address()),
                )
                .unwrap_or(BigInt::zero());
            let token1_usd_price = usd_price_store
                .get_last(StoreKey::TokenPrice.get_unique_token_key(&pool.token1_address()))
                .unwrap_or(BigDecimal::zero());

            let pool_tvl = (token0_native_tvl.to_decimal(pool.token0_decimals())
                * token0_usd_price)
                + (token1_native_tvl.to_decimal(pool.token1_decimals()) * token1_usd_price);

            output.set(
                0,
                StoreKey::PoolTVL.get_unique_pool_key(&pool_address),
                &pool_tvl,
            );
        }
    }
}
