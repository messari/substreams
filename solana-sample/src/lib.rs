use substreams::{log, store};
use substreams_solana::pb::sol as solana;

#[substreams::handlers::store]
fn store_test(block: solana::v1::Block, _output: store::StoreSet) {
    log::info!("block height: {}", block.blockhash);
}
