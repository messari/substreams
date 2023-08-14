use substreams::store::{DeltaBigDecimal, Deltas, StoreAdd};
use substreams::store::{StoreAddBigDecimal, StoreNew};

use crate::store_key::StoreKey;
use crate::utils::delta_value;

#[substreams::handlers::store]
pub fn store_protocol_cumulative_fields(
    pool_cumulative_fields_deltas: Deltas<DeltaBigDecimal>,
    output_store: StoreAddBigDecimal,
) {
    for delta in pool_cumulative_fields_deltas.deltas {
        match &delta.key {
            key if key.starts_with(StoreKey::CumulativeVolumeUSD.unique_id().as_str()) => {
                output_store.add(
                    delta.ordinal,
                    StoreKey::CumulativeVolumeUSD.get_unique_protocol_key(),
                    &delta_value(&delta),
                )
            }
            key if key.starts_with(
                StoreKey::CumulativeSupplySideRevenueUSD
                    .unique_id()
                    .as_str(),
            ) =>
            {
                output_store.add(
                    delta.ordinal,
                    StoreKey::CumulativeSupplySideRevenueUSD.get_unique_protocol_key(),
                    &delta_value(&delta),
                )
            }
            key if key.starts_with(
                StoreKey::CumulativeProtocolSideRevenueUSD
                    .unique_id()
                    .as_str(),
            ) =>
            {
                output_store.add(
                    delta.ordinal,
                    StoreKey::CumulativeProtocolSideRevenueUSD.get_unique_protocol_key(),
                    &delta_value(&delta),
                )
            }
            key if key.starts_with(StoreKey::CumulativeTotalRevenueUSD.unique_id().as_str()) => {
                output_store.add(
                    delta.ordinal,
                    StoreKey::CumulativeTotalRevenueUSD.get_unique_protocol_key(),
                    &delta_value(&delta),
                )
            }
            _ => {}
        }
    }
}
