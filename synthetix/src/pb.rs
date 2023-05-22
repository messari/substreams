#[rustfmt::skip]
#[path = "../target/pb/messari.synthetix.v1.rs"]
pub(in crate::pb) mod synthetix_v1;

pub mod synthetix {
    pub mod v1 {
        pub use super::super::synthetix_v1::*;
    }
}
