use substreams::scalar::BigInt;
use substreams::store::{StoreNew, StoreSetBigInt};

use crate::aggregator::Aggregator;
use crate::pb::uniswap::v2::{Events, Pools};
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_unique_tracking(
    pool_created_map: Pools,
    pool_events_map: Events,
    unique_tracking_store: StoreSetBigInt,
) {
    let mut aggregator = Aggregator::<StoreSetBigInt>::new(&unique_tracking_store, None);

    for new_pool in pool_created_map.pools {
        aggregator.set_global_cumulative_unique_field(
            StoreKey::Pool,
            &new_pool.address,
            &BigInt::one(),
        );
    }

    for event in pool_events_map.events {
        if event.r#type.is_none() {
            continue;
        }

        aggregator.set_day_and_hour_timestamp(event.clone().timestamp);

        aggregator.set_global_daily_and_hourly_unique_field(
            StoreKey::ActiveUser,
            &event.to,
            &BigInt::one(),
        );
        aggregator.set_global_daily_and_hourly_unique_field(
            StoreKey::ActiveUser,
            &event.from,
            &BigInt::one(),
        );

        aggregator.set_global_cumulative_unique_field(StoreKey::User, &event.to, &BigInt::one());
        aggregator.set_global_cumulative_unique_field(StoreKey::User, &event.from, &BigInt::one());

        aggregator.set_latest_timestamp(&BigInt::from(event.clone().timestamp));
        aggregator.set_latest_block_number(&BigInt::from(event.block_number));
    }
}
