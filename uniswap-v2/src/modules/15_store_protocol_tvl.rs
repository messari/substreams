use substreams::store::{DeltaBigDecimal, Deltas, StoreAdd};
use substreams::store::{StoreAddBigDecimal, StoreNew};

use crate::store_key::StoreKey;
use crate::utils::delta_value;

#[substreams::handlers::store]
pub fn store_protocol_tvl(
    pool_tvl_deltas: Deltas<DeltaBigDecimal>,
    output_store: StoreAddBigDecimal,
) {
    for delta in pool_tvl_deltas.deltas {
        output_store.add(
            delta.ordinal,
            StoreKey::TotalValueLockedUSD.get_unique_protocol_key(),
            &delta_value(&delta),
        )
    }
}
