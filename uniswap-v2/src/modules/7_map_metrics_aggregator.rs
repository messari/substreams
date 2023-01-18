use substreams::errors::Error;
use substreams::pb::substreams::store_delta::Operation;
use substreams::pb::substreams::Clock;
use substreams::store::{DeltaBigInt, Deltas};
use substreams::store::{StoreGet, StoreGetBigInt};

use crate::pb::uniswap::v2 as uniswap;
use crate::store_key::StoreKey;
use crate::store_retriever::StoreRetriever;

#[substreams::handlers::map]
pub fn map_metrics_aggregator(
    clock: Clock,
    unique_users_deltas: Deltas<DeltaBigInt>,
    pre_aggregations_store: StoreGetBigInt,
) -> Result<uniswap::MetricsSnapshot, Error> {
    let mut store_retriever = StoreRetriever::new(&pre_aggregations_store, None);

    let mut metrics_aggregator = uniswap::MetricsSnapshot {
        timestamp: clock.timestamp.unwrap().seconds,
        cumulative_pool_count: store_retriever
            .get_cumulative_value(StoreKey::PoolCount)
            .into(),
        cumulative_unique_users: store_retriever.get_cumulative_value(StoreKey::User).into(),
        hourly_usage_metrics: None,
        daily_usage_metrics: None,
    };

    for unique_delta in unique_users_deltas.deltas.into_iter() {
        if unique_delta.key == "latest_hour_timestamp".to_string()
            && unique_delta.operation != Operation::Create
        {
            if store_retriever.hour_timestamp_is_not_set() {
                store_retriever.set_hour_timestamp(unique_delta.old_value);
            }

            metrics_aggregator.hourly_usage_metrics = Some(uniswap::UsageMetricsHourlySnapshot {
                hourly_active_users: store_retriever
                    .get_hourly_stats_value(StoreKey::ActiveUserCount),
                hourly_transaction_count: store_retriever
                    .get_hourly_stats_value(StoreKey::TransactionCount),
                hourly_deposit_count: store_retriever
                    .get_hourly_stats_value(StoreKey::DepositCount),
                hourly_withdraw_count: store_retriever
                    .get_hourly_stats_value(StoreKey::WithdrawCount),
                hourly_swap_count: store_retriever.get_hourly_stats_value(StoreKey::SwapCount),
            });
        } else if unique_delta.key == "latest_day_timestamp".to_string()
            && unique_delta.operation != Operation::Create
        {
            if store_retriever.day_timestamp_is_not_set() {
                store_retriever.set_day_timestamp(unique_delta.old_value);
            }

            metrics_aggregator.daily_usage_metrics = Some(uniswap::UsageMetricsDailySnapshot {
                daily_active_users: store_retriever
                    .get_daily_stats_value(StoreKey::ActiveUserCount),
                daily_transaction_count: store_retriever
                    .get_daily_stats_value(StoreKey::TransactionCount),
                daily_deposit_count: store_retriever.get_daily_stats_value(StoreKey::DepositCount),
                daily_withdraw_count: store_retriever
                    .get_daily_stats_value(StoreKey::WithdrawCount),
                daily_swap_count: store_retriever.get_daily_stats_value(StoreKey::SwapCount),
            });
        }
    }

    Ok(metrics_aggregator)
}
