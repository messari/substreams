use std::ops::Div;

use bigdecimal::BigDecimal;
use hex_literal::hex;

use crate::{abi, math, types};

const CONFIG: Config = Config {
    ethereum: NetworkConfig {
        yearn_lens_oracle: hex!("83d95e0d5f402511db06817aff3f9ea88224b030"),
        chainlink_feed_registry: hex!("47fb2585d2c56fe188d0e6ec628a38b74fceeedf"),
        sushiswap_calculations: hex!("8263e161a855b644f582d9c164c66aabee53f927"),
        usdc_decimals: 6,
        usd_denominations: hex!("0000000000000000000000000000000000000348"),
    },
};

/// Price lib config for all supported networks
struct Config {
    ethereum: NetworkConfig,
}

/// Price lib config for each supported network
struct NetworkConfig {
    yearn_lens_oracle: [u8; 20],
    chainlink_feed_registry: [u8; 20],
    sushiswap_calculations: [u8; 20],
    usdc_decimals: u8,
    usd_denominations: [u8; 20],
}

pub fn get_erc20_price(
    network: types::Network,
    token_address: Vec<u8>,
) -> Result<BigDecimal, String> {
    let network_config = match network {
        types::Network::Ethereum => CONFIG.ethereum,
    };
    via_yearn_lens_oracle(&network_config, token_address.clone())
        .or_else(|| via_chainlink_feed_registry(&network_config, token_address.clone()))
        .or_else(|| via_sushiswap_calculations(&network_config, token_address.clone()))
        .ok_or("price error".to_string())
}

fn via_yearn_lens_oracle(
    network_config: &NetworkConfig,
    token_address: Vec<u8>,
) -> Option<BigDecimal> {
    abi::yearn_lens_oracle::functions::GetPriceUsdcRecommended { token_address }
        .call(network_config.yearn_lens_oracle.to_vec())
        .map(|price_mantissa| {
            math::decimal_from_str(price_mantissa.to_string().as_str())
                .div(math::exponent_to_big_decimal(network_config.usdc_decimals))
        })
}

/// Reference: https://docs.chain.link/docs/feed-registry
fn via_chainlink_feed_registry(
    network_config: &NetworkConfig,
    token_address: Vec<u8>,
) -> Option<BigDecimal> {
    let price_mantissa_res = abi::chainlink_feed_registry::functions::LatestRoundData {
        base: token_address.clone(),
        quote: network_config.usd_denominations.to_vec(),
    }
    .call(network_config.chainlink_feed_registry.to_vec());
    let decimals_res = abi::chainlink_feed_registry::functions::Decimals {
        base: token_address,
        quote: network_config.usd_denominations.to_vec(),
    }
    .call(network_config.chainlink_feed_registry.to_vec());

    if let (Some(price_mantissa), Some(decimals)) = (price_mantissa_res, decimals_res) {
        Some(
            BigDecimal::from(price_mantissa.1)
                .div(math::exponent_to_big_decimal(decimals.as_u64() as u8)),
        )
    } else {
        None
    }
}

fn via_sushiswap_calculations(
    network_config: &NetworkConfig,
    token_address: Vec<u8>,
) -> Option<BigDecimal> {
    abi::sushiswap_calculations::functions::GetPriceUsdc { token_address }
        .call(network_config.sushiswap_calculations.to_vec())
        .map(|price_mantissa| {
            math::decimal_from_str(price_mantissa.to_string().as_str())
                .div(math::exponent_to_big_decimal(network_config.usdc_decimals))
        })
}
