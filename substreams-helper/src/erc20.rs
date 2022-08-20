use crate::{utils, math};
use substreams_ethereum::pb::eth as ethpb;
use bigdecimal::{BigDecimal};
use num_bigint::{BigInt};

pub struct Erc20Token {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
}

pub fn get_erc20_token(token_address: &String) -> Option<Erc20Token> {
    let rpc_calls = create_rpc_calls(&hex::decode(token_address).unwrap());
    let rpc_responses_unmarshalled: ethpb::rpc::RpcResponses =
        substreams_ethereum::rpc::eth_call(&rpc_calls);
    let responses = rpc_responses_unmarshalled.responses;
    let mut decimals: u64 = 0;
    match utils::read_uint32(responses[0].raw.as_ref()) {
        Ok(decoded_decimals) => {
            decimals = decoded_decimals as u64;
        }
        Err(err) => match get_static_uniswap_tokens(token_address.as_str()) {
            Some(token) => {
                decimals = token.decimals;
            }
            None => {
                // log::debug!(
                //     "{} is not a an ERC20 token contract decimal `eth_call` failed: {}",
                //     &token_address,
                //     err.msg,
                // );
                return None;
            }
        },
    }

    let mut name = "unknown".to_string();
    match utils::read_string(responses[1].raw.as_ref()) {
        Ok(decoded_name) => {
            name = decoded_name;
        }
        Err(_) => match get_static_uniswap_tokens(token_address.as_str()) {
            Some(token) => {
                name = token.name;
            }
            None => {
                name = utils::read_string_from_bytes(responses[1].raw.as_ref());
            }
        },
    }

    let mut symbol = "unknown".to_string();
    match utils::read_string(responses[2].raw.as_ref()) {
        Ok(s) => {
            symbol = s;
        }
        Err(_) => match get_static_uniswap_tokens(token_address.as_str()) {
            Some(token) => {
                symbol = token.symbol;
            }
            None => {
                symbol = utils::read_string_from_bytes(responses[2].raw.as_ref());
            }
        },
    }

    return Some(Erc20Token {
        address: String::from(token_address),
        name,
        symbol,
        decimals,
    });
}

fn create_rpc_calls(addr: &Vec<u8>) -> ethpb::rpc::RpcCalls {
    let decimals = hex::decode("313ce567").unwrap();
    let name = hex::decode("06fdde03").unwrap();
    let symbol = hex::decode("95d89b41").unwrap();

    return ethpb::rpc::RpcCalls {
        calls: vec![
            ethpb::rpc::RpcCall {
                to_addr: Vec::from(addr.clone()),
                method_signature: decimals,
            },
            ethpb::rpc::RpcCall {
                to_addr: Vec::from(addr.clone()),
                method_signature: name,
            },
            ethpb::rpc::RpcCall {
                to_addr: Vec::from(addr.clone()),
                method_signature: symbol,
            },
        ],
    };
}


// hard-coded tokens which have various behaviours but for which a UniswapV3 valid pool
// exists, some are tokens which were migrated to a new address, etc.
pub fn get_static_uniswap_tokens(token_address: &str) -> Option<Erc20Token> {
    return match token_address {
        "e0b7927c4af23765cb51314a0e0521a9645f0e2a" => Some(Erc20Token {
            // add DGD
            address: "e0b7927c4af23765cb51314a0e0521a9645f0e2a".to_string(),
            name: "DGD".to_string(),
            symbol: "DGD".to_string(),
            decimals: 9,
        }),
        "7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9" => Some(Erc20Token {
            // add AAVE
            address: "7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9".to_string(),
            name: "Aave Token".to_string(),
            symbol: "AAVE".to_string(),
            decimals: 18,
        }),
        "eb9951021698b42e4399f9cbb6267aa35f82d59d" => Some(Erc20Token {
            // add LIF
            address: "eb9951021698b42e4399f9cbb6267aa35f82d59d".to_string(),
            name: "LIF".to_string(),
            symbol: "LIF".to_string(),
            decimals: 18,
        }),
        "bdeb4b83251fb146687fa19d1c660f99411eefe3" => Some(Erc20Token {
            // add SVD
            address: "bdeb4b83251fb146687fa19d1c660f99411eefe3".to_string(),
            name: "savedroid".to_string(),
            symbol: "SVD".to_string(),
            decimals: 18,
        }),
        "bb9bc244d798123fde783fcc1c72d3bb8c189413" => Some(Erc20Token {
            // add TheDAO
            address: "bb9bc244d798123fde783fcc1c72d3bb8c189413".to_string(),
            name: "TheDAO".to_string(),
            symbol: "TheDAO".to_string(),
            decimals: 16,
        }),
        "38c6a68304cdefb9bec48bbfaaba5c5b47818bb2" => Some(Erc20Token {
            // add HPB
            address: "38c6a68304cdefb9bec48bbfaaba5c5b47818bb2".to_string(),
            name: "HPBCoin".to_string(),
            symbol: "HPB".to_string(),
            decimals: 18,
        }),
        _ => None,
    };
}

pub fn log_token(token: &Erc20Token, index: u64) {
    // log::info!(
    //     "token {} addr: {}, name: {}, symbol: {}, decimals: {}",
    //     index,
    //     token.address,
    //     token.decimals,
    //     token.symbol,
    //     token.name
    // );
}
