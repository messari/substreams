use substreams::{log, store};
use substreams_solana::pb::sol as solana;

#[substreams::handlers::store]
fn store_test(block: solana::v1::Block, _output: store::StoreSet) {
    _output.set(0, "block".to_string(), &format!("{:?}", block).as_bytes().to_vec());
}
