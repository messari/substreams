#[rustfmt::skip]
#[path = "../target/pb/messari.sol_token.v1.rs"]
pub(in crate::pb) mod sol_token_v1;

pub mod sol_token {
    pub mod v1 {
        pub use super::super::sol_token_v1::*;
    }
}

#[rustfmt::skip]
#[path = "../target/pb/messari.solana.type.rs"]
pub(in crate::pb) mod solana_type;

pub mod solana {
    pub mod type {
        pub use super::super::solana_type::*;
    }
}
