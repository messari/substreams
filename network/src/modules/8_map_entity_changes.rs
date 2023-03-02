use prost::Message;
use substreams::pb::substreams::module_progress::Type;
use substreams::pb::substreams::store_delta;
use substreams::scalar::BigInt;
use substreams::store;
use substreams::store::{DeltaInt32, DeltaInt64, StoreGetBigInt};
use substreams::store::{DeltaBigInt, StoreGet};
use substreams_entity_change::change::ToField;
use substreams_entity_change::pb::entity::entity_change::Operation;
use substreams_entity_change::pb::entity::{EntityChange, EntityChanges, Field};
use substreams_entity_change::pb::entity::value::Typed;

use crate::network::SubgraphNetwork;
use crate::pb::aggregate_data::AggregateData;
use crate::stats_retriever::StatsRetriever;
use crate::store_key::StoreKey;
use crate::utils::{get_latest_day, get_latest_hour};

#[substreams::handlers::map]
pub fn map_entity_changes(
    aggregate_data: AggregateData,
    improper_aggregation_deltas: store::Deltas<DeltaBigInt>,
    aggregation_store: StoreGetBigInt,
    max_store: StoreGetBigInt,
    min_store: StoreGetBigInt,
    improper_non_aggregation_deltas: store::Deltas<DeltaBigInt>,
) -> Result<EntityChanges, ()> {
    let aggregation_deltas = get_proper_deltas(improper_aggregation_deltas);
    let non_aggregation_deltas = get_proper_deltas(improper_non_aggregation_deltas);

    let mut stats_retriever = StatsRetriever::new(&aggregation_store, &max_store, &min_store, aggregate_data.timestamp);

    let (network_entity, is_new_day, is_new_hour) = get_network_entity_change(&mut stats_retriever, &non_aggregation_deltas, &aggregation_deltas);

    let mut entity_changes = vec![network_entity];

    if is_new_day {
        entity_changes.push(create_daily_snapshot_entity_change(
            &aggregate_data,
            &mut stats_retriever,
            &non_aggregation_deltas,
        ))
    }

    if is_new_hour {
        entity_changes.push(create_hourly_snapshot_entity_change(
            &aggregate_data,
            &mut stats_retriever,
            &non_aggregation_deltas,
        ))
    }

    Ok(EntityChanges { entity_changes })
}

/// Returns network entity change and also if it is the time to create a day or hour snapshot. Return is in form => (network_entity_change, create_day_snapshot, create_hour_snapshot)
fn get_network_entity_change(stats_retriever: &mut StatsRetriever, non_aggregation_deltas: &store::Deltas<DeltaBigInt>, aggregation_deltas: &store::Deltas<DeltaBigInt>) -> (EntityChange, bool, bool) {
    let entity_id: String = SubgraphNetwork::MAINNET.to_string();
    stats_retriever.set_entity_id(entity_id.clone());
    let mut network_entity_change = EntityChange::new("Network", entity_id.to_string().as_str(), 0, Operation::Update);

    for non_aggregation_delta in non_aggregation_deltas.deltas.iter() {
        if non_aggregation_delta.key == StoreKey::GasLimit.get_unique_id() {
            network_entity_change.change("gasLimit", non_aggregation_delta);
        }
    }

    let mut is_first_block = false;
    let mut is_new_day = false;
    let mut is_new_hour = false;
    for store_delta in aggregation_deltas.deltas.iter() {
        match &store_delta.key {
            key if key == &StoreKey::CumulativeUniqueAuthors.get_total_sum_key() => {
                network_entity_change.change("cumulativeUniqueAuthors", store_delta.to_delta_int32());
            }
            key if key == &StoreKey::BlockHeight.get_total_sum_key() => {
                if store_delta.operation == store_delta::Operation::Create {
                    is_first_block = true;
                }
                network_entity_change.change("blockHeight", store_delta.to_delta_int32());
            }
            key if key == &StoreKey::CumulativeDifficulty.get_total_sum_key() => {
                network_entity_change.change("cumulativeDifficulty", store_delta);
            }
            key if key == &StoreKey::CumulativeGasUsed.get_total_sum_key() => {
                network_entity_change.change("cumulativeGasUsed", store_delta);
            }
            key if key == &StoreKey::CumulativeBurntFees.get_total_sum_key() => {
                network_entity_change.change("cumulativeBurntFees", store_delta);
            }
            key if key == &StoreKey::CumulativeRewards.get_total_sum_key() => {
                network_entity_change.change("cumulativeRewards", store_delta);
            }
            key if key == &StoreKey::CumulativeTransactions.get_total_sum_key() => {
                network_entity_change.change("cumulativeTransactions", store_delta);
            }
            key if key == &StoreKey::CumulativeSize.get_total_sum_key() => {
                network_entity_change.change("cumulativeSize", store_delta);
            }
            key if key == &StoreKey::TotalSupply.get_total_sum_key() => {
                network_entity_change.change("totalSupply", store_delta);
            }
            key if key == &StoreKey::NumDays.get_total_sum_key() => is_new_day = true,
            key if key == &StoreKey::NumHours.get_total_sum_key() => is_new_hour = true,
            _ => {}
        };
    }

    // We only want to create snapshots when we have at least some data from the day before
    if is_first_block {
        is_new_day = false;
        is_new_hour = false;
    }

    if is_new_day {
        network_entity_change.change("dailyBlocks", stats_retriever.get_total_stats(StoreKey::DailyBlocks, StoreKey::NumDays).encode_to_vec());
    }

    if is_first_block {
        network_entity_change.operation = Operation::Create as i32;

        network_entity_change.change("dailyBlocks", stats_retriever.get_zero_stats().encode_to_vec());

        // And finally we can add the constant fields that won't get changed over time
        network_entity_change
            .change("id", entity_id)
            .change("schemaVersion", "schema_version".to_string())
            .change("subgraphVersion", "subgraph_version".to_string())
            .change("methodologyVersion", "methodology_version".to_string());
    }

    (network_entity_change, is_new_day, is_new_hour)
}

fn create_daily_snapshot_entity_change(
    aggregate_data: &AggregateData,
    stats_retriever: &mut StatsRetriever,
    non_aggregation_deltas: &store::Deltas<DeltaBigInt>
) -> EntityChange {
    let entity_id = (get_latest_day(aggregate_data.timestamp) - 1).to_string();
    stats_retriever.set_entity_id(entity_id.clone());
    let mut day_snapshot_entity_change = EntityChange::new("DailySnapshot", entity_id.as_str(), 0, Operation::Create);

    let store_retriever = stats_retriever.get_aggregation_retriever();

    for non_aggregation_delta in non_aggregation_deltas.deltas.iter() {
        if non_aggregation_delta.key == StoreKey::GasPrice.get_unique_id() {
            // This makes sure we don't take the gas_price from the first block of the new day
            // rather than the last block of the day that we are taking the snapshot for
            day_snapshot_entity_change.change("gasPrice", non_aggregation_delta.old_value.clone());
        }
    }

    // Because we have gone a full block past the last block of the day we need to adjust all the aggregated data to account for this.
    let block_height_at_start_of_day = store_retriever.get_total_sum(StoreKey::BlockHeight) - store_retriever.get_day_sum(StoreKey::BlocksAcrossDay) - BigInt::one();
    let cumulative_difficulty_up_to_start_of_day =
        store_retriever.get_total_sum(StoreKey::CumulativeDifficulty) - store_retriever.get_day_sum(StoreKey::Difficulty) - aggregate_data.difficulty.as_ref().unwrap().clone().into();
    let cumulative_gas_used_up_to_start_of_day =
        store_retriever.get_total_sum(StoreKey::CumulativeGasUsed) - store_retriever.get_day_sum(StoreKey::GasUsed) - aggregate_data.gas_used.as_ref().unwrap().clone().into();
    let cumulative_unique_authors_up_to_end_of_day = store_retriever.get_total_sum(StoreKey::CumulativeUniqueAuthors) - aggregate_data.new_unique_authors.as_ref().unwrap().clone().into();
    let cumulative_burnt_fees_up_to_end_of_day = store_retriever.get_total_sum(StoreKey::CumulativeBurntFees) - aggregate_data.burnt_fees.as_ref().unwrap().clone().into();
    let cumulative_rewards_up_to_end_of_day = store_retriever.get_total_sum(StoreKey::CumulativeRewards) - aggregate_data.rewards.as_ref().unwrap().clone().into();
    let cumulative_transactions_up_to_end_of_day = store_retriever.get_total_sum(StoreKey::CumulativeTransactions) - aggregate_data.transactions.as_ref().unwrap().clone().into();

    day_snapshot_entity_change
        .change("id", entity_id)
        .change("network", SubgraphNetwork::MAINNET)
        .change("blockHeight", block_height_at_start_of_day.to_u64() as i32)
        .change("dailyBlocks", store_retriever.get_day_sum(StoreKey::BlocksAcrossDay).to_u64() as i32)
        .change("timestamp", BigInt::from(aggregate_data.timestamp))
        .change("cumulativeUniqueAuthors", cumulative_unique_authors_up_to_end_of_day.to_u64() as i32)
        .change("dailyUniqueAuthors", stats_retriever.get_total_stats(StoreKey::DailyUniqueAuthors, StoreKey::NumDays).encode_to_vec())
        .change("cumulativeDifficulty", cumulative_difficulty_up_to_start_of_day)
        .change("dailyDifficulty", stats_retriever.get_day_stats(StoreKey::Difficulty, StoreKey::BlocksAcrossDay).encode_to_vec())
        .change("cumulativeGasUsed", cumulative_gas_used_up_to_start_of_day)
        .change("dailyGasUsed", stats_retriever.get_day_stats(StoreKey::GasUsed, StoreKey::BlocksAcrossDay).encode_to_vec())
        .change("dailyGasLimit", stats_retriever.get_day_stats(StoreKey::GasLimit, StoreKey::BlocksAcrossDay).encode_to_vec())
        .change("cumulativeBurntFees", cumulative_burnt_fees_up_to_end_of_day)
        .change("dailyBurntFees", stats_retriever.get_day_stats(StoreKey::BurntFees, StoreKey::BlocksAcrossDay).encode_to_vec())
        .change("cumulativeRewards", cumulative_rewards_up_to_end_of_day)
        .change("dailyRewards", stats_retriever.get_day_stats(StoreKey::Rewards, StoreKey::BlocksAcrossDay).encode_to_vec())
        .change("cumulativeSize", store_retriever.get_day_sum(StoreKey::BlockSize))
        .change("dailySize", stats_retriever.get_day_stats(StoreKey::BlockSize, StoreKey::BlocksAcrossDay).encode_to_vec())
        .change("totalSupply", store_retriever.get_day_sum(StoreKey::TotalSupply))
        .change("dailySupply", stats_retriever.get_total_stats(StoreKey::DailySupply, StoreKey::NumDays).encode_to_vec())
        .change("cumulativeTransactions", cumulative_transactions_up_to_end_of_day)
        .change("dailyTransactions", stats_retriever.get_total_stats(StoreKey::DailyTransactions, StoreKey::NumDays).encode_to_vec())
        .change(
            "dailyBlockInterval",
            stats_retriever.get_day_stats(StoreKey::BlockInterval, StoreKey::BlocksAcrossDay).encode_to_vec(),
        )
        .change("dailyGasPrice", stats_retriever.get_day_stats(StoreKey::BlockInterval, StoreKey::BlocksAcrossDay).encode_to_vec());

    day_snapshot_entity_change
}

fn create_hourly_snapshot_entity_change(
    aggregate_data: &AggregateData,
    stats_retriever: &mut StatsRetriever,
    non_aggregation_deltas: &store::Deltas<DeltaBigInt>
) -> EntityChange {
    let entity_id = (get_latest_hour(aggregate_data.timestamp) - 1).to_string();
    stats_retriever.set_entity_id(entity_id.clone());
    let mut hour_snapshot_entity_change = EntityChange::new("HourlySnapshot", entity_id.as_str(), 0, Operation::Create);

    let store_retriever = stats_retriever.get_aggregation_retriever();

    for non_aggregation_delta in non_aggregation_deltas.deltas.iter() {
        if non_aggregation_delta.key == StoreKey::GasPrice.get_unique_id() {
            // This makes sure we don't take the gas_price from the first block of the new day
            // rather than the last block of the day that we are taking the snapshot for
            hour_snapshot_entity_change.change("gasPrice", non_aggregation_delta.old_value.clone());
        }
    }

    // Because we have gone a full block past the last block of the day we need to adjust all the aggregated data to account for this.
    let block_height_at_start_of_hour = store_retriever.get_total_sum(StoreKey::BlockHeight) - store_retriever.get_hour_sum(StoreKey::BlocksAcrossHour) - BigInt::one();
    let cumulative_difficulty_up_to_start_of_hour =
        store_retriever.get_total_sum(StoreKey::CumulativeDifficulty) - store_retriever.get_hour_sum(StoreKey::Difficulty) - aggregate_data.difficulty.as_ref().unwrap().clone().into();
    let cumulative_gas_used_up_to_start_of_hour =
        store_retriever.get_total_sum(StoreKey::CumulativeGasUsed) - store_retriever.get_hour_sum(StoreKey::GasUsed) - aggregate_data.gas_used.as_ref().unwrap().clone().into();
    let cumulative_unique_authors_up_to_end_of_hour = store_retriever.get_total_sum(StoreKey::CumulativeUniqueAuthors) - aggregate_data.new_unique_authors.as_ref().unwrap().clone().into();
    let cumulative_burnt_fees_up_to_end_of_hour = store_retriever.get_total_sum(StoreKey::CumulativeBurntFees) - aggregate_data.burnt_fees.as_ref().unwrap().clone().into();
    let cumulative_rewards_up_to_end_of_hour = store_retriever.get_total_sum(StoreKey::CumulativeRewards) - aggregate_data.rewards.as_ref().unwrap().clone().into();
    let cumulative_transactions_up_to_end_of_hour = store_retriever.get_total_sum(StoreKey::CumulativeTransactions) - aggregate_data.transactions.as_ref().unwrap().clone().into();

    hour_snapshot_entity_change
        .change("id", entity_id)
        .change("network", SubgraphNetwork::MAINNET)
        .change("blockHeight", block_height_at_start_of_hour.to_u64() as i32)
        .change("hourlyBlocks", store_retriever.get_hour_sum(StoreKey::BlocksAcrossHour).to_u64() as i32)
        .change("timestamp", BigInt::from(aggregate_data.timestamp))
        .change("cumulativeUniqueAuthors", cumulative_unique_authors_up_to_end_of_hour.to_u64() as i32)
        .change(
            "hourlyUniqueAuthors",
            stats_retriever.get_total_stats(StoreKey::HourlyUniqueAuthors, StoreKey::NumHours).encode_to_vec(),
        )
        .change("cumulativeDifficulty", cumulative_difficulty_up_to_start_of_hour)
        .change("hourlyDifficulty", stats_retriever.get_hour_stats(StoreKey::Difficulty, StoreKey::BlocksAcrossHour).encode_to_vec())
        .change("cumulativeGasUsed", cumulative_gas_used_up_to_start_of_hour)
        .change("hourlyGasUsed", stats_retriever.get_hour_stats(StoreKey::GasUsed, StoreKey::BlocksAcrossHour).encode_to_vec())
        .change("hourlyGasLimit", stats_retriever.get_hour_stats(StoreKey::GasLimit, StoreKey::BlocksAcrossHour).encode_to_vec())
        .change("cumulativeBurntFees", cumulative_burnt_fees_up_to_end_of_hour)
        .change("hourlyBurntFees", stats_retriever.get_hour_stats(StoreKey::BurntFees, StoreKey::BlocksAcrossHour).encode_to_vec())
        .change("cumulativeRewards", cumulative_rewards_up_to_end_of_hour)
        .change("hourlyRewards", stats_retriever.get_hour_stats(StoreKey::Rewards, StoreKey::BlocksAcrossHour).encode_to_vec())
        .change("cumulativeSize", store_retriever.get_hour_sum(StoreKey::BlockSize))
        .change("hourlySize", stats_retriever.get_hour_stats(StoreKey::BlockSize, StoreKey::BlocksAcrossHour).encode_to_vec())
        .change("totalSupply", store_retriever.get_hour_sum(StoreKey::TotalSupply))
        .change("hourlySupply", stats_retriever.get_total_stats(StoreKey::HourlySupply, StoreKey::NumHours).encode_to_vec())
        .change("cumulativeTransactions", cumulative_transactions_up_to_end_of_hour)
        .change("hourlyTransactions", stats_retriever.get_total_stats(StoreKey::HourlyTransactions, StoreKey::NumHours).encode_to_vec())
        .change(
            "hourlyBlockInterval",
            stats_retriever.get_hour_stats(StoreKey::BlockInterval, StoreKey::BlocksAcrossHour).encode_to_vec(),
        )
        .change("hourlyGasPrice", stats_retriever.get_hour_stats(StoreKey::BlockInterval, StoreKey::BlocksAcrossHour).encode_to_vec());

    hour_snapshot_entity_change
}

fn get_proper_deltas(incorrect_deltas: store::Deltas<DeltaBigInt>) -> store::Deltas<DeltaBigInt> {
    store::Deltas::<DeltaBigInt> {
        deltas: incorrect_deltas
            .deltas
            .into_iter()
            .filter_map(|delta| {
                if delta.operation == substreams::pb::substreams::store_delta::Operation::Update && delta.old_value == delta.new_value {
                    // THEN IT'S NOT ACTUALLY A DELTA!!!!!
                    None
                } else {
                    Some(delta)
                }
            })
            .collect(),
    }
}

trait ToDeltaInt32 {
    fn to_delta_int32(&self) -> DeltaInt32;
}

impl ToDeltaInt32 for DeltaBigInt {
    fn to_delta_int32(&self) -> DeltaInt32 {
        DeltaInt32 {
            operation: self.operation,
            ordinal: self.ordinal,
            key: self.key.clone(),
            old_value: self.old_value.to_u64() as i32,
            new_value: self.new_value.to_u64() as i32,
        }
    }
}

//
// trait ToTypedI64 {
//     fn to_typed_i64(self) -> Typed;
// }
//
// impl ToTypedI64 for BigInt {
//     fn to_typed_i64(self) -> Typed {
//         Typed::Bigint(self.to_string())
//     }
// }