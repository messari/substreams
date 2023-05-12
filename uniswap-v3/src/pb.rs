#[rustfmt::skip]
#[path = "../target/pb/messari.common.v1.rs"]
pub(in crate::pb) mod common_v1;

pub mod common {
    pub mod v1 {
        pub use super::super::common_v1::*;
    }
}

#[rustfmt::skip]
#[path = "../target/pb/messari.dex_amm.v3_0_3.rs"]
pub(in crate::pb) mod dex_amm_v3_0_3;

pub mod dex_amm {
    pub mod v3_0_3 {
        pub use super::super::dex_amm_v3_0_3::*;
    }
}

#[rustfmt::skip]
#[path = "../target/pb/messari.erc20.v1.rs"]
pub(in crate::pb) mod erc20_v1;

pub mod erc20 {
    pub mod v1 {
        pub use super::super::erc20_v1::*;
    }
}

#[rustfmt::skip]
#[path = "../target/pb/messari.store.v1.rs"]
pub(in crate::pb) mod store_v1;

pub mod store {
    pub mod v1 {
        pub use super::super::store_v1::*;
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
