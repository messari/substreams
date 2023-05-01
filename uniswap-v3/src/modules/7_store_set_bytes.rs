use substreams::prelude::*;
use substreams::store::{StoreSetRaw};
use crate::pb::uniswap::v3::{PrunedBlock};

#[substreams::handlers::store]
pub fn store_set_bytes(
    pruned_block: PrunedBlock,
    set_bytes_store: StoreSetRaw,
) {
    
}