pub fn chainlink_aggregator_key(address: &String) -> String {
    format!("aggregator:{}", address)
}

pub fn chainlink_asset_key(asset_address: &String) -> String {
    format!("chainlink_price:{}", asset_address)
}

pub fn pair_info_key(asset_address: &String) -> String {
    format!("pair_info:{}", asset_address)
}

pub fn uniswap_asset_key(asset_address: &String) -> String {
    format!("uniswap_price:{}", asset_address)
}
