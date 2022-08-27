use hex_literal::hex;
use substreams::{log, proto, store};
use substreams_ethereum::{Event as EventTrait, pb::eth::v2 as eth};
use substreams_helper::{erc20, utils};

use abi::factory;

use pb::common;
use pb::dex_amm;
use pb::uniswap_v2;

mod abi;
mod pb;

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
fn map_pair_created_event(block: eth::Block) -> Result<uniswap_v2::PairCreatedEvents, substreams::errors::Error> {
    let mut pair_created_events = uniswap_v2::PairCreatedEvents { items: vec![] };

    for log in block.logs() {
        if let Some(event) = factory::events::PairCreated::match_and_decode(log) {
            if log.log.address != UNISWAP_V2_FACTORY {
                continue;
            }

            pair_created_events.items.push(uniswap_v2::PairCreatedEvent {
                token0: hex::encode(event.token0),
                token1: hex::encode(event.token1),
                pair: hex::encode(event.pair),
                block_number: block.number,
                block_timestamp: block.timestamp(),
            })
        }
    }

    Ok(pair_created_events)
}

#[substreams::handlers::store]
fn store_pair_created_event(pair_created_events: uniswap_v2::PairCreatedEvents, output: store::StoreSet) {
    log::info!("Stored pairs {}", pair_created_events.items.len());
    for event in pair_created_events.items {
        output.set(
            0,
            &event.pair,
            &proto::encode(&event).unwrap(),
        );
    }
}

#[substreams::handlers::map]
fn map_pool(pair_created_events: uniswap_v2::PairCreatedEvents) -> Result<dex_amm::Pools, substreams::errors::Error> {
    let mut pools = dex_amm::Pools { items: vec![] };

    for event in pair_created_events.items {
        match erc20::get_erc20_token(&event.token0) {
            None => {
                continue;
            }
            Some(token) => {
                log::info!("token name: {}", token.name);
            }
        }

        pools.items.push(dex_amm::Pool {
            name: event.pair.clone(),
            token0: event.token0,
            token1: event.token1,
            address: event.pair.clone(),
        })
    }

    Ok(pools)
}
