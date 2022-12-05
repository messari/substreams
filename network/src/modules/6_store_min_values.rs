use substreams::store;
use substreams::store::StoreMin;

use crate::min_max_updater::MinMaxUpdater;
use crate::pb::aggregate_data::AggregateData;
use crate::store_key::StoreKey;
use crate::utils::BigIntDeserializeExt;

#[substreams::handlers::store]
pub fn store_min_values(aggregate_data: AggregateData, mut min_value_store: store::StoreMinBigInt) {
    let mut min_store = MinMaxUpdater::new(&mut min_value_store, aggregate_data.timestamp);

    // Here the necessary contributions are made for the network entity
    if let Some(daily_aggregated_data) = aggregate_data.daily_aggregated_data.as_ref() {
        min_store.update_total_value(StoreKey::DailyBlocks, &daily_aggregated_data.blocks.as_ref().unwrap().into());
    }

    // And here are the remaining contributions needed for the daily and hourly snapshot entities
    min_store.update_hourly_and_daily_values(StoreKey::Difficulty, &aggregate_data.difficulty.unwrap().into());
    min_store.update_hourly_and_daily_values(StoreKey::GasUsed, &aggregate_data.gas_used.unwrap().into());
    min_store.update_hourly_and_daily_values(StoreKey::GasLimit, &aggregate_data.gas_limit.unwrap().into());
    min_store.update_hourly_and_daily_values(StoreKey::BurntFees, &aggregate_data.burnt_fees.unwrap().into());
    min_store.update_hourly_and_daily_values(StoreKey::Rewards, &aggregate_data.rewards.unwrap().into());
    min_store.update_hourly_and_daily_values(StoreKey::BlockSize, &aggregate_data.block_size.unwrap().into());
    min_store.update_hourly_and_daily_values(StoreKey::BlockInterval, &aggregate_data.block_interval.unwrap().into());
    if let Some(daily_aggregated_data) = aggregate_data.daily_aggregated_data.as_ref() {
        min_store.update_hourly_and_daily_values(StoreKey::DailyUniqueAuthors, &daily_aggregated_data.unique_authors.as_ref().unwrap().into());
        min_store.update_hourly_and_daily_values(StoreKey::DailySupply, &daily_aggregated_data.supply.as_ref().unwrap().into());
        min_store.update_hourly_and_daily_values(StoreKey::DailyTransactions, &daily_aggregated_data.transactions.as_ref().unwrap().into());
    }
    if let Some(hourly_aggregated_data) = aggregate_data.hourly_aggregated_data.as_ref() {
        min_store.update_hourly_and_daily_values(StoreKey::HourlyUniqueAuthors, &hourly_aggregated_data.unique_authors.as_ref().unwrap().into());
        min_store.update_hourly_and_daily_values(StoreKey::HourlySupply, &hourly_aggregated_data.supply.as_ref().unwrap().into());
        min_store.update_hourly_and_daily_values(StoreKey::HourlyTransactions, &hourly_aggregated_data.transactions.as_ref().unwrap().into());
    }
}