use substreams::pb::substreams::{store_delta, Clock};
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{DeltaBigInt, Deltas};
use substreams::store::{StoreGet, StoreGetBigDecimal, StoreGetBigInt};
use substreams_entity_change::pb::entity::entity_change::Operation;
use substreams_entity_change::pb::entity::{EntityChange, EntityChanges};

use crate::store_key::StoreKey;
use crate::utils::{get_day_id, UNISWAP_V2_FACTORY};

#[substreams::handlers::map]
pub fn map_financial_daily_snapshot_entity(
    clock: Clock,
    input_token_balances_store: StoreGetBigInt,
    input_token_balances_deltas: Deltas<DeltaBigInt>,
    protocol_tvl_store: StoreGetBigDecimal,
    protocol_cumulative_fields_store: StoreGetBigDecimal,
    protocol_daily_fields_store: StoreGetBigDecimal,
) -> Result<EntityChanges, ()> {
    let mut entity_changes: Vec<EntityChange> = vec![];
    let timestamp = BigInt::from(clock.timestamp.unwrap().seconds);

    for delta in input_token_balances_deltas.deltas.iter() {
        match &delta.key {
            key if key.starts_with(StoreKey::LatestTimestamp.unique_id().as_str()) => {
                if delta.operation != store_delta::Operation::Create {
                    continue;
                }

                let delta_timestamp = delta.new_value.clone().to_u64() as i64;
                let day_id = get_day_id(delta_timestamp) - BigInt::one();

                let block_number = input_token_balances_store
                    .get_last(StoreKey::LatestBlockNumber.unique_id())
                    .unwrap();

                entity_changes.push(create_financial_daily_snapshot(
                    day_id,
                    &protocol_tvl_store,
                    &protocol_cumulative_fields_store,
                    &protocol_daily_fields_store,
                    &block_number,
                    &timestamp,
                ));
            }
            _ => {}
        }
    }

    Ok(EntityChanges { entity_changes })
}

fn create_financial_daily_snapshot(
    day_id: BigInt,
    protocol_tvl_store: &StoreGetBigDecimal,
    protocol_cumulative_fields_store: &StoreGetBigDecimal,
    protocol_daily_fields_store: &StoreGetBigDecimal,
    block_number: &BigInt,
    timestamp: &BigInt,
) -> EntityChange {
    let id = [UNISWAP_V2_FACTORY.to_string(), day_id.clone().to_string()].join("-");

    let mut financial_daily_snapshot: EntityChange =
        EntityChange::new("FinancialsDailySnapshot", id.as_str(), 0, Operation::Create);

    financial_daily_snapshot
        .change("id", id)
        .change("protocol", "DexAmmProtocol".to_string())
        .change(
            "totalValueLockedUSD",
            protocol_tvl_store
                .get_last(StoreKey::TotalValueLockedUSD.get_unique_protocol_key())
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "dailyVolumeUSD",
            protocol_daily_fields_store
                .get_last(StoreKey::DailyVolumeUSD.get_unique_daily_protocol_key(day_id.clone()))
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "cumulativeVolumeUSD",
            protocol_cumulative_fields_store
                .get_last(StoreKey::CumulativeVolumeUSD.get_unique_protocol_key())
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "dailySupplySideRevenueUSD",
            protocol_daily_fields_store
                .get_last(
                    StoreKey::DailySupplySideRevenueUSD
                        .get_unique_daily_protocol_key(day_id.clone()),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "cumulativeSupplySideRevenueUSD",
            protocol_cumulative_fields_store
                .get_last(StoreKey::CumulativeSupplySideRevenueUSD.get_unique_protocol_key())
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "dailyProtocolSideRevenueUSD",
            protocol_daily_fields_store
                .get_last(
                    StoreKey::DailyProtocolSideRevenueUSD
                        .get_unique_daily_protocol_key(day_id.clone()),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "cumulativeProtocolSideRevenueUSD",
            protocol_cumulative_fields_store
                .get_last(StoreKey::CumulativeProtocolSideRevenueUSD.get_unique_protocol_key())
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "dailyTotalRevenueUSD",
            protocol_daily_fields_store
                .get_last(
                    StoreKey::DailyTotalRevenueUSD.get_unique_daily_protocol_key(day_id.clone()),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "cumulativeTotalRevenueUSD",
            protocol_cumulative_fields_store
                .get_last(StoreKey::CumulativeTotalRevenueUSD.get_unique_protocol_key())
                .unwrap_or(BigDecimal::zero()),
        )
        .change("blockNumber", block_number)
        .change("timestamp", timestamp);

    financial_daily_snapshot
}
