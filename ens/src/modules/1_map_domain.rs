use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::abi::controller;
use crate::pb::ens::v1 as ENS;
use crate::utils::name_hash;

#[substreams::handlers::map]
pub fn map_domain(block: eth::Block) -> Result<ENS::Domains, substreams::errors::Error> {
    let mut items: Vec<ENS::Domain> = vec![];

    for log in block.logs() {
        if let Some(event) = controller::events::NameRegistered::match_and_decode(log) {
            let ens_name = format!("{}.eth", &event.name);

            items.push(ENS::Domain {
                ens_name: ens_name.clone(),
                name_hash: Hex(name_hash(ens_name.as_str()).0.to_vec()).to_string(),
                label_name: event.name.clone(),
                label_hash: Hex(&event.label.to_vec()).to_string(),
                controller_address: Hex(&log.address()).to_string(),
                registrant_address: Hex(&event.owner.to_vec()).to_string(),
                transaction_hash: Hex(&log.receipt.transaction.hash).to_string(),
            })
        }
    }

    Ok(ENS::Domains { items })
}
