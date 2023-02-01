use crate::pb::erc20::v1::Erc20Token;
use num_bigint;
use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth as pbeth;

pub const ETH_ADDRESS: &str = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE";

pub fn get_eth_token() -> Option<Erc20Token> {
    let eth_token = Erc20Token {
        address: ETH_ADDRESS.to_string(),
        name: "Ethereum".to_string(),
        symbol: "ETH".to_string(),
        decimals: 18_u64,
        tx_created: "".to_string(),
        block_created: 0_u64,
    };

    Some(eth_token)
}

pub fn bigint_to_string(number: Option<pbeth::v2::BigInt>) -> String {
    number
        .as_ref()
        .map(|value| num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &value.bytes))
        .unwrap_or(BigInt::zero().into())
        .to_string()
}
