use substreams::scalar::BigInt;
use substreams::store::StoreGetProto;
use substreams::store::{DeltaBigInt, Deltas, StoreGet};
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_helper::event_handler::EventHandler;
use substreams_helper::hex::Hexable;

use crate::abi::Pool::events::{Burn, Mint, Swap, Sync};
use crate::common::traits::PoolAddresser;
use crate::pb::uniswap::v2::event::Type::{DepositType, SwapType, SyncType, WithdrawType};
use crate::pb::uniswap::v2::{DepositEvent, SwapEvent, SyncEvent, WithdrawEvent};
use crate::pb::uniswap::v2::{Event, Events, Pool};
use crate::store_key::StoreKey;
use crate::utils;

#[substreams::handlers::map]
pub fn map_pool_events(
    block: eth::Block,
    pools_store: StoreGetProto<Pool>,
    balance_deltas: Deltas<DeltaBigInt>,
) -> Result<Events, substreams::errors::Error> {
    let mut events = vec![];

    handle_mint(&block, &pools_store, &balance_deltas, &mut events);
    handle_burn(&block, &pools_store, &balance_deltas, &mut events);
    handle_sync(&block, &pools_store, &mut events);
    handle_swap(&block, &pools_store, &mut events);

    Ok(Events { events })
}

fn handle_mint(
    block: &eth::Block,
    store: &StoreGetProto<Pool>,
    balance_deltas: &Deltas<DeltaBigInt>,
    events: &mut Vec<Event>,
) {
    let mut on_mint = |event: Mint, tx: &eth::TransactionTrace, log: &eth::Log| {
        let pool_address = log.address.to_hex();
        let output_token_mint_amount =
            utils::get_output_token_amount(&balance_deltas, &pool_address);

        events.push(Event {
            hash: tx.hash.to_hex(),
            log_index: log.index,
            log_ordinal: log.ordinal,
            to: tx.to.to_hex(),
            from: tx.from.to_hex(),
            block_number: block.number,
            timestamp: block.timestamp_seconds(),
            pool: pool_address,
            r#type: Some(DepositType(DepositEvent {
                input_token_amounts: vec![event.amount0, event.amount1]
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
                output_token_amount: Some(output_token_mint_amount.to_string()),
            })),
        });
    };

    let mut eh = EventHandler::new(&block);
    eh.filter_by_address(PoolAddresser { store });
    eh.on::<Mint, _>(&mut on_mint);
    eh.handle_events();
}

fn handle_burn(
    block: &eth::Block,
    store: &StoreGetProto<Pool>,
    balance_deltas: &Deltas<DeltaBigInt>,
    events: &mut Vec<Event>,
) {
    let mut on_burn = |event: Burn, tx: &eth::TransactionTrace, log: &eth::Log| {
        let pool_address = log.address.to_hex();
        let output_token_burnt_amount =
            utils::get_output_token_amount(&balance_deltas, &pool_address);

        events.push(Event {
            hash: tx.hash.to_hex(),
            log_index: log.block_index,
            log_ordinal: log.ordinal,
            to: tx.to.to_hex(),
            from: tx.from.to_hex(),
            block_number: block.number,
            timestamp: block.timestamp_seconds(),
            pool: pool_address,
            r#type: Some(WithdrawType(WithdrawEvent {
                input_token_amounts: vec![event.amount0, event.amount1]
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
                output_token_amount: Some(output_token_burnt_amount.neg().to_string()),
            })),
        });
    };

    let mut eh = EventHandler::new(&block);
    eh.filter_by_address(PoolAddresser { store });
    eh.on::<Burn, _>(&mut on_burn);
    eh.handle_events();
}

fn handle_sync(block: &eth::Block, store: &StoreGetProto<Pool>, events: &mut Vec<Event>) {
    let mut on_sync = |event: Sync, tx: &eth::TransactionTrace, log: &eth::Log| {
        let pool_address = log.address.to_hex();

        events.push(Event {
            hash: tx.hash.to_hex(),
            log_index: log.block_index,
            log_ordinal: log.ordinal,
            to: tx.to.to_hex(),
            from: tx.from.to_hex(),
            block_number: block.number,
            timestamp: block.timestamp_seconds(),
            pool: pool_address,
            r#type: Some(SyncType(SyncEvent {
                reserve0: event.reserve0.to_string(),
                reserve1: event.reserve1.to_string(),
            })),
        });
    };

    let mut eh = EventHandler::new(&block);
    eh.filter_by_address(PoolAddresser { store });
    eh.on::<Sync, _>(&mut on_sync);
    eh.handle_events();
}

fn handle_swap(block: &eth::Block, store: &StoreGetProto<Pool>, events: &mut Vec<Event>) {
    let mut on_swap = |event: Swap, tx: &eth::TransactionTrace, log: &eth::Log| {
        let pool_address = log.address.to_hex();

        events.push(Event {
            hash: tx.hash.to_hex(),
            log_index: log.block_index,
            log_ordinal: log.ordinal,
            to: tx.to.to_hex(),
            from: tx.from.to_hex(),
            block_number: block.number,
            timestamp: block.timestamp_seconds(),
            pool: pool_address.clone(),
            r#type: Some(SwapType(get_swap_event(event, &pool_address, &store))),
        });
    };

    let mut eh = EventHandler::new(&block);
    eh.filter_by_address(PoolAddresser { store });
    eh.on::<Swap, _>(&mut on_swap);
    eh.handle_events();
}

fn get_swap_event(
    event: Swap,
    pool_address: &String,
    pools_store: &StoreGetProto<Pool>,
) -> SwapEvent {
    let pool = pools_store.must_get_last(StoreKey::Pool.get_unique_pool_key(pool_address));

    if event.amount0_out.gt(BigInt::zero().as_ref()) {
        SwapEvent {
            token_in: Some(pool.token1_ref()),
            amount_in: (event.amount1_in - event.amount1_out).to_string(),
            token_out: Some(pool.token0_ref()),
            amount_out: (event.amount0_out - event.amount0_in).to_string(),
        }
    } else {
        SwapEvent {
            token_in: Some(pool.token0_ref()),
            amount_in: (event.amount0_in - event.amount0_out).to_string(),
            token_out: Some(pool.token1_ref()),
            amount_out: (event.amount1_out - event.amount1_in).to_string(),
        }
    }
}
