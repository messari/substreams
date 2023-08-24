use substreams::pb::substreams::{store_delta, Clock};
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{DeltaBigInt, Deltas};
use substreams::store::{StoreGet, StoreGetBigDecimal, StoreGetBigInt};
use substreams_entity_change::pb::entity::entity_change::Operation;
use substreams_entity_change::pb::entity::{EntityChange, EntityChanges};

use crate::common::constants;
use crate::store_key::StoreKey;
use crate::utils;

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
                let is_initialized = delta.operation != store_delta::Operation::Create;

                let delta_timestamp = delta.new_value.to_u64() as i64;
                let day_id = utils::get_day_id(delta_timestamp);

                let block_number = input_token_balances_store
                    .get_at(delta.ordinal, StoreKey::LatestBlockNumber.unique_id())
                    .unwrap();

                entity_changes.push(create_financial_daily_snapshot(
                    delta.ordinal,
                    day_id,
                    &protocol_tvl_store,
                    &protocol_cumulative_fields_store,
                    &protocol_daily_fields_store,
                    &block_number,
                    &timestamp,
                    is_initialized,
                ));
            }
            _ => {}
        }
    }

    Ok(EntityChanges { entity_changes })
}

fn create_financial_daily_snapshot(
    ordinal: u64,
    day_id: i64,
    protocol_tvl_store: &StoreGetBigDecimal,
    protocol_cumulative_fields_store: &StoreGetBigDecimal,
    protocol_daily_fields_store: &StoreGetBigDecimal,
    block_number: &BigInt,
    timestamp: &BigInt,
    is_initialized: bool,
) -> EntityChange {
    let id = [constants::UNISWAP_V2_FACTORY, day_id.to_string().as_str()].join("-");

    let mut financial_daily_snapshot: EntityChange = EntityChange::new(
        "FinancialsDailySnapshot",
        id.as_str(),
        ordinal,
        Operation::Update,
    );

    if !is_initialized {
        financial_daily_snapshot.operation = Operation::Create as i32;
    }

    financial_daily_snapshot
        .change("id", id)
        .change("protocol", constants::UNISWAP_V2_FACTORY.to_string())
        .change(
            "totalValueLockedUSD",
            protocol_tvl_store
                .get_at(
                    ordinal,
                    StoreKey::TotalValueLockedUSD.get_unique_protocol_key(),
                )
                .unwrap_or(BigDecimal::zero())
                .with_prec(20),
        )
        .change(
            "dailyVolumeUSD",
            protocol_daily_fields_store
                .get_at(
                    ordinal,
                    StoreKey::DailyVolumeUSD.get_unique_daily_protocol_key(day_id),
                )
                .unwrap_or(BigDecimal::zero())
                .with_prec(20),
        )
        .change(
            "cumulativeVolumeUSD",
            protocol_cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeVolumeUSD.get_unique_protocol_key(),
                )
                .unwrap_or(BigDecimal::zero())
                .with_prec(20),
        )
        .change(
            "dailySupplySideRevenueUSD",
            protocol_daily_fields_store
                .get_at(
                    ordinal,
                    StoreKey::DailySupplySideRevenueUSD.get_unique_daily_protocol_key(day_id),
                )
                .unwrap_or(BigDecimal::zero())
                .with_prec(20),
        )
        .change(
            "cumulativeSupplySideRevenueUSD",
            protocol_cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeSupplySideRevenueUSD.get_unique_protocol_key(),
                )
                .unwrap_or(BigDecimal::zero())
                .with_prec(20),
        )
        .change(
            "dailyProtocolSideRevenueUSD",
            protocol_daily_fields_store
                .get_at(
                    ordinal,
                    StoreKey::DailyProtocolSideRevenueUSD.get_unique_daily_protocol_key(day_id),
                )
                .unwrap_or(BigDecimal::zero())
                .with_prec(20),
        )
        .change(
            "cumulativeProtocolSideRevenueUSD",
            protocol_cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeProtocolSideRevenueUSD.get_unique_protocol_key(),
                )
                .unwrap_or(BigDecimal::zero())
                .with_prec(20),
        )
        .change(
            "dailyTotalRevenueUSD",
            protocol_daily_fields_store
                .get_at(
                    ordinal,
                    StoreKey::DailyTotalRevenueUSD.get_unique_daily_protocol_key(day_id),
                )
                .unwrap_or(BigDecimal::zero())
                .with_prec(20),
        )
        .change(
            "cumulativeTotalRevenueUSD",
            protocol_cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeTotalRevenueUSD.get_unique_protocol_key(),
                )
                .unwrap_or(BigDecimal::zero())
                .with_prec(20),
        )
        .change("blockNumber", block_number)
        .change("timestamp", timestamp);

    financial_daily_snapshot
}
