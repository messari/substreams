use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::abi::factory as FactoryContract;
use crate::pb::uniswap::v2::Pools;
use crate::utils::UNISWAP_V2_FACTORY_SLICE;
use crate::PoolContract;

#[substreams::handlers::map]
pub fn map_pool_created(block: eth::Block) -> Result<Pools, Error> {
    let mut pools = vec![];

    for log in block.logs() {
        if let Some(event) = FactoryContract::events::PairCreated::match_and_decode(log) {
            if log.address().ne(&UNISWAP_V2_FACTORY_SLICE) {
                continue;
            }

            let pool_address = event.pair;

            pools.push(PoolContract::new(pool_address).as_struct(&block));
        }
    }

    Ok(Pools { pools })
}
