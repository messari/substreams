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

use crate::network::Network;
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

    let mut entity_changes = vec![];

    let (is_new_day, is_new_hour) = get_network_entity_change(&mut stats_retriever, &non_aggregation_deltas, &aggregation_deltas, &mut entity_changes);

    if is_new_day {
        add_daily_snapshot_entity_changes(
            &aggregate_data,
            &mut stats_retriever,
            &non_aggregation_deltas,
            &mut entity_changes
        )
    }

    if is_new_hour {
        add_hourly_snapshot_entity_changes(
            &aggregate_data,
            &mut stats_retriever,
            &non_aggregation_deltas,
            &mut entity_changes
        )
    }

    Ok(EntityChanges { entity_changes })
}

/// Returns network entity change and also if it is the time to create a day or hour snapshot. Return is in form => (network_entity_change, create_day_snapshot, create_hour_snapshot)
fn get_network_entity_change(stats_retriever: &mut StatsRetriever, non_aggregation_deltas: &store::Deltas<DeltaBigInt>, aggregation_deltas: &store::Deltas<DeltaBigInt>, entity_changes: &mut Vec<EntityChange>) -> (bool, bool) {
    let entity_id: String = Network::MAINNET.to_string();
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
        entity_changes.push(stats_retriever.get_total_stats("dailyBlocks", StoreKey::DailyBlocks, StoreKey::NumDays, false));
        network_entity_change.change("dailyBlocks", stats_retriever.get_id("dailyBlocks"));
    }

    if is_first_block {
        network_entity_change.operation = Operation::Create as i32;

        entity_changes.push(stats_retriever.get_zero_stats_entity_change("dailyBlocks"));
        network_entity_change.change("dailyBlocks", stats_retriever.get_id("dailyBlocks"));

        // And finally we can add the constant fields that won't get changed over time
        network_entity_change
            .change("id", entity_id)
            .change("schemaVersion", "schema_version".to_string())
            .change("subgraphVersion", "subgraph_version".to_string())
            .change("methodologyVersion", "methodology_version".to_string());
    }

    entity_changes.push(network_entity_change);

    (is_new_day, is_new_hour)
}

fn add_daily_snapshot_entity_changes(
    aggregate_data: &AggregateData,
    stats_retriever: &mut StatsRetriever,
    non_aggregation_deltas: &store::Deltas<DeltaBigInt>,
    entity_changes: &mut Vec<EntityChange>
) {
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

    entity_changes.push(stats_retriever.get_total_stats("dailyUniqueAuthors", StoreKey::DailyUniqueAuthors, StoreKey::NumDays, true));
    entity_changes.push(stats_retriever.get_day_stats("dailyDifficulty", StoreKey::Difficulty, StoreKey::BlocksAcrossDay));
    entity_changes.push(stats_retriever.get_day_stats("dailyGasUsed", StoreKey::GasUsed, StoreKey::BlocksAcrossDay));
    entity_changes.push(stats_retriever.get_day_stats("dailyGasLimit", StoreKey::GasLimit, StoreKey::BlocksAcrossDay));
    entity_changes.push(stats_retriever.get_day_stats("dailyBurntFees", StoreKey::BurntFees, StoreKey::BlocksAcrossDay));
    entity_changes.push(stats_retriever.get_day_stats("dailyRewards", StoreKey::Rewards, StoreKey::BlocksAcrossDay));
    entity_changes.push(stats_retriever.get_day_stats("dailySize", StoreKey::BlockSize, StoreKey::BlocksAcrossDay));
    entity_changes.push(stats_retriever.get_total_stats("dailySupply", StoreKey::DailySupply, StoreKey::NumDays, true));
    entity_changes.push(stats_retriever.get_total_stats("dailyTransactions", StoreKey::DailyTransactions, StoreKey::NumDays, true));
    entity_changes.push(stats_retriever.get_day_stats("dailyBlockInterval", StoreKey::BlockInterval, StoreKey::BlocksAcrossDay));
    entity_changes.push(stats_retriever.get_day_stats("dailyGasPrice", StoreKey::BlockInterval, StoreKey::BlocksAcrossDay));

    day_snapshot_entity_change
        .change("id", entity_id)
        .change("network", Network::MAINNET.to_string())
        .change("blockHeight", block_height_at_start_of_day.to_u64() as i32)
        .change("dailyBlocks", store_retriever.get_day_sum(StoreKey::BlocksAcrossDay).to_u64() as i32)
        .change("timestamp", BigInt::from(aggregate_data.timestamp))
        .change("cumulativeUniqueAuthors", cumulative_unique_authors_up_to_end_of_day.to_u64() as i32)
        .change("dailyUniqueAuthors", stats_retriever.get_id("dailyUniqueAuthors"))
        .change("cumulativeDifficulty", cumulative_difficulty_up_to_start_of_day)
        .change("dailyDifficulty", stats_retriever.get_id("dailyDifficulty"))
        .change("cumulativeGasUsed", cumulative_gas_used_up_to_start_of_day)
        .change("dailyGasUsed", stats_retriever.get_id("dailyGasUsed"))
        .change("dailyGasLimit", stats_retriever.get_id("dailyGasLimit"))
        .change("cumulativeBurntFees", cumulative_burnt_fees_up_to_end_of_day)
        .change("dailyBurntFees", stats_retriever.get_id("dailyBurntFees"))
        .change("cumulativeRewards", cumulative_rewards_up_to_end_of_day)
        .change("dailyRewards", stats_retriever.get_id("dailyRewards"))
        .change("cumulativeSize", store_retriever.get_day_sum(StoreKey::BlockSize))
        .change("dailySize", stats_retriever.get_id("dailySize"))
        .change("totalSupply", store_retriever.get_day_sum(StoreKey::TotalSupply))
        .change("dailySupply", stats_retriever.get_id("dailySupply"))
        .change("cumulativeTransactions", cumulative_transactions_up_to_end_of_day)
        .change("dailyTransactions", stats_retriever.get_id("dailyTransactions"))
        .change(
            "dailyBlockInterval",
            stats_retriever.get_id("dailyBlockInterval"),
        )
        .change("dailyGasPrice", stats_retriever.get_id("dailyGasPrice"));

    entity_changes.push(day_snapshot_entity_change);
}

fn add_hourly_snapshot_entity_changes(
    aggregate_data: &AggregateData,
    stats_retriever: &mut StatsRetriever,
    non_aggregation_deltas: &store::Deltas<DeltaBigInt>,
    entity_changes: &mut Vec<EntityChange>
) {
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

    entity_changes.push(stats_retriever.get_total_stats("hourlyUniqueAuthors", StoreKey::HourlyUniqueAuthors, StoreKey::NumHours, true));
    entity_changes.push(stats_retriever.get_hour_stats_entity_change("hourlyDifficulty", StoreKey::Difficulty, StoreKey::BlocksAcrossHour));
    entity_changes.push(stats_retriever.get_hour_stats_entity_change("hourlyGasUsed", StoreKey::GasUsed, StoreKey::BlocksAcrossHour));
    entity_changes.push(stats_retriever.get_hour_stats_entity_change("hourlyGasLimit", StoreKey::GasLimit, StoreKey::BlocksAcrossHour));
    entity_changes.push(stats_retriever.get_hour_stats_entity_change("hourlyBurntFees", StoreKey::BurntFees, StoreKey::BlocksAcrossHour));
    entity_changes.push(stats_retriever.get_hour_stats_entity_change("hourlyRewards", StoreKey::Rewards, StoreKey::BlocksAcrossHour));
    entity_changes.push(stats_retriever.get_hour_stats_entity_change("hourlySize", StoreKey::BlockSize, StoreKey::BlocksAcrossHour));
    entity_changes.push(stats_retriever.get_total_stats("hourlySupply", StoreKey::HourlySupply, StoreKey::NumHours, true));
    entity_changes.push(stats_retriever.get_total_stats("hourlyTransactions", StoreKey::HourlyTransactions, StoreKey::NumHours, true));
    entity_changes.push(stats_retriever.get_hour_stats_entity_change("hourlyBlockInterval", StoreKey::BlockInterval, StoreKey::BlocksAcrossHour));
    entity_changes.push(stats_retriever.get_hour_stats_entity_change("hourlyGasPrice", StoreKey::BlockInterval, StoreKey::BlocksAcrossHour));

    hour_snapshot_entity_change
        .change("id", entity_id)
        .change("network", Network::MAINNET.to_string())
        .change("blockHeight", block_height_at_start_of_hour.to_u64() as i32)
        .change("hourlyBlocks", store_retriever.get_hour_sum(StoreKey::BlocksAcrossHour).to_u64() as i32)
        .change("timestamp", BigInt::from(aggregate_data.timestamp))
        .change("cumulativeUniqueAuthors", cumulative_unique_authors_up_to_end_of_hour.to_u64() as i32)
        .change(
            "hourlyUniqueAuthors",
            stats_retriever.get_id("hourlyUniqueAuthors"),
        )
        .change("cumulativeDifficulty", cumulative_difficulty_up_to_start_of_hour)
        .change("hourlyDifficulty", stats_retriever.get_id("hourlyDifficulty"))
        .change("cumulativeGasUsed", cumulative_gas_used_up_to_start_of_hour)
        .change("hourlyGasUsed", stats_retriever.get_id("hourlyGasUsed"))
        .change("hourlyGasLimit", stats_retriever.get_id("hourlyGasLimit"))
        .change("cumulativeBurntFees", cumulative_burnt_fees_up_to_end_of_hour)
        .change("hourlyBurntFees", stats_retriever.get_id("hourlyBurntFees"))
        .change("cumulativeRewards", cumulative_rewards_up_to_end_of_hour)
        .change("hourlyRewards", stats_retriever.get_id("hourlyRewards"))
        .change("cumulativeSize", store_retriever.get_hour_sum(StoreKey::BlockSize))
        .change("hourlySize", stats_retriever.get_id("hourlySize"))
        .change("totalSupply", store_retriever.get_hour_sum(StoreKey::TotalSupply))
        .change("hourlySupply", stats_retriever.get_id("hourlySupply"))
        .change("cumulativeTransactions", cumulative_transactions_up_to_end_of_hour)
        .change("hourlyTransactions", stats_retriever.get_id("hourlyTransactions"))
        .change(
            "hourlyBlockInterval",
            stats_retriever.get_id("hourlyBlockInterval"),
        )
        .change("hourlyGasPrice", stats_retriever.get_id("hourlyGasPrice"));

    entity_changes.push(hour_snapshot_entity_change);
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