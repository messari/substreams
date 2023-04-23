use substreams::scalar::BigDecimal;
use substreams::store::{DeltaBigDecimal, Deltas};
use substreams::store::{StoreAddBigDecimal, StoreNew};

use crate::common::traits::StoreSetter;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_cumulative_fields(
    volume_deltas: Deltas<DeltaBigDecimal>,
    output_store: StoreAddBigDecimal,
) {
    for delta in volume_deltas.deltas {
        if let Some(pool_address) = StoreKey::Volume.get_pool(&delta.key) {
            let volume: BigDecimal = delta.new_value;
            let (supply_side_revenue, protocol_side_revenue) = calculate_revenues(volume.clone());

            output_store.add_value(
                StoreKey::CumulativeVolumeUSD.get_unique_pool_key(&pool_address),
                &volume,
            );
            output_store.add_value(
                StoreKey::CumulativeSupplySideRevenueUSD.get_unique_pool_key(&pool_address),
                &supply_side_revenue.clone(),
            );
            output_store.add_value(
                StoreKey::CumulativeProtocolSideRevenueUSD.get_unique_pool_key(&pool_address),
                &protocol_side_revenue.clone(),
            );
            output_store.add_value(
                StoreKey::CumulativeTotalRevenueUSD.get_unique_pool_key(&pool_address),
                &(supply_side_revenue + protocol_side_revenue),
            );
        }
    }
}

pub fn calculate_revenues(volume: BigDecimal) -> (BigDecimal, BigDecimal) {
    let supply_side_revenue =
        volume.clone() * BigDecimal::from(25_i32) / BigDecimal::from(10000_i32);
    let protocol_side_revenue =
        volume.clone() * BigDecimal::from(5_i32) / BigDecimal::from(10000_i32);

    (supply_side_revenue, protocol_side_revenue)
}
