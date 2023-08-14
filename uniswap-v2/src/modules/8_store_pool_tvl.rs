use substreams::store::{DeltaBigInt, Deltas, StoreGetBigDecimal, StoreGetProto, StoreSet};
use substreams::store::{StoreGet, StoreGetBigInt, StoreNew, StoreSetBigDecimal};

use crate::pb::uniswap::v2::Pool;
use crate::store_key::StoreKey;
use crate::utils;

#[substreams::handlers::store]
pub fn store_pool_tvl(
    pool_store: StoreGetProto<Pool>,
    balances_store: StoreGetBigInt,
    balances_deltas: Deltas<DeltaBigInt>,
    prices_store: StoreGetBigDecimal,
    output_store: StoreSetBigDecimal,
) {
    for delta in balances_deltas.deltas {
        if let Some(pool_address) = StoreKey::TotalBalance.get_pool(&delta.key) {
            let ordinal = delta.ordinal;
            let pool = pool_store.must_get_last(StoreKey::Pool.get_unique_pool_key(&pool_address));

            let token0_price =
                utils::get_token_price(ordinal, &prices_store, &pool.token0_address());
            let token1_price =
                utils::get_token_price(ordinal, &prices_store, &pool.token1_address());

            let token0_native_tvl = pool
                .token0_balance(ordinal, &balances_store)
                .to_decimal(pool.token0_decimals());
            let token1_native_tvl = pool
                .token1_balance(ordinal, &balances_store)
                .to_decimal(pool.token1_decimals());

            let pool_tvl = (token0_native_tvl * token0_price) + (token1_native_tvl * token1_price);

            output_store.set(
                ordinal,
                StoreKey::TotalValueLockedUSD.get_unique_pool_key(&pool_address),
                &pool_tvl,
            )
        }
    }
}
