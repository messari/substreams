use substreams::pb::substreams::{store_delta, Clock};
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{DeltaBigInt, Deltas};
use substreams::store::{StoreGet, StoreGetBigDecimal, StoreGetBigInt, StoreGetProto};
use substreams_entity_change::pb::entity::entity_change::Operation;
use substreams_entity_change::pb::entity::{EntityChange, EntityChanges};

use crate::pb::uniswap::v2::Pool;
use crate::store_key::StoreKey;
use crate::utils::{get_day_id, get_hour_id};

#[substreams::handlers::map]
pub fn map_liquidity_pool_snapshots_entity(
    clock: Clock,
    pools_store: StoreGetProto<Pool>,
    output_token_supply_store: StoreGetBigInt,
    input_token_balances_store: StoreGetBigInt,
    input_token_balances_deltas: Deltas<DeltaBigInt>,
    pool_tvl_store: StoreGetBigDecimal,
    cumulative_fields_store: StoreGetBigDecimal,
    daily_and_hourly_fields_store: StoreGetBigDecimal,
) -> Result<EntityChanges, ()> {
    let mut entity_changes: Vec<EntityChange> = vec![];
    let timestamp = BigInt::from(clock.timestamp.unwrap().seconds);

    for delta in input_token_balances_deltas.deltas.iter() {
        if let Some(pool_address) = StoreKey::LatestTimestamp.get_pool(&delta.key) {
            if delta.operation != store_delta::Operation::Create {
                continue;
            }

            let delta_timestamp = delta.new_value.clone().to_u64() as i64;

            let day_id = get_day_id(delta_timestamp) - BigInt::one();
            let hour_id = get_hour_id(delta_timestamp) - BigInt::one();

            let block_number = input_token_balances_store
                .get_last(StoreKey::LatestBlockNumber.unique_id())
                .unwrap();

            let pool: Pool =
                pools_store.must_get_last(StoreKey::Pool.get_unique_pool_key(&pool_address));

            entity_changes.push(create_liquidity_pool_daily_snapshot(
                &pool,
                day_id,
                &output_token_supply_store,
                &input_token_balances_store,
                &pool_tvl_store,
                &cumulative_fields_store,
                &daily_and_hourly_fields_store,
                &block_number,
                &timestamp,
            ));

            entity_changes.push(create_liquidity_pool_hourly_snapshot(
                &pool,
                hour_id,
                &output_token_supply_store,
                &input_token_balances_store,
                &pool_tvl_store,
                &cumulative_fields_store,
                &daily_and_hourly_fields_store,
                &block_number,
                &timestamp,
            ));
        }
    }

    Ok(EntityChanges { entity_changes })
}

fn create_liquidity_pool_daily_snapshot(
    pool: &Pool,
    day_id: BigInt,
    output_token_supply_store: &StoreGetBigInt,
    input_token_balances_store: &StoreGetBigInt,
    pool_tvl_store: &StoreGetBigDecimal,
    cumulative_fields_store: &StoreGetBigDecimal,
    daily_and_hourly_fields_store: &StoreGetBigDecimal,
    block_number: &BigInt,
    timestamp: &BigInt,
) -> EntityChange {
    let id = [pool.clone().address, day_id.clone().to_string()].join("-");
    let pool_address = &pool.address;

    let mut pool_entity_change: EntityChange = EntityChange::new(
        "LiquidityPoolDailySnapshot",
        id.as_str(),
        0,
        Operation::Create,
    );

    pool_entity_change
        .change("id", id)
        .change("protocol", "DexAmmProtocol".to_string())
        .change("pool", pool_address)
        .change(
            "totalValueLockedUSD",
            pool_tvl_store
                .get_last(StoreKey::TotalValueLockedUSD.get_unique_pool_key(&pool_address))
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "dailySupplySideRevenueUSD",
            daily_and_hourly_fields_store
                .get_last(
                    StoreKey::DailySupplySideRevenueUSD
                        .get_unique_daily_pool_key(day_id.clone(), &pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "cumulativeSupplySideRevenueUSD",
            cumulative_fields_store
                .get_last(
                    StoreKey::CumulativeSupplySideRevenueUSD.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "dailyProtocolSideRevenueUSD",
            daily_and_hourly_fields_store
                .get_last(
                    StoreKey::DailyProtocolSideRevenueUSD
                        .get_unique_daily_pool_key(day_id.clone(), &pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "cumulativeProtocolSideRevenueUSD",
            cumulative_fields_store
                .get_last(
                    StoreKey::CumulativeProtocolSideRevenueUSD.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "dailyTotalRevenueUSD",
            daily_and_hourly_fields_store
                .get_last(
                    StoreKey::DailyTotalRevenueUSD
                        .get_unique_daily_pool_key(day_id.clone(), &pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "cumulativeTotalRevenueUSD",
            cumulative_fields_store
                .get_last(StoreKey::CumulativeTotalRevenueUSD.get_unique_pool_key(&pool_address))
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "dailyVolumeUSD",
            daily_and_hourly_fields_store
                .get_last(
                    StoreKey::DailyVolumeUSD
                        .get_unique_daily_pool_key(day_id.clone(), &pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change("dailyVolumeByTokenAmount", vec!["0".to_string(); 2])
        .change("dailyVolumeByTokenUSD", vec!["0".to_string(); 2])
        .change(
            "cumulativeVolumeUSD",
            cumulative_fields_store
                .get_last(StoreKey::CumulativeVolumeUSD.get_unique_pool_key(&pool_address))
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "inputTokenBalances",
            vec![
                input_token_balances_store
                    .get_last(StoreKey::Token0Balance.get_unique_pool_key(&pool_address))
                    .unwrap_or(BigInt::zero())
                    .to_string(),
                input_token_balances_store
                    .get_last(StoreKey::Token1Balance.get_unique_pool_key(&pool_address))
                    .unwrap_or(BigInt::zero())
                    .to_string(),
            ],
        )
        .change("inputTokenWeights", vec!["0".to_string(); 2])
        .change(
            "outputTokenSupply",
            output_token_supply_store
                .get_last(StoreKey::OutputTokenBalance.get_unique_pool_key(&pool_address))
                .unwrap_or(BigInt::zero()),
        )
        .change("blockNumber", block_number)
        .change("timestamp", timestamp);

    pool_entity_change
}

fn create_liquidity_pool_hourly_snapshot(
    pool: &Pool,
    hour_id: BigInt,
    output_token_supply_store: &StoreGetBigInt,
    input_token_balances_store: &StoreGetBigInt,
    pool_tvl_store: &StoreGetBigDecimal,
    cumulative_fields_store: &StoreGetBigDecimal,
    daily_and_hourly_fields_store: &StoreGetBigDecimal,
    block_number: &BigInt,
    timestamp: &BigInt,
) -> EntityChange {
    let id = [pool.clone().address, hour_id.clone().to_string()].join("-");
    let pool_address = &pool.address;

    let mut pool_entity_change: EntityChange = EntityChange::new(
        "LiquidityPoolHourlySnapshot",
        id.as_str(),
        0,
        Operation::Create,
    );

    pool_entity_change
        .change("id", id)
        .change("protocol", "DexAmmProtocol".to_string())
        .change("pool", pool_address)
        .change(
            "totalValueLockedUSD",
            pool_tvl_store
                .get_last(StoreKey::TotalValueLockedUSD.get_unique_pool_key(&pool_address))
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "hourlySupplySideRevenueUSD",
            daily_and_hourly_fields_store
                .get_last(
                    StoreKey::HourlySupplySideRevenueUSD
                        .get_unique_hourly_pool_key(hour_id.clone(), &pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "cumulativeSupplySideRevenueUSD",
            cumulative_fields_store
                .get_last(
                    StoreKey::CumulativeSupplySideRevenueUSD.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "hourlyProtocolSideRevenueUSD",
            daily_and_hourly_fields_store
                .get_last(
                    StoreKey::HourlyProtocolSideRevenueUSD
                        .get_unique_hourly_pool_key(hour_id.clone(), &pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "cumulativeProtocolSideRevenueUSD",
            cumulative_fields_store
                .get_last(
                    StoreKey::CumulativeProtocolSideRevenueUSD.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "hourlyTotalRevenueUSD",
            daily_and_hourly_fields_store
                .get_last(
                    StoreKey::HourlyTotalRevenueUSD
                        .get_unique_hourly_pool_key(hour_id.clone(), &pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "cumulativeTotalRevenueUSD",
            cumulative_fields_store
                .get_last(StoreKey::CumulativeTotalRevenueUSD.get_unique_pool_key(&pool_address))
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "hourlyVolumeUSD",
            daily_and_hourly_fields_store
                .get_last(
                    StoreKey::HourlyVolumeUSD
                        .get_unique_hourly_pool_key(hour_id.clone(), &pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change("hourlyVolumeByTokenAmount", vec!["0".to_string(); 2])
        .change("hourlyVolumeByTokenUSD", vec!["0".to_string(); 2])
        .change(
            "cumulativeVolumeUSD",
            cumulative_fields_store
                .get_last(StoreKey::CumulativeVolumeUSD.get_unique_pool_key(&pool_address))
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "inputTokenBalances",
            vec![
                input_token_balances_store
                    .get_last(StoreKey::Token0Balance.get_unique_pool_key(&pool_address))
                    .unwrap_or(BigInt::zero())
                    .to_string(),
                input_token_balances_store
                    .get_last(StoreKey::Token1Balance.get_unique_pool_key(&pool_address))
                    .unwrap_or(BigInt::zero())
                    .to_string(),
            ],
        )
        .change("inputTokenWeights", vec!["0".to_string(); 2])
        .change(
            "outputTokenSupply",
            output_token_supply_store
                .get_last(StoreKey::OutputTokenBalance.get_unique_pool_key(&pool_address))
                .unwrap_or(BigInt::zero()),
        )
        .change("blockNumber", block_number)
        .change("timestamp", timestamp);

    pool_entity_change
}
