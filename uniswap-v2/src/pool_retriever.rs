use std::ops::Deref;
use substreams::Hex;

use crate::abi::pair;
use crate::pb::dex_amm::v1::Token;
use crate::token_retriever::TokenRetriever;

pub(crate) struct PoolRetriever {
    pool_address: Vec<u8>,
}

impl PoolRetriever {
    pub(crate) fn new(pool_address: &Vec<u8>) -> Self {
        PoolRetriever {
            pool_address: pool_address.deref().to_vec(),
        }
    }

    pub(crate) fn get_name(&self) -> String {
        pair::functions::Name {}
            .call(self.pool_address.to_vec())
            .unwrap_or(String::new())
    }

    pub(crate) fn get_address(&self) -> String {
        Hex(&self.pool_address).to_string()
    }

    pub(crate) fn get_symbol(&self) -> String {
        pair::functions::Symbol {}
            .call(self.pool_address.to_vec())
            .unwrap_or(String::new())
    }

    pub(crate) fn get_token0_address(&self) -> Vec<u8> {
        pair::functions::Token0 {}
            .call(self.pool_address.to_vec())
            .unwrap()
    }

    pub(crate) fn get_token0(&self) -> Token {
        TokenRetriever::new(self.get_token0_address()).to_struct()
    }

    pub(crate) fn get_token1_address(&self) -> Vec<u8> {
        pair::functions::Token1 {}
            .call(self.pool_address.to_vec())
            .unwrap()
    }

    pub(crate) fn get_token1(&self) -> Token {
        TokenRetriever::new(self.get_token1_address()).to_struct()
    }

    pub(crate) fn get_input_tokens(&self) -> Vec<Token> {
        vec![self.get_token0(), self.get_token1()]
    }

    pub(crate) fn get_output_token(&self) -> Option<Token> {
        Some(TokenRetriever::new(self.pool_address.to_vec()).to_struct())
    }
}
