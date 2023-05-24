use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;
use substreams::prelude::*;
use substreams::store::{StoreSetProto};

use crate::abi::factory as FactoryContract;
use crate::pb::dex_amm::v3_0_3::{DataSource, DataSources, DataSourceType};
use crate::utils::{UNISWAP_V3_FACTORY_SLICE, NFT_POSITION_MANAGER_SLICE};

use crate::keyer::{get_data_source_key};

#[substreams::handlers::map]
pub fn map_data_sources(block: eth::Block) -> Result<DataSources, Error> {
    let mut data_sources = vec![];

    // Fix so does not store multiple times.
    data_sources.push(
        DataSource {
            data_source_type: DataSourceType::UniswapV3Factory as i32,
            address: UNISWAP_V3_FACTORY_SLICE.to_vec(),
        }
    );

    data_sources.push(
        DataSource {
            data_source_type: DataSourceType::NftPositionManager as i32,
            address: NFT_POSITION_MANAGER_SLICE.to_vec(),
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

#[substreams::handlers::store]
pub fn store_data_sources(data_sources: DataSources, data_sources_store: StoreSetProto<DataSource>) {
    for data_source in data_sources.data_sources {
        data_sources_store.set(0, get_data_source_key(&data_source.address), &data_source);
    }
}

