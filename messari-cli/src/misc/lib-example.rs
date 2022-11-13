#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

use pb::erc20::v1 as erc20;
use substreams::store::StoreNew;
use substreams::store::StoreSet;
use substreams::store::StoreSetRaw;
use substreams::{hex, log, proto, store, Hex};
use substreams_ethereum::{pb::eth as pbeth, Event};
use substreams_helper::types::Address;

/// Extracts transfer events from the blocks
#[substreams::handlers::map]
fn map_test(
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
                log_ordinal: log.ordinal(),
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
fn store_test(transfers: erc20::TransferEvents, output: store::StoreSetRaw) {
    log::info!("Stored events {}", transfers.items.len());
    for transfer in transfers.items {
        output.set(
            transfer.log_ordinal,
            Hex::encode(&transfer.token_address),
            &proto::encode(&transfer).unwrap(),
        );
    }
}