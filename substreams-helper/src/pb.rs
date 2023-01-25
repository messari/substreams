#[rustfmt::skip]
#[path = "../target/pb/messari.sol_token.v1.rs"]
pub(in crate::pb) mod sol_token_v1;

pub mod sol_token {
    pub mod v1 {
        pub use super::super::sol_token_v1::*;
    }
}
