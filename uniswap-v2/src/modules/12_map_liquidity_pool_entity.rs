use substreams::pb::substreams::store_delta;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{DeltaBigInt, Deltas};
use substreams::store::{StoreGet, StoreGetBigDecimal, StoreGetBigInt, StoreGetProto};
use substreams_entity_change::pb::entity::entity_change::Operation;
use substreams_entity_change::pb::entity::{EntityChange, EntityChanges};

use crate::common::constants;
use crate::pb::uniswap::v2::Pool;
use crate::store_key::StoreKey;

#[substreams::handlers::map]
pub fn map_liquidity_pool_entity(
    pools_store: StoreGetProto<Pool>,
    pool_supply_store: StoreGetBigInt,
    balances_store: StoreGetBigInt,
    balances_deltas: Deltas<DeltaBigInt>,
    pool_tvl_store: StoreGetBigDecimal,
    cumulative_fields_store: StoreGetBigDecimal,
) -> Result<EntityChanges, ()> {
    let mut entity_changes: Vec<EntityChange> = vec![];

    for delta in balances_deltas.deltas.iter() {
        if let Some(pool_address) = StoreKey::TotalBalance.get_pool(&delta.key) {
            let is_initialized = delta.operation != store_delta::Operation::Create;

            let pool: Pool =
                pools_store.must_get_last(StoreKey::Pool.get_unique_pool_key(&pool_address));

            entity_changes.push(create_liquidity_pool(
                delta.ordinal,
                &pool,
                &pool_supply_store,
                &balances_store,
                &pool_tvl_store,
                &cumulative_fields_store,
                is_initialized,
            ));
        }
    }

    Ok(EntityChanges { entity_changes })
}

fn create_liquidity_pool(
    ordinal: u64,
    pool: &Pool,
    output_token_supply_store: &StoreGetBigInt,
    input_token_balances_store: &StoreGetBigInt,
    pool_tvl_store: &StoreGetBigDecimal,
    cumulative_fields_store: &StoreGetBigDecimal,
    is_initialized: bool,
) -> EntityChange {
    let pool_address = &pool.address;

    let mut entity_change: EntityChange =
        EntityChange::new("LiquidityPool", pool_address, ordinal, Operation::Update);

    if !is_initialized {
        let fees: Vec<String> = vec![];
        entity_change.operation = Operation::Create as i32;

        entity_change
            .change("id", pool_address)
            .change("protocol", constants::UNISWAP_V2_FACTORY.to_string())
            .change("name", &pool.name)
            .change("symbol", &pool.symbol)
            .change("inputTokens", pool.input_tokens())
            .change("outputToken", pool.output_token_address())
            .change("fees", &fees)
            .change("isSingleSided", false)
            .change("createdTimestamp", BigInt::from(pool.created_timestamp))
            .change(
                "createdBlockNumber",
                BigInt::from(pool.created_block_number),
            );
    }

    entity_change
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
            "cumulativeSupplySideRevenueUSD",
            cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeSupplySideRevenueUSD.get_unique_pool_key(&pool_address),
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
            "cumulativeTotalRevenueUSD",
            cumulative_fields_store
                .get_at(
                    ordinal,
                    StoreKey::CumulativeTotalRevenueUSD.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigDecimal::zero()),
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
                input_token_balances_store
                    .get_at(
                        ordinal,
                        StoreKey::Token0Balance.get_unique_pool_key(&pool_address),
                    )
                    .unwrap_or(BigInt::zero()),
                input_token_balances_store
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
            output_token_supply_store
                .get_at(
                    ordinal,
                    StoreKey::OutputTokenBalance.get_unique_pool_key(&pool_address),
                )
                .unwrap_or(BigInt::zero()),
        );

    entity_change
}
