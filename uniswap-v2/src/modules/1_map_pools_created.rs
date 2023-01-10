use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::abi::factory;
use crate::pb::uniswap::v2::{Pool, Pools};
use crate::pool_retriever::PoolRetriever;

#[substreams::handlers::map]
pub fn map_pools_created(block: eth::Block) -> Result<Pools, Error> {
    let mut pools = vec![];

    for log in block.logs() {
        if let Some(event) = factory::events::PairCreated::match_and_decode(log) {
            let pool_address = event.pair;
            let pool_retriever = PoolRetriever::new(&pool_address);

            pools.push(Pool {
                name: pool_retriever.get_name(),
                address: pool_retriever.get_address(),
                symbol: pool_retriever.get_symbol(),
                input_tokens: pool_retriever.get_input_tokens(),
                output_token: pool_retriever.get_output_token(),
                created_timestamp: block.timestamp_seconds(),
                created_block_number: block.number,
            });
        }
    }

    Ok(Pools { pools })
}
