#[rustfmt::skip]
#[path = "../target/pb/messari.common.v1.rs"]
pub(in crate::pb) mod common_v1;

pub mod common {
    pub mod v1 {
        pub use super::super::common_v1::*;
    }
}

#[rustfmt::skip]
#[path = "../target/pb/messari.ens.v1.rs"]
pub(in crate::pb) mod ens_v1;

pub mod ens {
    pub mod v1 {
        pub use super::super::ens_v1::*;
    }
}
