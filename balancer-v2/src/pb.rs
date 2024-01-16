#[rustfmt::skip]
#[path = "../target/pb/vault.v1.rs"]
pub(in crate::pb) mod vault_v1;

pub mod vault {
    pub mod v1 {
        pub use super::super::vault_v1::*;
    }
}
