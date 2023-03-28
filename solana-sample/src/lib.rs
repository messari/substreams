use substreams::pb;
use substreams::store::{StoreSetRaw, StoreSet, StoreNew};
use substreams_solana::pb::sol as solana;

#[substreams::handlers::store]
fn store_test(block: solana::v1::Block, _output: StoreSetRaw) {
    _output.set(0, 0_u64, &block);
}
