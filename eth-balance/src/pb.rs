#[rustfmt::skip]
#[path = "../target/pb/messari.token.v1.rs"]
pub(in crate::pb) mod token_v1;

pub mod token {
    pub mod v1 {
        pub use super::super::token_v1::*;
    }
}
