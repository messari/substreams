use crate::pb::eth_balance::v1::Token;
use num_bigint;
use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth as pbeth;

pub fn get_eth_token() -> Option<Token> {
    let eth_token = Token {
        address: "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE".to_string(),
        name: "Ethereum".to_string(),
        symbol: "ETH".to_string(),
        decimals: 18_u64,
    };

    Some(eth_token)
}

// TODO: replace this with substreams::scalar::BigInt once the wrapper is integrated
// TODO: make this an impl of fmt::Display
pub fn bigint_to_string(number: Option<pbeth::v2::BigInt>) -> String {
    number
        .as_ref()
        .map(|value| num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &value.bytes))
        .unwrap_or(BigInt::zero().into())
        .to_string()
}
