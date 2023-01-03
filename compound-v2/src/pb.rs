#[rustfmt::skip]
#[allow(unused_imports, dead_code)]
#[path = "../target/pb/messari.compound.v1.rs"]
pub(self) mod compound_v1;

pub mod compound {
    pub mod v1 {
        pub use super::super::compound_v1::*;
    }
}
