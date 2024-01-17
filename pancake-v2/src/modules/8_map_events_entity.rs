use std::ops::Mul;
use std::str::FromStr;

use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{StoreGet, StoreGetBigDecimal, StoreGetProto};
use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChange, EntityChanges};

use crate::utils;
use crate::common::constants;
use crate::store_key::StoreKey;
use crate::pb::erc20::v1::Erc20Token;
use crate::pb::pancake::v2::event::Type::{DepositType, SwapType, WithdrawType};
use crate::pb::pancake::v2::{DepositEvent, Event, Events, Pool, SwapEvent, WithdrawEvent};

#[substreams::handlers::map]
pub fn map_events_entity(
    pool_events_map: Events,
    pool_store: StoreGetProto<Pool>,
) -> Result<EntityChanges, ()> {
    let mut entity_changes: Vec<EntityChange> = vec![];

    for event in pool_events_map.events {
        let ordinal = event.log_ordinal as u64;

        let pool =
            pool_store.must_get_last(StoreKey::Pool.get_unique_pool_key(&event.clone().pool));

        match event.clone().r#type.unwrap() {
            DepositType(deposit) => {
                entity_changes.push(create_deposit_transaction(ordinal, &pool, &event, &deposit))
            }
            WithdrawType(withdraw) => entity_changes.push(create_withdraw_transaction(
                ordinal, &pool, &event, &withdraw,
            )),
            SwapType(swap) => entity_changes.push(create_swap_transaction(ordinal, &event, &swap)),
            _ => {}
        }
    }

    Ok(EntityChanges { entity_changes })
}

fn get_event_id(event: &Event) -> String {
    [event.hash.clone(), event.log_index.to_string()].join("-")
}

fn calculate_event_amount_usd(
    ordinal: u64,
    input_tokens: Vec<Erc20Token>,
    amounts: Vec<String>,
    store: &StoreGetBigDecimal,
) -> BigDecimal {
    let mut amount_usd = BigDecimal::zero();

    for (idx, token) in input_tokens.iter().enumerate() {
        let token_amount = BigInt::try_from(amounts[idx].clone()).unwrap();
        let token_price = utils::get_token_price(ordinal, store, &token.address);

        amount_usd = amount_usd + token_price.mul(token_amount.to_decimal(token.decimals))
    }

    amount_usd
}

fn create_deposit_transaction(
    ordinal: u64,
    pool: &Pool,
    event: &Event,
    deposit: &DepositEvent,
) -> EntityChange {
    let id = get_event_id(event);

    let mut deposit_entity_change =
        EntityChange::new("Deposit", id.as_str(), ordinal, Operation::Create);

    let input_tokens = pool.input_tokens.as_ref().unwrap().items.clone();
    let input_token_amounts = deposit.input_token_amounts.clone();
    let output_token_amount =
        BigInt::try_from(deposit.output_token_amount.as_ref().unwrap().clone()).unwrap();

    deposit_entity_change
        .change("id", id)
        .change("hash", event.hash.clone())
        .change("logIndex", event.log_index as i32)
        .change("protocol", constants::PANCAKE_V2_FACTORY.to_string())
        .change("to", event.to.clone())
        .change("from", event.from.clone())
        .change("blockNumber", BigInt::from(event.block_number))
        .change("timestamp", BigInt::from(event.timestamp))
        .change("inputTokens", pool.input_tokens())
        .change("outputToken", pool.output_token_address())
        .change("inputTokenAmounts", input_token_amounts)
        .change("outputTokenAmount", output_token_amount)
        .change("amountUSD", BigDecimal::zero())
        .change("pool", pool.address.clone());

    deposit_entity_change
}

fn create_withdraw_transaction(
    ordinal: u64,
    pool: &Pool,
    event: &Event,
    withdraw: &WithdrawEvent,
) -> EntityChange {
    let id = get_event_id(event);

    let mut withdraw_entity_change: EntityChange =
        EntityChange::new("Withdraw", id.as_str(), ordinal, Operation::Create);

    let input_tokens = pool.input_tokens.as_ref().unwrap().items.clone();
    let input_token_amounts = withdraw.input_token_amounts.clone();
    let output_token_amount =
        BigInt::try_from(withdraw.output_token_amount.as_ref().unwrap().clone()).unwrap();

    withdraw_entity_change
        .change("id", id)
        .change("hash", event.hash.clone())
        .change("logIndex", event.log_index as i32)
        .change("protocol", constants::PANCAKE_V2_FACTORY.to_string())
        .change("to", event.to.clone())
        .change("from", event.from.clone())
        .change("blockNumber", BigInt::from(event.block_number))
        .change("timestamp", BigInt::from(event.timestamp))
        .change("inputTokens", pool.input_tokens())
        .change("outputToken", pool.output_token_address())
        .change("inputTokenAmounts", input_token_amounts)
        .change("outputTokenAmount", output_token_amount)
        .change("amountUSD", BigDecimal::zero())
        .change("pool", pool.address.clone());

    withdraw_entity_change
}

fn create_swap_transaction(ordinal: u64, event: &Event, swap: &SwapEvent) -> EntityChange {
    let id = get_event_id(event);

    let mut swap_entity_change: EntityChange =
        EntityChange::new("Swap", id.as_str(), ordinal, Operation::Create);

    let token_in = swap.token_in.clone().unwrap();
    let token_out = swap.token_out.clone().unwrap();

    let token_in_price = BigDecimal::zero();
    let token_out_price = BigDecimal::zero();

    let amount_in = BigInt::from_str(swap.amount_in.as_str()).unwrap();
    let amount_out = BigInt::from_str(swap.amount_out.as_str()).unwrap();

    let amount_in_usd = amount_in.to_decimal(token_in.decimals) * token_in_price;
    let amount_out_usd = amount_out.to_decimal(token_out.decimals) * token_out_price;

    swap_entity_change
        .change("id", id)
        .change("hash", event.hash.clone())
        .change("logIndex", event.log_index as i32)
        .change("protocol", constants::PANCAKE_V2_FACTORY.to_string())
        .change("to", event.to.clone())
        .change("from", event.from.clone())
        .change("blockNumber", BigInt::from(event.block_number))
        .change("timestamp", BigInt::from(event.timestamp))
        .change("tokenIn", token_in.address)
        .change("amountIn", amount_in)
        .change("amountInUSD", amount_in_usd)
        .change("tokenOut", token_out.address)
        .change("amountOut", amount_out)
        .change("amountOutUSD", amount_out_usd)
        .change("pool", event.pool.clone());

    swap_entity_change
}
