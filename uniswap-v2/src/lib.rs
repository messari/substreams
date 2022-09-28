#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

use hex_literal::hex;
use substreams::{log, proto, store, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};
use substreams_helper::erc20;

use abi::factory;

use pb::dex_amm::v1 as dex_amm;
use pb::uniswap::v2 as uniswap;

type Address = [u8; 20];

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
fn map_pair_created_event(
    block: eth::Block,
) -> Result<uniswap::PairCreatedEvents, substreams::errors::Error> {
    let mut pair_created_events = uniswap::PairCreatedEvents { items: vec![] };

    for log in block.logs() {
        if let Some(event) = factory::events::PairCreated::match_and_decode(log) {
            if log.log.address != UNISWAP_V2_FACTORY {
                continue;
            }

            pair_created_events.items.push(uniswap::PairCreatedEvent {
                token0: hex::encode(event.token0.clone()),
                token1: hex::encode(event.token1.clone()),
                pair: hex::encode(event.pair.clone()),
                tx_hash: hex::encode(log.receipt.transaction.clone().hash),
                log_index: log.index(),
            })
        }
    }

    Ok(pair_created_events)
}

#[substreams::handlers::store]
fn store_pair_created_event(
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
fn store_pools(pools: dex_amm::Pools, output: store::StoreSet) {
    log::info!("Stored pools {}", pools.items.len());
    for event in pools.items {
        output.set(0, &event.address, &proto::encode(&event).unwrap());
    }
}
