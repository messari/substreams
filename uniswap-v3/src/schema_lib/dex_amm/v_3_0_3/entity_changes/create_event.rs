use ethabi::Bytes;
use substreams::{Hex};
use substreams::prelude::*;
use substreams_entity_change::pb::entity::{EntityChange, entity_change::Operation};
use substreams::store::{DeltaBigDecimal, DeltaInt64, DeltaBigInt, StoreGetRaw, StoreGet, StoreGetArray};
use substreams::scalar::{BigDecimal, BigInt};
use substreams_ethereum::NULL_ADDRESS;

use crate::pb;
use crate::pb::dex_amm::v3_0_3::{PrunedTransaction, Swap, Deposit, Withdraw};
use crate::schema_lib::dex_amm::v_3_0_3::keys;
use crate::tables::{Tables, Row};
use crate::constants;


pub fn create_swap_entity_change(
    tables: &mut Tables,
    block_number: &u64,
    timestamp: &i64,    
    pruned_transaction: &PrunedTransaction,
    swap: &Swap,
    store_append_string: &StoreGetArray<String>
) {

    let input_tokens: Vec<Bytes> = vec![NULL_ADDRESS.to_vec(), NULL_ADDRESS.to_vec()];

    tables.create_row("Swap", &keys::get_event_key(&pruned_transaction.hash, &swap.log_index.clone().unwrap()))
        .set("hash", &pruned_transaction.hash)
        .set("nonce", &pruned_transaction.nonce.clone().unwrap())
        .set("gasLimit", &pruned_transaction.gas_limit.clone().unwrap())
        .set("gasUsed", &pruned_transaction.gas_used.clone().unwrap())
        .set("gasPrice", &pruned_transaction.gas_price.clone().unwrap())
        .set("logIndex", &swap.log_index.clone().unwrap())
        .set("protocol", &swap.protocol)
        .set("account", &pruned_transaction.from)
        .set("pool", &swap.pool)
        .set("blockNumber", BigInt::from(*block_number))
        .set("timestamp", BigInt::from(*timestamp))
        .set("tick", &swap.tick.clone().unwrap())
        .set("tokenIn", &NULL_ADDRESS.to_vec())
        .set("amountIn", constants::BIGINT_ZERO.clone())
        .set("amountInUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("tokenOut", &NULL_ADDRESS.to_vec())
        .set("amountOut", constants::BIGINT_ZERO.clone())
        .set("amountOutUSD", constants::BIGDECIMAL_ZERO.clone());
}

struct SwapTokens {
    token_in: SwapToken, 
    token_out: SwapToken,
}

struct SwapToken {
    token: String,
    amount: pb::dex_amm::v3_0_3::BigInt,
}

fn get_swap_tokens(tokens: Vec<String>, amounts: Vec<pb::dex_amm::v3_0_3::BigInt>) -> SwapTokens {
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
        }
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
        }
    }
}

pub fn create_deposit_entity_change(
    tables: &mut Tables,
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    deposit: &Deposit,
    store_append_string: &StoreGetArray<String>
) {
    let input_tokens: Vec<Bytes> = Vec::new();

    let row: &mut Row = tables.create_row("Deposit", &keys::get_event_key(&pruned_transaction.hash, &deposit.log_index.clone().unwrap()));
    row
        .set("hash", &pruned_transaction.hash)
        .set("nonce", &pruned_transaction.nonce.clone().unwrap())
        .set("gasLimit", &pruned_transaction.gas_limit.clone().unwrap())
        .set("gasUsed", &pruned_transaction.gas_used.clone().unwrap())
        .set("gasPrice", &pruned_transaction.gas_price.clone().unwrap())
        .set("logIndex", &deposit.log_index.clone().unwrap())
        .set("protocol", &deposit.protocol)
        .set("account", &pruned_transaction.from)
        // .set("position", &position)
        .set("pool", &deposit.pool)
        .set("blockNumber", BigInt::from(*block_number))
        .set("timestamp", BigInt::from(*timestamp))
        .set("liquidity", &deposit.liquidity.clone().unwrap())
        .set("inputTokens", &input_tokens)
        .set("inputTokenAmounts", &deposit.input_token_amounts)
        .set("amountUSD", constants::BIGDECIMAL_ZERO.clone());

    match (&deposit.tick_lower, &deposit.tick_upper) {
        (Some(tick_lower), Some(tick_upper)) => {
            row
                .set("tickLower", tick_lower)
                .set("tickUpper", tick_upper);
        }
        _ => {}
    }
}

pub fn create_withdraw_entity_change(
    tables: &mut Tables,
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    withdraw: &Withdraw,
    store_append_string: &StoreGetArray<String>
) {
    let input_tokens: Vec<Bytes> = Vec::new();

    let row: &mut Row = tables.create_row("Withdraw", &keys::get_event_key(&pruned_transaction.hash, &withdraw.log_index.clone().unwrap()));
    row
        .set("hash", &pruned_transaction.hash)
        .set("nonce", &pruned_transaction.nonce.clone().unwrap())
        .set("gasLimit", &pruned_transaction.gas_limit.clone().unwrap())
        .set("gasUsed", &pruned_transaction.gas_used.clone().unwrap())
        .set("gasPrice", &pruned_transaction.gas_price.clone().unwrap())
        .set("logIndex", &withdraw.log_index.clone().unwrap())
        .set("protocol", &withdraw.protocol)
        .set("account", &pruned_transaction.from)
        // .set("position", &position)
        .set("pool", &withdraw.pool)
        .set("blockNumber", BigInt::from(*block_number))
        .set("timestamp", BigInt::from(*timestamp))
        .set("liquidity", &withdraw.liquidity.clone().unwrap())
        .set("inputTokens", &input_tokens)
        .set("inputTokenAmounts", &withdraw.input_token_amounts)
        .set("amountUSD", constants::BIGDECIMAL_ZERO.clone());

    match (&withdraw.tick_lower, &withdraw.tick_upper) {
        (Some(tick_lower), Some(tick_upper)) => {
            row
                .set("tickLower", tick_lower)
                .set("tickUpper", tick_upper);
        }
        _ => {}
    }
}
