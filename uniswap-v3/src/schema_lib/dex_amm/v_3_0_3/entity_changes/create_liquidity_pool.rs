use substreams::{Hex};
use substreams::prelude::*;
use substreams::pb::substreams::Clock;
use substreams_entity_change::pb::entity::{EntityChange, entity_change::Operation};

use crate::pb::dex_amm::v3_0_3::{PrunedTransaction, CreateLiquidityPool};
use crate::schema_lib::dex_amm::v_3_0_3::keys;

pub fn create_liquidity_pool_entity_change(
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    create_liquidity_pool: CreateLiquidityPool,
) -> EntityChange {
    let mut liquidity_pool_change: EntityChange =
        EntityChange::new("LiquidityPool", &format!("0x{}", hex::encode(create_liquidity_pool.pool_address)), 0, Operation::Create);
    
    
    liquidity_pool_change
        .change("protocol", create_liquidity_pool.protocol)
        .change("name", keys::get_pool_name("Uniswap V3", &create_liquidity_pool.input_tokens))
        .change("symbol", keys::get_pool_symbol(&create_liquidity_pool.input_tokens))
        .change("inputTokens", create_liquidity_pool.input_tokens)
        .change("fees", create_liquidity_pool.fees)
        .change("isSingleSided", create_liquidity_pool.is_single_sided)
        .change("CreatedBlockNumber", BigInt::from(*block_number))
        .change("CreatedTimestamp", BigInt::from(*timestamp))
        .change("totalValueLockedUSD", BigDecimal::from(0))
        .change("totalLiquidity", BigInt::from(0))
        .change("totalLiquidityUSD", BigDecimal::from(0))
        .change("activeLiquidity", BigInt::from(0))
        .change("activeLiquidityUSD", BigDecimal::from(0))
        .change("uncollectedProtocolSideTokenAmounts", &vec![BigInt::from(0).to_string(); create_liquidity_pool.input_tokens.len()])
        .change("uncollectedProtocolSideValuesUSD", &vec![BigDecimal::from(0).to_string(); create_liquidity_pool.input_tokens.len()])
        .change("uncollectedSupplySideTokenAmounts", &vec![BigInt::from(0).to_string(); create_liquidity_pool.input_tokens.len()])
        .change("uncollectedSupplySideValuesUSD", &vec![BigDecimal::from(0).to_string(); create_liquidity_pool.input_tokens.len()])
        .change("cumulativeSupplySideRevenueUSD", BigDecimal::from(0))
        .change("cumulativeProtocolSideRevenueUSD", BigDecimal::from(0))
        .change("cumulativeTotalRevenueUSD", BigDecimal::from(0))
        .change("cumulativeVolumeTokenAmounts", &vec![BigInt::from(0).to_string(); create_liquidity_pool.input_tokens.len()])
        .change("cumulativeVolumesUSD", &vec![BigDecimal::from(0).to_string(); create_liquidity_pool.input_tokens.len()])
        .change("cumulativeVolumeUSD", &vec![BigDecimal::from(0).to_string(); create_liquidity_pool.input_tokens.len()])
        .change("inputTokenBalances", &vec![BigInt::from(0).to_string(); create_liquidity_pool.input_tokens.len()])
        .change("inputTokenBalancesUSD", &vec![BigDecimal::from(0).to_string(); create_liquidity_pool.input_tokens.len()])
        .change("inputTokenWeights", &vec![BigDecimal::from(0).to_string(); create_liquidity_pool.input_tokens.len()])
        .change("stakedOutputTokenAmount", BigInt::from(0))
        .change("cumulativeDepositCount", 0)
        .change("cumulativeWithdrawCount", 0)
        .change("cumulativeSwapCount", 0)
        .change("positionCount", 0)
        .change("openPositionCount", 0)
        .change("closedPositionCount", 0);


    
    if let Some(tick) = create_liquidity_pool.tick {
        liquidity_pool_change.change("tick", tick);
    }
    if let Some(liquidity_token) = create_liquidity_pool.liquidity_token {
        liquidity_pool_change.change("liquidityToken", liquidity_token);
    }
    if let Some(liquidity_token_type) = create_liquidity_pool.liquidity_token_type {
        liquidity_pool_change.change("liquidityTokenType", liquidity_token_type);
    }

    if create_liquidity_pool.reward_tokens.len() > 0 {
        liquidity_pool_change.change("rewardTokens", create_liquidity_pool.reward_tokens);
        liquidity_pool_change.change("rewardTokenEmissionsAmount", &vec![BigInt::from(0).to_string(); create_liquidity_pool.reward_tokens.len()]);
        liquidity_pool_change.change("rewardTokenEmissionsUSD", &vec![BigDecimal::from(0).to_string(); create_liquidity_pool.reward_tokens.len()]);
    }
    
    liquidity_pool_change
}

// pub fn liquidity_pool_balance_entity_change(
//     pool_address: &Vec<u8>,
//     input_token_balance: &Vec<String>,
// ) -> EntityChange {
//     let mut liquidity_pool_change: EntityChange =
//         EntityChange::new("LiquidityPool", &format!("0x{}", hex::encode(pool_address)), 0, Operation::Create);
    
//     liquidity_pool_change
//         .change("totalValueLockedUSD", BigDecimal::from(0))
//         .change("totalLiquidity", BigInt::from(0))
//         .change("totalLiquidityUSD", BigDecimal::from(0))
//         .change("activeLiquidity", BigInt::from(0))
//         .change("activeLiquidityUSD", BigDecimal::from(0))
//         .change("uncollectedProtocolSideTokenAmounts", &vec![BigInt::from(0); input_tokens.len()])
//         .change("uncollectedProtocolSideValuesUSD", &vec![BigDecimal::from(0); input_tokens.len()])
//         .change("uncollectedSupplySideTokenAmounts", &vec![BigInt::from(0); input_tokens.len()])
//         .change("uncollectedSupplySideValuesUSD", &vec![BigDecimal::from(0); input_tokens.len()])
//         .change("cumulativeSupplySideRevenueUSD", BigDecimal::from(0))
//         .change("cumulativeProtocolSideRevenueUSD", BigDecimal::from(0))
//         .change("cumulativeTotalRevenueUSD", BigDecimal::from(0))
//         .change("cumulativeVolumeTokenAmounts", &vec![BigInt::from(0); input_tokens.len()])
//         .change("cumulativeVolumesUSD", &vec![BigDecimal::from(0); input_tokens.len()])
//         .change("cumulativeVolumeUSD", &vec![BigDecimal::from(0); input_tokens.len()])
//         .change("inputTokenBalances", &vec![BigInt::from(0); input_tokens.len()])
//         .change("inputTokenBalancesUSD", &vec![BigDecimal::from(0); input_tokens.len()])
//         .change("inputTokenWeights", &vec![BigDecimal::from(0); input_tokens.len()])
//         .change("stakedOutputTokenAmount", BigInt::from(0));

//     liquidity_pool_change
// }
