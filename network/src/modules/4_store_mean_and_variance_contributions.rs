use substreams::scalar::BigInt;
use substreams::store;
use substreams::store::{DeltaBytes, DeltaString, StoreAdd, StoreGet};
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::scalar::BigIntSign;
use substreams::store::StoreAddBigInt;
use substreams::store::StoreMaxBigInt;

use crate::aggregator::Aggregator;
use crate::block_handler::BlockHandler;
use crate::pb::aggregate_data::{self, AggregateData, PreCalculatedAggregates};
use crate::store_key::StoreKey;
use crate::utils::BigIntDeserializeExt;

#[substreams::handlers::store]
pub fn store_mean_and_variance_contributions(aggregate_data: AggregateData, mut aggregation_store: store::StoreAddBigInt) {
    let mut aggregator = Aggregator::new(&mut aggregation_store, aggregate_data.day_timestamp, aggregate_data.hour_timestamp);

    let difficulty = aggregate_data.difficulty.unwrap().into();
    let gas_used = aggregate_data.gas_used.unwrap().into();
    let burnt_fees = aggregate_data.burnt_fees.unwrap().into();
    let rewards = aggregate_data.rewards.unwrap().into();
    let block_size = aggregate_data.block_size.unwrap().into();

    // Here the necessary aggregations are made for the network entity
    if let Some(new_unique_authors) = aggregate_data.new_unique_authors {
        // Has to be optionally checked as we are optionally adding this field in the mapping stage
        aggregator.store_total_sum_contribution(StoreKey::CumulativeUniqueAuthors, &new_unique_authors.into());
    }
    aggregator.store_total_sum_contribution(StoreKey::BlockHeight, &BigInt::one());
    aggregator.store_total_sum_contribution(StoreKey::CumulativeDifficulty, &difficulty);
    aggregator.store_total_sum_contribution(StoreKey::CumulativeGasUsed, &gas_used);
    aggregator.store_total_sum_contribution(StoreKey::CumulativeBurntFees, &burnt_fees);
    aggregator.store_total_sum_contribution(StoreKey::CumulativeRewards, &rewards);
    aggregator.store_total_sum_contribution(StoreKey::CumulativeTransactions, &aggregate_data.transactions.unwrap().into());
    aggregator.store_total_sum_contribution(StoreKey::CumulativeSize, &block_size);
    aggregator.store_total_sum_contribution(StoreKey::TotalSupply, &aggregate_data.supply.unwrap().into());
    if let Some(daily_aggregated_data) = aggregate_data.daily_aggregated_data.as_ref() {
        aggregator.store_total_stats_contribution(StoreKey::DailyBlocks, &daily_aggregated_data.blocks.as_ref().unwrap().into());
    }

    // And here are the remaining aggregations needed for the daily and hourly snapshot entities
    aggregator.store_day_sum_contribution(StoreKey::BlocksAcrossDay, &BigInt::one());
    aggregator.store_hour_sum_contribution(StoreKey::BlocksAcrossHour, &BigInt::one());
    aggregator.store_day_and_hour_stats_contributions(StoreKey::Difficulty, &difficulty);
    aggregator.store_day_and_hour_stats_contributions(StoreKey::GasUsed, &gas_used);
    aggregator.store_day_and_hour_stats_contributions(StoreKey::GasLimit, &aggregate_data.gas_limit.unwrap().into());
    aggregator.store_day_and_hour_stats_contributions(StoreKey::BurntFees, &burnt_fees);
    aggregator.store_day_and_hour_stats_contributions(StoreKey::Rewards, &rewards);
    aggregator.store_day_and_hour_stats_contributions(StoreKey::BlockSize, &block_size);
    aggregator.store_day_and_hour_stats_contributions(StoreKey::BlockInterval, &aggregate_data.block_interval.unwrap().into());
    aggregator.store_day_and_hour_sum_contributions(StoreKey::TotalSupply, &aggregate_data.supply.unwrap().into());
    if let Some(daily_aggregated_data) = aggregate_data.daily_aggregated_data.as_ref() {
        aggregator.store_total_sum_contribution(StoreKey::NumDays, &BigInt::one());
        aggregator.store_total_stats_contribution(StoreKey::DailyUniqueAuthors, &daily_aggregated_data.unique_authors.as_ref().unwrap().into());
        aggregator.store_total_stats_contribution(StoreKey::DailySupply, &daily_aggregated_data.supply.as_ref().unwrap().into());
        aggregator.store_total_stats_contribution(StoreKey::DailyTransactions, &daily_aggregated_data.transactions.as_ref().unwrap().into());
    }
    if let Some(hourly_aggregated_data) = aggregate_data.hourly_aggregated_data.as_ref() {
        aggregator.store_total_sum_contribution(StoreKey::NumHours, &BigInt::one());
        aggregator.store_total_stats_contribution(StoreKey::HourlyUniqueAuthors, &hourly_aggregated_data.unique_authors.as_ref().unwrap().into());
        aggregator.store_total_stats_contribution(StoreKey::HourlySupply, &hourly_aggregated_data.supply.as_ref().unwrap().into());
        aggregator.store_total_stats_contribution(StoreKey::HourlyTransactions, &hourly_aggregated_data.transactions.as_ref().unwrap().into());
    }
}
