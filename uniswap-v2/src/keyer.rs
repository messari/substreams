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

// Total Value Locked USD
pub fn pool_tvl_usd_key(pool_address: &String) -> String {
    format!("pool:{}:tvl_usd", pool_address)
}

// Cumulative Supply Side Revenue USD
pub fn pool_cumulative_supply_side_revenue_usd_key(pool_address: &String) -> String {
    format!("pool:{}:cumulative_supply_side_revenue_usd", pool_address)
}

// Cumulative Protocol Side Revenue USD
pub fn pool_cumulative_protocol_side_revenue_usd_key(pool_address: &String) -> String {
    format!("pool:{}:cumulative_protocol_side_revenue_usd", pool_address)
}

// Cumulative Total Revenue USD
pub fn pool_cumulative_total_revenue_usd_key(pool_address: &String) -> String {
    format!("pool:{}:cumulative_total_revenue_usd", pool_address)
}

// Pool input token amounts
pub fn pool_input_token_amounts_key(pool_address: &String, token_address: &String) -> String {
    format!("pool:{}:{}:input_token_amounts", pool_address, token_address)
}

// Pool output token amounts
pub fn pool_output_token_supply_key(pool_address: &String, token_address: &String) -> String {
    format!("pool:{}:{}:output_token_amounts", pool_address, token_address)
}

// Pool snapshot input token amounts
pub fn pool_snapshot_input_token_amounts_key(pool_address: &String, token_address: &String, day_id: &String) -> String {
    format!("pool:{}:{}:{}:snapshot_input_token_amounts", pool_address, token_address, day_id)
}

// Pool snapshot output token amounts
pub fn pool_snapshot_output_token_supply_key(pool_address: &String, token_address: &String, day_id: &String) -> String {
    format!("pool:{}:{}:{}:snapshot_output_token_amounts", pool_address, token_address, day_id)
}

// Pool snapshot input token volume
pub fn pool_snapshot_input_token_volume_key(pool_address: &String, token_address: &String, day_id: &String) -> String {
    format!("pool:{}:{}:{}:snapshot_input_token_volume", pool_address, token_address, day_id)
}

// -----------------------------------------
// Store Deposits
// -----------------------------------------
pub fn deposit_key(tx_hash: &String, log_index: u32) -> String {
    format!("deposit:{}:{}", tx_hash, log_index)
}

// -----------------------------------------
// Store Withdrawals
// -----------------------------------------
pub fn withdraw_key(tx_hash: &String, log_index: u32) -> String {
    format!("withdraw:{}:{}", tx_hash, log_index)
}

// -----------------------------------------
// Store Swaps
// -----------------------------------------
pub fn swap_key(tx_hash: &String, log_index: u32) -> String {
    format!("swap:{}:{}", tx_hash, log_index)
}

// -----------------------------------------
// Usage Metrics
// -----------------------------------------
// Hourly active users
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

// -----------------------------------------
// Daily Financial Metrics
// -----------------------------------------
// total value locked USD
pub fn total_value_locked_usd_key(day_id: &String) -> String {
    format!("total_value_locked_usd:{}", day_id)
}

// Protocol Controlled Value USD
pub fn protocol_controlled_value_usd_key(day_id: &String) -> String {
    format!("protocol_controlled_value_usd:{}", day_id)
}

// Daily Volume USD
pub fn daily_volume_usd_key(day_id: &String) -> String {
    format!("daily_volume_usd:{}", day_id)
}

// Cumulative Volume USD
pub fn cumulative_volume_usd_key(day_id: &String) -> String {
    format!("cumulative_volume_usd:{}", day_id)
}

// Daily Supply Side Revenue USD
pub fn daily_supply_side_revenue_usd_key(day_id: &String) -> String {
    format!("daily_supply_side_revenue_usd:{}", day_id)
}

// Cumulative Supply Side Revenue USD
pub fn cumulative_supply_side_revenue_usd_key(day_id: &String) -> String {
    format!("cumulative_supply_side_revenue_usd:{}", day_id)
}

// Daily Protocol Side Revenue USD
pub fn daily_protocol_side_revenue_usd_key(day_id: &String) -> String {
    format!("daily_protocol_side_revenue_usd:{}", day_id)
}

// Cumulative Protocol Side Revenue USD
pub fn cumulative_protocol_side_revenue_usd_key(day_id: &String) -> String {
    format!("cumulative_protocol_side_revenue_usd:{}", day_id)
}

// Daily Total Revenue USD
pub fn daily_total_revenue_usd_key(day_id: &String) -> String {
    format!("daily_total_revenue_usd:{}", day_id)
}

// Cumulative Total Revenue USD
pub fn cumulative_total_revenue_usd_key(day_id: &String) -> String {
    format!("cumulative_total_revenue_usd:{}", day_id)
}