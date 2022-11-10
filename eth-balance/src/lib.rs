#[rustfmt::skip]
pub mod pb;

use num_bigint;
use substreams::{store, Hex};
use substreams::scalar::BigInt;
use substreams_ethereum::{pb::eth as pbeth};
use substreams::store::{StoreSetRaw, StoreSet, StoreNew};

#[substreams::handlers::store]
fn store_balance(block: pbeth::v2::Block, output: store::StoreSetRaw) {
    for transaction in &block.transaction_traces {
        for call in &transaction.calls {
            for balance_change in &call.balance_changes {
                
                // TODO: replace this with substreams::scalar::BigInt once the wrapper is integrated
                let new_value = balance_change
                    .new_value
                    .as_ref()
                    .map(|value| {
                        num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &value.bytes).into()
                    })
                    .unwrap_or(BigInt::zero());

                output.set(
                    transaction.end_ordinal,
                    format!("Address:{}", Hex(&balance_change.address).to_string()),
                    &new_value.to_string()
                )
            }
        }
    }
}