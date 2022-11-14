use std::str::FromStr;
use substreams::store::StoreSet;
use substreams::{hex, log, proto, store, Hex};
use substreams_solana::pb::sol as solana;

#[substreams::handlers::store]
fn store_test(block: solana::v1::Block, output: store::StoreSet) {
    log::info!("block height: {}", block.blockhash);
}
