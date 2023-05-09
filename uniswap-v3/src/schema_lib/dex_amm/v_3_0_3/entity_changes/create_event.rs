use ethabi::Bytes;
use substreams::{Hex};
use substreams::prelude::*;
use substreams_entity_change::pb::entity::{EntityChange, entity_change::Operation};
use substreams::store::{DeltaBigDecimal, DeltaInt64, DeltaBigInt, StoreGetRaw, StoreGet, StoreGetArray};

use crate::pb::dex_amm::v3_0_3::{PrunedTransaction, Swap, Deposit, Withdraw};
use crate::schema_lib::dex_amm::v_3_0_3::keys;


pub fn create_swap_entity_change(
    block_number: &u64,
    timestamp: &i64,    
    pruned_transaction: &PrunedTransaction,
    swap: Swap,
    store_append_string: &StoreGetArray<String>
) -> EntityChange {

    let swap_tokens = match store_append_string.get_last(keys::get_input_tokens_key(&swap.pool)) {
        Some(input_tokens) => {
            get_swap_tokens(input_tokens, swap.amounts)
        },
        None => {
            return EntityChange::new("Swap", &keys::get_event_key(&pruned_transaction.hash, &swap.log_index), 0, Operation::Create);
        }
    };

    let mut swap_entity_change: EntityChange =
        EntityChange::new("Swap", &keys::get_event_key(&pruned_transaction.hash, &swap.log_index), 0, Operation::Create);
    
    swap_entity_change
        .change("hash", &pruned_transaction.hash)
        .change("nonce", pruned_transaction.nonce)
        .change("gasLimit", pruned_transaction.gas_limit)
        .change("gasUsed", pruned_transaction.gas_used)
        .change("gasPrice", BigInt::try_from(pruned_transaction.gas_price.clone()).unwrap())
        .change("logIndex", swap.log_index as u64)
        .change("protocol", swap.protocol)
        .change("account", &pruned_transaction.from)
        .change("pool", swap.pool)
        .change("blockNumber", BigInt::from(*block_number))
        .change("timestamp", BigInt::from(*timestamp))
        .change("tick", swap.tick)
        .change("tokenIn", swap_tokens.token_in.token)
        .change("amountIn", swap_tokens.token_in.amount)
        .change("amountInUSD", BigDecimal::from(0))
        .change("tokenOut", swap_tokens.token_out.token)
        .change("amountOut", swap_tokens.token_out.amount)
        .change("amountOutUSD", BigDecimal::from(0));

    swap_entity_change
}

struct SwapTokens {
    token_in: SwapToken, 
    token_out: SwapToken,
}

struct SwapToken {
    token: String,
    amount: String,
}

fn get_swap_tokens(tokens: Vec<String>, amounts: Vec<String>) -> SwapTokens {
    if tokens.len() != 2 && amounts.len() != 2 {
        panic!("Tokens and amounts must be equal to 2");
    }

    if BigInt::try_from(amounts[0]).unwrap() > BigInt::zero() {
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
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    deposit: Deposit,
    store_append_string: &StoreGetArray<String>
) -> EntityChange {
    let mut deposit_entity_change: EntityChange =
        EntityChange::new("Deposit", &keys::get_event_key(&pruned_transaction.hash, &deposit.log_index), 0, Operation::Create);

    let input_tokens = match store_append_string.get_last(keys::get_input_tokens_key(&deposit.pool)) {
        Some(input_tokens) => {
            input_tokens
        },
        None => {
            return EntityChange::new("Swap", &keys::get_event_key(&pruned_transaction.hash, &deposit.log_index), 0, Operation::Create);
        }
    };

    deposit_entity_change
        .change("hash", &pruned_transaction.hash)
        .change("nonce", pruned_transaction.nonce)
        .change("gasLimit", pruned_transaction.gas_limit)
        .change("gasUsed", pruned_transaction.gas_used)
        .change("gasPrice", BigInt::try_from(pruned_transaction.gas_price.clone()).unwrap())
        .change("logIndex", deposit.log_index as u64)
        .change("protocol", deposit.protocol)
        .change("account", &pruned_transaction.from)
        // .change("position", &position)
        .change("pool", deposit.pool)
        .change("blockNumber", BigInt::from(*block_number))
        .change("timestamp", BigInt::from(*timestamp))
        .change("liquidity", BigInt::try_from(deposit.liquidity).unwrap())
        .change("inputTokens", input_tokens)
        .change("inputTokenAmounts", deposit.input_token_amounts)
        .change("amountUSD", BigDecimal::from(0));

    match (deposit.tick_lower, deposit.tick_upper) {
        (Some(tick_lower), Some(tick_upper)) => {
            deposit_entity_change
                .change("tickLower", tick_lower)
                .change("tickUpper", tick_upper);
        }
        _ => {}
    }
    deposit_entity_change
}

pub fn create_withdraw_entity_change(
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    withdraw: Withdraw,
    store_append_string: &StoreGetArray<String>
) -> EntityChange {
    let mut withdraw_entity_change: EntityChange =
        EntityChange::new("Withdraw", &keys::get_event_key(&pruned_transaction.hash, &withdraw.log_index), 0, Operation::Create);

    let input_tokens = match store_append_string.get_last(keys::get_input_tokens_key(&withdraw.pool)) {
        Some(input_tokens) => {
            input_tokens
        },
        None => {
            return EntityChange::new("Swap", &keys::get_event_key(&pruned_transaction.hash, &withdraw.log_index), 0, Operation::Create);
        }
    };
    
    withdraw_entity_change
        .change("hash", &pruned_transaction.hash)
        .change("nonce", pruned_transaction.nonce)
        .change("gasLimit", pruned_transaction.gas_limit)
        .change("gasUsed", pruned_transaction.gas_used)
        .change("gasPrice", BigInt::try_from(pruned_transaction.gas_price.clone()).unwrap())
        .change("logIndex", withdraw.log_index as u64)
        .change("protocol", withdraw.protocol)
        .change("account", &pruned_transaction.from)
        // .change("position", &position)
        .change("pool", withdraw.pool)
        .change("blockNumber", BigInt::from(*block_number))
        .change("timestamp", BigInt::from(*timestamp))
        .change("liquidity", BigInt::try_from(withdraw.liquidity).unwrap())
        .change("inputTokens", input_tokens)
        .change("inputTokenAmounts", withdraw.input_token_amounts)
        .change("amountUSD", BigDecimal::from(0));

    match (withdraw.tick_lower, withdraw.tick_upper) {
        (Some(tick_lower), Some(tick_upper)) => {
            withdraw_entity_change
                .change("tickLower", tick_lower)
                .change("tickUpper", tick_upper);
        }
        _ => {}
    }
    withdraw_entity_change
}
