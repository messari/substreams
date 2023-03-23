#[rustfmt::skip]
#[path = "../target/pb/messari.erc20.v1.rs"]
pub(in crate::pb) mod erc20_v1;

pub mod erc20 {
    pub mod v1 {
        pub use super::super::erc20_v1::*;
    }
}

#[rustfmt::skip]
#[path = "../target/pb/messari.uniswap.v1.rs"]
pub(in crate::pb) mod uniswap_v1;

pub mod uniswap {
    pub mod v1 {
        pub use super::super::uniswap_v1::*;
    }
}
