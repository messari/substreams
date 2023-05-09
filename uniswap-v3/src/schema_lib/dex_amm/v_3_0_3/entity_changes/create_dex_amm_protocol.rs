use substreams::{Hex};
use substreams::prelude::*;
use substreams::pb::substreams::Clock;
use substreams_entity_change::pb::entity::{EntityChange, entity_change::Operation};

use crate::pb::dex_amm::v3_0_3::{PrunedTransaction, CreateDexAmmProtocol};
use crate::schema_lib::dex_amm::v_3_0_3::keys;

pub fn create_dex_amm_protocol_entity_change(
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    create_dex_amm_protocol: CreateDexAmmProtocol,
) -> EntityChange {
    let mut protocol_change: EntityChange =
        EntityChange::new("DexAmmProtocol", &format!("0x{}", hex::encode(create_dex_amm_protocol.protocol_address)), 0, Operation::Create);

    protocol_change
        .change("name", create_dex_amm_protocol.name)
        .change("slug", create_dex_amm_protocol.slug)
        .change("schemaVersion", create_dex_amm_protocol.schema_version)
        .change("subgraphVersion", create_dex_amm_protocol.subgraph_version)
        .change("methodologyVersion", create_dex_amm_protocol.methodology_version)
        .change("network", create_dex_amm_protocol.network)
        .change("type", create_dex_amm_protocol.r#type)
        .change("totalValueLockedUSD", BigDecimal::from(0))
        .change("totalLiquidityUSD", BigDecimal::from(0))
        .change("activeLiquidityUSD", BigDecimal::from(0))
        .change("uncollectedProtocolSideValueUSD", BigDecimal::from(0))
        .change("uncollectedSupplySideValueUSD", BigDecimal::from(0))
        .change("protocolControlledValueUSD", BigDecimal::from(0))
        .change("cumulativeVolumeUSD", BigDecimal::from(0))
        .change("cumulativeSupplySideRevenueUSD", BigDecimal::from(0))
        .change("cumulativeProtocolSideRevenueUSD", BigDecimal::from(0))
        .change("cumulativeTotalRevenueUSD", BigDecimal::from(0))
        .change("cumulativeUniqueUsers", 0)
        .change("cumulativeUniqueLPs", 0)
        .change("cumulativeUniqueTraders", 0)
        .change("totalPoolCount", 0)
        .change("openPositionCount", 0)
        .change("cumulativePositionCount", 0)
        .change("lastSnapshotDayID", 0)
        .change("lastUpdateTimestamp", BigInt::from(0))
        .change("lastUpdateBlockNumber", BigInt::from(0));

    protocol_change
}