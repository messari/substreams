use substreams::pb::substreams::Clock;
use substreams::store::{DeltaBigDecimal, Deltas, StoreAdd};
use substreams::store::{StoreAddBigDecimal, StoreNew};

use crate::store_key::StoreKey;
use crate::utils;

#[substreams::handlers::store]
pub fn store_daily_and_hourly_fields(
    clock: Clock,
    volume_deltas: Deltas<DeltaBigDecimal>,
    output_store: StoreAddBigDecimal,
) {
    let timestamp = clock.timestamp.unwrap().seconds;

    let day_id = utils::get_day_id(timestamp);
    let hour_id = utils::get_hour_id(timestamp);

    for delta in volume_deltas.deltas {
        let ordinal = delta.ordinal;
        let volume = utils::delta_value(&delta);

        if let Some(pool_address) = StoreKey::Volume.get_pool(&delta.key) {
            let (supply_side_revenue, protocol_side_revenue) =
                utils::calculate_revenue(volume.clone());

            output_store.add(
                ordinal,
                StoreKey::HourlyVolumeUSD
                    .get_unique_hourly_pool_key(hour_id.clone(), &pool_address),
                &volume,
            );
            output_store.add(
                ordinal,
                StoreKey::HourlySupplySideRevenueUSD
                    .get_unique_hourly_pool_key(hour_id.clone(), &pool_address),
                &supply_side_revenue.clone(),
            );
            output_store.add(
                ordinal,
                StoreKey::HourlyProtocolSideRevenueUSD
                    .get_unique_hourly_pool_key(hour_id.clone(), &pool_address),
                &protocol_side_revenue.clone(),
            );
            output_store.add(
                ordinal,
                StoreKey::HourlyTotalRevenueUSD
                    .get_unique_hourly_pool_key(hour_id.clone(), &pool_address),
                &(supply_side_revenue.clone() + protocol_side_revenue.clone()),
            );

            output_store.add(
                ordinal,
                StoreKey::DailyVolumeUSD.get_unique_daily_pool_key(day_id.clone(), &pool_address),
                &volume,
            );
            output_store.add(
                ordinal,
                StoreKey::DailySupplySideRevenueUSD
                    .get_unique_daily_pool_key(day_id.clone(), &pool_address),
                &supply_side_revenue.clone(),
            );
            output_store.add(
                ordinal,
                StoreKey::DailyProtocolSideRevenueUSD
                    .get_unique_daily_pool_key(day_id.clone(), &pool_address),
                &protocol_side_revenue.clone(),
            );
            output_store.add(
                ordinal,
                StoreKey::DailyTotalRevenueUSD
                    .get_unique_daily_pool_key(day_id.clone(), &pool_address),
                &(supply_side_revenue + protocol_side_revenue),
            );
        } else if let Some((pool_address, token_address)) =
            StoreKey::VolumeByTokenUSD.get_pool_and_token(&delta.key)
        {
            output_store.add(
                ordinal,
                StoreKey::DailyVolumeByTokenUSD.get_unique_hourly_pool_and_token_key(
                    day_id.clone(),
                    &pool_address,
                    &token_address,
                ),
                &volume,
            );

            output_store.add(
                ordinal,
                StoreKey::HourlyVolumeByTokenUSD.get_unique_hourly_pool_and_token_key(
                    hour_id.clone(),
                    &pool_address,
                    &token_address,
                ),
                &volume,
            );
        }
    }
}
