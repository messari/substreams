pub fn ens_domain_key(asset_address: &String) -> String {
    format!("domain:{}", asset_address)
}

pub fn ens_registrant_key(address: &String) -> String {
    format!("registrant:{}", address)
}
