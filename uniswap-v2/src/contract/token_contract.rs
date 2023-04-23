use substreams::scalar::BigInt;
use substreams::Hex;

use crate::abi::erc20 as Erc20Contract;
use crate::pb::erc20::v1::Erc20Token;

pub struct TokenContract(Vec<u8>);

impl TokenContract {
    pub fn new(address: Vec<u8>) -> Self {
        TokenContract(address)
    }

    fn get_name(&self) -> String {
        Erc20Contract::functions::Name {}
            .call(self.0.to_vec())
            .unwrap_or(String::new())
    }

    fn get_symbol(&self) -> String {
        Erc20Contract::functions::Symbol {}
            .call(self.0.to_vec())
            .unwrap_or(String::new())
    }

    fn get_decimals(&self) -> BigInt {
        Erc20Contract::functions::Decimals {}
            .call(self.0.to_vec())
            .unwrap_or(BigInt::from(18))
    }

    pub fn as_struct(&self) -> Erc20Token {
        Erc20Token {
            name: self.get_name(),
            symbol: self.get_symbol(),
            address: Hex(self.0.to_vec()).to_string(),
            decimals: self.get_decimals().to_u64(),
        }
    }
}
