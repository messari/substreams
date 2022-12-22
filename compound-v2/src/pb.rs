#[rustfmt::skip]
#[path = "../target/pb/messari.compound.v1.rs"]
pub(in crate::pb) mod compound_v1;

pub mod compound {
    pub mod v1 {
        pub use super::super::compound_v1::*;
    }
}
