
use strum_macros::{EnumString, ToString};

#[derive(EnumString, ToString)]
pub enum LiquidityPoolFeeType {
    FIXED_TRADING_FEE,
    FIXED_LP_FEE,
    FIXED_PROTOCOL_FEE,
    CUSTOM_TRADING_FEE,
    CUSTOM_LP_FEE,
    CUSTOM_PROTOCOL_FEE,
}
