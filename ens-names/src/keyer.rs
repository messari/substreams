pub fn ens_domain_key(ens_hash: &String) -> String {
    format!("domain:{}", ens_hash)
}

pub fn ens_registrant_key(registrant_address: &String) -> String {
    format!("registrant:{}", registrant_address)
}
