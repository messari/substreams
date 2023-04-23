use std::ops::Sub;

use substreams::hex;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{DeltaBigDecimal, StoreGet, StoreGetBigInt};

use crate::pb::erc20::v1::Erc20Token;
use crate::pb::uniswap::v2::Pool;
use crate::store_key::StoreKey;

pub const UNISWAP_V2_FACTORY: &str = "5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f";
pub const UNISWAP_V2_FACTORY_SLICE: [u8; 20] = hex!("5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f");

pub fn get_day_id(timestamp: i64) -> BigInt {
    const SECONDS_IN_DAY: i64 = 86400_i64;
    BigInt::from(timestamp / SECONDS_IN_DAY)
}

pub fn get_hour_id(timestamp: i64) -> BigInt {
    const SECONDS_IN_HOUR: i64 = 3600_i64;
    BigInt::from(timestamp / SECONDS_IN_HOUR)
}

pub fn delta_value(delta: &DeltaBigDecimal) -> BigDecimal {
    let old_value = delta.old_value.clone();
    let new_value = delta.new_value.clone();

    return new_value.clone().sub(old_value);
}

impl Pool {
    pub fn token0_ref(&self) -> Erc20Token {
        self.input_tokens.as_ref().unwrap().items[0].clone()
    }

    pub fn token0_address(&self) -> String {
        self.token0_ref().name
    }

    pub fn token0_decimals(&self) -> u64 {
        self.token0_ref().decimals as u64
    }

    pub fn token0_balance(&self, balances_store: &StoreGetBigInt) -> BigInt {
        balances_store
            .get_last(StoreKey::Token0Balance.get_unique_pool_key(&self.address))
            .unwrap_or(BigInt::zero())
    }

    pub fn token1_ref(&self) -> Erc20Token {
        self.input_tokens.as_ref().unwrap().items[1].clone()
    }

    pub fn token1_address(&self) -> String {
        self.token1_ref().name
    }

    pub fn token1_decimals(&self) -> u64 {
        self.token1_ref().decimals as u64
    }

    pub fn token1_balance(&self, balances_store: &StoreGetBigInt) -> BigInt {
        balances_store
            .get_last(StoreKey::Token1Balance.get_unique_pool_key(&self.address))
            .unwrap_or(BigInt::zero())
    }

    pub fn input_tokens(&self) -> Vec<String> {
        vec![self.token0_address(), self.token1_address()]
    }

    pub fn output_token(&self) -> String {
        self.output_token.clone().unwrap().address
    }
}
