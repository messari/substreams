use substreams::prelude::*;
use substreams::store::{StoreAddBigInt};
use crate::pb::uniswap::v3::{PrunedBlock};

#[substreams::handlers::store]
pub fn store_add_bigint(
    pruned_block: PrunedBlock,
    add_bigint_store: StoreAddBigInt,
) {

}