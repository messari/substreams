use crate::aggregator::Aggregator;
use crate::pb::uniswap::v2::event::Type::{Deposit, Swap, Withdraw};
use substreams::pb::substreams::store_delta::Operation;
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
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
    let aggregator = Aggregator::<StoreAddBigInt>::new(
        &pre_aggregation_store,
        Some(clock.timestamp.unwrap().seconds),
    );

    for unique_delta in unique_users_deltas.deltas.into_iter() {
        if unique_delta.key.starts_with("d") && unique_delta.operation == Operation::Create {
            aggregator.add_protocol_specific_daily_field(StoreKey::ActiveUserCount, &BigInt::one())
        }

        if unique_delta.key.starts_with("h") && unique_delta.operation == Operation::Create {
            aggregator.add_protocol_specific_hourly_field(StoreKey::ActiveUserCount, &BigInt::one())
        }

        if unique_delta.key.starts_with("c:User") && unique_delta.operation == Operation::Create {
            aggregator.add_protocol_specific_cumulative_field(StoreKey::User, &BigInt::one())
        }

        if unique_delta.key.starts_with("c:Pool") && unique_delta.operation == Operation::Create {
            aggregator.add_protocol_specific_cumulative_field(StoreKey::PoolCount, &BigInt::one())
        }
    }

    for event in pool_events_map.events {
        if event.r#type.is_none() {
            continue;
        }

        aggregator.add_protocol_specific_daily_and_hourly_field(
            StoreKey::TransactionCount,
            &BigInt::one(),
        );

        if event.r#type.is_some() {
            match event.r#type.unwrap() {
                Deposit(_) => {
                    aggregator.add_protocol_specific_daily_and_hourly_field(
                        StoreKey::DepositCount,
                        &BigInt::one(),
                    );
                }
                Withdraw(_) => {
                    aggregator.add_protocol_specific_daily_and_hourly_field(
                        StoreKey::WithdrawCount,
                        &BigInt::one(),
                    );
                }
                Swap(_) => {
                    aggregator.add_protocol_specific_daily_and_hourly_field(
                        StoreKey::SwapCount,
                        &BigInt::one(),
                    );
                }
            }
        }
    }
}
