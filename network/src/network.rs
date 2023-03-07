use substreams_entity_change::pb::entity::value::Typed;
use std::string::ToString;
use strum_macros::Display;

#[derive(Display)]
pub(crate) enum Network {
    ArbitrumOne,
    ArweaveMainnet,
    AURORA,
    AVALANCHE,
    BOBA,
    BSC, // aka BNB Chain
    CELO,
    CLOVER,
    COSMOS,
    CRONOS,
    MAINNET, // Ethereum Mainnet
    FANTOM,
    FUSE,
    HARMONY,
    JUNO,
    MOONBEAM,
    MOONRIVER,
    NearMainnet,
    OPTIMISM,
    OSMOSIS,
    MATIC, // aka Polygon
    GNOSIS,
}