use substreams::store::{StoreGet, StoreNew, StoreSet};
use substreams::store::{StoreGetProto, StoreSetProto};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::abi::registrar;
use crate::keyer;
use crate::pb::ens::v1 as ENS;

#[substreams::handlers::store]
pub fn store_registrant_address(
    block: eth::Block,
    store: StoreGetProto<ENS::Domain>,
    output: StoreSetProto<ENS::Domain>,
) {
    for log in block.logs() {
        if let Some(event) = registrar::events::Transfer::match_and_decode(log) {
            let label_hash = Hex::encode(event.token_id.clone().to_signed_bytes_be());
            if let Some(domain) = store.get_last(keyer::ens_domain_key(&label_hash)) {
                let domain = ENS::Domain {
                    registrant_address: Hex(event.to.clone()).to_string(),
                    ..domain
                };

                output.set(
                    log.ordinal(),
                    keyer::ens_domain_key(&domain.label_hash),
                    &domain,
                );
                output.set(
                    log.ordinal(),
                    keyer::ens_domain_key(&domain.ens_name),
                    &domain,
                )
            }
        }
    }
}
