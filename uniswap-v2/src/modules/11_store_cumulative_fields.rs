use substreams::store::{DeltaBigDecimal, Deltas, StoreAdd};
use substreams::store::{StoreAddBigDecimal, StoreNew};

use crate::store_key::StoreKey;
use crate::utils;

#[substreams::handlers::store]
pub fn store_cumulative_fields(
    volume_deltas: Deltas<DeltaBigDecimal>,
    output_store: StoreAddBigDecimal,
) {
    for delta in volume_deltas.deltas {
        if let Some(pool_address) = StoreKey::Volume.get_pool(&delta.key) {
            let volume = utils::delta_value(&delta);
            output_store.add(
                delta.ordinal,
                StoreKey::CumulativeVolumeUSD.get_unique_pool_key(&pool_address),
                &volume,
            );

            let (supply_side_revenue, protocol_side_revenue) = utils::calculate_revenue(volume);
            output_store.add(
                delta.ordinal,
                StoreKey::CumulativeSupplySideRevenueUSD.get_unique_pool_key(&pool_address),
                &supply_side_revenue,
            );
            output_store.add(
                delta.ordinal,
                StoreKey::CumulativeProtocolSideRevenueUSD.get_unique_pool_key(&pool_address),
                &protocol_side_revenue,
            );
            output_store.add(
                delta.ordinal,
                StoreKey::CumulativeTotalRevenueUSD.get_unique_pool_key(&pool_address),
                &(supply_side_revenue + protocol_side_revenue),
            );
        }
    }
}
