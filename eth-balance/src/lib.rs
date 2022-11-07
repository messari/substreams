#[rustfmt::skip]
pub mod abi;
#[rustfmt::skip]
pub mod pb;

use std::str::FromStr;
use substreams::store::StoreAdd;
use substreams::store::StoreAddBigInt;
use substreams::store::StoreNew;
use substreams::scalar::BigInt;
use substreams_helper::types::Address;
use substreams::{hex, log, store, Hex};
use pb::eth_balance::v1::{TransferEvents};
use substreams_ethereum::{pb::eth as pbeth, NULL_ADDRESS};


pub fn account_balance_key(account_address: &String) -> String {
    format!("account_balance:{}", account_address)
}


#[substreams::handlers::map]
fn map_block_to_transfers(
    block: pbeth::v2::Block
) ->Result<TransferEvents, substreams::errors::Error> {
    const ETH_ADDRESS: Address = hex!("0c10bf8fcb7bf5412187a595ab97a3609160b5c6"); // USDD

    let transfer_events = TransferEvents { items: vec![] };

    for transaction in block.transaction_traces {
        let to = Hex(transaction.clone().to).to_string();
        let from = Hex(transaction.clone().from).to_string();
        let transaction_hash = Hex(transaction.clone().hash).to_string();
        let gas_used = transaction.clone().gas_used;
        let gas_price = match transaction.clone().gas_price {
            Some(value) => BigInt::try_from(value.bytes).unwrap(),
            None => BigInt::from(0)
        };

        let value = match transaction.clone().value {
            Some(value) => BigInt::try_from(value.bytes).unwrap(),
            None => BigInt::from(0)
        };

        log::info!(
            "tx_hash: {}, from: {}, to: {}, gas_used: {}, gas_price: {}, amount: {:?}", 
            transaction_hash,
            from,
            to,
            gas_used,
            gas_price,
            value
        );
    }
    Ok(transfer_events)
} 

#[substreams::handlers::store]
fn store_balance(transfers: TransferEvents, output: store::StoreAddBigInt) {
    log::info!("Stored events {}", transfers.items.len());
    for transfer in transfers.items {
        output.add(
            transfer.log_ordinal,
            account_balance_key(&transfer.to),
            &BigInt::from_str(transfer.amount.as_str()).unwrap(),
        );

        if Hex::decode(transfer.from.clone()).unwrap() != NULL_ADDRESS {
            output.add(
                transfer.log_ordinal,
                account_balance_key(&transfer.from),
                &BigInt::from_str((transfer.amount).as_str()).unwrap().neg(),
            );
        }
    }
}