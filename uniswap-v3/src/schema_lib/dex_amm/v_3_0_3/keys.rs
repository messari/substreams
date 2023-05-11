use substreams::{Hex};
use crate::pb::dex_amm::v3_0_3::{BigInt};

pub fn get_event_key(hash: &Vec<u8>, log_index: &BigInt) -> String {
    format!("{}:{}", Hex(hash).to_string(), log_index.value)
}

pub fn get_pool_name(protocol: &str, input_tokens: &Vec<Vec<u8>>) -> String {
    // "Uniswap V3 token0/token1/token2..."
    let mut pool_name = protocol.to_string();
    pool_name.push_str(" ");
    for input_token in input_tokens {
        pool_name.push_str(&Hex(input_token).to_string());
        pool_name.push_str("/");
    }
    pool_name.pop();
    pool_name
}

pub fn get_pool_symbol(input_tokens: &Vec<Vec<u8>>) -> String {
    // "UNI-V3-token0-token1-token2..."
    let mut pool_symbol = "".to_string();
    for input_token in input_tokens {
        pool_symbol.push_str(&Hex(input_token).to_string());
        pool_symbol.push_str("/");
    }
    pool_symbol.pop();
    pool_symbol
}
