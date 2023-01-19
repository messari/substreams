use crate::pb::evm_token::v1::Token;
use num_bigint;
use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth as pbeth;

pub fn get_eth_token() -> Option<Token> {
    let eth_token = Token {
        address: "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE".to_string(),
        name: "Ethereum".to_string(),
        symbol: "ETH".to_string(),
        decimals: 18 as u64,
        total_supply: None,
    };

    Some(eth_token)
}

// TODO: replace this with substreams::scalar::BigInt once the wrapper is integrated
pub fn bigint_to_string(number: Option<pbeth::v2::BigInt>) -> String {
    number
        .as_ref()
        .map(|value| num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &value.bytes))
        .unwrap_or(BigInt::zero().into())
        .to_string()
}
