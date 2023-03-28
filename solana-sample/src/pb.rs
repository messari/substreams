#[rustfmt::skip]
#[path = "../target/pb/messari.common.v1.rs"]
pub(in crate::pb) mod common_v1;

pub mod common {
    pub mod v1 {
        pub use super::super::common_v1::*;
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
