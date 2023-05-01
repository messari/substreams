use substreams::prelude::*;
use substreams::store::{StoreAddInt64};
use crate::pb::uniswap::v3::{PrunedBlock};

#[substreams::handlers::store]
pub fn store_add_int64(
    pruned_block: PrunedBlock,
    add_int64_store: StoreAddInt64,
) {

}