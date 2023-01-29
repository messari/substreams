use substreams::pb::substreams::Clock;
use substreams::store::{DeltaBigDecimal, Deltas, StoreAddBigDecimal, StoreNew};

use crate::aggregator::Aggregator;
use crate::store_key::StoreKey;
use crate::utils::get_delta;

#[substreams::handlers::store]
pub fn store_financials_pre_aggregation(
    clock: Clock,
    pool_tvl_deltas: Deltas<DeltaBigDecimal>,
    pool_volume_and_revenue_deltas: Deltas<DeltaBigDecimal>,
    financials_pre_aggregation_store: StoreAddBigDecimal,
) {
    let aggregator = Aggregator::<StoreAddBigDecimal>::new(
        &financials_pre_aggregation_store,
        Some(clock.timestamp.unwrap().seconds),
    );

    for pool_tvl_delta in pool_tvl_deltas.deltas.into_iter() {
        match &pool_tvl_delta.key {
            key if key.starts_with("c:PoolTVL") => {
                aggregator.add_protocol_specific_cumulative_field(
                    StoreKey::PoolTVL,
                    &get_delta(pool_tvl_delta),
                );
            }
            _ => {}
        }
    }

    for pool_volume_and_revenue_delta in pool_volume_and_revenue_deltas.deltas.into_iter() {
        match &pool_volume_and_revenue_delta.key {
            key if key.starts_with("c:PoolSupplySideRevenue") => {
                aggregator.add_protocol_specific_cumulative_field(
                    StoreKey::PoolSupplySideRevenue,
                    &get_delta(pool_volume_and_revenue_delta),
                );
            }
            key if key.starts_with("d:PoolSupplySideRevenue") => {
                aggregator.add_protocol_specific_daily_field(
                    StoreKey::PoolSupplySideRevenue,
                    &get_delta(pool_volume_and_revenue_delta),
                );
            }
            key if key.starts_with("h:PoolSupplySideRevenue") => {
                aggregator.add_protocol_specific_hourly_field(
                    StoreKey::PoolSupplySideRevenue,
                    &get_delta(pool_volume_and_revenue_delta),
                );
            }
            key if key.starts_with("c:PoolProtocolSideRevenue") => {
                aggregator.add_protocol_specific_cumulative_field(
                    StoreKey::PoolProtocolSideRevenue,
                    &get_delta(pool_volume_and_revenue_delta),
                );
            }
            key if key.starts_with("d:PoolProtocolSideRevenue") => {
                aggregator.add_protocol_specific_daily_field(
                    StoreKey::PoolProtocolSideRevenue,
                    &get_delta(pool_volume_and_revenue_delta),
                );
            }
            key if key.starts_with("h:PoolProtocolSideRevenue") => {
                aggregator.add_protocol_specific_hourly_field(
                    StoreKey::PoolProtocolSideRevenue,
                    &get_delta(pool_volume_and_revenue_delta),
                );
            }
            key if key.starts_with("c:PoolTotalRevenue") => {
                aggregator.add_protocol_specific_cumulative_field(
                    StoreKey::PoolTotalRevenue,
                    &get_delta(pool_volume_and_revenue_delta),
                );
            }
            key if key.starts_with("d:PoolTotalRevenue") => {
                aggregator.add_protocol_specific_daily_field(
                    StoreKey::PoolTotalRevenue,
                    &get_delta(pool_volume_and_revenue_delta),
                );
            }
            key if key.starts_with("h:PoolTotalRevenue") => {
                aggregator.add_protocol_specific_hourly_field(
                    StoreKey::PoolTotalRevenue,
                    &get_delta(pool_volume_and_revenue_delta),
                );
            }

            _ => {}
        }
    }
}
