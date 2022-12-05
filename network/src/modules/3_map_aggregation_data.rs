use substreams::pb::substreams::store_delta::Operation;
use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams::store;
use substreams::store::{DeltaBytes, DeltaI64, DeltaString, StoreAdd, StoreGet};

use crate::block_handler::BlockHandler;
use crate::pb::aggregate_data::{AggregateData, self, PreCalculatedAggregates};
use crate::store_key::StoreKey;
use crate::store_retriever::StoreRetriever;
use crate::utils::BigIntSerializeExt;

#[substreams::handlers::map]
pub fn map_aggregation_data(block: eth::Block, unique_deltas: store::Deltas<DeltaI64>, pre_aggregation_store: store::StoreGetBigInt) -> AggregateData {
    let block_handler = BlockHandler::new(&block);
    let mut store_retriever = StoreRetriever::new(&pre_aggregation_store, None, None);

    let mut aggregation_data = AggregateData {
        timestamp: block_handler.timestamp(),
        new_unique_authors: None,
        difficulty: Some(block_handler.difficulty().into()),
        gas_used: Some(block_handler.gas_used().into()),
        gas_limit: Some(block_handler.gas_limit().into()),
        burnt_fees: Some(block_handler.burnt_fees().into()),
        rewards: Some(block_handler.rewards().into()),
        block_size: Some(block_handler.block_size().into()),
        chunks: None, // Only tracked for NEAR protocol
        supply: Some(block_handler.supply().into()),
        transactions: Some(block_handler.transactions().into()),
        gas_price: Some(block_handler.gas_price().into()),
        block_interval: None,
        daily_aggregated_data: None,
        hourly_aggregated_data: None
    };

    let mut new_unique_authors = 0;
    for unique_delta in unique_deltas.deltas.into_iter() {
        if unique_delta.key == "latest_day_timestamp".to_string() && unique_delta.operation != Operation::Create {
            if store_retriever.day_timestamp_is_not_set() {
                store_retriever.set_day_timestamp(unique_delta.old_value);
            }

            aggregation_data.daily_aggregated_data = Some(PreCalculatedAggregates {
                timestamp: unique_delta.old_value.to_string(),
                blocks: Some(store_retriever.get_day_sum(StoreKey::Blocks).into()),
                unique_authors: Some(store_retriever.get_day_sum(StoreKey::UniqueAuthors).into()),
                supply: Some(store_retriever.get_day_sum(StoreKey::Supply).into()),
                transactions: Some(store_retriever.get_day_sum(StoreKey::Transactions).into())
            });
        } else if unique_delta.key == "latest_hour_timestamp".to_string() && unique_delta.operation != Operation::Create {
            if store_retriever.hour_timestamp_is_not_set() {
                store_retriever.set_hour_timestamp(unique_delta.old_value);
            }

            aggregation_data.hourly_aggregated_data = Some(PreCalculatedAggregates {
                timestamp: unique_delta.old_value.to_string(),
                blocks: Some(store_retriever.get_hour_sum(StoreKey::Blocks).into()),
                unique_authors: Some(store_retriever.get_hour_sum(StoreKey::UniqueAuthors).into()),
                supply: Some(store_retriever.get_hour_sum(StoreKey::Supply).into()),
                transactions: Some(store_retriever.get_hour_sum(StoreKey::Transactions).into())
            });
        } else if unique_delta.key == "block_timestamp".to_string() && unique_delta.operation != Operation::Create {
            aggregation_data.block_interval = Some(BigInt::from(unique_delta.new_value-unique_delta.old_value).into());
        } else if unique_delta.key.starts_with("t") && unique_delta.old_value == 0 {
            new_unique_authors += 1;
        }
    }

    if new_unique_authors > 0 {
        aggregation_data.new_unique_authors = Some(BigInt::from(new_unique_authors).into());
    }

    aggregation_data
}