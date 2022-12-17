use substreams::scalar::BigInt;
use substreams::Hex;

use crate::abi::erc20;
use crate::pb::dex_amm::v1::Token;

pub(crate) struct TokenRetriever {
    token_address: Vec<u8>,
}

impl TokenRetriever {
    pub(crate) fn new(token_address: Vec<u8>) -> Self {
        TokenRetriever { token_address }
    }

    pub(crate) fn get_name(&self) -> String {
        erc20::functions::Name {}
            .call(self.token_address.to_vec())
            .unwrap_or(String::new())
    }

    pub(crate) fn get_symbol(&self) -> String {
        erc20::functions::Symbol {}
            .call(self.token_address.to_vec())
            .unwrap_or(String::new())
    }

    pub(crate) fn get_decimals(&self) -> BigInt {
        erc20::functions::Decimals {}
            .call(self.token_address.to_vec())
            .unwrap_or(BigInt::from(18))
    }

    pub(crate) fn to_struct(&self) -> Token {
        Token {
            name: self.get_name(),
            symbol: self.get_symbol(),
            address: Hex(self.token_address.to_vec()).to_string(),
            decimals: self.get_decimals().to_u64(),
            ..Default::default()
        }
    }
}
