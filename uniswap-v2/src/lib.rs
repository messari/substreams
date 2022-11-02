#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

mod keyer;

use hex_literal::hex;
use substreams::store::StoreAdd;
use substreams::store::StoreNew;
use substreams::store::StoreSetRaw;
use substreams::store::{StoreAddInt64, StoreGet, StoreGetRaw, StoreSet};
use substreams::{log, proto, store, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};
use substreams_helper::erc20;
use substreams_helper::types::Address;

use abi::factory;
use abi::pair;

use pb::dex_amm::v1 as dex_amm;
use pb::uniswap::v2 as uniswap;
use pb::uniswap::v2::event::Type::{Mint as MintEvent, Burn as BurnEvent, Swap as SwapEvent};

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
        output.set(0, Hex::encode(&event.pair), &proto::encode(&event).unwrap());
    }
}

#[substreams::handlers::map]
fn map_pools(
    pair_created_events: uniswap::PairCreatedEvents,
) -> Result<dex_amm::Pools, substreams::errors::Error> {
    let mut pools = dex_amm::Pools { items: vec![] };

    for event in pair_created_events.items {
        let token0 = erc20::get_erc20_token(event.token0.clone());
        let token1 = erc20::get_erc20_token(event.token1.clone());

        if let (Some(token0), Some(token1)) = (token0, token1) {
            pools.items.push(dex_amm::Pool {
                name: format!("Uniswap LP: {} / {}", token0.symbol, token1.symbol),
                address: event.pair,
                input_tokens: vec![],
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
fn store_pools(pools: dex_amm::Pools, output: store::StoreSetRaw) {
    log::info!("Stored pools {}", pools.items.len());
    for event in pools.items {
        let pool_key = keyer::pool_key(&event.address);
        output.set(0, &pool_key, &proto::encode(&event).unwrap());
    }
}

struct MintSwapBurnPairCreatedEvents {
    mint_swap_burn_events: uniswap::Events,
    pair_create_events: uniswap::PairCreatedEvents,
}

#[substreams::handlers::map]
fn map_events(
    block: eth::Block,
    pools_store: StoreGetRaw,
) -> Result<uniswap::MintSwapBurnPairCreatedEvents, substreams::errors::Error> {
    let mut events = uniswap::Events { items: vec![] };
    let mut pair_created_events = uniswap::PairCreatedEvents { items: vec![] };

    for log in block.logs() {
        let pool_address = Hex(&log.address()).to_string();
        let pool_key = keyer::pool_key(&pool_address);
        let tx_hash = Hex(&log.receipt.transaction.hash).to_string();

        if let Some(event) = pair::events::Swap::match_and_decode(log) {
            match pools_store.get_last(pool_key) {
                None => {
                    log::info!(
                        "invalid swap. pool does not exist. pool address {} transaction {}",
                        Hex(&log.address()).to_string(),
                        Hex(&log.receipt.transaction.hash).to_string()
                    );
                    continue;
                }
                Some(pool) => {
                    events.items.push(uniswap::Event {
                        log_ordinal: log.ordinal(),
                        log_index: log.block_index() as u64,
                        pool_address: pool_address.to_string(),
                        tx_hash: Hex(&log.receipt.transaction.hash).to_string(),
                        timestamp: block.timestamp_seconds(),
                        created_at_block_number: block.number,
                        r#type: Some(SwapEvent(uniswap::SwapEvent {
                            sender: hex::encode(event.sender.clone()),
                            amount0_in: event.amount0_in.to_string(),
                            amount1_in: event.amount1_in.to_string(),
                            amount0_out: event.amount0_out.to_string(),
                            amount1_out: event.amount1_out.to_string(),
                            to: hex::encode(event.to.clone()),
                        }))
                    })
                }
            }
        } else if let Some(event) = pair::events::Mint::match_and_decode(log) {
            match pools_store.get_last(pool_key) {
                None => {
                    log::info!(
                        "invalid mint. pool does not exist. pool address {} transaction {}",
                        Hex(&log.address()).to_string(),
                        Hex(&log.receipt.transaction.hash).to_string()
                    );
                    continue;
                }
                Some(pool) => {
                    events.items.push(uniswap::Event {
                        log_ordinal: log.ordinal(),
                        log_index: log.block_index() as u64,
                        pool_address: pool_address.to_string(),
                        tx_hash: Hex(&log.receipt.transaction.hash).to_string(),
                        timestamp: block.timestamp_seconds(),
                        created_at_block_number: block.number,
                        r#type: Some(MintEvent(uniswap::MintEvent {
                            sender: hex::encode(event.sender.clone()),
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                        }))
                    })
                }
            }
        } else if let Some(event) = pair::events::Burn::match_and_decode(log) {
            match pools_store.get_last(pool_key) {
                None => {
                    log::info!(
                        "invalid burn. pool does not exist. pool address {} transaction {}",
                        Hex(&log.address()).to_string(),
                        Hex(&log.receipt.transaction.hash).to_string()
                    );
                    continue;
                }
                Some(pool) => {
                    events.items.push(uniswap::Event {
                        log_ordinal: log.ordinal(),
                        log_index: log.block_index() as u64,
                        pool_address: pool_address.to_string(),
                        tx_hash: Hex(&log.receipt.transaction.hash).to_string(),
                        timestamp: block.timestamp_seconds(),
                        created_at_block_number: block.number,
                        r#type: Some(BurnEvent(uniswap::BurnEvent {
                            sender: hex::encode(event.sender.clone()),
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                            to: hex::encode(event.to.clone()),
                        }))
                    })
                }
            }
        } else if let Some(event) = factory::events::PairCreated::match_and_decode(log) {
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

    let MintSwapBurnPairCreatedEvents: MintSwapBurnPairCreatedEvents = MintSwapBurnPairCreatedEvents {
        mint_swap_burn_events: events,
        pair_create_events: pair_created_events,
    };

    Ok(MintSwapBurnPairCreatedEvents)
}

// #[substreams::handlers::store]
// fn store_swap_events(swap_events: uniswap::SwapEvents, output: store::StoreSetRaw) {
//     log::info!("Stored events {}", swap_events.items.len());
//     for event in swap_events.items {
//         let pool_key = keyer::swap_key(&event.tx_hash, event.log_index);
//         output.set(0, &pool_key, &proto::encode(&event).unwrap());
//     }
// }
//
// #[substreams::handlers::store]
// fn store_swap_count(swap_events: uniswap::SwapEvents, s: store::StoreAddInt64) {
//     s.add(0, "total", swap_events.items.len() as i64);
// }
