#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

mod keyer;

use pb::common::v1 as common;
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

fn contract_bytecode_len(call: &pbeth::v2::Call) -> usize {
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
            if contract_bytecode_len(call) < 150 {
                continue;
            }

            let address = Hex(call.address.clone()).to_string();

            // check if contract is an erc20 token
            if substreams_helper::erc20::get_erc20_token(address.clone()).is_none() {
                continue;
            }

            log::info!("Create {}, len {}", address, contract_bytecode_len(call));
            erc20_contracts.items.push(common::Address { address });
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
    const TRACKED_CONTRACT: Address = hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"); // WETH

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
fn store_transfers(transfers: erc20::TransferEvents, output: store::StoreSetRaw) {
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
                transfer.log_ordinal,
                keyer::account_balance_usd_key(&transfer.to),
                &(token_price.clone() * balance.to_decimal(token_decimals.into())),
            ),
            None => {}
        }

        if Hex::decode(transfer.from.clone()).unwrap() != NULL_ADDRESS {
            match balances.get_last(keyer::account_balance_key(&transfer.from)) {
                Some(balance) => output.set(
                    transfer.log_ordinal,
                    keyer::account_balance_usd_key(&transfer.from),
                    &(token_price.clone() * balance.to_decimal(token_decimals.into())),
                ),
                None => {}
            };
        }
    }
}
