#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

mod keyer;

use hex_literal::hex;
use substreams::store::{StoreAddBigFloat, StoreAddBigInt, StoreAppend, StoreGet, StoreSet};
use substreams::{log, proto, store, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};
use substreams_helper::erc20;
use substreams_helper::types::Address;

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
                tokens: vec![hex::encode(event.token0.clone()), hex::encode(event.token1.clone())],
                pair: hex::encode(event.pair.clone()),
            })
        }
    }

    Ok(pair_created_events)
}

#[substreams::handlers::store]
fn store_pair_created_events(
    pair_created_events: uniswap::PairCreatedEvents,
    output: store::StoreSet,
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
fn store_pools(pools: dex_amm::Pools, output: StoreSet) {
    log::info!("Stored pools {}", pools.items.len());
    for event in pools.items {
        let pool_key = keyer::pool_key(&event.address);
        output.set(0, &pool_key, &proto::encode(&event).unwrap());
    }
}

#[substreams::handlers::map]
fn map_mint_events(
    block: eth::Block,
    pools_store: StoreGet,
) -> Result<uniswap::MintEvents, substreams::errors::Error> {
    let mut mint_events = uniswap::MintEvents { items: vec![] };

    for log in block.logs() {
        let pool_address = Hex(&log.address()).to_string();
        let pool_key = keyer::pool_key(&pool_address);
        let tx_hash = Hex(&log.receipt.transaction.hash).to_string();

        if let Some(event) = pair::events::Mint::match_and_decode(log) {
            // Check if pool has been created
            if pools_store.get_last(pool_key).is_none() {
                log::info!(
                    "invalid mint. pool does not exist. pool address {} transaction {}",
                    pool_address,
                    tx_hash
                );
                continue;
            }

            mint_events.items.push(uniswap::MintEvent {
                tx_hash: tx_hash,
                log_index: log.index(),
                log_ordinal: log.ordinal(),
                block_number: block.number,
                timestamp: block.timestamp(),
                sender: hex::encode(event.sender.clone()),
                amount0: event.amount0.to_string(),
                amount1: event.amount1.to_string(),
                pool: pool_address,
            })
        }
    }

    Ok(mint_events)
}

#[substreams::handlers::map]
fn map_burn_events(
    block: eth::Block,
    pools_store: StoreGet,
) -> Result<uniswap::BurnEvents, substreams::errors::Error> {
    let mut burn_events = uniswap::BurnEvents { items: vec![] };

    for log in block.logs() {
        let pool_address = Hex(&log.address()).to_string();
        let pool_key = keyer::pool_key(&pool_address);
        let tx_hash = Hex(&log.receipt.transaction.hash).to_string();

        if let Some(event) = pair::events::Burn::match_and_decode(log) {
            // Check if pool has been created
            if pools_store.get_last(pool_key).is_none() {
                log::info!(
                    "invalid burn. pool does not exist. pool address {} transaction {}",
                    pool_address,
                    tx_hash
                );
                continue;
            }

            burn_events.items.push(uniswap::BurnEvent {
                tx_hash: tx_hash,
                log_index: log.index(),
                log_ordinal: log.ordinal(),
                block_number: block.number,
                timestamp: block.timestamp(),
                sender: hex::encode(event.sender.clone()),
                amount0: event.amount0.to_string(),
                amount1: event.amount1.to_string(),
                pool: pool_address,
            })
        }
    }

    Ok(burn_events)
}

#[substreams::handlers::map]
fn map_swap_events(
    block: eth::Block,
    pools_store: StoreGet,
) -> Result<uniswap::SwapEvents, substreams::errors::Error> {
    let mut swap_events = uniswap::SwapEvents { items: vec![] };

    for log in block.logs() {
        if let Some(event) = pair::events::Swap::match_and_decode(log) {
            let pool_address = Hex(&log.address()).to_string();
            let pool_key = keyer::pool_key(&pool_address);
            let tx_hash = Hex(&log.receipt.transaction.hash).to_string();

            // Check if pool has been created
            match pools_store.get_last(pool_key) {
                None => {
                    log::info!(
                        "invalid swap. pool does not exist. pool address {} transaction {}",
                        pool_address,
                        tx_hash
                    );
                    continue;
                }
                Some(pool) => {
                    swap_events.items.push(uniswap::SwapEvent {
                        tx_hash: tx_hash,
                        log_index: log.index(),
                        log_ordinal: log.ordinal(),
                        block_number: block.number,
                        timestamp: block.timestamp(),
                        sender: hex::encode(event.sender.clone()),
                        tokens: pool.tokens,
                        amounts_in: vec![event.amount0_in.to_string(), event.amount1_in.to_string()],
                        amounts_out: vec![event.amount0_out.to_string(), event.amount1_out.to_string()],
                        pool: pool_address,
                    })
                }
            }
        }
    }

    Ok(swap_events)
}
