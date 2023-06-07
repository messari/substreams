use substreams::scalar::{BigInt};

use crate::pb::dex_amm::v3_0_3::LiquidityPoolEntityCreation;
use crate::schema_lib::dex_amm::v_3_0_3::keys;
use crate::tables::{Tables, Row};
use crate::constants;

pub fn create_liquidity_pool_entity(
    tables: &mut Tables,
    entity_id: &Vec<u8>,
    block_number: &u64,
    timestamp: &i64,
    liquidity_pool_entity_creation: &LiquidityPoolEntityCreation,
) {
    let row: &mut Row = tables.create_row("LiquidityPool", std::str::from_utf8(entity_id).unwrap());
    row
        .set("protocol", &liquidity_pool_entity_creation.protocol)
        .set("name", keys::get_pool_name("Uniswap V3", &liquidity_pool_entity_creation.input_token_symbols))
        .set("symbol", keys::get_pool_symbol(&liquidity_pool_entity_creation.input_token_symbols))
        .set("inputTokens", &liquidity_pool_entity_creation.input_tokens)
        .set("fees", &liquidity_pool_entity_creation.fees)
        .set("isSingleSided", liquidity_pool_entity_creation.is_single_sided)
        .set("createdBlockNumber", BigInt::from(*block_number))
        .set("createdTimestamp", BigInt::from(*timestamp))
        .set("totalValueLockedUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("totalLiquidity", constants::BIGINT_ZERO.clone())
        .set("totalLiquidityUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("activeLiquidity", constants::BIGINT_ZERO.clone())
        .set("activeLiquidityUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("uncollectedProtocolSideTokenAmounts", &vec![constants::BIGINT_ZERO.clone(); liquidity_pool_entity_creation.input_tokens.len()])
        .set("uncollectedProtocolSideValuesUSD", &vec![constants::BIGDECIMAL_ZERO.clone(); liquidity_pool_entity_creation.input_tokens.len()])
        .set("uncollectedSupplySideTokenAmounts", &vec![constants::BIGINT_ZERO.clone(); liquidity_pool_entity_creation.input_tokens.len()])
        .set("uncollectedSupplySideValuesUSD", &vec![constants::BIGDECIMAL_ZERO.clone(); liquidity_pool_entity_creation.input_tokens.len()])
        .set("cumulativeSupplySideRevenueTokenAmounts", &vec![constants::BIGINT_ZERO.clone(); liquidity_pool_entity_creation.input_tokens.len()])
        .set("cumulativeSupplySideRevenueUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("cumulativeProtocolSideRevenueTokenAmounts", &vec![constants::BIGINT_ZERO.clone(); liquidity_pool_entity_creation.input_tokens.len()])
        .set("cumulativeProtocolSideRevenueUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("cumulativeTotalRevenueTokenAmounts", &vec![constants::BIGINT_ZERO.clone(); liquidity_pool_entity_creation.input_tokens.len()])
        .set("cumulativeTotalRevenueUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("cumulativeVolumeTokenAmounts", &vec![constants::BIGINT_ZERO.clone(); liquidity_pool_entity_creation.input_tokens.len()])
        .set("cumulativeVolumesUSD", &vec![constants::BIGDECIMAL_ZERO.clone(); liquidity_pool_entity_creation.input_tokens.len()])
        .set("cumulativeVolumeUSD", &constants::BIGDECIMAL_ZERO.clone())
        .set("inputTokenBalances", &vec![constants::BIGINT_ZERO.clone(); liquidity_pool_entity_creation.input_tokens.len()])
        .set("inputTokenBalancesUSD", &vec![constants::BIGDECIMAL_ZERO.clone(); liquidity_pool_entity_creation.input_tokens.len()])
        .set("inputTokenWeights", &liquidity_pool_entity_creation.input_token_weights)
        .set("stakedOutputTokenAmount", constants::BIGINT_ZERO.clone())
        .set("cumulativeDepositCount", 0)
        .set("cumulativeWithdrawCount", 0)
        .set("cumulativeSwapCount", 0)
        .set("positionCount", 0)
        .set("openPositionCount", 0)
        .set("closedPositionCount", 0);


    
    if let Some(tick) = &liquidity_pool_entity_creation.tick {
        row.set("tick", tick);
    }
    if let Some(liquidity_token) = &liquidity_pool_entity_creation.liquidity_token {
        row.set("liquidityToken", liquidity_token);
    }
    if let Some(liquidity_token_type) = &liquidity_pool_entity_creation.liquidity_token_type {
        row.set("liquidityTokenType", liquidity_token_type);
    }

    if liquidity_pool_entity_creation.reward_tokens.len() > 0 {
        row.set("rewardTokens", &liquidity_pool_entity_creation.reward_tokens);
        row.set("rewardTokenEmissionsAmount", &vec![constants::BIGINT_ZERO.clone(); liquidity_pool_entity_creation.reward_tokens.len()]);
        row.set("rewardTokenEmissionsUSD", &vec![constants::BIGDECIMAL_ZERO.clone(); liquidity_pool_entity_creation.reward_tokens.len()]);
    }
}
