use substreams::scalar::{BigInt};

use crate::pb::dex_amm::v3_0_3::{PrunedTransaction, CreateLiquidityPool};
use crate::schema_lib::dex_amm::v_3_0_3::keys;
use crate::tables::{Tables, Row};
use crate::constants;

pub fn create_liquidity_pool_entity_change(
    tables: &mut Tables,
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    create_liquidity_pool: &CreateLiquidityPool,
) {
    let row: &mut Row = tables.create_row("LiquidityPool", &format!("0x{}", hex::encode(&create_liquidity_pool.pool_address)));
    row
        .set("protocol", &create_liquidity_pool.protocol)
        .set("name", keys::get_pool_name("Uniswap V3", &create_liquidity_pool.input_tokens))
        .set("symbol", keys::get_pool_symbol(&create_liquidity_pool.input_tokens))
        .set("inputTokens", &create_liquidity_pool.input_tokens)
        .set("fees", &create_liquidity_pool.fees)
        .set("isSingleSided", create_liquidity_pool.is_single_sided)
        .set("createdBlockNumber", BigInt::from(*block_number))
        .set("createdTimestamp", BigInt::from(*timestamp))
        .set("totalValueLockedUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("totalLiquidity", constants::BIGINT_ZERO.clone())
        .set("totalLiquidityUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("activeLiquidity", constants::BIGINT_ZERO.clone())
        .set("activeLiquidityUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("uncollectedProtocolSideTokenAmounts", &vec![constants::BIGINT_ZERO.clone(); create_liquidity_pool.input_tokens.len()])
        .set("uncollectedProtocolSideValuesUSD", &vec![constants::BIGDECIMAL_ZERO.clone(); create_liquidity_pool.input_tokens.len()])
        .set("uncollectedSupplySideTokenAmounts", &vec![constants::BIGINT_ZERO.clone(); create_liquidity_pool.input_tokens.len()])
        .set("uncollectedSupplySideValuesUSD", &vec![constants::BIGDECIMAL_ZERO.clone(); create_liquidity_pool.input_tokens.len()])
        .set("cumulativeSupplySideRevenueUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("cumulativeProtocolSideRevenueUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("cumulativeTotalRevenueUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("cumulativeVolumeTokenAmounts", &vec![constants::BIGINT_ZERO.clone(); create_liquidity_pool.input_tokens.len()])
        .set("cumulativeVolumesUSD", &vec![constants::BIGDECIMAL_ZERO.clone(); create_liquidity_pool.input_tokens.len()])
        .set("cumulativeVolumeUSD", &vec![constants::BIGDECIMAL_ZERO.clone(); create_liquidity_pool.input_tokens.len()])
        .set("inputTokenBalances", &vec![constants::BIGINT_ZERO.clone(); create_liquidity_pool.input_tokens.len()])
        .set("inputTokenBalancesUSD", &vec![constants::BIGDECIMAL_ZERO.clone(); create_liquidity_pool.input_tokens.len()])
        .set("inputTokenWeights", &vec![constants::BIGDECIMAL_ZERO.clone(); create_liquidity_pool.input_tokens.len()])
        .set("stakedOutputTokenAmount", constants::BIGINT_ZERO.clone())
        .set("cumulativeDepositCount", 0)
        .set("cumulativeWithdrawCount", 0)
        .set("cumulativeSwapCount", 0)
        .set("positionCount", 0)
        .set("openPositionCount", 0)
        .set("closedPositionCount", 0);


    
    if let Some(tick) = &create_liquidity_pool.tick {
        row.set("tick", tick);
    }
    if let Some(liquidity_token) = &create_liquidity_pool.liquidity_token {
        row.set("liquidityToken", liquidity_token);
    }
    if let Some(liquidity_token_type) = &create_liquidity_pool.liquidity_token_type {
        row.set("liquidityTokenType", liquidity_token_type);
    }

    if create_liquidity_pool.reward_tokens.len() > 0 {
        row.set("rewardTokens", &create_liquidity_pool.reward_tokens);
        row.set("rewardTokenEmissionsAmount", &vec![constants::BIGINT_ZERO.clone(); create_liquidity_pool.reward_tokens.len()]);
        row.set("rewardTokenEmissionsUSD", &vec![constants::BIGDECIMAL_ZERO.clone(); create_liquidity_pool.reward_tokens.len()]);
    }
}

// pub fn liquidity_pool_balance_entity_change(
//     pool_address: &Vec<u8>,
//     input_token_balance: &Vec<String>,
// ) -> EntityChange {
//     let mut liquidity_pool_change: EntityChange =
//         EntityChange::new("LiquidityPool", &format!("0x{}", hex::encode(pool_address)), 0, Operation::Create);
    
//     liquidity_pool_change
//         .change("totalValueLockedUSD", constants::BIGDECIMAL_ZERO.clone())
//         .change("totalLiquidity", constants::BIGINT_ZERO.clone())
//         .change("totalLiquidityUSD", constants::BIGDECIMAL_ZERO.clone())
//         .change("activeLiquidity", constants::BIGINT_ZERO.clone())
//         .change("activeLiquidityUSD", constants::BIGDECIMAL_ZERO.clone())
//         .change("uncollectedProtocolSideTokenAmounts", &vec![constants::BIGINT_ZERO.clone(); input_tokens.len()])
//         .change("uncollectedProtocolSideValuesUSD", &vec![constants::BIGDECIMAL_ZERO.clone(); input_tokens.len()])
//         .change("uncollectedSupplySideTokenAmounts", &vec![constants::BIGINT_ZERO.clone(); input_tokens.len()])
//         .change("uncollectedSupplySideValuesUSD", &vec![constants::BIGDECIMAL_ZERO.clone(); input_tokens.len()])
//         .change("cumulativeSupplySideRevenueUSD", constants::BIGDECIMAL_ZERO.clone())
//         .change("cumulativeProtocolSideRevenueUSD", constants::BIGDECIMAL_ZERO.clone())
//         .change("cumulativeTotalRevenueUSD", constants::BIGDECIMAL_ZERO.clone())
//         .change("cumulativeVolumeTokenAmounts", &vec![constants::BIGINT_ZERO.clone(); input_tokens.len()])
//         .change("cumulativeVolumesUSD", &vec![constants::BIGDECIMAL_ZERO.clone(); input_tokens.len()])
//         .change("cumulativeVolumeUSD", &vec![constants::BIGDECIMAL_ZERO.clone(); input_tokens.len()])
//         .change("inputTokenBalances", &vec![constants::BIGINT_ZERO.clone(); input_tokens.len()])
//         .change("inputTokenBalancesUSD", &vec![constants::BIGDECIMAL_ZERO.clone(); input_tokens.len()])
//         .change("inputTokenWeights", &vec![constants::BIGDECIMAL_ZERO.clone(); input_tokens.len()])
//         .change("stakedOutputTokenAmount", constants::BIGINT_ZERO.clone());

//     liquidity_pool_change
// }
