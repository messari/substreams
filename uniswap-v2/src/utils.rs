use substreams::scalar::BigDecimal;

use crate::pb::{erc20::v1::Erc20Token, uniswap::v2::Pool};

/// Returns the timestamp for the start of the most recent day
pub(crate) fn get_latest_day(timestamp: Option<i64>) -> Option<i64> {
    if timestamp.is_none() {
        return None;
    }

    const SECONDS_IN_DAY: i64 = 86400_i64;
    Some(timestamp.unwrap() / SECONDS_IN_DAY)
}

/// Returns the timestamp for the start of the most recent hour
pub(crate) fn get_latest_hour(timestamp: Option<i64>) -> Option<i64> {
    if timestamp.is_none() {
        return None;
    }

    const SECONDS_IN_HOUR: i64 = 3600_i64;
    Some(timestamp.unwrap() / SECONDS_IN_HOUR)
}

pub struct SwappedTokens {
    pub token_in: Option<Erc20Token>,
    pub amount_in: u64,
    pub amount_in_usd: BigDecimal,
    pub token_out: Option<Erc20Token>,
    pub amount_out: u64,
    pub amount_out_usd: BigDecimal,
}

impl Pool {
    pub fn token0_address(&self) -> &String {
        &self.token0_ref().address
    }

    pub fn token0_decimals(&self) -> u64 {
        self.token0_ref().decimals
    }

    pub fn token0_ref(&self) -> &Erc20Token {
        &self.input_tokens.as_ref().unwrap().items[0]
    }

    pub fn token1_address(&self) -> &String {
        &self.token1_ref().address
    }

    pub fn token1_decimals(&self) -> u64 {
        self.token1_ref().decimals
    }

    pub fn token1_ref(&self) -> &Erc20Token {
        &self.input_tokens.as_ref().unwrap().items[1]
    }
}
