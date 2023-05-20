use substreams::{Hex};
use crate::pb;

pub fn get_event_key(hash: &Vec<u8>, log_index: &pb::common::v1::BigInt) -> String {
    format!("{}:{}", Hex(hash).to_string(), log_index.value)
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
