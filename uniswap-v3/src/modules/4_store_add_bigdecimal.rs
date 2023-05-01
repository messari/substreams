use substreams::prelude::*;
use substreams::store::StoreAddBigDecimal;
use crate::pb::uniswap::v3::{PrunedBlock};

#[substreams::handlers::store]
pub fn store_add_bigdecimal(
    pruned_block: PrunedBlock,
    add_bigdecimal_store: StoreAddBigDecimal,
) {
    
}