use crate::aggregator::Aggregator;
use crate::pb::uniswap::v2::event::Type::{Deposit, Swap, Withdraw};
use substreams::pb::substreams::store_delta::Operation;
use substreams::pb::substreams::Clock;
use substreams::store::{DeltaBigInt, Deltas};
use substreams::store::{StoreAddBigInt, StoreNew};

use crate::pb::uniswap::v2::Events;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_metrics_pre_aggregations(
    clock: Clock,
    pool_events_map: Events,
    unique_users_deltas: Deltas<DeltaBigInt>,
    pre_aggregation_store: StoreAddBigInt,
) {
    let mut aggregator = Aggregator::<StoreAddBigInt>::new(
        pre_aggregation_store,
        Some(clock.timestamp.unwrap().seconds),
    );

    for unique_delta in unique_users_deltas.deltas.into_iter() {
        if unique_delta.key.starts_with("d") && unique_delta.operation == Operation::Create {
            aggregator.add_daily_field_stats(StoreKey::ActiveUserCount)
        }

        if unique_delta.key.starts_with("h") && unique_delta.operation == Operation::Create {
            aggregator.add_hourly_field_stats(StoreKey::ActiveUserCount)
        }

        if unique_delta.key.starts_with("c:User") && unique_delta.operation == Operation::Create {
            aggregator.add_cumulative_field_stats(StoreKey::User)
        }

        if unique_delta.key.starts_with("c:Pool") && unique_delta.operation == Operation::Create {
            aggregator.add_cumulative_field_stats(StoreKey::PoolCount)
        }
    }

    for event in pool_events_map.events {
        if event.r#type.is_none() {
            continue;
        }

        aggregator.add_daily_and_hourly_field_stats(StoreKey::TransactionCount);

        if event.r#type.is_some() {
            match event.r#type.unwrap() {
                Deposit(_) => {
                    aggregator.add_daily_and_hourly_field_stats(StoreKey::DepositCount);
                }
                Withdraw(_) => {
                    aggregator.add_daily_and_hourly_field_stats(StoreKey::WithdrawCount);
                }
                Swap(_) => {
                    aggregator.add_daily_and_hourly_field_stats(StoreKey::SwapCount);
                }
            }
        }
    }
}
