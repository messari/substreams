use substreams::pb::substreams::Clock;
use substreams::store::{DeltaBigDecimal, Deltas};
use substreams::store::{StoreAddBigDecimal, StoreNew};

use crate::common::traits::StoreAddSnapshot;
use crate::store_key::StoreKey;
use crate::utils;

#[substreams::handlers::store]
pub fn store_protocol_daily_fields(
    clock: Clock,
    volume_deltas: Deltas<DeltaBigDecimal>,
    output_store: StoreAddBigDecimal,
) {
    let timestamp = clock.timestamp.unwrap().seconds;
    let day_id = utils::get_day_id(timestamp);

    for delta in volume_deltas.deltas {
        let ordinal = delta.ordinal;
        let volume = utils::delta_value(&delta);

        if let Some(_) = StoreKey::Volume.get_pool(&delta.key) {
            let (supply_side_revenue, protocol_side_revenue) =
                utils::calculate_revenue(volume.clone());

            output_store.add_protocol_snapshot(ordinal, day_id, StoreKey::DailyVolumeUSD, &volume);
            output_store.add_protocol_snapshot(
                ordinal,
                day_id,
                StoreKey::DailySupplySideRevenueUSD,
                &supply_side_revenue,
            );
            output_store.add_protocol_snapshot(
                ordinal,
                day_id,
                StoreKey::DailyProtocolSideRevenueUSD,
                &protocol_side_revenue,
            );
            output_store.add_protocol_snapshot(
                ordinal,
                day_id,
                StoreKey::DailyTotalRevenueUSD,
                &(supply_side_revenue + protocol_side_revenue),
            );
        }
    }
}
