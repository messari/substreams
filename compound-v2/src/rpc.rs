use substreams::Hex;
use substreams_ethereum::{pb::eth, rpc};

use crate::utils::rpc_data;
use crate::{
    pb::compound::Token,
    utils::{read_string, read_uint32},
};

// TODO: add transformer function to Params
#[derive(Debug)]
pub struct RpcCallParams {
    pub to: Vec<u8>,
    pub method: String,
    pub args: Vec<Vec<u8>>,
}

pub fn fetch_token(addr: Vec<u8>) -> Result<Token, String> {
    let responses = fetch_many(vec![
        RpcCallParams {
            to: addr.clone(),
            method: "decimals()".to_string(),
            args: vec![],
        },
        RpcCallParams {
            to: addr.clone(),
            method: "name()".to_string(),
            args: vec![],
        },
        RpcCallParams {
            to: addr.clone(),
            method: "symbol()".to_string(),
            args: vec![],
        },
    ]);

    // TODO: remove dangerous unwrap
    let decoded_decimals = read_uint32(responses[0].clone().unwrap().as_ref());
    if decoded_decimals.is_err() {
        return Err(format!(
            "({}).decimal() decode failed: {}",
            Hex(addr),
            decoded_decimals.err().unwrap()
        ));
    }

    let decoded_name = read_string(responses[1].clone().unwrap().as_ref());
    if decoded_name.is_err() {
        return Err(format!(
            "({}).name() decode failed: {}",
            Hex(addr),
            decoded_name.err().unwrap()
        ));
    }

    let decoded_symbol = read_string(responses[2].clone().unwrap().as_ref());
    if decoded_symbol.is_err() {
        return Err(format!(
            "({}).symbol() decode failed: {}",
            Hex(addr),
            decoded_symbol.err().unwrap()
        ));
    }

    return Ok(Token {
        id: addr,
        name: decoded_name.unwrap(),
        symbol: decoded_symbol.unwrap(),
        decimals: decoded_decimals.unwrap() as u64,
    });
}

pub fn fetch_many(params: Vec<RpcCallParams>) -> Vec<Result<Vec<u8>, String>> {
    let rpc_calls = eth::rpc::RpcCalls {
        calls: params
            .iter()
            .map(|p| eth::rpc::RpcCall {
                to_addr: p.to.clone(),
                method_signature: rpc_data(&p.method, &p.args),
            })
            .collect(),
    };

    return rpc::eth_call(&rpc_calls)
        .responses
        .iter()
        .enumerate()
        .map(|(i, r)| {
            if r.failed {
                Err(format!("eth_call failed: {:?}", params[i]))
            } else {
                Ok(r.raw.clone())
            }
        })
        .collect();
}

pub fn fetch(param: RpcCallParams) -> Result<Vec<u8>, String> {
    return fetch_many(vec![param]).into_iter().next().unwrap();
}
