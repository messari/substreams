
use substreams::store::StoreNew;
use substreams::store::StoreSetRaw;
use substreams_solana::{pb::sol as solana};
use substreams::{hex, log, proto, store, Hex};
use std::str::FromStr;


#[substreams::handlers::store]
fn store_transfers(block: solana::v1::Block, output: store::StoreSetRaw) {
    log::info!("Stored events");
}
