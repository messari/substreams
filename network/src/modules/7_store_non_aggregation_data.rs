use substreams::store::StoreSet;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams::store::StoreSetBigInt;
use substreams::store::StoreNew;

use crate::block_handler::BlockHandler;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_non_aggregation_data(block: eth::Block, store: StoreSetBigInt) {
    let block_handler = BlockHandler::new(&block);

    store.set(0, StoreKey::GasLimit.get_unique_id(), &block_handler.gas_limit());
    store.set(0, StoreKey::GasPrice.get_unique_id(), &block_handler.gas_price());
}