#[rustfmt::skip]
pub mod pb;

use num_bigint;
use substreams::scalar::BigInt;
use substreams::store::{StoreNew, StoreSet, StoreSetRaw};
use substreams::{store, Hex};
use substreams_ethereum::pb::eth as pbeth;

#[substreams::handlers::store]
fn store_balance(block: pbeth::v2::Block, output: store::StoreSetRaw) {

}
