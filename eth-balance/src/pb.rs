#[rustfmt::skip]
#[path = "../target/pb/messari.evm.token.v1.rs"]
pub(in crate::pb) mod evm_token;

pub mod evm {
    pub mod token {
        pub use super::super::evm_token::*;
    }
}
