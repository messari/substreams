use substreams::scalar::BigDecimal;

use crate::pb::{erc20::v1::Erc20Token, uniswap::v2::Pool};

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

    pub fn token0_ref(&self) -> &Erc20Token {
        &self.input_tokens.as_ref().unwrap().items[0]
    }

    pub fn token1_address(&self) -> &String {
        &self.token1_ref().address
    }

    pub fn token1_ref(&self) -> &Erc20Token {
        &self.input_tokens.as_ref().unwrap().items[1]
    }
}
