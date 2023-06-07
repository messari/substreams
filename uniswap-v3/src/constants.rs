use substreams::hex;
use substreams::scalar::{BigDecimal, BigInt};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref  UNISWAP_V3_FACTORY_SLICE: [u8; 20] = hex!("1f98431c8ad98523631ae4a59f267346ea31f984");
    pub static ref  NFT_POSITION_MANAGER_SLICE: [u8; 20] = hex!("c36442b4a4522e871399cd717abdd847ab11fe88");

    pub static ref BIGDECIMAL_ZERO: BigDecimal = BigDecimal::from(0);
    pub static ref BIGDECIMAL_50: BigDecimal = BigDecimal::from(50);
    pub static ref BIGDECIMAL_100: BigDecimal = BigDecimal::from(100);
    pub static ref BIGDECIMAL_10000: BigDecimal = BigDecimal::from(10000);
    pub static ref BIGINT_ZERO: BigInt = BigInt::from(0);
}
