#[rustfmt::skip]
#[allow(unused_imports, dead_code)]
#[path = "../target/pb/messari.chainlink.v1.rs"]
pub(self) mod chainlink_v1;

pub mod chainlink {
    pub mod v1 {
        pub use super::super::chainlink_v1::*;
    }
}

#[rustfmt::skip]
#[allow(unused_imports, dead_code)]
#[path = "../target/pb/messari.erc20.v1.rs"]
pub(self) mod erc20_v1;

pub mod erc20 {
    pub mod v1 {
        pub use super::super::erc20_v1::*;
    }
}

#[rustfmt::skip]
#[allow(unused_imports, dead_code)]
#[path = "../target/pb/messari.erc20_market_cap.v1.rs"]
pub(self) mod erc20_market_cap_v1;

pub mod erc20_market_cap {
    pub mod v1 {
        pub use super::super::erc20_market_cap_v1::*;
    }
}

#[rustfmt::skip]
#[allow(unused_imports, dead_code)]
#[path = "../target/pb/messari.erc20_price.v1.rs"]
pub(self) mod erc20_price_v1;

pub mod erc20_price {
    pub mod v1 {
        pub use super::super::erc20_price_v1::*;
    }
}

#[rustfmt::skip]
#[allow(unused_imports, dead_code)]
#[path = "../target/pb/messari.uniswap.v1.rs"]
pub(self) mod uniswap_v1;

pub mod uniswap {
    pub mod v1 {
        pub use super::super::uniswap_v1::*;
    }
}
