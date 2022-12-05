// -----------------------------------------
// Pools
// -----------------------------------------
// Pair Create key
pub fn pair_created_key(pool_address: &String) -> String {
    format!("pair_created:{}", pool_address)
}

// PoolKey
pub fn pool_key(pool_address: &String) -> String {
    format!("pool:{}", pool_address)
}

// Pool input token amounts
pub fn pool_input_token_amounts_key(pool_address: &String, token_address: &String) -> String {
    format!("pool:{} :{} :input_token_amounts", pool_address, token_address)
}

// -----------------------------------------
// Store Swaps/Deposits/Withdraw
// -----------------------------------------
pub fn usage_event_key(usage_type: &String, tx_hash: &String, log_index: u32) -> String {
    format!("{}: {}: {}", usage_type, tx_hash, log_index)
}

// -----------------------------------------
// Usage Metrics
// -----------------------------------------
// Hourly active user
pub fn active_users_key(time_window: &String, id: &String) -> String {
    format!("Active Users: (Time Window:{} ID:{})", time_window, id)
}

// Cumulative unique users
pub fn cumulative_unique_users_key() -> String {
    format!("Cumulative Unique Users")
}

// Hourly transaction count key
pub fn transaction_count_key(time_window: &String, id: &String) -> String {
    format!("Transaction: (Time Window:{} ID:{})", time_window, id)
}

// Hourly Swap count key
pub fn usage_count_key(event_name: &String, time_window: &String, id: &String) -> String {
    format!("Usage: (Event Name:{} Time Window:{} ID:{})", event_name, time_window, id)
}

