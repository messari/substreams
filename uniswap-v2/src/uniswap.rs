use crate::{pb, math};
use bigdecimal::{BigDecimal, One, Zero};
use num_bigint::BigInt;
use std::borrow::Borrow;
use std::ops::{Add, Mul, Neg};
use std::str;
use std::str::FromStr;
use substreams::{hex, log};
use pb::uniswap_v2::Erc20Token;

pub const UNISWAP_V3_FACTORY: [u8; 20] = hex!("1f98431c8ad98523631ae4a59f267346ea31f984");

pub const _STABLE_COINS: [&str; 6] = [
    "6b175474e89094c44da98b954eedeac495271d0f",
    "a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
    "dac17f958d2ee523a2206206994597c13d831ec7",
    "0000000000085d4780b73119b644ae5ecd22b376",
    "956f47f50a910163d8bf957cf5846d573e7f87ca",
    "4dd28568d05f09b02220b09c2cb307bfd837cb95",
];

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

pub fn convert_token_to_decimal(amount: &BigInt, decimals: u64) -> BigDecimal {
    let big_float_amount = BigDecimal::from_str(amount.to_string().as_str())
        .unwrap()
        .with_prec(100);

    return math::divide_by_decimals(big_float_amount, decimals);
}

pub fn log_token(token: &Erc20Token, index: u64) {
    log::info!(
        "token {} addr: {}, name: {}, symbol: {}, decimals: {}",
        index,
        token.address,
        token.decimals,
        token.symbol,
        token.name
    );
}
