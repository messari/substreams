use prost::Message;
use substreams::scalar::BigInt;
use substreams::store;
use substreams::store::{DeltaBigInt, DeltaI64};
use substreams_entity_change::change::ToField;
use substreams_entity_change::pb::entity::entity_change::Operation;
use substreams_entity_change::pb::entity::value::Typed;
use substreams_entity_change::pb::entity::{EntityChange, EntityChanges, Field, Value};
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams::store::StoreGetBigInt;

use crate::block_handler::BlockHandler;
use crate::pb::aggregate_data::AggregateData;
use crate::pb::network::v1::Stats;
use crate::stats_retriever::StatsRetriever;
use crate::store_key::StoreKey;
use crate::store_retriever::StoreRetriever;
use crate::utils::{get_latest_day, get_latest_hour, i64_to_str};

#[substreams::handlers::map]
pub fn map_entity_changes(
    block: eth::Block,
    aggregation_deltas: store::Deltas<DeltaBigInt>,
    aggregation_store: StoreGetBigInt,
    max_store: StoreGetBigInt,
    min_store: StoreGetBigInt,
    non_aggregation_deltas: store::Deltas<DeltaBigInt>,
    non_aggregation_store: StoreGetBigInt,
) -> EntityChanges {
    let block_handler = BlockHandler::new(&block);
    let timestamp = block_handler.timestamp();
    let mut stats_retriever = StatsRetriever::new(&aggregation_store, &max_store, &min_store, timestamp);

    let (network_entity, is_new_day, is_new_hour) = get_network_entity_change(&mut stats_retriever, &non_aggregation_deltas, &aggregation_deltas);

    let mut entity_changes = vec![network_entity];

    if is_new_day {
        entity_changes.push(create_daily_snapshot_entity_change(timestamp, &mut stats_retriever))
    }

    if is_new_hour {
        entity_changes.push(create_hourly_snapshot_entity_change(timestamp, &mut stats_retriever))
    }

    EntityChanges { entity_changes }
}

/// Returns network entity change and also if it is the time to create a day or hour snapshot. Return is in form => (network_entity_change, create_day_snapshot, create_hour_snapshot)
fn get_network_entity_change(
    stats_retriever: &mut StatsRetriever,
    non_aggregation_deltas: &store::Deltas<DeltaBigInt>,
    aggregation_deltas: &store::Deltas<DeltaBigInt>,
) -> (EntityChange, bool, bool) {
    const ENTITY_ID: &str = "MAINNET";
    stats_retriever.set_entity_id(ENTITY_ID.to_string());
    let mut network_entity_change = EntityChange::new("network", ENTITY_ID, 0, Operation::Update);

    for non_aggregation_delta in non_aggregation_deltas.deltas.into_iter() {
        if non_aggregation_delta.key == StoreKey::GasLimit.get_unique_id() {
            network_entity_change.change("gas_limit", non_aggregation_delta)
        }
    }

    let mut is_first_block = false;
    let mut is_new_day = false;
    let mut is_new_hour = false;
    for store_delta in aggregation_deltas.deltas.into_iter() {
        match &store_delta.key {
            key if key == &StoreKey::CumulativeUniqueAuthors.get_total_sum_key() => network_entity_change.change("cumulative_unique_authors", store_delta),
            key if key == &StoreKey::BlockHeight.get_total_sum_key() => {
                if store_delta.operation == Operation::Create {
                    is_first_block = true;
                }
                network_entity_change.change("block_height", store_delta)
            }
            key if key == &StoreKey::CumulativeDifficulty.get_total_sum_key() => network_entity_change.change("cumulative_difficulty", store_delta),
            key if key == &StoreKey::CumulativeGasUsed.get_total_sum_key() => network_entity_change.change("cumulative_gas_used", store_delta),
            key if key == &StoreKey::CumulativeBurntFees.get_total_sum_key() => network_entity_change.change("cumulative_burnt_fees", store_delta),
            key if key == &StoreKey::CumulativeRewards.get_total_sum_key() => network_entity_change.change("cumulative_rewards", store_delta),
            key if key == &StoreKey::CumulativeTransactions.get_total_sum_key() => network_entity_change.change("cumulative_transactions", store_delta),
            key if key == &StoreKey::CumulativeSize.get_total_sum_key() => network_entity_change.change("cumulative_size", store_delta),
            key if key == &StoreKey::TotalSupply.get_total_sum_key() => network_entity_change.change("total_supply", store_delta),
            key if key == &StoreKey::NumDays.get_total_sum_key() => is_new_day = true,
            key if key == &StoreKey::NumHours.get_total_sum_key() => is_new_hour = true,
            _ => {}
        }
    }

    // Stats values are always changing so we can add them directly to the entity changes
    network_entity_change.change("daily_blocks", stats_retriever.get_total_stats(StoreKey::DailyBlocks, StoreKey::BlockHeight).encode_to_vec());

    if is_first_block {
        network_entity_change.operation = Operation::Create as i32;

        // Stats fields will always be changing although non stats fields may not so we should add
        // default values for any non stats fields that have not already been added to the entity
        let all_non_stats_fields = vec![
            "cumulative_unique_authors".to_string(),
            "block_height".to_string(),
            "cumulative_difficulty".to_string(),
            "cumulative_gas_used".to_string(),
            "gas_limit".to_string(),
            "cumulative_burnt_fees".to_string(),
            "cumulative_rewards".to_string(),
            "cumulative_transactions".to_string(),
            "cumulative_size".to_string(),
            "total_supply".to_string(),
            "gas_limit".to_string()
        ];
        let currently_added_fields = network_entity_change.fields.iter().map(|field| field.name.clone()).collect::<Vec<_>>();
        for field in all_non_stats_fields {
            if !currently_added_fields.contains(&field) {
                network_entity_change.change(field, BigInt::zero());
            }
        }

        // And finally we can add the constant fields that won't get changed over time
        network_entity_change
            .change("id", ENTITY_ID)
            .change("schema_version", "schema_version")
            .change("subgraph_version", "subgraph_version")
            .change("methodology_version", "methodology_version");
    }

    if is_first_block {
        // We don't want to make a snapshot on the first block
        (network_entity_change, false, false)
    } else {
        (network_entity_change, is_new_day, is_new_hour)
    }
}

fn create_daily_snapshot_entity_change(aggregate_data: AggregateData, stats_retriever: &mut StatsRetriever, non_aggregation_deltas: &store::Deltas<DeltaBigInt>, non_aggregation_store: StoreGetBigInt) -> EntityChange {
    let entity_id = (get_latest_day(aggregate_data.timestamp) - 1).to_string();
    stats_retriever.set_entity_id(entity_id.clone());
    let mut day_snapshot_entity_change = EntityChange::new("daily_snapshot", entity_id.as_ref(), 0, Operation::Create);

    let store_retriever = stats_retriever.get_aggregation_retriever();

    // Because we have gone a full block past the last block of the day we need to adjust all the aggregated data to account for this.
    let block_height_at_start_of_day = store_retriever.get_total_sum(StoreKey::BlockHeight)
    let cumulative_difficulty_up_to_start_of_day = store_retriever.get_total_sum(StoreKey::CumulativeDifficulty) - store_retriever.get_day_sum(StoreKey::Difficulty) - aggregate_data.difficulty.unwrap().into();
    let cumulative_gas_used_up_to_start_of_day = store_retriever.get_total_sum(StoreKey::CumulativeGasUsed) - store_retriever.get_day_sum(StoreKey::GasUsed) - aggregate_data.gas_used.unwrap().into();

    day_snapshot_entity_change
        .change("id", entity_id)
        .change("network", "MAINNET")
        .change("block_height", store_retriever.get_total_sum(StoreKey::BlockHeight))
        .change("daily_blocks", store_retriever.get_day_sum(StoreKey::BlocksAcrossDay))
        .change("timestamp", aggregate_data.timestamp)
        .change("cumulative_unique_authors", store_retriever.get_total_sum(StoreKey::CumulativeUniqueAuthors))
        .change("daily_unique_authors", stats_retriever.get_total_stats(StoreKey::DailyUniqueAuthors, StoreKey::NumDays).encode_to_vec())
        .change("cumulative_difficulty", cumulative_difficulty_up_to_start_of_day)
        .change("daily_difficulty", stats_retriever.get_day_stats(StoreKey::Difficulty, StoreKey::BlocksAcrossDay).encode_to_vec())
        .change("cumulative_gas_used", cumulative_gas_used_up_to_start_of_day)
        .change("daily_gas_used", stats_retriever.get_day_stats(StoreKey::GasUsed, StoreKey::BlocksAcrossDay).encode_to_vec())
        .change("daily_gas_limit", stats_retriever.get_day_stats(StoreKey::GasLimit, StoreKey::BlocksAcrossDay).encode_to_vec())
        .change("cumulative_burnt_fees", store_retriever.get_total_sum(StoreKey::CumulativeBurntFees))
        .change("daily_burnt_fees", stats_retriever.get_day_stats(StoreKey::BurntFees, StoreKey::BlocksAcrossDay).encode_to_vec())
        .change("cumulative_rewards", store_retriever.get_total_sum(StoreKey::CumulativeRewards))
        .change("daily_rewards", stats_retriever.get_day_stats(StoreKey::Rewards, StoreKey::BlocksAcrossDay).encode_to_vec())
        .change("cumulative_size", store_retriever.get_day_sum(StoreKey::BlockSize))
        .change("daily_size", stats_retriever.get_day_stats(StoreKey::BlockSize, StoreKey::BlocksAcrossDay).encode_to_vec())
        .change("daily_chunks", stats_retriever.get_empty_day_stats(StoreKey::BlocksAcrossDay).encode_to_vec())
        .change("total_supply", store_retriever.get_day_sum(StoreKey::TotalSupply))
        .change("daily_supply", stats_retriever.get_total_stats(StoreKey::DailySupply, StoreKey::NumDays).encode_to_vec())
        .change("cumulative_transactions", store_retriever.get_total_sum(StoreKey::CumulativeTransactions))
        .change("daily_transactions", stats_retriever.get_total_stats(StoreKey::DailyTransactions, StoreKey::NumDays).encode_to_vec())
        .change(
            "daily_block_interval",
            stats_retriever.get_day_stats(StoreKey::BlockInterval, StoreKey::BlocksAcrossDay).encode_to_vec(),
        )
        .change("gas_price", "TODO")
        .change("daily_gas_price", stats_retriever.get_day_stats(StoreKey::BlockInterval, StoreKey::BlocksAcrossDay).encode_to_vec());

    day_snapshot_entity_change
}

fn create_hourly_snapshot_entity_change(timestamp: i64, stats_retriever: &mut StatsRetriever) -> EntityChange {
    let entity_id = (get_latest_hour(timestamp) - 1).to_string();
    stats_retriever.set_entity_id(entity_id.clone());
    let mut hour_snapshot_entity_change = EntityChange::new("hourly_snapshot", entity_id.as_ref(), 0, Operation::Create);

    let store_retriever = stats_retriever.get_aggregation_retriever();

    // Some aggregations are wanted based on the start of the day rather than the end of the day
    let cumulative_difficulty_up_to_start_of_hour = store_retriever.get_total_sum(StoreKey::CumulativeDifficulty) - store_retriever.get_hour_sum(StoreKey::Difficulty);
    let cumulative_gas_used_up_to_start_of_hour = store_retriever.get_total_sum(StoreKey::CumulativeGasUsed) - store_retriever.get_hour_sum(StoreKey::GasUsed);

    hour_snapshot_entity_change
        .change("id", entity_id)
        .change("network", "MAINNET")
        .change("block_height", store_retriever.get_total_sum(StoreKey::BlockHeight))
        .change("daily_blocks", store_retriever.get_hour_sum(StoreKey::BlocksAcrossHour))
        .change("timestamp", timestamp)
        .change("cumulative_unique_authors", store_retriever.get_total_sum(StoreKey::CumulativeUniqueAuthors))
        .change(
            "daily_unique_authors",
            stats_retriever.get_total_stats(StoreKey::DailyUniqueAuthors, StoreKey::NumHours).encode_to_vec(),
        )
        .change("cumulative_difficulty", cumulative_difficulty_up_to_start_of_hour)
        .change("daily_difficulty", stats_retriever.get_hour_stats(StoreKey::Difficulty, StoreKey::BlocksAcrossHour).encode_to_vec())
        .change("cumulative_gas_used", cumulative_gas_used_up_to_start_of_hour)
        .change("daily_gas_used", stats_retriever.get_hour_stats(StoreKey::GasUsed, StoreKey::BlocksAcrossHour).encode_to_vec())
        .change("daily_gas_limit", stats_retriever.get_hour_stats(StoreKey::GasLimit, StoreKey::BlocksAcrossHour).encode_to_vec())
        .change("cumulative_burnt_fees", store_retriever.get_total_sum(StoreKey::CumulativeBurntFees))
        .change("daily_burnt_fees", stats_retriever.get_hour_stats(StoreKey::BurntFees, StoreKey::BlocksAcrossHour).encode_to_vec())
        .change("cumulative_rewards", store_retriever.get_total_sum(StoreKey::CumulativeRewards))
        .change("daily_rewards", stats_retriever.get_hour_stats(StoreKey::Rewards, StoreKey::BlocksAcrossHour).encode_to_vec())
        .change("cumulative_size", store_retriever.get_hour_sum(StoreKey::BlockSize))
        .change(
            "daily_size",
            stats_retriever.get_hour_stats(StoreKey::BlockSize, StoreKey::BlocksAcrossHour).encode_to_vec().encode_to_vec(),
        )
        .change("daily_chunks", stats_retriever.get_empty_hour_stats(StoreKey::BlocksAcrossHour).encode_to_vec())
        .change("total_supply", store_retriever.get_hour_sum(StoreKey::TotalSupply))
        .change("daily_supply", stats_retriever.get_total_stats(StoreKey::DailySupply, StoreKey::NumHours).encode_to_vec())
        .change("cumulative_transactions", store_retriever.get_total_sum(StoreKey::CumulativeTransactions))
        .change("daily_transactions", stats_retriever.get_total_stats(StoreKey::DailyTransactions, StoreKey::NumHours).encode_to_vec())
        .change(
            "daily_block_interval",
            stats_retriever.get_hour_stats(StoreKey::BlockInterval, StoreKey::BlocksAcrossHour).encode_to_vec(),
        )
        .change("gas_price", "TODO")
        .change("daily_gas_price", stats_retriever.get_hour_stats(StoreKey::BlockInterval, StoreKey::BlocksAcrossHour).encode_to_vec());

    hour_snapshot_entity_change
}
