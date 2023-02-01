use std::ops::Add;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{DeltaBigDecimal, StoreGet, StoreGetBigDecimal, StoreGetBigInt};

use crate::pb::erc20::v1::Erc20Token;
use crate::pb::uniswap::v2::{self as uniswap, Pool};
use crate::store_key::StoreKey;

pub(crate) fn get_latest_day(timestamp: Option<i64>) -> Option<i64> {
    if timestamp.is_none() {
        return None;
    }

    const SECONDS_IN_DAY: i64 = 86400_i64;
    Some(timestamp.unwrap() / SECONDS_IN_DAY)
}

pub(crate) fn get_latest_hour(timestamp: Option<i64>) -> Option<i64> {
    if timestamp.is_none() {
        return None;
    }

    const SECONDS_IN_HOUR: i64 = 3600_i64;
    Some(timestamp.unwrap() / SECONDS_IN_HOUR)
}

pub(crate) fn get_delta(value: DeltaBigDecimal) -> BigDecimal {
    value.new_value.clone() - value.old_value.clone()
}

impl Into<BigInt> for uniswap::BigInt {
    fn into(self) -> BigInt {
        BigInt::from_unsigned_bytes_le(self.value.as_slice())
    }
}

impl From<BigInt> for uniswap::BigInt {
    fn from(big_int: BigInt) -> Self {
        uniswap::BigInt {
            value: big_int.to_bytes_le().1,
            repr: big_int.to_string(),
        }
    }
}

impl Into<BigDecimal> for uniswap::BigDecimal {
    fn into(self) -> BigDecimal {
        BigDecimal::new(self.int_val.unwrap().into(), -self.scale)
    }
}

impl From<BigDecimal> for uniswap::BigDecimal {
    fn from(big_decimal: BigDecimal) -> Self {
        let (int_val, scale) = big_decimal.with_prec(18).as_bigint_and_exponent();

        uniswap::BigDecimal {
            int_val: Some(uniswap::BigInt::from(BigInt::from(int_val))),
            scale,
        }
    }
}

impl Add for uniswap::BigDecimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        (Into::<BigDecimal>::into(self) + Into::<BigDecimal>::into(other)).into()
    }
}

impl Pool {
    pub fn token0_ref(&self) -> &Erc20Token {
        &self.input_tokens.as_ref().unwrap().items[0]
    }

    pub fn token0_address(&self) -> &String {
        &self.token0_ref().address
    }

    pub fn token0_decimals(&self) -> u64 {
        self.token0_ref().decimals
    }

    pub fn token0_balance(&self, native_tvl_store: &StoreGetBigInt) -> BigInt {
        native_tvl_store
            .get_last(
                StoreKey::InputTokenBalance
                    .get_pool_token_balance_key(&self.address, self.token0_address()),
            )
            .unwrap_or(BigInt::zero())
    }

    pub fn token0_price(&self, price_store: &StoreGetBigDecimal) -> BigDecimal {
        price_store
            .get_last(StoreKey::TokenPrice.get_unique_token_key(&self.token0_address()))
            .unwrap_or(BigDecimal::zero())
    }

    pub fn token1_ref(&self) -> &Erc20Token {
        &self.input_tokens.as_ref().unwrap().items[1]
    }

    pub fn token1_address(&self) -> &String {
        &self.token1_ref().address
    }

    pub fn token1_decimals(&self) -> u64 {
        self.token1_ref().decimals
    }

    pub fn token1_balance(&self, native_tvl_store: &StoreGetBigInt) -> BigInt {
        native_tvl_store
            .get_last(
                StoreKey::InputTokenBalance
                    .get_pool_token_balance_key(&self.address, self.token1_address()),
            )
            .unwrap_or(BigInt::zero())
    }

    pub fn token1_price(&self, price_store: &StoreGetBigDecimal) -> BigDecimal {
        price_store
            .get_last(StoreKey::TokenPrice.get_unique_token_key(&self.token1_address()))
            .unwrap_or(BigDecimal::zero())
    }

    pub fn input_token_balances(&self, native_tvl_store: &StoreGetBigInt) -> Vec<String> {
        vec![
            self.token0_balance(native_tvl_store).to_string(),
            self.token1_balance(native_tvl_store).to_string(),
        ]
    }
}
