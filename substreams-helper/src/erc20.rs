use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use substreams::Hex;
use substreams_ethereum::pb::eth as ethpb;

use crate::{abi, math, utils};

pub struct Erc20Token {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
    pub total_supply: u64,
}

pub fn get_erc20_token(token_address: Vec<u8>) -> Option<Erc20Token> {
    use abi::erc20::functions;

    let name_res = functions::Name {}.call(token_address.clone());
    let symbol_res = functions::Symbol {}.call(token_address.clone());
    let decimals_res = functions::Decimals {}.call(token_address.clone());
    let total_supply_res = functions::Decimals {}.call(token_address.clone());

    if let (Some(name), Some(symbol), Some(decimals), Some(total_supply)) =
        (name_res, symbol_res, decimals_res, total_supply_res)
    {
        Some(Erc20Token {
            address: Hex::encode(token_address),
            name,
            symbol,
            decimals: decimals.as_u64(),
            total_supply: total_supply.as_u64(),
        })
    } else {
        None
    }
}
