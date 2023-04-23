use substreams::scalar::BigDecimal;
use substreams::store::{DeltaBigInt, Deltas, StoreGetProto};
use substreams::store::{StoreGet, StoreGetBigInt, StoreNew, StoreSetBigDecimal};

use crate::common::traits::StoreSetter;
use crate::pb::uniswap::v2::Pool;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_pool_tvl(
    pool_store: StoreGetProto<Pool>,
    balances_store: StoreGetBigInt,
    balances_deltas: Deltas<DeltaBigInt>,
    output_store: StoreSetBigDecimal,
) {
    for delta in balances_deltas.deltas {
        if let Some(pool_address) = StoreKey::TotalBalance.get_pool(&delta.key) {
            let pool = pool_store.must_get_last(StoreKey::Pool.get_unique_pool_key(&pool_address));

            // TODO: Handle prices for input tokens
            let token0_price = BigDecimal::one();
            let token1_price = BigDecimal::one();

            let token0_native_tvl = pool
                .token0_balance(&balances_store)
                .to_decimal(pool.token0_decimals());
            let token1_native_tvl = pool
                .token1_balance(&balances_store)
                .to_decimal(pool.token1_decimals());

            let pool_tvl = (token0_native_tvl * token0_price) + (token1_native_tvl * token1_price);

            output_store.set_value(
                StoreKey::TotalValueLockedUSD.get_unique_pool_key(&pool_address),
                &pool_tvl,
            )
        }
    }
}
