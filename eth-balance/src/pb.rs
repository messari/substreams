#[rustfmt::skip]
#[allow(unused_imports)]
#[path = "../target/pb/messari.eth_balance.v1.rs"]
pub(in crate::pb) mod eth_balance_v1;

pub mod eth_balance {
    pub mod v1 {
        pub use super::super::eth_balance_v1::*;
    }
}
