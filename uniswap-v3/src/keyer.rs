use substreams::scalar::BigInt;
use substreams::{log, Hex};


pub fn get_data_source_key(address: &Vec<u8>) -> String {
    format!("DataSource:{}", Hex(address).to_string())
}

pub fn get_event_key(hash: &Vec<u8>, log_index: &u32) -> String {
    format!("{}:{}", Hex(hash).to_string(), log_index)
}

pub fn get_input_tokens_key(address: &Vec<u8>) -> String {
    format!("LiquidityPool:InputTokens:{}", Hex(address).to_string())
}
