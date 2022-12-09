pub fn chainlink_aggregator_key(address: &String) -> String {
    format!("aggregator:{}", address)
}

pub fn chainlink_asset_key(asset_address: &String) -> String {
    format!("chainlink_price:{}", asset_address)
}
