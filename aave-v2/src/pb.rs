#[rustfmt::skip]
#[path = "../target/pb/messari.aave_v2.v1.rs"]
pub(in crate::pb) mod aave_v2_v1;

pub mod aave_v2 {
    pub mod v1 {
        pub use super::super::aave_v2_v1::*;
    }
}
