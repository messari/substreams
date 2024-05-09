use std::str::FromStr;

use ethabi::ethereum_types::Address;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_helper::event_handler::EventHandler;
use substreams_helper::hex::Hexable;

use crate::abi::address_provider::events::{AddressModified, NewAddressIdentifier};
use crate::common::constants;
use crate::pb::contract::v1::{Factories, Factory};

#[substreams::handlers::map]
pub fn map_factory_added(blk: eth::Block) -> Result<Factories, substreams::errors::Error> {
    let mut factories: Vec<Factory> = vec![];

    get_factory_from_new_address_identified_events(&blk, &mut factories);
    get_factory_from_address_modified_events(&blk, &mut factories);
    Ok(Factories { factories })
}

fn get_factory_from_new_address_identified_events(blk: &eth::Block, factories: &mut Vec<Factory>) {
    let mut on_new_address_identified =
        |event: NewAddressIdentifier, tx: &eth::TransactionTrace, log: &eth::Log| {
            factories.push(Factory {
                address: event.addr.to_hex(),
                log_index: log.index as i64,
                block_number: blk.number as i64,
                block_timestmap: Some(blk.timestamp().to_owned()),
                transaction_index: tx.index as i64,
                transaction_hash: tx.hash.to_hex(),
            })
        };

    let mut eh = EventHandler::new(blk);
    eh.filter_by_address(vec![Address::from_str(constants::ADRRESS_PROVIDER).unwrap()]);

    eh.on::<NewAddressIdentifier, _>(&mut on_new_address_identified);
    eh.handle_events();
}

fn get_factory_from_address_modified_events(blk: &eth::Block, factories: &mut Vec<Factory>) {
    let mut on_new_address_identified =
        |event: AddressModified, tx: &eth::TransactionTrace, log: &eth::Log| {
            factories.push(Factory {
                address: event.new_address.to_hex(),
                log_index: log.index as i64,
                block_number: blk.number as i64,
                block_timestmap: Some(blk.timestamp().to_owned()),
                transaction_index: tx.index as i64,
                transaction_hash: tx.hash.to_hex(),
            })
        };

    let mut eh = EventHandler::new(blk);
    eh.filter_by_address(vec![Address::from_str(constants::ADRRESS_PROVIDER).unwrap()]);

    eh.on::<AddressModified, _>(&mut on_new_address_identified);
    eh.handle_events();
}
