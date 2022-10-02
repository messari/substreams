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
use pb::common::v1 as common;

fn code_len(call: &pbeth::v2::Call) -> usize {
    let mut len = 0;
    for code_change in &call.code_changes {
        len += code_change.new_code.len()
    }

    len
}

/// Extracts erc20 contract deployments from the blocks
#[substreams::handlers::map]
fn map_block_to_erc20_contracts(
    block: pbeth::v2::Block,
) -> Result<common::Addresses, substreams::errors::Error> {
    let mut erc20_contracts = common::Addresses { items: vec![] };

    for call_view in block.calls() {
        let call = call_view.call;
        if call.call_type == pbeth::v2::CallType::Create as i32 {
            // skipping contracts that are too short to be an erc20 token
            if code_len(call) < 150 {
                continue;
            }

            let address = Hex(call.address.clone()).to_string();

            // check if contract is an erc20 token
            if substreams_helper::erc20::get_erc20_token(address.clone()).is_none() {
                continue;
            }

            log::info!("Create {}, len {}", address, code_len(call));
            erc20_contracts.items.push(common::Address {
                address
            });
        }
    }

    Ok(erc20_contracts)
}

/// Extracts transfer events from the blocks
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
fn store_transfers(transfers: erc20::TransferEvents, output: store::StoreSet) {
    log::info!("Stored events {}", transfers.items.len());
    for transfer in transfers.items {
        output.set(
            transfer.log_ordinal,
            Hex::encode(&transfer.token_address),
            &proto::encode(&transfer).unwrap(),
        );
    }
}

#[substreams::handlers::store]
fn store_balance(transfers: erc20::TransferEvents, output: store::StoreAddBigInt) {
    log::info!("Stored events {}", transfers.items.len());
    for transfer in transfers.items {
        output.add(
            transfer.log_ordinal,
            keyer::account_balance_key(&transfer.to),
            &BigInt::from_str(transfer.amount.as_str()).unwrap(),
        );

        if Hex::decode(transfer.from.clone()).unwrap() != NULL_ADDRESS {
            output.add(
                transfer.log_ordinal,
                keyer::account_balance_key(&transfer.from),
                &BigInt::from_str((transfer.amount).as_str()).unwrap().neg(),
            );
        }
    }
}
