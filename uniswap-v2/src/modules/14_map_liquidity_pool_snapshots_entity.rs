use substreams::pb::substreams::{store_delta, Clock};
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{DeltaBigInt, Deltas};
use substreams::store::{StoreGet, StoreGetBigDecimal, StoreGetBigInt, StoreGetProto};
use substreams_entity_change::pb::entity::entity_change::Operation;
use substreams_entity_change::pb::entity::{EntityChange, EntityChanges};

use crate::pb::uniswap::v2::Pool;
use crate::store_key::StoreKey;
use crate::utils;

#[substreams::handlers::map]
pub fn map_liquidity_pool_snapshots_entity(
    clock: Clock,
    pools_store: StoreGetProto<Pool>,
    pool_supply_store: StoreGetBigInt,
    balances_store: StoreGetBigInt,
    balances_deltas: Deltas<DeltaBigInt>,
    pool_tvl_store: StoreGetBigDecimal,
    cumulative_fields_store: StoreGetBigDecimal,
    daily_and_hourly_fields_store: StoreGetBigDecimal,
    volume_by_token_amount_store: StoreGetBigInt,
    prices_store: StoreGetBigDecimal,
) -> Result<EntityChanges, ()> {
    let mut entity_changes: Vec<EntityChange> = vec![];
    let timestamp = BigInt::from(clock.timestamp.unwrap().seconds);

    for delta in balances_deltas.deltas.iter() {
        if let Some(pool_address) = StoreKey::LatestTimestamp.get_pool(&delta.key) {
            let is_initialized = delta.operation != store_delta::Operation::Create;

            let delta_timestamp: i64 = delta.new_value.to_u64() as i64;

            let day_id = utils::get_day_id(delta_timestamp);
            let hour_id = utils::get_hour_id(delta_timestamp);

            let block_number = balances_store
                .get_at(delta.ordinal, StoreKey::LatestBlockNumber.unique_id())
                .unwrap();

            let pool: Pool =
                pools_store.must_get_last(StoreKey::Pool.get_unique_pool_key(&pool_address));

            entity_changes.push(create_liquidity_pool_daily_snapshot(
                delta.ordinal,
                &pool,
                day_id,
                &pool_supply_store,
                &balances_store,
                &pool_tvl_store,
                &cumulative_fields_store,
                &daily_and_hourly_fields_store,
                &volume_by_token_amount_store,
                &prices_store,
                &block_number,
                &timestamp,
                is_initialized,
            ));

            entity_changes.push(create_liquidity_pool_hourly_snapshot(
                delta.ordinal,
                &pool,
                hour_id,
                &pool_supply_store,
                &balances_store,
                &pool_tvl_store,
                &cumulative_fields_store,
                &daily_and_hourly_fields_store,
                &volume_by_token_amount_store,
                &block_number,
                &timestamp,
                is_initialized,
            ));
        }
    }

    Ok(EntityChanges { entity_changes })
}

fn create_liquidity_pool_daily_snapshot(
    ordinal: u64,
    pool: &Pool,
    day_id: i64,
    pool_supply_store: &StoreGetBigInt,
    balances_store: &StoreGetBigInt,
    pool_tvl_store: &StoreGetBigDecimal,
    cumulative_fields_store: &StoreGetBigDecimal,
    daily_and_hourly_fields_store: &StoreGetBigDecimal,
    volume_by_token_amount_store: &StoreGetBigInt,
    prices_store: &StoreGetBigDecimal,
    block_number: &BigInt,
    timestamp: &BigInt,
    is_initialized: bool,
) -> EntityChange {
    let pool_address = &pool.address;
    let id = [pool_address, day_id.to_string().as_str()].join("-");

    let mut pool_entity_change: EntityChange = EntityChange::new(
        "LiquidityPoolDailySnapshot",
        id.as_str(),
        ordinal,
        Operation::Update,
    );

    if !is_initialized {
        pool_entity_change.operation = Operation::Create as i32;
    }

    pool_entity_change
        .change("id", id)
        .change("protocol", "DexAmmProtocol".to_string())
        .change("pool", pool_address)
        .change(
            "totalValueLockedUSD",
            pool_tvl_store
                .get_at(
                    ordinal,
                    StoreKey::TotalValueLockedUSD.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "dailySupplySideRevenueUSD",
            daily_and_hourly_fields_store
                .get_at(
                    ordinal,
                    StoreKey::DailySupplySideRevenueUSD
                        .get_unique_snapshot_key(day_id, vec![&pool_address]),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "cumulativeSupplySideRevenueUSD",
            cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeSupplySideRevenueUSD.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "dailyProtocolSideRevenueUSD",
            daily_and_hourly_fields_store
                .get_at(
                    ordinal,
                    StoreKey::DailyProtocolSideRevenueUSD
                        .get_unique_snapshot_key(day_id, vec![&pool_address]),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "cumulativeProtocolSideRevenueUSD",
            cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeProtocolSideRevenueUSD.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "dailyTotalRevenueUSD",
            daily_and_hourly_fields_store
                .get_at(
                    ordinal,
                    StoreKey::DailyTotalRevenueUSD
                        .get_unique_snapshot_key(day_id, vec![&pool_address]),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "cumulativeTotalRevenueUSD",
            cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeTotalRevenueUSD.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "dailyVolumeUSD",
            daily_and_hourly_fields_store
                .get_at(
                    ordinal,
                    StoreKey::DailyVolumeUSD.get_unique_snapshot_key(day_id, vec![&pool_address]),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "dailyVolumeByTokenAmount",
            vec![
                volume_by_token_amount_store
                    .get_at(
                        ordinal,
                        StoreKey::DailyVolumeByTokenAmount.get_unique_snapshot_key(
                            day_id,
                            vec![&pool_address, &pool.token0_address()],
                        ),
                    )
                    .unwrap_or(BigInt::zero()),
                volume_by_token_amount_store
                    .get_at(
                        ordinal,
                        StoreKey::DailyVolumeByTokenAmount.get_unique_snapshot_key(
                            day_id,
                            vec![&pool_address, &pool.token1_address()],
                        ),
                    )
                    .unwrap_or(BigInt::zero()),
            ],
        )
        .change(
            "dailyVolumeByTokenUSD",
            vec![
                daily_and_hourly_fields_store
                    .get_at(
                        ordinal,
                        StoreKey::DailyVolumeByTokenUSD.get_unique_snapshot_key(
                            day_id,
                            vec![&pool_address, &pool.token0_address()],
                        ),
                    )
                    .unwrap_or(BigDecimal::zero())
                    .with_prec(3),
                daily_and_hourly_fields_store
                    .get_at(
                        ordinal,
                        StoreKey::DailyVolumeByTokenUSD.get_unique_snapshot_key(
                            day_id,
                            vec![&pool_address, &pool.token1_address()],
                        ),
                    )
                    .unwrap_or(BigDecimal::zero())
                    .with_prec(3),
            ],
        )
        .change(
            "cumulativeVolumeUSD",
            cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeVolumeUSD.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "inputTokenBalances",
            vec![
                balances_store
                    .get_at(
                        ordinal,
                        StoreKey::Token0Balance.get_unique_pool_key(&pool_address),
                    )
                    .unwrap_or(BigInt::zero()),
                balances_store
                    .get_at(
                        ordinal,
                        StoreKey::Token1Balance.get_unique_pool_key(&pool_address),
                    )
                    .unwrap_or(BigInt::zero()),
            ],
        )
        .change(
            "inputTokenWeights",
            vec![BigDecimal::try_from("0.5").unwrap(); 2],
        )
        .change(
            "outputTokenSupply",
            pool_supply_store
                .get_at(
                    ordinal,
                    StoreKey::OutputTokenBalance.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigInt::zero()),
        )
        .change(
            "_inputTokenPrices",
            vec![
                prices_store
                    .get_at(
                        ordinal,
                        StoreKey::TokenPrice.get_unique_pool_key(&pool.token0_address()),
                    )
                    .unwrap_or(BigDecimal::zero())
                    .with_prec(3),
                prices_store
                    .get_at(
                        ordinal,
                        StoreKey::TokenPrice.get_unique_pool_key(&pool.token1_address()),
                    )
                    .unwrap_or(BigDecimal::zero())
                    .with_prec(3),
            ],
        )
        .change("blockNumber", block_number)
        .change("timestamp", timestamp);

    pool_entity_change
}

fn create_liquidity_pool_hourly_snapshot(
    ordinal: u64,
    pool: &Pool,
    hour_id: i64,
    pool_supply_store: &StoreGetBigInt,
    balances_store: &StoreGetBigInt,
    pool_tvl_store: &StoreGetBigDecimal,
    cumulative_fields_store: &StoreGetBigDecimal,
    daily_and_hourly_fields_store: &StoreGetBigDecimal,
    volume_by_token_amount_store: &StoreGetBigInt,
    block_number: &BigInt,
    timestamp: &BigInt,
    is_initialized: bool,
) -> EntityChange {
    let id = [pool.clone().address, hour_id.clone().to_string()].join("-");
    let pool_address = &pool.address;

    let mut pool_entity_change: EntityChange = EntityChange::new(
        "LiquidityPoolHourlySnapshot",
        id.as_str(),
        ordinal,
        Operation::Update,
    );

    if !is_initialized {
        pool_entity_change.operation = Operation::Create as i32;
    }

    pool_entity_change
        .change("id", id)
        .change("protocol", "DexAmmProtocol".to_string())
        .change("pool", pool_address)
        .change(
            "totalValueLockedUSD",
            pool_tvl_store
                .get_at(
                    ordinal,
                    StoreKey::TotalValueLockedUSD.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "hourlySupplySideRevenueUSD",
            daily_and_hourly_fields_store
                .get_at(
                    ordinal,
                    StoreKey::HourlySupplySideRevenueUSD
                        .get_unique_snapshot_key(hour_id, vec![&pool_address]),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "cumulativeSupplySideRevenueUSD",
            cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeSupplySideRevenueUSD.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "hourlyProtocolSideRevenueUSD",
            daily_and_hourly_fields_store
                .get_at(
                    ordinal,
                    StoreKey::HourlyProtocolSideRevenueUSD
                        .get_unique_snapshot_key(hour_id, vec![&pool_address]),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "cumulativeProtocolSideRevenueUSD",
            cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeProtocolSideRevenueUSD.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "hourlyTotalRevenueUSD",
            daily_and_hourly_fields_store
                .get_at(
                    ordinal,
                    StoreKey::HourlyTotalRevenueUSD
                        .get_unique_snapshot_key(hour_id, vec![&pool_address]),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "cumulativeTotalRevenueUSD",
            cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeTotalRevenueUSD.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "hourlyVolumeUSD",
            daily_and_hourly_fields_store
                .get_at(
                    ordinal,
                    StoreKey::HourlyVolumeUSD.get_unique_snapshot_key(hour_id, vec![&pool_address]),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "hourlyVolumeByTokenAmount",
            vec![
                volume_by_token_amount_store
                    .get_at(
                        ordinal,
                        StoreKey::HourlyVolumeByTokenAmount.get_unique_snapshot_key(
                            hour_id,
                            vec![&pool_address, &pool.token0_address()],
                        ),
                    )
                    .unwrap_or(BigInt::zero()),
                volume_by_token_amount_store
                    .get_at(
                        ordinal,
                        StoreKey::HourlyVolumeByTokenAmount.get_unique_snapshot_key(
                            hour_id,
                            vec![&pool_address, &pool.token1_address()],
                        ),
                    )
                    .unwrap_or(BigInt::zero()),
            ],
        )
        .change(
            "hourlyVolumeByTokenUSD",
            vec![
                daily_and_hourly_fields_store
                    .get_at(
                        ordinal,
                        StoreKey::HourlyVolumeByTokenUSD.get_unique_snapshot_key(
                            hour_id,
                            vec![&pool_address, &pool.token0_address()],
                        ),
                    )
                    .unwrap_or(BigDecimal::zero())
                    .with_prec(3),
                daily_and_hourly_fields_store
                    .get_at(
                        ordinal,
                        StoreKey::HourlyVolumeByTokenUSD.get_unique_snapshot_key(
                            hour_id,
                            vec![&pool_address, &pool.token1_address()],
                        ),
                    )
                    .unwrap_or(BigDecimal::zero())
                    .with_prec(3),
            ],
        )
        .change(
            "cumulativeVolumeUSD",
            cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeVolumeUSD.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
        )
        .change(
            "inputTokenBalances",
            vec![
                balances_store
                    .get_at(
                        ordinal,
                        StoreKey::Token0Balance.get_unique_pool_key(&pool_address),
                    )
                    .unwrap_or(BigInt::zero()),
                balances_store
                    .get_at(
                        ordinal,
                        StoreKey::Token1Balance.get_unique_pool_key(&pool_address),
                    )
                    .unwrap_or(BigInt::zero()),
            ],
        )
        .change(
            "inputTokenWeights",
            vec![BigDecimal::try_from("0.5").unwrap(); 2],
        )
        .change(
            "outputTokenSupply",
            pool_supply_store
                .get_at(
                    ordinal,
                    StoreKey::OutputTokenBalance.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigInt::zero()),
        )
        .change("blockNumber", block_number)
        .change("timestamp", timestamp);

    pool_entity_change
}
