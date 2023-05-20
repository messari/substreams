use crate::tables::Tables;

use crate::pb::dex_amm::v3_0_3::{PrunedTransaction, DexAmmProtocolEntityCreation};
use crate::constants;

pub fn create_dex_amm_protocol_entity(
    tables: &mut Tables,
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    dex_amm_protocol_entity_creation: &DexAmmProtocolEntityCreation,
) {
    tables.create_row("DexAmmProtocol", &format!("0x{}", hex::encode(&dex_amm_protocol_entity_creation.protocol_address)))
        .set("name", &dex_amm_protocol_entity_creation.name)
        .set("slug", &dex_amm_protocol_entity_creation.slug)
        .set("schemaVersion", &dex_amm_protocol_entity_creation.schema_version)
        .set("subgraphVersion", &dex_amm_protocol_entity_creation.subgraph_version)
        .set("methodologyVersion", &dex_amm_protocol_entity_creation.methodology_version)
        .set("network", &dex_amm_protocol_entity_creation.network)
        .set("type", &dex_amm_protocol_entity_creation.r#type)
        .set("totalValueLockedUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("totalLiquidityUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("activeLiquidityUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("uncollectedProtocolSideValueUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("uncollectedSupplySideValueUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("protocolControlledValueUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("cumulativeVolumeUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("cumulativeSupplySideRevenueUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("cumulativeProtocolSideRevenueUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("cumulativeTotalRevenueUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("cumulativeUniqueUsers", 0)
        .set("cumulativeUniqueLPs", 0)
        .set("cumulativeUniqueTraders", 0)
        .set("totalPoolCount", 0)
        .set("openPositionCount", 0)
        .set("cumulativePositionCount", 0)
        .set("lastSnapshotDayID", 0)
        .set("lastUpdateTimestamp", constants::BIGINT_ZERO.clone())
        .set("lastUpdateBlockNumber", constants::BIGINT_ZERO.clone());
}
