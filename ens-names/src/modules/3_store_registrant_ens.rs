use substreams::store::Appender;
use substreams::store::StoreAppend;

use crate::keyer;
use crate::pb::ens::v1 as ENS;

#[substreams::handlers::store]
pub fn store_registrant_ens(map_domains: ENS::Domains, output: StoreAppend<String>) {
    for domain in map_domains.items {
        output.append(
            0,
            keyer::ens_registrant_key(&domain.registrant_address),
            domain.ens_name,
        )
    }
}
