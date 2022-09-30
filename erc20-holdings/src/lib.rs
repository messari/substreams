#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

use hex_literal::hex;
use prost;
use substreams::{log, store, Hex};
use substreams_ethereum::{pb::eth as pbeth, Event, NULL_ADDRESS};

use pb::erc721::v1 as erc721;

/// Extracts transfer events from the contract
#[substreams::handlers::map]
fn block_to_transfers(
    blk: pbeth::v2::Block,
) -> Result<erc721::Transfers, substreams::errors::Error> {
    // NOTE: Update TRACKED_CONTRACT to the address of the contract you want to track
    let TRACKED_CONTRACT: Vec<u8> = vec![];

    let mut transfers: Vec<erc721::Transfer> = vec![];
    for trx in blk.transaction_traces {
        transfers.extend(trx.receipt.unwrap().logs.iter().filter_map(|log| {
            // None
            if log.address != TRACKED_CONTRACT {
                None
            } else {
                abi::erc721::events::Transfer::match_and_decode(log).map(|transfer| {
                    erc721::Transfer {
                        trx_hash: trx.hash.clone(),
                        from: transfer.from,
                        to: transfer.to,
                        token_id: transfer.token_id.low_u64(),
                        ordinal: log.block_index as u64,
                    }
                })
            }
        }));
    }

    Ok(erc721::Transfers { transfers })
}
