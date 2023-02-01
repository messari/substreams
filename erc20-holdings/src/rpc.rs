use std::fmt::Error;
use substreams::{log, Hex};
use substreams_ethereum::pb::eth;
use substreams_ethereum::rpc::eth_call;
use substreams_helper::utils::{read_string, read_uint32};

// Functions to attempt to get erc20 contract calls

pub const DECIMALS: &str = "313ce567";
pub const NAME: &str = "06fdde03";
pub const SYMBOL: &str = "95d89b41";

pub fn get_erc20_decimals(call_addr: &Vec<u8>) -> Result<u64, Error> {
    let rpc_call_decimal = create_rpc_calls(call_addr, vec![DECIMALS]);
    let rpc_responses_unmarshalled_decimal = eth_call(&rpc_call_decimal);
    let response_decimal = rpc_responses_unmarshalled_decimal.responses;
    if response_decimal.len() < 1 || response_decimal[0].failed {
        return Err(Error);
    }

    let decoded_decimals = read_uint32(response_decimal[0].raw.as_ref());
    if decoded_decimals.is_err() {
        log::info!("Failed to decode decimals");
        return Err(Error);
    }

    return Ok(decoded_decimals.unwrap() as u64);
}

pub fn get_erc20_symbol(call_addr: &Vec<u8>) -> Result<String, Error> {
    let rpc_call_symbol = create_rpc_calls(call_addr, vec![SYMBOL]);
    let rpc_responses_unmarshalled = eth_call(&rpc_call_symbol);
    let responses = rpc_responses_unmarshalled.responses;
    if responses.len() < 2 || responses[1].failed {
        log::info!("Failed to get symbol");
        return Err(Error);
    };

    let decoded_symbol = read_string(responses[2].raw.as_ref());
    if decoded_symbol.is_err() {
        log::info!("Failed to decode symbol");
        return Err(Error);
    }

    return Ok(decoded_symbol.unwrap());
}

pub fn get_erc20_name(call_addr: &Vec<u8>) -> Result<String, Error> {
    let rpc_call_name = create_rpc_calls(call_addr, vec![NAME]);
    let rpc_responses_unmarshalled = eth_call(&rpc_call_name);
    let responses = rpc_responses_unmarshalled.responses;
    if responses.len() < 1 || responses[0].failed {
        return Err(Error);
    };

    let decoded_name = read_string(responses[1].raw.as_ref());
    if decoded_name.is_err() {
        return Err(Error);
    }

    return Ok(decoded_name.unwrap());
}

fn create_rpc_calls(addr: &Vec<u8>, method_signatures: Vec<&str>) -> eth::rpc::RpcCalls {
    let mut rpc_calls = eth::rpc::RpcCalls { calls: vec![] };

    for method_signature in method_signatures {
        rpc_calls.calls.push(eth::rpc::RpcCall {
            to_addr: Vec::from(addr.clone()),
            data: Hex::decode(method_signature).unwrap(),
        })
    }

    return rpc_calls;
}
