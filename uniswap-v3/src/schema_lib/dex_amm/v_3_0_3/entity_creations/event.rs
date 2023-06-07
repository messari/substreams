use ethabi::Bytes;
use substreams::scalar::BigInt;
use substreams::Hex;

use crate::constants;
use crate::pb;
use crate::pb::dex_amm::v3_0_3::{
    DepositEntityCreation, PrunedTransaction, SwapEntityCreation, WithdrawEntityCreation,
};
use crate::tables::{Row, Tables};

pub fn create_swap_entity(
    tables: &mut Tables,
    entity_id: &Vec<u8>,
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    swap_entity_creation: &SwapEntityCreation,
) {
    let swap_tokens: SwapTokens = get_swap_tokens(
        &swap_entity_creation.input_tokens,
        &swap_entity_creation.amounts,
    );
    tables
        .create_row("Swap", std::str::from_utf8(entity_id).unwrap())
        .set("hash", &pruned_transaction.hash)
        .set("nonce", &pruned_transaction.nonce.clone().unwrap())
        .set("gasLimit", &pruned_transaction.gas_limit.clone().unwrap())
        .set("gasUsed", &pruned_transaction.gas_used.clone().unwrap())
        .set("gasPrice", &pruned_transaction.gas_price.clone().unwrap())
        .set("logIndex", &swap_entity_creation.log_index.clone().unwrap())
        .set("protocol", &swap_entity_creation.protocol)
        .set("account", &pruned_transaction.from)
        .set("pool", &swap_entity_creation.pool)
        .set("blockNumber", BigInt::from(*block_number))
        .set("timestamp", BigInt::from(*timestamp))
        .set("tick", &swap_entity_creation.tick.clone().unwrap())
        .set("tokenIn", &swap_tokens.token_in.token)
        .set("amountIn", &swap_tokens.token_in.amount)
        .set("amountInUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("tokenOut", &swap_tokens.token_out.token)
        .set("amountOut", &swap_tokens.token_out.amount)
        .set("amountOutUSD", constants::BIGDECIMAL_ZERO.clone());
}

struct SwapTokens {
    token_in: SwapToken,
    token_out: SwapToken,
}

struct SwapToken {
    token: Bytes,
    amount: pb::common::v1::BigInt,
}

fn get_swap_tokens(tokens: &Vec<Bytes>, amounts: &Vec<pb::common::v1::BigInt>) -> SwapTokens {
    if tokens.len() != 2 && amounts.len() != 2 {
        panic!("Tokens and amounts must be equal to 2");
    }

    if BigInt::try_from(amounts[0].value.clone()).unwrap() > constants::BIGINT_ZERO.clone() {
        return SwapTokens {
            token_in: SwapToken {
                token: tokens[0].clone(),
                amount: amounts[0].clone(),
            },
            token_out: SwapToken {
                token: tokens[1].clone(),
                amount: amounts[1].clone(),
            },
        };
    } else {
        return SwapTokens {
            token_in: SwapToken {
                token: tokens[1].clone(),
                amount: amounts[1].clone(),
            },
            token_out: SwapToken {
                token: tokens[0].clone(),
                amount: amounts[0].clone(),
            },
        };
    }
}

pub fn create_deposit_entity(
    tables: &mut Tables,
    entity_id: &Vec<u8>,
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    deposit_entity_creation: &DepositEntityCreation,
) {
    let row: &mut Row = tables.create_row("Deposit", Hex(entity_id).to_string());
    row.set("hash", &pruned_transaction.hash)
        .set("nonce", &pruned_transaction.nonce.clone().unwrap())
        .set("gasLimit", &pruned_transaction.gas_limit.clone().unwrap())
        .set("gasUsed", &pruned_transaction.gas_used.clone().unwrap())
        .set("gasPrice", &pruned_transaction.gas_price.clone().unwrap())
        .set(
            "logIndex",
            &deposit_entity_creation.log_index.clone().unwrap(),
        )
        .set("protocol", &deposit_entity_creation.protocol)
        .set("account", &pruned_transaction.from)
        // .set("position", &position)
        .set("pool", &deposit_entity_creation.pool)
        .set("blockNumber", BigInt::from(*block_number))
        .set("timestamp", BigInt::from(*timestamp))
        .set(
            "liquidity",
            &deposit_entity_creation.liquidity.clone().unwrap(),
        )
        .set("inputTokens", &deposit_entity_creation.input_tokens)
        .set(
            "inputTokenAmounts",
            &deposit_entity_creation.input_token_amounts,
        )
        .set("amountUSD", constants::BIGDECIMAL_ZERO.clone());

    match (
        &deposit_entity_creation.tick_lower,
        &deposit_entity_creation.tick_upper,
    ) {
        (Some(tick_lower), Some(tick_upper)) => {
            row.set("tickLower", tick_lower)
                .set("tickUpper", tick_upper);
        }
        _ => {}
    }
}

pub fn create_withdraw_entity(
    tables: &mut Tables,
    entity_id: &Vec<u8>,
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    withdraw_entity_creation: &WithdrawEntityCreation,
) {
    let row: &mut Row = tables.create_row("Withdraw", Hex(entity_id).to_string());
    row.set("hash", &pruned_transaction.hash)
        .set("nonce", &pruned_transaction.nonce.clone().unwrap())
        .set("gasLimit", &pruned_transaction.gas_limit.clone().unwrap())
        .set("gasUsed", &pruned_transaction.gas_used.clone().unwrap())
        .set("gasPrice", &pruned_transaction.gas_price.clone().unwrap())
        .set(
            "logIndex",
            &withdraw_entity_creation.log_index.clone().unwrap(),
        )
        .set("protocol", &withdraw_entity_creation.protocol)
        .set("account", &pruned_transaction.from)
        // .set("position", &position)
        .set("pool", &withdraw_entity_creation.pool)
        .set("blockNumber", BigInt::from(*block_number))
        .set("timestamp", BigInt::from(*timestamp))
        .set(
            "liquidity",
            &withdraw_entity_creation.liquidity.clone().unwrap(),
        )
        .set("inputTokens", &withdraw_entity_creation.input_tokens)
        .set(
            "inputTokenAmounts",
            &withdraw_entity_creation.input_token_amounts,
        )
        .set("amountUSD", constants::BIGDECIMAL_ZERO.clone());

    match (
        &withdraw_entity_creation.tick_lower,
        &withdraw_entity_creation.tick_upper,
    ) {
        (Some(tick_lower), Some(tick_upper)) => {
            row.set("tickLower", tick_lower)
                .set("tickUpper", tick_upper);
        }
        _ => {}
    }
}
