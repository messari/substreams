#[rustfmt::skip]
#[allow(unused_imports, dead_code)]
#[path = "../target/pb/messari.eth_balance.v1.rs"]
pub(self) mod eth_balance_v1;

pub mod eth_balance {
    pub mod v1 {
        pub use super::super::eth_balance_v1::*;
    }
}
