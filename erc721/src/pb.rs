#[rustfmt::skip]
#[allow(unused_imports)]
#[path = "../target/pb/messari.erc721.v1.rs"]
pub(self) mod erc721_v1;

pub mod erc721 {
    pub mod v1 {
        pub use super::super::erc721_v1::*;
    }
}
