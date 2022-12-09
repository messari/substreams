use substreams::store::Appender;
use substreams::store::StoreAppend;

use crate::pb::ens::v1 as ENS;

#[substreams::handlers::store]
pub fn store_registrant_ens(map_domains: ENS::Domains, output: StoreAppend<String>) {
    for domain in map_domains.items {
        output.append(
            0,
            format!("Registrant:{}", domain.registrant_address),
            domain.ens_name,
        )
    }
}
