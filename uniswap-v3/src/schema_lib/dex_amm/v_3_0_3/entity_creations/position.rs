use substreams::scalar::BigInt;

use crate::constants;
use crate::pb::dex_amm::v3_0_3::{PositionEntityCreation, PrunedTransaction};
use crate::tables::{Row, Tables};

pub fn create_position_entity(
    tables: &mut Tables,
    entity_id: &Vec<u8>,
    block_number_opened: &u64,
    timestamp_opened: &i64,
    pruned_transaction: &PrunedTransaction,
    position_creation: &PositionEntityCreation,
) {
    let row: &mut Row = tables.create_row("Position", std::str::from_utf8(entity_id).unwrap());
    row.set("account", &position_creation.account)
        .set("pool", &position_creation.pool)
        .set("hashOpened", &pruned_transaction.hash)
        .set("blockNumberOpened", BigInt::from(*block_number_opened))
        .set("timestampOpened", BigInt::from(*timestamp_opened))
        .set("liquidity", constants::BIGINT_ZERO.clone())
        .set("liquidityUSD", constants::BIGDECIMAL_ZERO.clone())
        .set(
            "cumulativeDepositTokenAmounts",
            vec![constants::BIGINT_ZERO.clone(); position_creation.n_tokens as usize],
        )
        .set("cumulativeDepositUSD", constants::BIGDECIMAL_ZERO.clone())
        .set(
            "cumulativeWithdrawTokenAmounts",
            vec![constants::BIGINT_ZERO.clone(); position_creation.n_tokens as usize],
        )
        .set("cumulativeWithdrawUSD", constants::BIGDECIMAL_ZERO.clone())
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
        row.set(
            "cumulativeRewardUSD",
            vec![constants::BIGDECIMAL_ZERO.clone(); *n_reward_tokens as usize],
        );
    }
}
