use strum_macros::{EnumString, Display};

#[derive(EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum StoreKey {
    DataSource,

    LiquidityPoolInputTokenBalance,
    LiquidityPoolCumulativeVolumeTokenAmounts,

    LiquidityPoolActiveLiquidity,
    LiquidityPoolTotalLiquidity,

    LiquidityPoolCumulativeSwapCount,
    LiquidityPoolCumulativeDepositCount,
    LiquidityPoolCumulativeWithdrawCount,
}

pub fn get_store_key(store_key: StoreKey, entity_id: &str, index: i32) -> String {
    format!("{}:{}", store_key, entity_id)
}
