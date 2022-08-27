use substreams::Hex;
use substreams_ethereum::{pb::eth as ethpb, rpc};
use tiny_keccak::{Hasher, Keccak};

#[derive(Debug)]
pub struct RpcCallParams {
    pub to: Vec<u8>,
    pub method: String,
    pub args: Vec<Vec<u8>>,
}

pub fn fetch_many(params: Vec<RpcCallParams>) -> Vec<Result<Vec<u8>, String>> {
    let rpc_calls = ethpb::rpc::RpcCalls {
        calls: params
            .iter()
            .map(|p| ethpb::rpc::RpcCall {
                to_addr: p.to.clone(),
                data: rpc_data(&p.method, &p.args),
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

// Construct rpc data according to https://docs.soliditylang.org/en/develop/abi-spec.html#examples
fn rpc_data(method: &str, args: &Vec<Vec<u8>>) -> Vec<u8> {
    let method_sig = method_signature(method);
    if args.len() == 0 {
        return method_sig;
    }
    let mut data = Vec::with_capacity(8 + args.len() * 32);
    data.extend(method_sig);
    for arg in args {
        data.extend(vec![0u8].repeat(32 - arg.len()));
        data.extend(arg);
    }
    return data;
}

fn method_signature(method: &str) -> Vec<u8> {
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(&Vec::from(method));
    keccak.finalize(&mut output);
    return output[..4].to_vec();
}
