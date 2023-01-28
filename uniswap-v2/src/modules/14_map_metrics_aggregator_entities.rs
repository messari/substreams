use substreams::pb::substreams::store_delta;
use substreams::scalar::BigInt;
use substreams::store::{DeltaBigInt, Deltas, StoreGet};
use substreams::store::{StoreGetBigDecimal, StoreGetBigInt};
use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChange, EntityChanges};

use crate::common::constants;
use crate::store_key::StoreKey;
use crate::store_retriever::StoreRetriever;

#[substreams::handlers::map]
pub fn map_metrics_aggregator_entities(
    unique_users_deltas: Deltas<DeltaBigInt>,
    unique_users_store: StoreGetBigInt,
    pre_aggregations_store: StoreGetBigInt,
    financials_pre_aggregation_store: StoreGetBigDecimal,
) -> Result<EntityChanges, ()> {
    let latest_timestamp = unique_users_store.get_last("latest_timestamp");
    let latest_block_number = unique_users_store.get_last("latest_block_number");

    let mut store_retriever = StoreRetriever::<StoreGetBigInt>::new(&pre_aggregations_store, None);
    let mut financial_store_retriever =
        StoreRetriever::<StoreGetBigDecimal>::new(&financials_pre_aggregation_store, None);

    let metrics_entity = get_protocol_entity_change(&mut store_retriever, unique_users_deltas);
    let mut entity_changes = vec![metrics_entity];

    if latest_timestamp.is_some() && latest_block_number.is_some() {
        store_retriever.set_day_and_hour_timestamp(latest_timestamp.clone());
        financial_store_retriever.set_day_and_hour_timestamp(latest_timestamp.clone());

        entity_changes.push(create_daily_snapshot_entity_change(
            latest_timestamp.clone(),
            latest_block_number.clone(),
            &mut store_retriever,
        ));

        entity_changes.push(create_hourly_snapshot_entity_change(
            latest_timestamp.clone(),
            latest_block_number.clone(),
            &mut store_retriever,
        ));

        entity_changes.push(create_daily_financial_snapshot_entity_change(
            latest_timestamp,
            latest_block_number,
            &mut financial_store_retriever,
        ))
    }

    Ok(EntityChanges { entity_changes })
}

fn get_protocol_entity_change(
    store_retriever: &mut StoreRetriever<StoreGetBigInt>,
    unique_users_deltas: Deltas<DeltaBigInt>,
) -> EntityChange {
    let mut protocol_entity_change = EntityChange::new(
        "DexAmmProtocol",
        constants::PROTOCOL_ID,
        0,
        Operation::Update,
    );

    let mut is_first_operation = false;
    for store_delta in unique_users_deltas.deltas.iter() {
        match &store_delta.key {
            key if key == "latest_timestamp"
                && store_delta.operation == store_delta::Operation::Create =>
            {
                is_first_operation = true;
            }
            _ => {}
        };
    }

    if is_first_operation {
        protocol_entity_change.operation = Operation::Create as i32;

        protocol_entity_change
            .change("id", constants::PROTOCOL_ID.to_string())
            .change("name", constants::PROTOCOL_NAME.to_string())
            .change("slug", constants::PROTOCOL_SLUG.to_string())
            .change("network", constants::NETWORK.to_string())
            .change("protocol_type", constants::PROTOCOL_TYPE.to_string())
            .change(
                "schema_version",
                constants::PROTOCOL_SCHEMA_VERSION.to_string(),
            )
            .change(
                "substream_version",
                constants::PROTOCOL_SUBSTREAM_VERSION.to_string(),
            )
            .change(
                "methodology_version",
                constants::PROTOCOL_METHODOLOGY_VERSION.to_string(),
            );
    }

    protocol_entity_change
        .change(
            "cumulative_unique_users",
            store_retriever.get_cumulative_value(StoreKey::User),
        )
        .change(
            "total_pool_count",
            store_retriever.get_cumulative_value(StoreKey::PoolCount),
        );

    protocol_entity_change
}

fn create_daily_snapshot_entity_change(
    latest_timestamp: Option<BigInt>,
    latest_block_number: Option<BigInt>,
    store_retriever: &mut StoreRetriever<StoreGetBigInt>,
) -> EntityChange {
    let entity_id = store_retriever.get_day_timestamp().to_string();

    let mut day_snapshot_entity_change = EntityChange::new(
        "daily_usage_metrics_snapshots",
        entity_id.as_ref(),
        0,
        Operation::Create,
    );

    day_snapshot_entity_change
        .change("id", entity_id)
        .change("protocol", constants::PROTOCOL_ID.to_string())
        .change(
            "daily_active_users",
            store_retriever.get_daily_stats_value(StoreKey::ActiveUserCount),
        )
        .change(
            "cumulative_unique_users",
            store_retriever.get_cumulative_value(StoreKey::PoolCount),
        )
        .change(
            "daily_transaction_count",
            store_retriever.get_daily_stats_value(StoreKey::TransactionCount),
        )
        .change(
            "daily_deposit_count",
            store_retriever.get_daily_stats_value(StoreKey::DepositCount),
        )
        .change(
            "daily_withdraw_count",
            store_retriever.get_daily_stats_value(StoreKey::WithdrawCount),
        )
        .change(
            "daily_swap_count",
            store_retriever.get_daily_stats_value(StoreKey::SwapCount),
        )
        .change(
            "total_pool_count",
            store_retriever.get_cumulative_value(StoreKey::User),
        )
        .change("block_number", latest_block_number.unwrap())
        .change("timestamp", latest_timestamp.unwrap());

    day_snapshot_entity_change
}

fn create_hourly_snapshot_entity_change(
    latest_timestamp: Option<BigInt>,
    latest_block_number: Option<BigInt>,
    store_retriever: &mut StoreRetriever<StoreGetBigInt>,
) -> EntityChange {
    let entity_id = store_retriever.get_hour_timestamp().to_string();

    let mut hour_snapshot_entity_change = EntityChange::new(
        "hourly_usage_metrics_snapshots",
        entity_id.as_ref(),
        0,
        Operation::Create,
    );

    hour_snapshot_entity_change
        .change("id", entity_id)
        .change("protocol", constants::PROTOCOL_ID.to_string())
        .change(
            "hourly_active_users",
            store_retriever.get_hourly_stats_value(StoreKey::ActiveUserCount),
        )
        .change(
            "cumulative_unique_users",
            store_retriever.get_cumulative_value(StoreKey::PoolCount),
        )
        .change(
            "hourly_transaction_count",
            store_retriever.get_hourly_stats_value(StoreKey::TransactionCount),
        )
        .change(
            "hourly_deposit_count",
            store_retriever.get_hourly_stats_value(StoreKey::DepositCount),
        )
        .change(
            "hourly_withdraw_count",
            store_retriever.get_hourly_stats_value(StoreKey::WithdrawCount),
        )
        .change(
            "hourly_swap_count",
            store_retriever.get_hourly_stats_value(StoreKey::SwapCount),
        )
        .change("block_number", latest_block_number.unwrap())
        .change("timestamp", latest_timestamp.unwrap());

    hour_snapshot_entity_change
}

fn create_daily_financial_snapshot_entity_change(
    latest_timestamp: Option<BigInt>,
    latest_block_number: Option<BigInt>,
    financials_store_retriever: &mut StoreRetriever<StoreGetBigDecimal>,
) -> EntityChange {
    let entity_id = financials_store_retriever.get_day_timestamp().to_string();

    let mut hour_snapshot_entity_change = EntityChange::new(
        "daily_financials_snapshot",
        entity_id.as_ref(),
        0,
        Operation::Create,
    );

    hour_snapshot_entity_change
        .change("id", entity_id)
        .change("protocol", constants::PROTOCOL_ID.to_string())
        .change(
            "total_value_locked_usd",
            financials_store_retriever.get_cumulative_protocol_value(StoreKey::PoolTVL),
        )
        .change(
            "daily_volume_usd",
            financials_store_retriever.get_daily_protocol_field_value(StoreKey::PoolVolume),
        )
        .change(
            "cumulative_volume_usd",
            financials_store_retriever.get_cumulative_protocol_value(StoreKey::PoolVolume),
        )
        .change(
            "daily_supply_side_revenue_usd",
            financials_store_retriever
                .get_daily_protocol_field_value(StoreKey::PoolSupplySideRevenue),
        )
        .change(
            "cumulative_supply_side_revenue_usd",
            financials_store_retriever
                .get_cumulative_protocol_value(StoreKey::PoolSupplySideRevenue),
        )
        .change(
            "daily_protocol_side_revenue_usd",
            financials_store_retriever
                .get_daily_protocol_field_value(StoreKey::PoolProtocolSideRevenue),
        )
        .change(
            "cumulative_protocol_side_revenue_usd",
            financials_store_retriever
                .get_cumulative_protocol_value(StoreKey::PoolProtocolSideRevenue),
        )
        .change(
            "daily_total_revenue_usd",
            financials_store_retriever.get_daily_protocol_field_value(StoreKey::PoolTotalRevenue),
        )
        .change(
            "cumulative_total_revenue_usd",
            financials_store_retriever.get_cumulative_protocol_value(StoreKey::PoolTotalRevenue),
        )
        .change("block_number", latest_block_number.unwrap())
        .change("timestamp", latest_timestamp.unwrap());

    hour_snapshot_entity_change
}
