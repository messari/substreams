use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::NULL_ADDRESS;

use crate::pb::erc20::v1::{Erc20Token, Erc20Tokens};
use crate::TokenContract;
use crate::{abi::pair as PairContract, pb::uniswap::v2::Pool};

pub struct PoolContract(Vec<u8>);

impl PoolContract {
    pub fn new(pool_address: Vec<u8>) -> Self {
        PoolContract(pool_address)
    }

    fn get_name(&self) -> String {
        PairContract::functions::Name {}
            .call(self.0.to_vec())
            .unwrap_or(String::new())
    }

    fn get_symbol(&self) -> String {
        PairContract::functions::Symbol {}
            .call(self.0.to_vec())
            .unwrap_or(String::new())
    }

    fn get_token0(&self) -> Erc20Token {
        TokenContract::new(
            PairContract::functions::Token0 {}
                .call(self.0.to_vec())
                .unwrap_or(NULL_ADDRESS.to_vec()),
        )
        .as_struct()
    }

    fn get_token1(&self) -> Erc20Token {
        TokenContract::new(
            PairContract::functions::Token1 {}
                .call(self.0.to_vec())
                .unwrap_or(NULL_ADDRESS.to_vec()),
        )
        .as_struct()
    }

    fn get_output_token(&self) -> Erc20Token {
        TokenContract::new(self.0.to_vec()).as_struct()
    }

    pub fn as_struct(&self, block: &eth::Block) -> Pool {
        Pool {
            name: self.get_name(),
            symbol: self.get_symbol(),
            address: Hex(self.0.to_vec()).to_string(),
            input_tokens: Some(Erc20Tokens {
                items: vec![self.get_token0(), self.get_token1()],
            }),
            output_token: Some(self.get_output_token()),
            created_timestamp: block.timestamp_seconds() as i64,
            created_block_number: block.number as i64,
        }
    }
}
