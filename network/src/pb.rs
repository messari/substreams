#[rustfmt::skip]
#[path = "../target/pb/messari.network.v1.rs"]
pub(in crate::pb) mod network_v1;

pub mod network {
    pub mod v1 {
        pub use super::super::network_v1::*;
    }
}
