#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

mod keyer;
mod rpc;

use pb::erc20::v1 as erc20;
use pb::erc20_price::v1::Erc20Price;
use std::str::FromStr;
use substreams::scalar::BigDecimal;
use substreams::scalar::BigInt;
use substreams::store::StoreAdd;
use substreams::store::StoreAddBigInt;
use substreams::store::StoreGet;
use substreams::store::StoreGetBigInt;
use substreams::store::StoreGetProto;
use substreams::store::StoreNew;
use substreams::store::StoreSet;
use substreams::store::StoreSetBigDecimal;
use substreams::store::StoreSetRaw;
use substreams::{hex, log, proto, store, Hex};
use substreams_ethereum::{pb::eth as pbeth, Event, NULL_ADDRESS};
use substreams_helper::keyer::chainlink_asset_key;
use substreams_helper::types::Address;

const INITIALIZE_METHOD_HASH: [u8; 4] = hex!("1459457a");

fn code_len(call: &pbeth::v2::Call) -> usize {
    let mut len = 0;
    for code_change in &call.code_changes {
        len += code_change.new_code.len()
    }

    len
}

// TODO: this should be a store module
/// Extracts erc20 contract deployments from the blocks
#[substreams::handlers::map]
fn map_block_to_erc20_contracts(
    block: pbeth::v2::Block,
) -> Result<erc20::Erc20Tokens, substreams::errors::Error> {
    let mut erc20_tokens = erc20::Erc20Tokens { items: vec![] };

    for tx in block.transaction_traces {
        for call in tx.calls {
            if call.state_reverted {
                continue;
            }

            if call.call_type == pbeth::v2::CallType::Create as i32
                || call.call_type == pbeth::v2::CallType::Call as i32
            // proxy contract creation
            {
                let call_input_len = call.input.len();
                if call.call_type == pbeth::v2::CallType::Call as i32
                    && (call_input_len < 4 || call.input[0..4] != INITIALIZE_METHOD_HASH)
                {
                    // this will check if a proxy contract has been called to create a ERC20 contract.
                    // if that is the case the Proxy contract will call the initialize function on the ERC20 contract
                    // this is part of the OpenZeppelin Proxy contract standard
                    continue;
                }

                // Contract creation not from proxy contract
                if call.call_type == pbeth::v2::CallType::Create as i32 {
                    let code_change_len = code_len(&call);

                    if code_change_len <= 150 {
                        // skipping contracts with less than 150 bytes of code
                        log::info!(
                            "Skipping contract: {}. Contract code is less than 150 bytes.",
                            Hex::encode(&call.address)
                        );
                        continue;
                    }
                }

                log::info!(
                    "Attempting to get metadata for erc20: {}",
                    Hex::encode(&call.address)
                );

                let decimals: u64;
                let decimal_result = rpc::get_erc20_decimals(&call.address);
                match decimal_result {
                    Ok(_decimals) => decimals = _decimals,
                    Err(_e) => continue,
                };

                let symbol: String;
                let symbaol_result = rpc::get_erc20_symbol(&call.address);
                match symbaol_result {
                    Ok(_symbol) => symbol = _symbol,
                    Err(_e) => continue,
                };

                let name: String;
                let name_result = rpc::get_erc20_name(&call.address);
                match name_result {
                    Ok(_name) => name = _name,
                    Err(_e) => continue,
                };

                erc20_tokens.items.push(erc20::Erc20Token {
                    address: Hex::encode(call.address.clone()),
                    name: name,
                    symbol: symbol,
                    decimals: decimals,
                    tx_created: Hex::encode(&tx.hash),
                    block_created: block.number,
                });
            }
        }
    }

    Ok(erc20_tokens)
}

/// Extracts transfer events from the blocks
#[substreams::handlers::map]
fn map_block_to_transfers(
    block: pbeth::v2::Block,
) -> Result<erc20::TransferEvents, substreams::errors::Error> {
    // NOTE: Update TRACKED_CONTRACT to the address of the contract you want to track
    const TRACKED_CONTRACT: Address = hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"); // WETH

    let mut transfer_events = erc20::TransferEvents { items: vec![] };

    for log in block.logs() {
        if let Some(event) = abi::erc20::events::Transfer::match_and_decode(log) {
            if log.address() != TRACKED_CONTRACT {
                continue;
            }

            transfer_events.items.push(erc20::TransferEvent {
                tx_hash: Hex::encode(log.receipt.transaction.clone().hash),
                block_number: block.number,
                timestamp: block
                    .header
                    .as_ref()
                    .unwrap()
                    .timestamp
                    .as_ref()
                    .unwrap()
                    .seconds as u64,
                log_index: log.index(),
                log_ordinal: Some(log.ordinal()),
                token_address: Hex::encode(log.address()),
                from: Hex::encode(event.from),
                to: Hex::encode(event.to),
                amount: event.value.to_string(),
                balance_changes: vec![],
            })
        }
    }

    Ok(transfer_events)
}

#[substreams::handlers::store]
fn store_transfers(transfers: erc20::TransferEvents, output: store::StoreSetRaw) {
    log::info!("Stored events {}", transfers.items.len());
    for transfer in transfers.items {
        output.set(
            transfer.log_ordinal.unwrap(),
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
            transfer.log_ordinal.unwrap(),
            keyer::account_balance_key(&transfer.to),
            &BigInt::from_str(transfer.amount.as_str()).unwrap(),
        );

        if Hex::decode(transfer.from.clone()).unwrap() != NULL_ADDRESS {
            output.add(
                transfer.log_ordinal.unwrap(),
                keyer::account_balance_key(&transfer.from),
                &BigInt::from_str((transfer.amount).as_str()).unwrap().neg(),
            );
        }
    }
}

#[substreams::handlers::store]
fn store_balance_usd(
    transfers: erc20::TransferEvents,
    balances: store::StoreGetBigInt,
    prices: StoreGetProto<Erc20Price>,
    output: store::StoreSetBigDecimal,
) {
    for transfer in transfers.items {
        let mut token_price = BigDecimal::zero();
        let mut token_decimals = 0;

        match prices.get_last(chainlink_asset_key(&transfer.token_address)) {
            Some(price_store) => {
                token_price = BigDecimal::from_str(price_store.price_usd.as_str()).unwrap();
                token_decimals = price_store.token.unwrap().decimals;
            }
            None => {}
        };

        match balances.get_last(keyer::account_balance_key(&transfer.to)) {
            Some(balance) => output.set(
                transfer.log_ordinal.unwrap(),
                keyer::account_balance_usd_key(&transfer.to),
                &(token_price.clone() * balance.to_decimal(token_decimals.into())),
            ),
            None => {}
        }

        if Hex::decode(transfer.from.clone()).unwrap() != NULL_ADDRESS {
            match balances.get_last(keyer::account_balance_key(&transfer.from)) {
                Some(balance) => output.set(
                    transfer.log_ordinal.unwrap(),
                    keyer::account_balance_usd_key(&transfer.from),
                    &(token_price.clone() * balance.to_decimal(token_decimals.into())),
                ),
                None => {}
            };
        }
    }
}
