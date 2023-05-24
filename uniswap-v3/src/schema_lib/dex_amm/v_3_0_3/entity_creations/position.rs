use substreams::scalar::{BigInt};
use substreams::Hex;

use crate::pb::dex_amm::v3_0_3::{PrunedTransaction, PositionEntityCreation};
use crate::pb::entity;
use crate::schema_lib::dex_amm::v_3_0_3::keys;
use crate::tables::{Tables, Row};
use crate::constants;

pub fn create_position_entity(
    tables: &mut Tables,
    entity_id: &Vec<u8>,
    block_number_opened: &u64,
    timestamp_opened: &i64,
    pruned_transaction: &PrunedTransaction,
    position_creation: &PositionEntityCreation,
) {
    let row: &mut Row = tables.create_row("Position", Hex(entity_id).to_string());
    row
        .set("account", &position_creation.account)
        .set("pool", &position_creation.pool)
        .set("hashOpened", &pruned_transaction.hash)
        .set("blockNumberOpened", BigInt::from(*block_number_opened))
        .set("timestampOpened", BigInt::from(*timestamp_opened))
        .set("liquidity", constants::BIGINT_ZERO.clone())
        .set("liquidityUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("cumulativeDepositTokenAmounts", vec![constants::BIGINT_ZERO.clone(); position_creation.n_tokens as usize])
        .set("cumulativeDepositUSD", vec![constants::BIGDECIMAL_ZERO.clone(); position_creation.n_tokens as usize])
        .set("cumulativeWithdrawTokenAmounts", vec![constants::BIGINT_ZERO.clone(); position_creation.n_tokens as usize])
        .set("cumulativeWithdrawUSD", vec![constants::BIGDECIMAL_ZERO.clone(); position_creation.n_tokens as usize])
        .set("depositCount", 0)
        .set("withdrawCount", 0);
        
    if let Some(tick_lower) = &position_creation.tick_lower {
        row.set("tickLower", tick_lower);
    }
    if let Some(tick_upper) = &position_creation.tick_upper {
        row.set("tickUpper", tick_upper);
    }
    if let Some(liquidity_token) = &position_creation.liquidity_token {
        row.set("liquidityToken", liquidity_token);
    }
    if let Some(liquidity_token_type) = &position_creation.liquidity_token_type {
        row.set("liquidityTokenType", liquidity_token_type);
    }
    if let Some(n_reward_tokens) = &position_creation.n_reward_tokens {
        row.set("cumulativeRewardUSD", vec![constants::BIGDECIMAL_ZERO.clone(); *n_reward_tokens as usize]);
    }
}
