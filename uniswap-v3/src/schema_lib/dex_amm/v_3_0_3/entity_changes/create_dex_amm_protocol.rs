use crate::tables::Tables;

use crate::pb::dex_amm::v3_0_3::{PrunedTransaction, CreateDexAmmProtocol};
use crate::constants;

pub fn create_dex_amm_protocol_entity_change(
    tables: &mut Tables,
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    create_dex_amm_protocol: &CreateDexAmmProtocol,
) {
    tables.create_row("DexAmmProtocol", &format!("0x{}", hex::encode(&create_dex_amm_protocol.protocol_address)))
        .set("name", &create_dex_amm_protocol.name)
        .set("slug", &create_dex_amm_protocol.slug)
        .set("schemaVersion", &create_dex_amm_protocol.schema_version)
        .set("subgraphVersion", &create_dex_amm_protocol.subgraph_version)
        .set("methodologyVersion", &create_dex_amm_protocol.methodology_version)
        .set("network", &create_dex_amm_protocol.network)
        .set("type", &create_dex_amm_protocol.r#type)
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
