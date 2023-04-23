use substreams::pb::substreams::Clock;
use substreams::scalar::BigDecimal;
use substreams::store::{DeltaBigDecimal, Deltas};
use substreams::store::{StoreAddBigDecimal, StoreNew};

use crate::common::traits::StoreSetter;
use crate::store_key::StoreKey;
use crate::utils::{get_day_id, get_hour_id};

use super::store_cumulative_fields::calculate_revenues;

#[substreams::handlers::store]
pub fn store_daily_and_hourly_fields(
    clock: Clock,
    volume_deltas: Deltas<DeltaBigDecimal>,
    output_store: StoreAddBigDecimal,
) {
    let timestamp = clock.timestamp.unwrap().seconds;

    for delta in volume_deltas.deltas {
        if let Some(pool_address) = StoreKey::Volume.get_pool(&delta.key) {
            let day_id = get_day_id(timestamp);
            let hour_id = get_hour_id(timestamp);

            let volume: BigDecimal = delta.new_value;
            let (supply_side_revenue, protocol_side_revenue) = calculate_revenues(volume.clone());

            output_store.add_value(
                StoreKey::HourlyVolumeUSD
                    .get_unique_hourly_pool_key(hour_id.clone(), &pool_address),
                &volume,
            );
            output_store.add_value(
                StoreKey::HourlySupplySideRevenueUSD
                    .get_unique_hourly_pool_key(hour_id.clone(), &pool_address),
                &supply_side_revenue.clone(),
            );
            output_store.add_value(
                StoreKey::HourlyProtocolSideRevenueUSD
                    .get_unique_hourly_pool_key(hour_id.clone(), &pool_address),
                &protocol_side_revenue.clone(),
            );
            output_store.add_value(
                StoreKey::HourlyTotalRevenueUSD
                    .get_unique_hourly_pool_key(hour_id.clone(), &pool_address),
                &(supply_side_revenue.clone() + protocol_side_revenue.clone()),
            );

            output_store.add_value(
                StoreKey::DailyVolumeUSD.get_unique_daily_pool_key(day_id.clone(), &pool_address),
                &volume,
            );
            output_store.add_value(
                StoreKey::DailySupplySideRevenueUSD
                    .get_unique_daily_pool_key(day_id.clone(), &pool_address),
                &supply_side_revenue.clone(),
            );
            output_store.add_value(
                StoreKey::DailyProtocolSideRevenueUSD
                    .get_unique_daily_pool_key(day_id.clone(), &pool_address),
                &protocol_side_revenue.clone(),
            );
            output_store.add_value(
                StoreKey::DailyTotalRevenueUSD
                    .get_unique_daily_pool_key(day_id.clone(), &pool_address),
                &(supply_side_revenue + protocol_side_revenue),
            );
        }
    }
}
