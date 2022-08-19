mod abi;
mod pb;

use hex;
use pb::uniswap_v2;
use abi::factory;
use substreams::{log, store};
use substreams_ethereum::{pb::eth::v1 as eth, Event as EventTrait};

substreams_ethereum::init!();

pub const UNISWAP_V2_FACTORY: &str = "5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f";
pub const UNISWAP_V2_FACTORY_START_BLOCK: u64 = 10000835;

fn is_pair_created_event(sig: &str) -> bool {
    /* keccak value for PoolCreated(address,address,uint24,int24,address) */
    return sig == "783cca1c0412dd0d695e784568c96da2e9c22ff989357a2e8b1d9b2b4e6b7118";
}

#[substreams::handlers::map]
fn block_to_pairs(block: eth::Block) -> Result<uniswap_v2::Pairs, substreams::errors::Error> {
    let mut pairs = uniswap_v2::Pairs { pairs: vec![] };

    for log in block.logs() {
        if let Some(_event) = factory::events::PairCreated::match_and_decode(log) {
            // Uniswap v2 Factory
            if hex::encode(&log.log.address) != UNISWAP_V2_FACTORY {
                continue;
            }

            log::info!("matched");

            let sig = hex::encode(&log.log.topics[0]);

            if !is_pair_created_event(sig.as_str()) {
                continue;
            }

            pairs.pairs.push(pb::uniswap_v2::Pair {
                name: "name".to_string(),
                address: "address".to_string(),
                token0: "token0".to_string(),
                token1: "token1".to_string(),
            })
        }
    }

    Ok(pairs)
}

#[substreams::handlers::store]
fn store_pairs(_pairs: uniswap_v2::Pairs, _s: store::StoreAddInt64) {
    log::info!("Stored pairs {}", _pairs.pairs.len());
}
