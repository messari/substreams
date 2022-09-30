#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

use substreams::{hex, log, store, Hex};
use substreams_ethereum::{pb::eth as pbeth, Event, NULL_ADDRESS};

use substreams_helper::types::Address;

use pb::erc20::v1 as erc20;

/// Extracts transfer events from the contract
#[substreams::handlers::map]
fn map_block_to_transfers(
    block: pbeth::v2::Block,
) -> Result<erc20::TransferEvents, substreams::errors::Error> {
    // NOTE: Update TRACKED_CONTRACT to the address of the contract you want to track
    const TRACKED_CONTRACT: Address = hex!("5a98fcbea516cf06857215779fd812ca3bef1b32"); // LDO

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
