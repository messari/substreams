use substreams::pb::substreams::store_delta;
use substreams::scalar::BigDecimal;
use substreams::store::{DeltaBigInt, Deltas};
use substreams::store::{StoreGet, StoreGetBigDecimal};
use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChange, EntityChanges};

use crate::common::constants;
use crate::store_key::StoreKey;

#[substreams::handlers::map]
pub fn map_protocol_entity(
    input_token_balances_deltas: Deltas<DeltaBigInt>,
    protocol_tvl_store: StoreGetBigDecimal,
    protocol_cumulative_fields_store: StoreGetBigDecimal,
) -> Result<EntityChanges, ()> {
    let mut entity_changes: Vec<EntityChange> = vec![];

    for delta in input_token_balances_deltas.deltas.iter() {
        if let Some(_) = StoreKey::TotalBalance.get_pool(&delta.key) {
            let is_initialized = delta.operation != store_delta::Operation::Create;

            entity_changes.push(create_protocol(
                delta.ordinal,
                &protocol_tvl_store,
                &protocol_cumulative_fields_store,
                is_initialized,
            ));
        }
    }

    Ok(EntityChanges { entity_changes })
}

fn create_protocol(
    ordinal: u64,
    protocol_tvl_store: &StoreGetBigDecimal,
    protocol_cumulative_fields_store: &StoreGetBigDecimal,
    is_initialized: bool,
) -> EntityChange {
    let mut protocol_entity_change: EntityChange = EntityChange::new(
        "DexAmmProtocol",
        constants::UNISWAP_V2_FACTORY,
        ordinal,
        Operation::Update,
    );

    if !is_initialized {
        protocol_entity_change.operation = Operation::Create as i32;

        protocol_entity_change
            .change("id", constants::UNISWAP_V2_FACTORY.to_string())
            .change("name", "Uniswap V2".to_string())
            .change("slug", "uniswap-v2".to_string())
            .change("schemaVersion", "1.0.0".to_string())
            .change("subgraphVersion", "1.0.0".to_string())
            .change("methodologyVersion", "1.0.0".to_string())
            .change("network", "MAINNET".to_string())
            .change("type", "EXCHANGE".to_string());
    }

    protocol_entity_change
        .change(
            "totalValueLockedUSD",
            protocol_tvl_store
                .get_at(
                    ordinal,
                    StoreKey::TotalValueLockedUSD.get_unique_protocol_key(),
                )
                .unwrap_or(BigDecimal::zero())
                .with_prec(3),
        )
        .change(
            "cumulativeVolumeUSD",
            protocol_cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeVolumeUSD.get_unique_protocol_key(),
                )
                .unwrap_or(BigDecimal::zero())
                .with_prec(3),
        )
        .change(
            "cumulativeSupplySideRevenueUSD",
            protocol_cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeSupplySideRevenueUSD.get_unique_protocol_key(),
                )
                .unwrap_or(BigDecimal::zero())
                .with_prec(3),
        )
        .change(
            "cumulativeProtocolSideRevenueUSD",
            protocol_cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeProtocolSideRevenueUSD.get_unique_protocol_key(),
                )
                .unwrap_or(BigDecimal::zero())
                .with_prec(3),
        )
        .change(
            "cumulativeTotalRevenueUSD",
            protocol_cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeTotalRevenueUSD.get_unique_protocol_key(),
                )
                .unwrap_or(BigDecimal::zero())
                .with_prec(3),
        )
        .change("cumulativeUniqueUsers", 0)
        .change("totalPoolCount", 0);

    protocol_entity_change
}
