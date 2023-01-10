use crate::pb::uniswap::v2::event::Type::{Deposit, Swap, Withdraw};
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreNew};

use crate::pb::uniswap::v2::{Events, Pools};
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn protocol_usage_metrics(
    _clock: Clock,
    pools_created: Pools,
    pool_events: Events,
    output: StoreAddBigInt,
) {
    for _ in pools_created.pools {
        output.add(0, StoreKey::PoolCount.get_unique_id(), BigInt::one())
    }

    for event in pool_events.events {
        if event.r#type.is_none() {
            continue;
        }

        if event.r#type.is_some() {
            output.add(0, StoreKey::TransactionCount.get_unique_id(), BigInt::one());

            match event.r#type.unwrap() {
                Deposit(_) => output.add(0, StoreKey::DepositCount.get_unique_id(), BigInt::one()),
                Withdraw(_) => {
                    output.add(0, StoreKey::WithdrawCount.get_unique_id(), BigInt::one())
                }
                Swap(_) => output.add(0, StoreKey::SwapCount.get_unique_id(), BigInt::one()),
            }
        }
    }
}
