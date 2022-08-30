use std::ops::Div;
use std::str::FromStr;

use bigdecimal::BigDecimal;
use hex_literal::hex;

use crate::{abi, math, types};

const CONFIG: Config = Config {
    ethereum: NetworkConfig {
        yearn_lens_oracle: hex!("83d95e0d5f402511db06817aff3f9ea88224b030"),
        usdc_decimals: 6,
    },
};

/// Price lib config for all supported networks
struct Config {
    ethereum: NetworkConfig,
}

/// Price lib config for each supported network
struct NetworkConfig {
    yearn_lens_oracle: [u8; 20],
    usdc_decimals: u8,
}

pub fn get_price(network: types::Network, token_address: Vec<u8>) -> Result<BigDecimal, String> {
    let network_config = match network {
        types::Network::Ethereum => CONFIG.ethereum,
    };
    let yearn_lens_price = via_yearn_lens_oracle(&network_config, token_address)
        .unwrap();
    Ok(yearn_lens_price)
}

fn via_yearn_lens_oracle(
    network_config: &NetworkConfig,
    token_address: Vec<u8>,
) -> Option<BigDecimal> {
    abi::yearn_lens_oracle::functions::GetPriceUsdcRecommended { token_address }
        .call(network_config.yearn_lens_oracle.to_vec())
        .map(|price_mantissa| math::decimal_from_str(price_mantissa.to_string().as_str()).div(math::exponent_to_big_decimal(network_config.usdc_decimals)))
}
