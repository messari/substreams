#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

mod keyer;
mod utils;

use hex_literal::hex;
use std::str::FromStr;

use substreams::store::StoreAdd;
use substreams::store::StoreNew;
use substreams::store::StoreSetRaw;
use substreams::store::{StoreAddInt64, StoreGet, StoreGetRaw, StoreSet, StoreGetProto, StoreSetProto};
use substreams::{log, proto, store, Hex};
use substreams::scalar::{BigDecimal, BigInt};
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};
use substreams_helper::erc20 as erc20_helper;
use substreams_helper::types::Address;
use substreams::pb::substreams::Clock;


use abi::factory;
use abi::pair;

use pb::dex_amm::v1 as dex_amm;
use pb::uniswap::v2 as uniswap;

pub const UNISWAP_V2_FACTORY: Address = hex!("5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f");
pub const UNISWAP_V2_FACTORY_START_BLOCK: u64 = 10_000_835;

trait BlockExt {
    fn timestamp(&self) -> u64;
}

impl BlockExt for eth::Block {
    fn timestamp(&self) -> u64 {
        self.header
            .as_ref()
            .unwrap()
            .timestamp
            .as_ref()
            .unwrap()
            .seconds as u64
    }
}

#[substreams::handlers::map]
fn map_pair_created_events(
    block: eth::Block,
) -> Result<uniswap::PairCreatedEvents, substreams::errors::Error> {
    let mut pair_created_events = uniswap::PairCreatedEvents { items: vec![] };

    for log in block.logs() {
        if let Some(event) = factory::events::PairCreated::match_and_decode(log) {
            if log.log.address != UNISWAP_V2_FACTORY {
                continue;
            }

            pair_created_events.items.push(uniswap::PairCreatedEvent {
                tx_hash: hex::encode(log.receipt.transaction.clone().hash),
                log_index: log.index(),
                log_ordinal: log.ordinal(),
                token0: hex::encode(event.token0.clone()),
                token1: hex::encode(event.token1.clone()),
                pair: hex::encode(event.pair.clone()),
            })
        }
    }

    Ok(pair_created_events)
}

#[substreams::handlers::store]
fn store_pair_created_events(
    pair_created_events: uniswap::PairCreatedEvents,
    output: store::StoreSetRaw,
) {
    log::info!("Stored events {}", pair_created_events.items.len());
    for event in pair_created_events.items {
        output.set(0, keyer::pair_created_key(&event.pair), &proto::encode(&event).unwrap());
    }
}

#[substreams::handlers::map]
fn map_mint_events(
    block: eth::Block,
    pair_created_events_store: StoreGetProto<uniswap::PairCreatedEvent>,
) -> Result<uniswap::MintEvents, substreams::errors::Error> {
    let mut mint_events = uniswap::MintEvents { items: vec![] };

    for log in block.logs() {
        let pool_address = Hex(&log.address()).to_string();
        let pool_key = keyer::pair_created_key(&pool_address);
        let tx_hash = Hex(&log.receipt.transaction.hash).to_string();

        if let Some(event) = pair::events::Mint::match_and_decode(log) {
            // Check if pool has been created
            if pair_created_events_store.get_last(pool_key).is_none() {
                log::info!(
                    "invalid swap. pool does not exist. pool address {} transaction {}",
                    pool_address,
                    tx_hash
                );
                continue;
            }

            mint_events.items.push(uniswap::MintEvent {
                tx_hash,
                log_index: log.index(),
                log_ordinal: log.ordinal(),
                pool_address,
                sender: hex::encode(event.sender.clone()),
                amount0: event.amount0.to_string(),
                amount1: event.amount1.to_string(),
            })
        }
    }

    Ok(mint_events)
}

#[substreams::handlers::map]
fn map_burn_events(
    block: eth::Block,
    pair_created_events_store: StoreGetProto<uniswap::PairCreatedEvent>,
) -> Result<uniswap::BurnEvents, substreams::errors::Error> {
    let mut burn_events = uniswap::BurnEvents { items: vec![] };

    for log in block.logs() {
        let pool_address = Hex(&log.address()).to_string();
        let pool_key = keyer::pair_created_key(&pool_address);
        let tx_hash = Hex(&log.receipt.transaction.hash).to_string();

        if let Some(event) = pair::events::Burn::match_and_decode(log) {
            // Check if pool has been created
            if pair_created_events_store.get_last(pool_key).is_none() {
                log::info!(
                    "invalid swap. pool does not exist. pool address {} transaction {}",
                    pool_address,
                    tx_hash
                );
                continue;
            }

            burn_events.items.push(uniswap::BurnEvent {
                tx_hash,
                log_index: log.index(),
                log_ordinal: log.ordinal(),
                pool_address,
                sender: hex::encode(event.sender.clone()),
                amount0: event.amount0.to_string(),
                amount1: event.amount1.to_string(),
                to: hex::encode(event.to.clone()),
            })
        }
    }

    Ok(burn_events)
}

#[substreams::handlers::map]
fn map_swap_events(
    block: eth::Block,
    pair_created_events_store: StoreGetProto<uniswap::PairCreatedEvent>,
) -> Result<uniswap::SwapEvents, substreams::errors::Error> {
    let mut swap_events = uniswap::SwapEvents { items: vec![] };

    for log in block.logs() {
        if let Some(event) = pair::events::Swap::match_and_decode(log) {
            let pool_address = Hex(&log.address()).to_string();
            let pool_key = keyer::pair_created_key(&pool_address);
            let tx_hash = Hex(&log.receipt.transaction.hash).to_string();

            // Check if pool has been created
            if pair_created_events_store.get_last(pool_key).is_none() {
                log::info!(
                    "invalid swap. pool does not exist. pool address {} transaction {}",
                    pool_address,
                    tx_hash
                );
                continue;
            }

            swap_events.items.push(uniswap::SwapEvent {
                tx_hash,
                log_index: log.index(),
                log_ordinal: log.ordinal(),
                pool_address,
                sender: hex::encode(event.sender.clone()),
                amount0_in: event.amount0_in.to_string(),
                amount1_in: event.amount1_in.to_string(),
                amount0_out: event.amount0_out.to_string(),
                amount1_out: event.amount1_out.to_string(),
                to: hex::encode(event.to.clone()),
            })
        }
    }

    Ok(swap_events)
}

#[substreams::handlers::map]
fn map_deposits(
    block: eth::Block,
    mint_events: uniswap::MintEvents,
    pair_created_events_store: StoreGetProto<uniswap::PairCreatedEvent>,
) -> Result<dex_amm::Deposits, substreams::errors::Error> {
    let mut deposits = dex_amm::Deposits { items: vec![] };

    for mint_event in mint_events.items {
        let pair_created_key = keyer::pair_created_key(&mint_event.pool_address);
        match pair_created_events_store.get_last(pair_created_key) {
            None => {
                log::info!(
                    "invalid deposit. pool does not exist. pool address {} transaction {}",
                    mint_event.pool_address,
                    mint_event.tx_hash
                );
                continue;
            }
            Some(pair_created_event) => {
                deposits.items.push(dex_amm::Deposit {
                    tx_hash: mint_event.tx_hash.clone(),
                    log_index: mint_event.log_index,
                    to: pair_created_event.pair.clone(),
                    from: mint_event.sender.clone(),
                    block_number: block.number,
                    timestamp: block.timestamp(),
                    input_tokens: vec![
                        pair_created_event.token0.clone(),
                        pair_created_event.token1.clone(),
                    ],
                    output_token: pair_created_event.pair.clone(),
                    input_token_amounts: vec![
                        mint_event.amount0.clone(),
                        mint_event.amount1.clone(),
                    ],
                    output_token_amount: "0".to_string(),
                    pool_address: pair_created_event.pair.clone(),
                })
            }
        }
    }

    Ok(deposits)
}

#[substreams::handlers::map]
fn map_withdraws(
    block: eth::Block,
    burn_events: uniswap::BurnEvents,
    pair_created_events_store: StoreGetProto<uniswap::PairCreatedEvent>,
) -> Result<dex_amm::Withdraws, substreams::errors::Error> {
    let mut withdraws = dex_amm::Withdraws  { items: vec![] };

    for burn_event in burn_events.items {
        let pair_created_key = keyer::pair_created_key(&burn_event.pool_address);
        match pair_created_events_store.get_last(pair_created_key) {
            None => {
                log::info!(
                    "invalid withdraw. pool does not exist. pool address {} transaction {}",
                    burn_event.pool_address,
                    burn_event.tx_hash
                );
                continue;
            }
            Some(pair_created_event) => {
                withdraws.items.push(dex_amm::Withdraw {
                    tx_hash: burn_event.tx_hash.clone(),
                    log_index: burn_event.log_index,
                    to: burn_event.to.clone(),
                    from: burn_event.sender.clone(),
                    block_number: block.number,
                    timestamp: block.timestamp(),
                    input_tokens: vec![
                        pair_created_event.token0.clone(),
                        pair_created_event.token1.clone(),
                    ],
                    output_token: pair_created_event.pair.clone(),
                    input_token_amounts: vec![
                        burn_event.amount0.clone(),
                        burn_event.amount1.clone(),
                    ],
                    output_token_amount: "0".to_string(),
                    pool_address: pair_created_event.pair.clone(),
                })
            }
        }
    }

    Ok(withdraws)
}

#[substreams::handlers::map]
fn map_swaps(
    block: eth::Block,
    swap_events: uniswap::SwapEvents,
    pair_created_events_store: StoreGetProto<uniswap::PairCreatedEvent>,
) -> Result<dex_amm::Swaps, substreams::errors::Error> {
    let mut swaps = dex_amm::Swaps { items: vec![] };

    for swap_event in swap_events.items {
        let pair_created_key = keyer::pair_created_key(&swap_event.pool_address);
        match pair_created_events_store.get_last(pair_created_key) {
            None => {
                log::info!(
                    "invalid swap. pool does not exist. pool address {} transaction {}",
                    swap_event.pool_address,
                    swap_event.tx_hash
                );
                continue;
            }
            Some(pair_created_event) => {
                swaps.items.push(dex_amm::Swap {
                    tx_hash: swap_event.tx_hash.clone(),
                    log_index: swap_event.log_index,
                    to: swap_event.to.clone(),
                    from: swap_event.sender.clone(),
                    block_number: block.number,
                    timestamp: block.timestamp(),
                    tokens_in: vec![
                        pair_created_event.token0.clone(),
                        pair_created_event.token1.clone(),
                    ],
                    amounts_in: vec![
                        swap_event.amount0_in.clone(),
                        swap_event.amount1_in.clone(),
                    ],
                    tokens_out: vec![
                        pair_created_event.token0.clone(),
                        pair_created_event.token1.clone(),
                    ],
                    amounts_out: vec![
                        swap_event.amount0_out.clone(),
                        swap_event.amount1_out.clone(),
                    ],
                    pool_address: pair_created_event.pair.clone(),
                })
            }
        }
    }

    Ok(swaps)
}

#[substreams::handlers::store]
fn store_deposits(deposits: dex_amm::Deposits, output: store::StoreSetRaw) {
    log::info!("Stored events {}", deposits.items.len());
    for event in deposits.items {
        let pool_key = keyer::deposit_key(&event.tx_hash, event.log_index);
        output.set(0, &pool_key, &proto::encode(&event).unwrap());
    }
}

#[substreams::handlers::store]
fn store_withdraws(withdraws: dex_amm::Withdraws, output: store::StoreSetRaw) {
    log::info!("Stored events {}", withdraws.items.len());
    for event in withdraws.items {
        let pool_key = keyer::withdraw_key(&event.tx_hash, event.log_index);
        output.set(0, &pool_key, &proto::encode(&event).unwrap());
    }
}

#[substreams::handlers::store]
fn store_swaps(swaps: dex_amm::Swaps, output: store::StoreSetRaw) {
    log::info!("Stored events {}", swaps.items.len());
    for event in swaps.items {
        let pool_key = keyer::swap_key(&event.tx_hash, event.log_index);
        output.set(0, &pool_key, &proto::encode(&event).unwrap());
    }
}

#[substreams::handlers::store]
fn store_counts(clock: Clock, pair_created_events: uniswap::PairCreatedEvents, mint_events: uniswap::MintEvents, burn_events: uniswap::BurnEvents, swap_events: uniswap::SwapEvents, s: store::StoreAddInt64) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id: String = (timestamp_seconds / 86400).to_string();
    let hour_id: String = (timestamp_seconds / 3600).to_string();

    for event in mint_events.items {
        s.add(event.log_ordinal, &keyer::usage_count_key(&"Deposit".to_string(), &"Day".to_string(), &day_id), 1);
        s.add(event.log_ordinal, &keyer::usage_count_key(&"Deposit".to_string(), &"Hour".to_string(), &hour_id), 1);
        s.add (event.log_ordinal, &keyer::transaction_count_key(&"Day".to_string(), &day_id), 1);
        s.add (event.log_ordinal, &keyer::transaction_count_key(&"Hour".to_string(), &hour_id), 1);
    }

    for event in burn_events.items {
        s.add(event.log_ordinal, &keyer::usage_count_key(&"Withdraw".to_string(), &"Day".to_string(), &day_id), 1);
        s.add(event.log_ordinal, &keyer::usage_count_key(&"Withdraw".to_string(), &"Hour".to_string(), &hour_id), 1);
        s.add (event.log_ordinal, &keyer::transaction_count_key(&"Day".to_string(), &day_id), 1);
        s.add (event.log_ordinal, &keyer::transaction_count_key(&"Hour".to_string(), &hour_id), 1);
    }

    for event in swap_events.items {
        s.add(event.log_ordinal, &keyer::usage_count_key(&"Swap".to_string(), &"Day".to_string(), &day_id), 1);
        s.add(event.log_ordinal, &keyer::usage_count_key(&"Swap".to_string(), &"Hour".to_string(), &hour_id), 1);
        s.add (event.log_ordinal, &keyer::transaction_count_key(&"Day".to_string(), &day_id), 1);
        s.add (event.log_ordinal, &keyer::transaction_count_key(&"Hour".to_string(), &hour_id), 1);
    }
}

#[substreams::handlers::store]
fn map_volumes_from_swaps(swaps: dex_amm::Swaps, s: store::StoreAddBigInt) {
    for swap in swaps.items {
        for (token_index, token_address) in swap.tokens_in.iter().enumerate() {
            let amount_bi = utils::convert_string_to_bigint(&swap.amounts_in[token_index]);
            s.add(swap.log_ordinal, &keyer::pool_input_token_amounts_key(&swap.pool_address, &swap.tokens_in[token_index]), &amount_bi);
        }
    }
}

#[substreams::handlers::map]
fn map_pools(
    block: eth::Block,
    pair_created_events: uniswap::PairCreatedEvents,
) -> Result<dex_amm::Pools, substreams::errors::Error> {
    let mut pools = dex_amm::Pools { items: vec![] };

    for event in pair_created_events.items {
        let token0 = erc20_helper::get_erc20_token(event.token0.clone());
        let token1 = erc20_helper::get_erc20_token(event.token1.clone());

        if let (Some(token0), Some(token1)) = (token0, token1) {
            pools.items.push(dex_amm::Pool {
                address: event.pair.clone(),
                name: format!("Uniswap LP: {} / {}", token0.symbol, token1.symbol),
                created_timestamp: block.timestamp(),
                created_block_number: block.number,
                input_tokens: vec![token0.address, token1.address],
                input_token_amounts: vec!["0".to_string(),"0".to_string()],
                output_token: event.pair,
                output_token_supply: "0".to_string(),
                total_value_locked: "0".to_string(),
            })
        } else {
            log::info!(
                "Failed to fetch token for {} {}",
                Hex(&event.token0),
                Hex(&event.token1)
            );
        }
    }

    Ok(pools)
}

#[substreams::handlers::store]
fn store_pools(pools: dex_amm::Pools, store: store::StoreSetProto<dex_amm::Pool>) {
    log::info!("Stored pools {}", pools.items.len());
    for event in pools.items {
        let pool_key = keyer::pool_key(&event.address);
        store.set(0, &pool_key, &event);
    }
}
