#[rustfmt::skip]
#[path = "../target/pb/messari.eth_supply.v1.rs"]
pub(in crate::pb) mod eth_supply_v1;

pub mod eth_supply {
    pub mod v1 {
        pub use super::super::eth_supply_v1::*;
    }
}

#[rustfmt::skip]
#[path = "../target/pb/substreams.entity.v1.rs"]
pub(in crate::pb) mod entity_v1;

pub mod entity {
    pub mod v1 {
        pub use super::super::entity_v1::*;
    }
}
