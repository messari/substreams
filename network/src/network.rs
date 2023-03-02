use substreams_entity_change::pb::entity::value::Typed;
use std::string::ToString;
use strum_macros::Display;

#[repr(u8)]
#[derive(Display)]
pub(crate) enum SubgraphNetwork {
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

impl Into<Typed> for SubgraphNetwork {
    fn into(self) -> Typed {
        Typed::Bigint((self as u8).to_string())
    }
}
