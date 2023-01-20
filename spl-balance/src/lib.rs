#[rustfmt::skip]
pub mod pb;

use num_bigint;
use substreams::scalar::BigInt;
use substreams::store::{StoreNew, StoreSet, StoreSetRaw};
use substreams::{store, Hex};
use substreams_ethereum::pb::eth as pbeth;
use substreams_solana::pb::sol as solana;
use pb::sol_token::v1 as proto;

#[substreams::handlers::map]
fn map_balances(block: solana::v1::Block) -> Result<proto::BalanceChanges, substreams::errors::Error> {

}
