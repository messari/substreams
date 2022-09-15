use std::ops::Div;

use bigdecimal::{BigDecimal, Num};
use ethabi::ethereum_types::U256;
use num_bigint::BigInt;
use substreams::{log, Hex};
use substreams_ethereum::pb::eth as ethpb;

use crate::{abi, math, utils};

pub struct Erc20Token {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
    pub total_supply: U256,
}

pub fn get_erc20_token(token_address: Vec<u8>) -> Option<Erc20Token> {
    use abi::erc20::functions;

    let name_res = functions::Name {}.call(token_address.clone());
    let symbol_res = functions::Symbol {}.call(token_address.clone());
    let decimals_res = functions::Decimals {}.call(token_address.clone());
    let total_supply_res = functions::TotalSupply {}.call(token_address.clone());

    if let (Some(name), Some(symbol), Some(decimals), Some(total_supply)) =
        (name_res, symbol_res, decimals_res, total_supply_res)
    {
        let total_supply = total_supply.div(U256::from(10 as i32).pow(decimals.into()));

        Some(Erc20Token {
            address: Hex::encode(token_address),
            name,
            symbol,
            decimals: decimals.as_u64(),
            total_supply,
        })
    } else {
        None
    }
}
