use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use substreams_ethereum::pb::eth as ethpb;

use crate::{abi, math, utils};

pub struct Erc20Token {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
}

pub fn get_erc20_token(token_address: &String) -> Option<Erc20Token> {
    use abi::erc20::functions;

    let token_address_vu8 = hex::decode(token_address).unwrap();
    let name_res = functions::Name {}.call(token_address_vu8.clone());
    let symbol_res = functions::Symbol {}.call(token_address_vu8.clone());
    let decimals_res = functions::Decimals {}.call(token_address_vu8.clone());

    if let (Some(name), Some(symbol), Some(decimals)) = (name_res, symbol_res, decimals_res) {
        Some(
            Erc20Token {
                address: token_address.to_string(),
                name,
                symbol,
                decimals: decimals.as_u64(),
            }
        )
    } else {
        None
    }
}
