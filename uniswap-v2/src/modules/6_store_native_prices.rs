use substreams::store::{DeltaBigInt, Deltas, StoreSet};
use substreams::store::{StoreGet, StoreGetBigInt, StoreGetProto, StoreNew, StoreSetBigDecimal};

use crate::pb::uniswap::v2::Pool;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_native_prices(
    pool_store: StoreGetProto<Pool>,
    balances_store: StoreGetBigInt,
    balances_deltas: Deltas<DeltaBigInt>,
    output_store: StoreSetBigDecimal,
) {
    for delta in balances_deltas.deltas {
        if let Some(pool_address) = StoreKey::TotalBalance.get_pool(&delta.key) {
            let ordinal = delta.ordinal;
            let pool = pool_store.must_get_last(StoreKey::Pool.get_unique_pool_key(&pool_address));

            let token0_native_tvl = pool
                .token0_balance(ordinal, &balances_store)
                .to_decimal(pool.token0_decimals());
            let token1_native_tvl = pool
                .token1_balance(ordinal, &balances_store)
                .to_decimal(pool.token1_decimals());

            let token0_price = token1_native_tvl.clone() / token0_native_tvl.clone();
            let token1_price = token0_native_tvl.clone() / token1_native_tvl.clone();

            output_store.set(
                ordinal,
                StoreKey::TokenPrice
                    .get_unique_pair_key(&pool.token0_address(), &pool.token1_address()),
                &token0_price,
            );
            output_store.set(
                ordinal,
                StoreKey::TokenBalance
                    .get_unique_pair_key(&pool.token0_address(), &pool.token1_address()),
                &token1_native_tvl,
            );

            output_store.set(
                ordinal,
                StoreKey::TokenPrice
                    .get_unique_pair_key(&pool.token1_address(), &pool.token0_address()),
                &token1_price,
            );
            output_store.set(
                ordinal,
                StoreKey::TokenBalance
                    .get_unique_pair_key(&pool.token1_address(), &pool.token0_address()),
                &token0_native_tvl,
            );
        }
    }
}
