use substreams::prelude::*;
use substreams::store::{StoreSetProto};
use substreams::{log, Hex};

use crate::pb::uniswap::v3::{DataSource, DataSources};
use crate::keyer::{get_data_source_key};

#[substreams::handlers::store]
pub fn store_data_sources(data_sources: DataSources, data_sources_store: StoreSetProto<DataSource>) {
    for data_source in data_sources.data_sources {
        data_sources_store.set(0, get_data_source_key(&data_source.address), &data_source);
    }
}
