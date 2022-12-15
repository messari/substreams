use substreams::store;
use substreams::store::StoreMaxBigInt;
use substreams::store::StoreNew;

use crate::min_max_updater::MinMaxUpdater;
use crate::pb::aggregate_data::AggregateData;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_max_values(aggregate_data: AggregateData, max_value_store: store::StoreMaxBigInt) {
    let mut max_store = MinMaxUpdater::new(max_value_store, aggregate_data.timestamp);

    // Here the necessary contributions are made for the network entity
    if let Some(daily_aggregated_data) = aggregate_data.daily_aggregated_data.as_ref() {
        max_store.update_total_value(StoreKey::DailyBlocks, &daily_aggregated_data.blocks.as_ref().unwrap().clone().into());
    }

    // And here are the remaining contributions needed for the daily and hourly snapshot entities
    max_store.update_hourly_and_daily_values(StoreKey::Difficulty, &aggregate_data.difficulty.unwrap().into());
    max_store.update_hourly_and_daily_values(StoreKey::GasUsed, &aggregate_data.gas_used.unwrap().into());
    max_store.update_hourly_and_daily_values(StoreKey::GasLimit, &aggregate_data.gas_limit.unwrap().into());
    max_store.update_hourly_and_daily_values(StoreKey::BurntFees, &aggregate_data.burnt_fees.unwrap().into());
    max_store.update_hourly_and_daily_values(StoreKey::Rewards, &aggregate_data.rewards.unwrap().into());
    max_store.update_hourly_and_daily_values(StoreKey::BlockSize, &aggregate_data.block_size.unwrap().into());
    max_store.update_hourly_and_daily_values(StoreKey::BlockInterval, &aggregate_data.block_interval.unwrap().into());
    if let Some(daily_aggregated_data) = aggregate_data.daily_aggregated_data.as_ref() {
        max_store.update_hourly_and_daily_values(StoreKey::DailyUniqueAuthors, &daily_aggregated_data.unique_authors.as_ref().unwrap().clone().into());
        max_store.update_hourly_and_daily_values(StoreKey::DailySupply, &daily_aggregated_data.supply.as_ref().unwrap().clone().into());
        max_store.update_hourly_and_daily_values(StoreKey::DailyTransactions, &daily_aggregated_data.transactions.as_ref().unwrap().clone().into());
    }
    if let Some(hourly_aggregated_data) = aggregate_data.hourly_aggregated_data.as_ref() {
        max_store.update_hourly_and_daily_values(StoreKey::HourlyUniqueAuthors, &hourly_aggregated_data.unique_authors.as_ref().unwrap().clone().into());
        max_store.update_hourly_and_daily_values(StoreKey::HourlySupply, &hourly_aggregated_data.supply.as_ref().unwrap().clone().into());
        max_store.update_hourly_and_daily_values(StoreKey::HourlyTransactions, &hourly_aggregated_data.transactions.as_ref().unwrap().clone().into());
    }
}
