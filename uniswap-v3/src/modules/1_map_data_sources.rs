use substreams::prelude::*;
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::abi::factory as FactoryContract;
use crate::pb::uniswap::v3::{DataSources, DataSource, DataSourceType};
use crate::utils::UNISWAP_V3_FACTORY_SLICE;

#[substreams::handlers::map]
pub fn map_data_sources(block: eth::Block) -> Result<DataSources, Error> {
    let mut data_sources = vec![];

    data_sources.push(
        DataSource {
            data_source_type: DataSourceType::UniswapV3Factory as i32,
            address: UNISWAP_V3_FACTORY_SLICE.to_vec(),
        }
    );

    for log in block.logs() {
        if let Some(event) = FactoryContract::events::PoolCreated::match_and_decode(log) {
            if log.address().ne(&UNISWAP_V3_FACTORY_SLICE) {
                continue;
            }
            data_sources.push(
                DataSource {
                    data_source_type: DataSourceType::UniswapV3Pool as i32,
                    address: event.pool.clone(),
                }
            );
        }
    }

    Ok(DataSources { data_sources })
}
