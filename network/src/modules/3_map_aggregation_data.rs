use substreams::pb::substreams::store_delta::Operation;
use substreams::pb::substreams::StoreDelta;
use substreams::scalar::BigInt;
use substreams::store;
use substreams::store::StoreGetBigInt;
use substreams::store::{DeltaBytes, DeltaInt64, StoreGet};
use substreams_ethereum::pb::eth::v2::{self as eth};

use crate::block_handler::BlockHandler;
use crate::pb::aggregate_data::{AggregateData, PreCalculatedAggregates};
use crate::store_key::StoreKey;
use crate::store_retriever::StoreRetriever;

#[substreams::handlers::map]
pub fn map_aggregation_data(block: eth::Block, buggy_deltas: store::Deltas<DeltaBytes>, pre_aggregation_store: store::StoreGetBigInt) -> Result<AggregateData, ()> {
    let unique_deltas = get_non_buggy_deltas(buggy_deltas);

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
        hourly_aggregated_data: None,
    };

    let mut new_unique_authors = 0;
    for unique_delta in unique_deltas.deltas.into_iter() {
        if unique_delta.key == "latest_day_timestamp".to_string() && unique_delta.operation != Operation::Create && unique_delta.new_value==unique_delta.old_value+1 {
            if store_retriever.day_timestamp_is_not_set() {
                store_retriever.set_day_timestamp(unique_delta.old_value);
            }

            aggregation_data.daily_aggregated_data = Some(PreCalculatedAggregates {
                timestamp: unique_delta.old_value.to_string(),
                blocks: Some(store_retriever.get_day_sum(StoreKey::Blocks).into()),
                unique_authors: Some(store_retriever.get_day_sum(StoreKey::UniqueAuthors).into()),
                supply: Some(store_retriever.get_day_sum(StoreKey::Supply).into()),
                transactions: Some(store_retriever.get_day_sum(StoreKey::Transactions).into()),
            });
        } else if unique_delta.key == "latest_hour_timestamp".to_string() && unique_delta.operation != Operation::Create && unique_delta.new_value==unique_delta.old_value+1 {
            if store_retriever.hour_timestamp_is_not_set() {
                store_retriever.set_hour_timestamp(unique_delta.old_value);
            }

            aggregation_data.hourly_aggregated_data = Some(PreCalculatedAggregates {
                timestamp: unique_delta.old_value.to_string(),
                blocks: Some(store_retriever.get_hour_sum(StoreKey::Blocks).into()),
                unique_authors: Some(store_retriever.get_hour_sum(StoreKey::UniqueAuthors).into()),
                supply: Some(store_retriever.get_hour_sum(StoreKey::Supply).into()),
                transactions: Some(store_retriever.get_hour_sum(StoreKey::Transactions).into()),
            });
        } else if unique_delta.key == "block_timestamp".to_string() && unique_delta.operation != Operation::Create {
            aggregation_data.block_interval = Some(BigInt::from(unique_delta.new_value - unique_delta.old_value).into());
        } else if unique_delta.key.starts_with("t") && unique_delta.operation == Operation::Create {
            new_unique_authors += 1;
        }
    }

    aggregation_data.new_unique_authors = Some(BigInt::from(new_unique_authors).into());

    if aggregation_data.block_interval.is_none() {
        aggregation_data.block_interval = Some(BigInt::zero().into())
    }

    Ok(aggregation_data)
}

fn get_non_buggy_deltas(buggy_deltas: store::Deltas<DeltaBytes>) -> store::Deltas<DeltaInt64> {
    store::Deltas::<DeltaInt64>::new(
        buggy_deltas
            .deltas
            .into_iter()
            .filter_map(|delta| {
                let old_value = if delta.operation == Operation::Create {
                    0_i64.to_string().into_bytes()
                } else {
                    if delta.new_value == delta.old_value {
                        // THEN IT'S NOT ACTUALLY A DELTA!!!!!
                        return None;
                    }

                    delta.old_value
                };

                Some(StoreDelta {
                    operation: delta.operation as i32,
                    ordinal: delta.ordinal,
                    key: delta.key,
                    old_value,
                    new_value: delta.new_value,
                })
            })
            .collect(),
    )
}
