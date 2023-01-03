#[rustfmt::skip]
#[allow(unused_imports, dead_code)]
#[path = "../target/pb/aggregate_data.rs"]
pub mod aggregate_data;

#[rustfmt::skip]
#[allow(unused_imports, dead_code)]
#[path = "../target/pb/messari.network.v1.rs"]
pub(self) mod network_v1;

pub mod network {
    pub mod v1 {
        pub use super::super::network_v1::*;
    }
}
