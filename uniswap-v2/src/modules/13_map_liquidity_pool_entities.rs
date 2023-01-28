use prost::Message;
use substreams::pb::substreams::store_delta;
use substreams::scalar::BigInt;
use substreams::store::{DeltaBigInt, Deltas};
use substreams::store::{StoreGet, StoreGetProto};
use substreams::store::{StoreGetBigDecimal, StoreGetBigInt};
use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChange, EntityChanges};

use crate::common::constants;
use crate::pb::uniswap::v2::Pool;
use crate::store_key::StoreKey;
use crate::store_retriever::StoreRetriever;

#[substreams::handlers::map]
pub fn map_liquidity_pool_entities(
    pool_store: StoreGetProto<Pool>,
    native_tvl_deltas: Deltas<DeltaBigInt>,
    native_tvl_store: StoreGetBigInt,
    tvl_store: StoreGetBigDecimal,
    volume_and_revenue_store: StoreGetBigDecimal,
    pool_balance_store: StoreGetBigInt,
) -> Result<EntityChanges, ()> {
    let mut entity_changes: Vec<EntityChange> = vec![];

    let mut native_tvl_store_retriever =
        StoreRetriever::<StoreGetBigInt>::new(&native_tvl_store, None);
    let mut pool_balance_store_retriever =
        StoreRetriever::<StoreGetBigInt>::new(&pool_balance_store, None);
    let mut volume_and_revenue_store_retriever =
        StoreRetriever::<StoreGetBigDecimal>::new(&volume_and_revenue_store, None);
    let mut tvl_store_retriever = StoreRetriever::<StoreGetBigDecimal>::new(&tvl_store, None);

    for native_pool_delta in native_tvl_deltas.deltas.iter() {
        match &native_pool_delta.key {
            key if key.starts_with("1Pool") => {
                let mut is_initialized = true;
                if native_pool_delta.operation == store_delta::Operation::Create {
                    is_initialized = false;
                }

                let pool_address = StoreKey::Pool.get_pool_from_key(key);

                if let Some(pool) =
                    pool_store.get_last(StoreKey::Pool.get_unique_pool_key(&pool_address))
                {
                    let latest_timestamp = native_tvl_store.get_last("latest_timestamp");
                    let latest_block_number = native_tvl_store.get_last("latest_block_number");

                    entity_changes.push(get_liquidity_pool_entity_change(
                        pool.clone(),
                        &native_tvl_store,
                        &mut pool_balance_store_retriever,
                        &mut tvl_store_retriever,
                        &mut volume_and_revenue_store_retriever,
                        is_initialized,
                    ));

                    if latest_timestamp.is_some() && latest_block_number.is_some() {
                        tvl_store_retriever.set_day_and_hour_timestamp(latest_timestamp.clone());
                        native_tvl_store_retriever
                            .set_day_and_hour_timestamp(latest_timestamp.clone());
                        pool_balance_store_retriever
                            .set_day_and_hour_timestamp(latest_timestamp.clone());
                        volume_and_revenue_store_retriever
                            .set_day_and_hour_timestamp(latest_timestamp.clone());

                        entity_changes.push(create_daily_snapshot_entity_change(
                            pool.clone(),
                            &native_tvl_store,
                            &mut native_tvl_store_retriever,
                            &mut pool_balance_store_retriever,
                            &mut tvl_store_retriever,
                            &mut volume_and_revenue_store_retriever,
                        ));

                        entity_changes.push(create_hourly_snapshot_entity_change(
                            pool.clone(),
                            &native_tvl_store,
                            &mut native_tvl_store_retriever,
                            &mut pool_balance_store_retriever,
                            &mut tvl_store_retriever,
                            &mut volume_and_revenue_store_retriever,
                        ));
                    }
                }
            }
            _ => {}
        }
    }

    Ok(EntityChanges { entity_changes })
}

fn get_liquidity_pool_entity_change(
    pool: Pool,
    native_tvl_store: &StoreGetBigInt,
    pool_balance_store_retriever: &mut StoreRetriever<StoreGetBigInt>,
    tvl_store_retriever: &mut StoreRetriever<StoreGetBigDecimal>,
    volume_and_revenue_store_retriever: &mut StoreRetriever<StoreGetBigDecimal>,
    is_initialized: bool,
) -> EntityChange {
    let pool_address = &pool.address;

    let mut pool_entity_change =
        EntityChange::new("DexAmmProtocol", pool_address, 0, Operation::Update);

    if !is_initialized {
        pool_entity_change.operation = Operation::Create as i32;

        pool_entity_change
            .change("id", pool_address)
            .change("name", &pool.name)
            .change("symbol", &pool.symbol)
            .change(
                "input_tokens",
                &pool.input_tokens.as_ref().unwrap().encode_to_vec(),
            )
            .change(
                "output_token",
                &pool.output_token.as_ref().unwrap().encode_to_vec(),
            )
            .change("is_single_sided", false)
            .change("created_timestamp", BigInt::from(pool.created_timestamp))
            .change(
                "created_block_number",
                BigInt::from(pool.created_block_number),
            );
    }

    pool_entity_change
        .change(
            "total_value_locked_usd",
            tvl_store_retriever.get_cumulative_pool_value(StoreKey::PoolTVL, pool_address),
        )
        .change(
            "cumulative_supply_side_revenue_usd",
            volume_and_revenue_store_retriever
                .get_cumulative_pool_value(StoreKey::PoolSupplySideRevenue, pool_address),
        )
        .change(
            "cumulative_protocol_side_revenue_usd",
            volume_and_revenue_store_retriever
                .get_cumulative_pool_value(StoreKey::PoolProtocolSideRevenue, pool_address),
        )
        .change(
            "cumulative_total_revenue_usd",
            volume_and_revenue_store_retriever
                .get_cumulative_pool_value(StoreKey::PoolTotalRevenue, pool_address),
        )
        .change(
            "cumulative_volume_usd",
            volume_and_revenue_store_retriever
                .get_cumulative_pool_value(StoreKey::PoolVolume, pool_address),
        )
        .change(
            "input_token_balances",
            pool.input_token_balances(native_tvl_store),
        )
        .change(
            "output_token_supply",
            pool_balance_store_retriever
                .get_pool_non_static_field(StoreKey::PoolOutputTokenSupply, pool_address),
        );

    pool_entity_change
}

fn create_daily_snapshot_entity_change(
    pool: Pool,
    native_tvl_store: &StoreGetBigInt,
    native_tvl_store_retriever: &mut StoreRetriever<StoreGetBigInt>,
    pool_balance_store_retriever: &mut StoreRetriever<StoreGetBigInt>,
    tvl_store_retriever: &mut StoreRetriever<StoreGetBigDecimal>,
    volume_and_revenue_store_retriever: &mut StoreRetriever<StoreGetBigDecimal>,
) -> EntityChange {
    let pool_address = &pool.address;
    let entity_id = format!(
        "{}-{}",
        pool_address,
        native_tvl_store_retriever.get_day_timestamp().to_string()
    );

    let mut day_snapshot_entity_change = EntityChange::new(
        "daily_usage_metrics_snapshots",
        entity_id.as_ref(),
        0,
        Operation::Create,
    );

    day_snapshot_entity_change
        .change("id", entity_id)
        .change("protocol", constants::PROTOCOL_ID.to_string())
        .change("pool", pool_address)
        .change(
            "total_value_locked_usd",
            tvl_store_retriever.get_cumulative_pool_value(StoreKey::PoolTVL, pool_address),
        )
        .change(
            "cumulative_supply_side_revenue_usd",
            volume_and_revenue_store_retriever
                .get_cumulative_pool_value(StoreKey::PoolSupplySideRevenue, pool_address),
        )
        .change(
            "daily_supply_side_revenue_usd",
            volume_and_revenue_store_retriever
                .get_daily_pool_field_value(StoreKey::PoolSupplySideRevenue, pool_address),
        )
        .change(
            "cumulative_protocol_side_revenue_usd",
            volume_and_revenue_store_retriever
                .get_cumulative_pool_value(StoreKey::PoolProtocolSideRevenue, pool_address),
        )
        .change(
            "daily_protocol_side_revenue_usd",
            volume_and_revenue_store_retriever
                .get_daily_pool_field_value(StoreKey::PoolProtocolSideRevenue, pool_address),
        )
        .change(
            "cumulative_total_revenue_usd",
            volume_and_revenue_store_retriever
                .get_cumulative_pool_value(StoreKey::PoolTotalRevenue, pool_address),
        )
        .change(
            "daily_total_revenue_usd",
            volume_and_revenue_store_retriever
                .get_daily_pool_field_value(StoreKey::PoolTotalRevenue, pool_address),
        )
        .change(
            "cumulative_volume_usd",
            volume_and_revenue_store_retriever
                .get_cumulative_pool_value(StoreKey::PoolVolume, pool_address),
        )
        .change(
            "daily_volume_usd",
            volume_and_revenue_store_retriever
                .get_daily_pool_field_value(StoreKey::PoolVolume, pool_address),
        )
        .change(
            "input_token_balances",
            pool.input_token_balances(native_tvl_store),
        )
        .change(
            "output_token_supply",
            pool_balance_store_retriever
                .get_pool_non_static_field(StoreKey::PoolOutputTokenSupply, pool_address),
        )
        .change(
            "block_number",
            native_tvl_store.get_last("latest_block_number").unwrap(),
        )
        .change(
            "timestamp",
            native_tvl_store.get_last("latest_timestamp").unwrap(),
        );

    day_snapshot_entity_change
}

fn create_hourly_snapshot_entity_change(
    pool: Pool,
    native_tvl_store: &StoreGetBigInt,
    native_tvl_store_retriever: &mut StoreRetriever<StoreGetBigInt>,
    pool_balance_store_retriever: &mut StoreRetriever<StoreGetBigInt>,
    tvl_store_retriever: &mut StoreRetriever<StoreGetBigDecimal>,
    volume_and_revenue_store_retriever: &mut StoreRetriever<StoreGetBigDecimal>,
) -> EntityChange {
    let pool_address = &pool.address;
    let entity_id = format!(
        "{}-{}",
        pool_address,
        native_tvl_store_retriever.get_hour_timestamp().to_string()
    );

    let mut hour_snapshot_entity_change = EntityChange::new(
        "hourly_usage_metrics_snapshots",
        entity_id.as_ref(),
        0,
        Operation::Create,
    );

    hour_snapshot_entity_change
        .change("id", entity_id)
        .change("protocol", constants::PROTOCOL_ID.to_string())
        .change("pool", pool_address)
        .change(
            "total_value_locked_usd",
            tvl_store_retriever.get_cumulative_pool_value(StoreKey::PoolTVL, pool_address),
        )
        .change(
            "cumulative_supply_side_revenue_usd",
            volume_and_revenue_store_retriever
                .get_cumulative_pool_value(StoreKey::PoolSupplySideRevenue, pool_address),
        )
        .change(
            "hourly_supply_side_revenue_usd",
            volume_and_revenue_store_retriever
                .get_hourly_pool_field_value(StoreKey::PoolSupplySideRevenue, pool_address),
        )
        .change(
            "cumulative_protocol_side_revenue_usd",
            volume_and_revenue_store_retriever
                .get_cumulative_pool_value(StoreKey::PoolProtocolSideRevenue, pool_address),
        )
        .change(
            "hourly_protocol_side_revenue_usd",
            volume_and_revenue_store_retriever
                .get_hourly_pool_field_value(StoreKey::PoolProtocolSideRevenue, pool_address),
        )
        .change(
            "cumulative_total_revenue_usd",
            volume_and_revenue_store_retriever
                .get_cumulative_pool_value(StoreKey::PoolTotalRevenue, pool_address),
        )
        .change(
            "hourly_total_revenue_usd",
            volume_and_revenue_store_retriever
                .get_hourly_pool_field_value(StoreKey::PoolTotalRevenue, pool_address),
        )
        .change(
            "cumulative_volume_usd",
            volume_and_revenue_store_retriever
                .get_cumulative_pool_value(StoreKey::PoolVolume, pool_address),
        )
        .change(
            "hourly_volume_usd",
            volume_and_revenue_store_retriever
                .get_hourly_pool_field_value(StoreKey::PoolVolume, pool_address),
        )
        .change(
            "input_token_balances",
            pool.input_token_balances(native_tvl_store),
        )
        .change(
            "output_token_supply",
            pool_balance_store_retriever
                .get_pool_non_static_field(StoreKey::PoolOutputTokenSupply, pool_address),
        )
        .change(
            "block_number",
            native_tvl_store.get_last("latest_block_number").unwrap(),
        )
        .change(
            "timestamp",
            native_tvl_store.get_last("latest_timestamp").unwrap(),
        );

    hour_snapshot_entity_change
}
