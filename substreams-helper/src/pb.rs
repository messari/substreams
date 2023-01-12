#[rustfmt::skip]
#[path = "../target/pb/messari.evm_token.v1.rs"]
pub(in crate::pb) mod evm_token_v1;

pub mod evm_token {
    pub mod v1 {
        pub use super::super::evm_token_v1::*;
    }
}
