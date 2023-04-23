use substreams::store::{DeltaBigDecimal, Deltas};
use substreams::store::{StoreAddBigDecimal, StoreNew};

use crate::common::traits::StoreSetter;
use crate::store_key::StoreKey;
use crate::utils::delta_value;

#[substreams::handlers::store]
pub fn store_protocol_tvl(pool_tvl_deltas: Deltas<DeltaBigDecimal>, output_store: StoreAddBigDecimal) {
    for delta in pool_tvl_deltas.deltas {
        output_store.add_value(
            StoreKey::TotalValueLockedUSD.get_unique_protocol_key(),
            &delta_value(&delta),
        )
    }
}
