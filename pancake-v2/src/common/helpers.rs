use ethabi::ethereum_types::H160;
use substreams::scalar::BigInt;
use substreams_helper::hex::Hexable;

use crate::pb::erc20::v1::Erc20Token;
use crate::pb::pancake::v2::Pool;
use crate::{abi::ERC20, store_key::StoreKey};
use substreams::store::{StoreGet, StoreGetBigInt};

pub struct TokenContract(H160);

impl TokenContract {
    pub fn new(address: H160) -> Self {
        TokenContract(address)
    }

    fn get_name(&self) -> String {
        ERC20::functions::Name {}
            .call(self.0.as_bytes().to_vec())
            .unwrap_or(String::new())
    }

    fn get_symbol(&self) -> String {
        ERC20::functions::Symbol {}
            .call(self.0.as_bytes().to_vec())
            .unwrap_or(String::new())
    }

    fn get_decimals(&self) -> BigInt {
        ERC20::functions::Decimals {}
            .call(self.0.as_bytes().to_vec())
            .unwrap_or(BigInt::from(18))
    }

    pub fn as_struct(&self) -> Erc20Token {
        Erc20Token {
            name: self.get_name(),
            symbol: self.get_symbol(),
            address: self.0.to_hex(),
            decimals: self.get_decimals().try_into().unwrap_or(18),
        }
    }
}

impl Pool {
    pub fn token0_ref(&self) -> Erc20Token {
        self.input_tokens.as_ref().unwrap().items[0].clone()
    }

    pub fn token0_address(&self) -> String {
        self.token0_ref().address
    }

    pub fn token0_decimals(&self) -> u64 {
        self.token0_ref().decimals as u64
    }

    pub fn token0_balance(&self, ordinal: u64, balances_store: &StoreGetBigInt) -> BigInt {
        balances_store
            .get_at(
                ordinal,
                StoreKey::Token0Balance.get_unique_pool_key(&self.address),
            )
            .unwrap_or(BigInt::zero())
    }

    pub fn token1_ref(&self) -> Erc20Token {
        self.input_tokens.as_ref().unwrap().items[1].clone()
    }

    pub fn token1_address(&self) -> String {
        self.token1_ref().address
    }

    pub fn token1_decimals(&self) -> u64 {
        self.token1_ref().decimals as u64
    }

    pub fn token1_balance(&self, ordinal: u64, balances_store: &StoreGetBigInt) -> BigInt {
        balances_store
            .get_at(
                ordinal,
                StoreKey::Token1Balance.get_unique_pool_key(&self.address),
            )
            .unwrap_or(BigInt::zero())
    }

    pub fn input_tokens(&self) -> Vec<String> {
        vec![self.token0_address(), self.token1_address()]
    }

    pub fn output_token_ref(&self) -> Erc20Token {
        self.output_token.clone().unwrap()
    }

    pub fn output_token_address(&self) -> String {
        self.output_token.clone().unwrap().address
    }
}
