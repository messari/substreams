use substreams::pb::substreams::Clock;
use substreams::store::{StoreNew, StoreSetBigInt};

use crate::aggregator::Aggregator;
use crate::pb::uniswap::v2::{Events, Pools};
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_unique_tracking(
    clock: Clock,
    pool_created_map: Pools,
    pool_events_map: Events,
    unique_tracking_store: StoreSetBigInt,
) {
    let mut aggregator = Aggregator::new(
        Some(unique_tracking_store),
        None,
        clock.timestamp.unwrap().seconds,
    );

    aggregator.set_latest_daily_and_hourly_timestamp();

    for new_pool in pool_created_map.pools {
        aggregator.set_cumulative_field(StoreKey::Pool, &new_pool.address);
    }

    for event in pool_events_map.events {
        if event.r#type.is_none() {
            continue;
        }

        aggregator.set_daily_and_hourly_active_user(&event.to);
        aggregator.set_daily_and_hourly_active_user(&event.from);

        aggregator.set_cumulative_field(StoreKey::User, &event.to);
        aggregator.set_cumulative_field(StoreKey::User, &event.from);
    }
}
