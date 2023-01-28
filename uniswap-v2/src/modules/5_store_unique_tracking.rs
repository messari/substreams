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
    let mut aggregator = Aggregator::<StoreSetBigInt>::new(unique_tracking_store, None);

    for new_pool in pool_created_map.pools {
        aggregator.set_cumulative_field(StoreKey::Pool, &new_pool.address);
    }

    for event in pool_events_map.events {
        if event.r#type.is_none() {
            continue;
        }

        aggregator.set_day_and_hour_timestamp(event.clone().timestamp);

        aggregator.set_daily_and_hourly_active_user(&event.to);
        aggregator.set_daily_and_hourly_active_user(&event.from);

        aggregator.set_cumulative_field(StoreKey::User, &event.to);
        aggregator.set_cumulative_field(StoreKey::User, &event.from);

        aggregator.set_latest_timestamp(event.clone().timestamp);
        aggregator.set_latest_block_number(event.block_number);
    }
}
