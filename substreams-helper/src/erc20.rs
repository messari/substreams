use std::ops::Div;

use substreams::Hex;
use substreams::scalar::BigInt;
use crate::abi;

pub struct Erc20Token {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
    pub total_supply: BigInt,
}

pub fn get_erc20_token(token_address: String) -> Option<Erc20Token> {
    use abi::erc20::functions;

    let token_address_bytes = Hex::decode(token_address.clone()).unwrap();
    let name_res = functions::Name {}.call(token_address_bytes.clone());
    let symbol_res = functions::Symbol {}.call(token_address_bytes.clone());
    let decimals_res = functions::Decimals {}.call(token_address_bytes.clone());
    let total_supply_res = functions::TotalSupply {}.call(token_address_bytes.clone());

    if let (Some(name), Some(symbol), Some(decimals), Some(total_supply)) =
        (name_res, symbol_res, decimals_res, total_supply_res)
    {
        let decimals_u64 = decimals.to_u64();
        let total_supply = total_supply.div(BigInt::from(10).pow(decimals_u64 as u32));

        Some(Erc20Token {
            address: token_address.clone(),
            name,
            symbol,
            decimals: decimals_u64,
            total_supply,
        })
    } else {
        None
    }
}
