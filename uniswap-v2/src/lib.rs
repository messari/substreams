#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

mod keyer;
mod utils;

use hex_literal::hex;

use substreams::store::StoreAdd;
use substreams::store::StoreNew;
use substreams::store::StoreSetRaw;
use substreams::store::{StoreAddInt64, StoreAddBigInt, StoreGet, StoreSet, StoreGetProto, StoreSetProto};
use substreams::{log, proto, store, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};
use substreams_helper::erc20 as erc20_helper;
use substreams_helper::types::Address;
use substreams::pb::substreams::Clock;


use abi::factory;
use abi::pair;

use pb::dex_amm::v1 as dex_amm;
use pb::uniswap::v2 as uniswap;
use crate::pb::dex_amm::v1::usage_event::Type::{Swap as SwapType, Deposit as DepositType, Withdraw as WithdrawType};

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
fn map_usage_events(
    block: eth::Block,
    swap_events: uniswap::SwapEvents,
    mint_events: uniswap::MintEvents,
    burn_events: uniswap::BurnEvents,
    pair_created_events_store: StoreGetProto<uniswap::PairCreatedEvent>,
) -> Result<dex_amm::UsageEvents, substreams::errors::Error> {
    let mut usage_events = dex_amm::UsageEvents { items: vec![] };

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
                usage_events.items.push(dex_amm::UsageEvent {
                    tx_hash: swap_event.tx_hash.clone(),
                    log_index: swap_event.log_index,
                    log_ordinal: swap_event.log_ordinal,
                    to: swap_event.to.clone(),
                    from: swap_event.sender.clone(),
                    block_number: block.number,
                    timestamp: block.timestamp(),
                    r#type: Some(SwapType(dex_amm::Swap {
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
                    })),
                })
            }
        }
    }

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
                usage_events.items.push(dex_amm::UsageEvent {
                    tx_hash: mint_event.tx_hash.clone(),
                    log_index: mint_event.log_index,
                    log_ordinal: mint_event.log_ordinal,
                    to: pair_created_event.pair.clone(),
                    from: mint_event.sender.clone(),
                    block_number: block.number,
                    timestamp: block.timestamp(),
                    r#type: Some(DepositType(dex_amm::Deposit {
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
                    })),
                })
            }
        }
    }

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
                usage_events.items.push(dex_amm::UsageEvent {
                    tx_hash: burn_event.tx_hash.clone(),
                    log_index: burn_event.log_index,
                    log_ordinal: burn_event.log_ordinal,
                    to: burn_event.to.clone(),
                    from: burn_event.sender.clone(),
                    block_number: block.number,
                    timestamp: block.timestamp(),
                    r#type: Some(WithdrawType(dex_amm::Withdraw {
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
                    })),

                })
            }
        }
    }

    Ok(usage_events)
}

#[substreams::handlers::store]
fn store_usage_events(usage_events: dex_amm::UsageEvents, usage_event_store: StoreSetProto<dex_amm::UsageEvent>) {
    log::info!("Stored usage events {}", usage_events.items.len());

    for event in usage_events.items {
        // get the string representation of the type of the event
        let event_type = match &event.r#type {
            Some(SwapType(_)) => "swap",
            Some(DepositType(_)) => "deposit",
            Some(WithdrawType(_)) => "withdraw",
            None => "unknown",
        }.to_string();

        usage_event_store.set(event.log_ordinal, &keyer::usage_event_key(&event_type, &event.tx_hash, event.log_index), &event);
    }
}

#[substreams::handlers::store]
fn store_usage_counts(clock: Clock, usage_events: dex_amm::UsageEvents, s: store::StoreAddInt64) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id: String = (timestamp_seconds / 86400).to_string();
    let hour_id: String = (timestamp_seconds / 3600).to_string();

    for event in usage_events.items {
        // get the string representation of the type of the event
        let event_type = match &event.r#type {
            Some(SwapType(_)) => "swap",
            Some(DepositType(_)) => "deposit",
            Some(WithdrawType(_)) => "withdraw",
            None => "unknown",
        }.to_string();

        s.add(event.log_ordinal, &keyer::usage_count_key(&event_type, &"Day".to_string(), &day_id), 1);
        s.add(event.log_ordinal, &keyer::usage_count_key(&event_type, &"Hour".to_string(), &hour_id), 1);
    }
}

#[substreams::handlers::store]
fn store_volumes_from_swaps(swaps: dex_amm::Swaps, s: store::StoreAddBigInt) {
    for swap in swaps.items {
        for (token_index, token_address) in swap.tokens_in.iter().enumerate() {
            let amount_bi = utils::convert_string_to_bigint(&swap.amounts_in[token_index]);
            s.add(0, &keyer::pool_input_token_amounts_key(&swap.pool_address, &token_address), &amount_bi);
        }
    }
}

#[substreams::handlers::map]
pub fn map_pool_deltas(events: dex_amm::UsageEvents) -> Result<uniswap::PoolTokenTvlDeltas, substreams::errors::Error> {
    let mut pool_token_tvl_deltas = uniswap::PoolTokenTvlDeltas { items: vec![] };

    for event in events.items {
        log::debug!("trx_id: {}", event.tx_hash);

        match event.r#type.unwrap() {
            WithdrawType(withdraw) => {
                pool_token_tvl_deltas.items.push(uniswap::PoolTokenTvlDelta {
                    log_ordinal: event.log_ordinal,
                    pool_address: withdraw.pool_address.clone(),
                    token_addresses: withdraw.input_tokens.clone(),
                    deltas: utils::negative_bi_array(withdraw.input_token_amounts.clone()),
                });
            }
            DepositType(deposit) => {
                pool_token_tvl_deltas.items.push(uniswap::PoolTokenTvlDelta {
                    log_ordinal: event.log_ordinal,
                    pool_address: deposit.pool_address.clone(),
                    token_addresses: deposit.input_tokens.clone(),
                    deltas: deposit.input_token_amounts.clone(),
                });
            }
            SwapType(swap) => {
                pool_token_tvl_deltas.items.push(uniswap::PoolTokenTvlDelta {
                    log_ordinal: event.log_ordinal,
                    pool_address: swap.pool_address.clone(),
                    token_addresses: swap.tokens_in.clone(),
                    deltas: utils::get_delta_bi_array(&swap.amounts_in, &swap.amounts_out),
                });
            }
        }
    }

    Ok(pool_token_tvl_deltas)
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
