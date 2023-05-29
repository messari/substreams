use substreams::pb::substreams::Clock;
use substreams::store::{DeltaBigDecimal, Deltas, StoreAdd};
use substreams::store::{StoreAddBigDecimal, StoreNew};

use crate::store_key::StoreKey;
use crate::utils::{delta_value, get_day_id};

#[substreams::handlers::store]
pub fn store_protocol_daily_fields(
    clock: Clock,
    pool_daily_fields_deltas: Deltas<DeltaBigDecimal>,
    output_store: StoreAddBigDecimal,
) {
    let timestamp = clock.timestamp.unwrap().seconds;

    for delta in pool_daily_fields_deltas.deltas {
        let day_id = get_day_id(timestamp);

        match &delta.key {
            key if key.starts_with(StoreKey::DailyVolumeUSD.unique_id().as_str()) => {
                output_store.add(
                    delta.ordinal,
                    StoreKey::DailyVolumeUSD.get_unique_daily_protocol_key(day_id.clone()),
                    &delta_value(&delta),
                );
            }
            key if key.starts_with(StoreKey::DailySupplySideRevenueUSD.unique_id().as_str()) => {
                output_store.add(
                    delta.ordinal,
                    StoreKey::DailySupplySideRevenueUSD
                        .get_unique_daily_protocol_key(day_id.clone()),
                    &delta_value(&delta),
                );
            }
            key if key.starts_with(StoreKey::DailyProtocolSideRevenueUSD.unique_id().as_str()) => {
                output_store.add(
                    delta.ordinal,
                    StoreKey::DailyProtocolSideRevenueUSD
                        .get_unique_daily_protocol_key(day_id.clone()),
                    &delta_value(&delta),
                );
            }
            key if key.starts_with(StoreKey::DailyTotalRevenueUSD.unique_id().as_str()) => {
                output_store.add(
                    delta.ordinal,
                    StoreKey::DailyTotalRevenueUSD.get_unique_daily_protocol_key(day_id.clone()),
                    &delta_value(&delta),
                );
            }

            _ => {}
        }
    }
}
