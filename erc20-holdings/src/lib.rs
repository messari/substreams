#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

mod keyer;

use num_bigint::BigInt;
use std::str::FromStr;
use std::ops::Neg;
use substreams::{hex, log, proto, store, Hex};
use substreams_ethereum::{pb::eth as pbeth, Event, NULL_ADDRESS};

use substreams_helper::types::Address;

use pb::erc20::v1 as erc20;

/// Extracts transfer events from the contract
#[substreams::handlers::map]
fn map_block_to_transfers(
    block: pbeth::v2::Block,
) -> Result<erc20::TransferEvents, substreams::errors::Error> {
    // NOTE: Update TRACKED_CONTRACT to the address of the contract you want to track
    const TRACKED_CONTRACT: Address = hex!("0c10bf8fcb7bf5412187a595ab97a3609160b5c6"); // USDD

    let mut transfer_events = erc20::TransferEvents { items: vec![] };

    for log in block.logs() {
        if let Some(event) = abi::erc20::events::Transfer::match_and_decode(log) {
            if log.address() != TRACKED_CONTRACT {
                continue;
            }

            transfer_events.items.push(erc20::TransferEvent {
                tx_hash: Hex(log.receipt.transaction.clone().hash).to_string(),
                log_index: log.index(),
                token_address: Hex(log.address()).to_string(),
                from: Hex(event.from).to_string(),
                to: Hex(event.to).to_string(),
                amount: event.value.to_string(),
            })
        }
    }

    Ok(transfer_events)
}

#[substreams::handlers::store]
fn store_transfers(transfers: erc20::TransferEvents, output: store::StoreSet) {
    log::info!("Stored events {}", transfers.items.len());
    for transfer in transfers.items {
        output.set(
            transfer.log_index as u64,
            Hex::encode(&transfer.token_address),
            &proto::encode(&transfer).unwrap(),
        );
    }
}

#[substreams::handlers::store]
fn store_balance(transfers: erc20::TransferEvents, output: store::StoreAddBigInt) {
    log::info!("Stored events {}", transfers.items.len());
    for transfer in transfers.items {
        log::info!("log index {}", transfer.log_index);

        output.add(
            transfer.log_index as u64,
            keyer::account_balance_key(&transfer.to),
            &BigInt::from_str(transfer.amount.as_str()).unwrap(),
        );

        if Hex::decode(transfer.from.clone()).unwrap() != NULL_ADDRESS {
            output.add(
                transfer.log_index as u64,
                keyer::account_balance_key(&transfer.from),
                &BigInt::from_str((transfer.amount).as_str()).unwrap().neg(),
            );
        }
    }
}
