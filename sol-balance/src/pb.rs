#[rustfmt::skip]
#[path = "../target/pb/messari.sol.token.rs"]
pub(in crate::pb) mod sol_token;

pub mod sol {
    pub mod token {
        pub use super::super::sol_token::*;
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
