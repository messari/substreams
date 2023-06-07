use strum_macros::{Display, EnumString};

#[allow(non_camel_case_types)]
#[derive(EnumString, Display)]
pub enum LiquidityPoolFeeType {
    FIXED_TRADING_FEE,
    FIXED_LP_FEE,
    FIXED_PROTOCOL_FEE,
    CUSTOM_TRADING_FEE,
    CUSTOM_LP_FEE,
    CUSTOM_PROTOCOL_FEE,
}

#[allow(non_camel_case_types)]
#[derive(EnumString, Display)]
pub enum TokenType {
    MULTIPLE,
    UNKNOWN,
    ERC20,
    ERC721,
    ERC1155,
    BEP20,
    BEP721,
    BEP1155,
}

#[allow(non_camel_case_types)]
#[derive(EnumString, Display)]
pub enum ProtocolType {
    EXCHANGE,
    LENDING,
    YIELD,
    BRIDGE,
    GENERIC,
}

#[allow(non_camel_case_types)]
#[derive(EnumString, Display)]
pub enum Network {
    ARBITRUM_ONE,
    ARWEAVE_MAINNET,
    AURORA,
    AVALANCHE,
    BOBA,
    BSC,
    CELO,
    COSMOS,
    CRONOS,
    MAINNET,
    FANTOM,
    FUSE,
    HARMONY,
    JUNO,
    MOONBEAM,
    MOONRIVER,
    NEAR_MAINNET,
    OPTIMISM,
    OSMOSIS,
    MATIC,
    XDAI,
}
