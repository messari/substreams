#[rustfmt::skip]
#[path = "../target/pb/messari.common.v1.rs"]
pub(in crate::pb) mod common_v1;

pub mod common {
    pub mod v1 {
        pub use super::super::common_v1::*;
    }
}

#[rustfmt::skip]
#[path = "../target/pb/messari.dex_amm.v1.rs"]
pub(in crate::pb) mod dex_amm_v1;

pub mod dex_amm {
    pub mod v1 {
        pub use super::super::dex_amm_v1::*;
    }
}

#[rustfmt::skip]
#[path = "../target/pb/messari.erc20.v1.rs"]
pub(in crate::pb) mod erc20_v1;

pub mod erc20 {
    pub mod v1 {
        pub use super::super::erc20_v1::*;
    }
}

#[rustfmt::skip]
#[path = "../target/pb/messari.uniswap.v2.rs"]
pub(in crate::pb) mod uniswap_v2;

pub mod uniswap {
    pub mod v2 {
        pub use super::super::uniswap_v2::*;
    }
}
