
use substreams::store::StoreSet;
use substreams_solana::{pb::sol as solana};
use substreams::{hex, log, proto, store, Hex};
use std::str::FromStr;


#[substreams::handlers::store]
fn store_test(block: solana::v1::Block, output: store::StoreSet) {
    log::info!("block height: {}", block.blockhash);
}
