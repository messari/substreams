#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

use num_bigint;
use substreams::{store, Hex};
use substreams::scalar::BigInt;
use substreams_ethereum::{pb::eth as pbeth};
use substreams::store::{StoreSetRaw, StoreSet, StoreNew};
use pb::eth_balance::v1::{EthBalanceChanges, EthBalanceChange};

#[substreams::handlers::map]
fn map_block_to_balance_changes(
    block: pbeth::v2::Block
) ->Result<EthBalanceChanges, substreams::errors::Error> {
    let mut eth_balance_changes = EthBalanceChanges { items: vec![] };

    for transaction in block.transactions() {
        for calls in transaction.calls() {
            for balance_change in calls.call.balance_changes.clone() {
                let new_value = balance_change
                    .new_value
                    .as_ref()
                    .map(|value| {
                        num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &value.bytes).into()
                    })
                    .unwrap_or(BigInt::zero());

                
                eth_balance_changes.items.push(EthBalanceChange{
                    address: Hex(&balance_change.address).to_string(),
                    ordinal: transaction.end_ordinal,
                    value: new_value.to_string()
                })
            }
        }
    }
    Ok(eth_balance_changes)
} 

#[substreams::handlers::store]
fn store_balance(balance_changes: EthBalanceChanges, output: store::StoreSetRaw) {
    for balance_change in balance_changes.items {
        output.set(
            balance_change.ordinal,
            format!("Address:{}", balance_change.address),
            &balance_change.value
        );
    }
}