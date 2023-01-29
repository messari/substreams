use substreams::scalar::BigDecimal;
use substreams::store::{StoreAddBigDecimal, StoreNew};

use crate::aggregator::Aggregator;
use crate::pb::uniswap::v2::event::Type::Swap as SwapEvent;
use crate::pb::uniswap::v2::Events;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_swaps_volume_and_revenue(
    pool_events: Events,
    swap_and_volume_store: StoreAddBigDecimal,
) {
    let mut aggregator = Aggregator::<StoreAddBigDecimal>::new(&swap_and_volume_store, None);

    for event in pool_events.events {
        aggregator.set_day_and_hour_timestamp(event.timestamp);

        match event.clone().r#type.unwrap() {
            SwapEvent(swap) => {
                let pool_address = &event.clone().pool;

                let volume: BigDecimal =
                    (swap.amount_in_usd.unwrap() + swap.amount_out_usd.unwrap()).convert()
                        / BigDecimal::from(2);

                let supply_side_revenue =
                    volume.clone() * BigDecimal::from(25_i32) / BigDecimal::from(10000_i32);
                let protocol_side_revenue =
                    volume.clone() * BigDecimal::from(5_i32) / BigDecimal::from(10000_i32);
                let total_revenue = supply_side_revenue.clone() + protocol_side_revenue.clone();

                aggregator.add_pool_specific_cumulative_field(
                    StoreKey::PoolVolume,
                    &pool_address,
                    &volume.clone(),
                );
                aggregator.add_pool_specific_daily_and_hourly_field(
                    StoreKey::PoolVolume,
                    &pool_address,
                    &volume,
                );

                aggregator.add_pool_specific_cumulative_field(
                    StoreKey::PoolSupplySideRevenue,
                    &pool_address,
                    &supply_side_revenue.clone(),
                );
                aggregator.add_pool_specific_daily_and_hourly_field(
                    StoreKey::PoolSupplySideRevenue,
                    &pool_address,
                    &supply_side_revenue,
                );

                aggregator.add_pool_specific_cumulative_field(
                    StoreKey::PoolProtocolSideRevenue,
                    &pool_address,
                    &protocol_side_revenue.clone(),
                );
                aggregator.add_pool_specific_daily_and_hourly_field(
                    StoreKey::PoolProtocolSideRevenue,
                    &pool_address,
                    &protocol_side_revenue,
                );

                aggregator.add_pool_specific_cumulative_field(
                    StoreKey::PoolTotalRevenue,
                    &pool_address,
                    &total_revenue.clone(),
                );
                aggregator.add_pool_specific_daily_and_hourly_field(
                    StoreKey::PoolTotalRevenue,
                    &pool_address,
                    &total_revenue,
                );
            }
            _ => {}
        }
    }
}
