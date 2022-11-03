pub fn pool_key(pool_address: &String) -> String {
    format!("pool:{}", pool_address)
}

pub fn swap_key(tx_hash: &String, log_index: u32) -> String {
    format!("swap:{}:{}", tx_hash, log_index)
}
