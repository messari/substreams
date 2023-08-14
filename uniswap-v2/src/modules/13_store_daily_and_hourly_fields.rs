use substreams::pb::substreams::Clock;
use substreams::store::{DeltaBigDecimal, Deltas};
use substreams::store::{StoreAddBigDecimal, StoreNew};

use crate::common::traits::StoreAddSnapshot;
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

            output_store.add_snapshot(
                ordinal,
                hour_id,
                StoreKey::HourlyVolumeUSD,
                vec![&pool_address],
                &volume,
            );
            output_store.add_snapshot(
                ordinal,
                hour_id,
                StoreKey::HourlySupplySideRevenueUSD,
                vec![&pool_address],
                &supply_side_revenue,
            );
            output_store.add_snapshot(
                ordinal,
                hour_id,
                StoreKey::HourlyProtocolSideRevenueUSD,
                vec![&pool_address],
                &protocol_side_revenue,
            );
            output_store.add_snapshot(
                ordinal,
                hour_id,
                StoreKey::HourlyTotalRevenueUSD,
                vec![&pool_address],
                &(supply_side_revenue.clone() + protocol_side_revenue.clone()),
            );

            output_store.add_snapshot(
                ordinal,
                day_id,
                StoreKey::DailyVolumeUSD,
                vec![&pool_address],
                &volume,
            );
            output_store.add_snapshot(
                ordinal,
                day_id,
                StoreKey::DailySupplySideRevenueUSD,
                vec![&pool_address],
                &supply_side_revenue,
            );
            output_store.add_snapshot(
                ordinal,
                day_id,
                StoreKey::DailyProtocolSideRevenueUSD,
                vec![&pool_address],
                &protocol_side_revenue,
            );
            output_store.add_snapshot(
                ordinal,
                day_id,
                StoreKey::DailyTotalRevenueUSD,
                vec![&pool_address],
                &(supply_side_revenue + protocol_side_revenue),
            );
        } else if let Some((pool_address, token_address)) =
            StoreKey::VolumeByTokenUSD.get_pool_and_token(&delta.key)
        {
            output_store.add_snapshot(
                ordinal,
                hour_id,
                StoreKey::HourlyVolumeByTokenUSD,
                vec![&pool_address, &token_address],
                &volume,
            );
            output_store.add_snapshot(
                ordinal,
                day_id,
                StoreKey::DailyVolumeByTokenUSD,
                vec![&pool_address, &token_address],
                &volume,
            );
        }
    }
}
