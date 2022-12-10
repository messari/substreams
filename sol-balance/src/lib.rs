use substreams::{log};
use substreams_solana::pb::sol as solana;

#[substreams::handlers::store]
fn store_balance(block: solana::v1::Block) {
    log::info!("block height: {}", block.blockhash);
}
