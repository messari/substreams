use crate::schema_lib::dex_amm::v_3_0_3::enums;
use substreams::{scalar, Hex};

pub fn get_event_key(hash: &Vec<u8>, log_index: u32) -> Vec<u8> {
    format!("{}-{}", Hex(hash).to_string(), log_index.to_string())
        .as_bytes()
        .to_vec()
}

pub fn get_liquidity_pool_fee_key(
    pool_address: &str,
    fee_type: &enums::LiquidityPoolFeeType,
) -> String {
    format!("{}-{}", pool_address, fee_type.to_string())
}

pub fn get_tick_key(pool_address: &str, tick_index: &scalar::BigInt) -> String {
    format!("{}-{}", pool_address, tick_index.to_string())
}

pub fn get_position_key(pool_address: &str, position_id: &str) -> String {
    format!("{}-{}", pool_address, position_id)
}

pub fn get_pool_name(protocol: &str, input_token_symbols: &Vec<String>) -> String {
    // "Uniswap V3 token0/token1/token2..."
    let mut pool_name = protocol.to_string();
    pool_name.push_str(" ");
    for input_token_symbol in input_token_symbols {
        pool_name.push_str(&input_token_symbol);
        pool_name.push_str("/");
    }
    pool_name.pop();
    pool_name
}

pub fn get_pool_symbol(input_token_symbols: &Vec<String>) -> String {
    // "UNI-V3-token0-token1-token2..."
    let mut pool_symbol = "".to_string();
    for input_token_symbol in input_token_symbols {
        pool_symbol.push_str(&input_token_symbol);
        pool_symbol.push_str("/");
    }
    pool_symbol.pop();
    pool_symbol
}
