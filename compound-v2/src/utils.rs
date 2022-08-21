use crate::rpc;
use bigdecimal::{BigDecimal, One, Zero};
use num_bigint::BigUint;
use std::ops::{Div, Mul};
use std::str;
use std::str::FromStr;
use substreams::Hex;
use tiny_keccak::{Hasher, Keccak};

pub const MANTISSA_FACTOR: u64 = 18;
pub const CTOKEN_DECIMALS: u64 = 8;

pub fn read_uint32(input: &[u8]) -> Result<u32, String> {
    if input.len() != 32 {
        return Err(format!("uint32 invalid length: {}", input.len()));
    }
    let as_array: [u8; 4] = input[28..32].try_into().unwrap();
    Ok(u32::from_be_bytes(as_array))
}

pub fn read_string(input: &[u8]) -> Result<String, String> {
    if input.len() < 96 {
        return Err(format!("string invalid length: {}", input.len()));
    }

    let next = read_uint32(&input[0..32])?;
    if next != 32 {
        return Err(format!("invalid string uint32 value: {}", next));
    };

    let size = read_uint32(&input[32..64])?;
    let end: usize = (size as usize) + 64;

    if end > input.len() {
        return Err(format!(
            "invalid input: end {:?}, length: {:?}, next: {:?}, size: {:?}, whole: {:?}",
            end,
            input.len(),
            next,
            size,
            Hex::encode(&input[32..64])
        ));
    }

    Ok(String::from_utf8_lossy(&input[64..end]).to_string())
}

pub fn string_to_bigdecimal(input: &[u8]) -> BigDecimal {
    return BigDecimal::from_str(str::from_utf8(input).unwrap()).unwrap();
}

pub fn bytes_to_bigdecimal(input: &[u8]) -> BigDecimal {
    return BigDecimal::from_str(&BigUint::from_bytes_be(input).to_string()).unwrap();
}

pub fn exponent_to_big_decimal(decimals: u64) -> BigDecimal {
    let mut result = BigDecimal::one();
    let big_decimal_ten: &BigDecimal = &BigDecimal::from(10 as u64);
    for _ in 0..decimals {
        result = result.mul(big_decimal_ten);
    }
    return result;
}

// Construct rpc data according to https://docs.soliditylang.org/en/develop/abi-spec.html#examples
pub fn rpc_data(method: &str, args: &Vec<Vec<u8>>) -> Vec<u8> {
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

// "name()" -> "06fdde03"
// Same effect as: printf "name()" | keccak256 --no-0x | cut -c 1-8
fn method_signature(method: &str) -> Vec<u8> {
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(&Vec::from(method));
    keccak.finalize(&mut output);
    return output[..4].to_vec();
}

// Based on official subgraph of Compound V2 https://github.com/graphprotocol/compound-v2-subgraph/blob/master/src/mappings/markets.ts
// The result could mean either eth or usd, depending on the block_number
// We delegate the check to get_underlying_price_usd
// TODO: consider removing getPrice given it always return 0 for block_number < 7710795
fn get_underlying_price_eth_or_usd(
    ctoken_address: Vec<u8>,
    underlying_address: Vec<u8>,
    oracle: Vec<u8>,
    block_number: u64,
    underlying_decimals: u64,
) -> Result<BigDecimal, String> {
    if block_number < 7710795 {
        // oracle gains new schema from this block on
        rpc::fetch(rpc::RpcCallParams {
            to: oracle,
            method: "getPrice(address)".to_string(),
            args: vec![underlying_address],
        })
        .map(|x| bytes_to_bigdecimal(x.as_ref()).div(exponent_to_big_decimal(18)))
    } else {
        rpc::fetch(rpc::RpcCallParams {
            to: oracle,
            method: "getUnderlyingPrice(address)".to_string(),
            args: vec![ctoken_address],
        })
        .map(|x| {
            bytes_to_bigdecimal(x.as_ref())
                .div(exponent_to_big_decimal(18 - underlying_decimals + 18))
        })
    }
}

// Based on official subgraph of Compound V2 https://github.com/graphprotocol/compound-v2-subgraph/blob/master/src/mappings/markets.ts
// Rule 1: after block 10678764 price is calculated based on USD instead of ETH
// Rule 2: use sai until usdc is listed in the market
pub fn get_underlying_price_usd(
    ctoken_address: Vec<u8>,
    underlying_address: Vec<u8>,
    oracle: Vec<u8>,
    block_number: u64,
    underlying_decimals: u64,
) -> Result<BigDecimal, String> {
    let price_res = get_underlying_price_eth_or_usd(
        ctoken_address,
        underlying_address,
        oracle.clone(),
        block_number,
        underlying_decimals,
    );
    if block_number > 10678764 {
        price_res
    } else {
        price_res.and_then(|token_price_eth| {
            if block_number < 7715827 {
                // use sai until usdc is listed in the market
                get_underlying_price_eth_or_usd(
                    Hex::decode("f5dce57282a584d2746faf1593d3121fcac444dc").unwrap(),
                    Hex::decode("89d24a6b4ccb1b6faa2625fe562bdd9a23260359").unwrap(),
                    oracle,
                    block_number,
                    18,
                )
            } else {
                // usdc
                get_underlying_price_eth_or_usd(
                    Hex::decode("39aa39c021dfbae8fac545936693ac917d5e7563").unwrap(),
                    Hex::decode("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap(),
                    oracle,
                    block_number,
                    6,
                )
            }
            .map(|stablecoin_price_eth| {
                if stablecoin_price_eth.is_zero() {
                    BigDecimal::zero()
                } else {
                    token_price_eth.div(stablecoin_price_eth)
                }
            })
        })
    }
}
