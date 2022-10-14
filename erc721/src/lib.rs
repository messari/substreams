#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

use hex_literal::hex;
use substreams::{log, store::{StoreAddInt64, self}, Hex};
use substreams_ethereum::{pb::eth as pbeth, Event, NULL_ADDRESS};

use pb::erc721::v1 as erc721;

// Bored Ape Yacht Club Contract
const TRACKED_CONTRACT: [u8; 20] = hex!("bc4ca0eda7647a8ab7c2061c2e118a18a936f13d");

/// Extracts transfer events from the contract
#[substreams::handlers::map]
fn block_to_transfers(
    blk: pbeth::v2::Block,
) -> Result<erc721::Transfers, substreams::errors::Error> {
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

// Store the total balance of NFT tokens by address for the specific TRACKED_CONTRACT by holder
#[substreams::handlers::store]
fn nft_state(transfers: erc721::Transfers, s: store::StoreAddInt64) {
    log::info!("NFT state builder");
    for transfer in transfers.transfers {
        if transfer.from != NULL_ADDRESS {
            log::info!("Found a transfer out");

            s.add(transfer.ordinal, generate_key(&transfer.from), -1);
        }

        if transfer.to != NULL_ADDRESS {
            log::info!("Found a transfer in");

            s.add(transfer.ordinal, generate_key(&transfer.to), 1);
        }
    }
}

fn generate_key(holder: &Vec<u8>) -> String {
    return format!("total:{}:{}", Hex(holder), Hex(TRACKED_CONTRACT));
}
