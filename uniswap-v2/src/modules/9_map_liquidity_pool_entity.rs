use substreams::pb::substreams::store_delta;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{DeltaBigInt, Deltas};
use substreams::store::{StoreGet, StoreGetBigDecimal, StoreGetBigInt, StoreGetProto};
use substreams_entity_change::pb::entity::entity_change::Operation;
use substreams_entity_change::pb::entity::{entity_change, EntityChange, EntityChanges};

use crate::pb::erc20::v1::Erc20Token;
use crate::pb::uniswap::v2::Pool;
use crate::store_key::StoreKey;
use crate::utils::UNISWAP_V2_FACTORY;

#[substreams::handlers::map]
pub fn map_liquidity_pool_entity(
    pools_store: StoreGetProto<Pool>,
    output_token_supply_store: StoreGetBigInt,
    input_token_balances_store: StoreGetBigInt,
    input_token_balances_deltas: Deltas<DeltaBigInt>,
    pool_tvl_store: StoreGetBigDecimal,
    cumulative_fields_store: StoreGetBigDecimal,
) -> Result<EntityChanges, ()> {
    let mut entity_changes: Vec<EntityChange> = vec![];

    for delta in input_token_balances_deltas.deltas.iter() {
        if let Some(pool_address) = StoreKey::TotalBalance.get_pool(&delta.key) {
            let mut is_initialized: bool = true;

            if delta.operation == store_delta::Operation::Create {
                is_initialized = false;
            }

            let pool: Pool =
                pools_store.must_get_last(StoreKey::Pool.get_unique_pool_key(&pool_address));

            entity_changes.push(create_liquidity_pool(
                &pool,
                &output_token_supply_store,
                &input_token_balances_store,
                &pool_tvl_store,
                &cumulative_fields_store,
                is_initialized,
            ));
        }
    }

    Ok(EntityChanges { entity_changes })
}

fn _create_token(token: Erc20Token) -> EntityChange {
    let token_address = &token.address;

    let mut token_entity_change =
        EntityChange::new("Token", token_address, 0, entity_change::Operation::Create);

    token_entity_change
        .change("id", token_address)
        .change("name", &token.name)
        .change("symbol", &token.symbol)
        .change("decimals", token.decimals as i32)
        .change("lastPriceUSD", BigDecimal::zero());

    token_entity_change
}

fn create_liquidity_pool(
    pool: &Pool,
    output_token_supply_store: &StoreGetBigInt,
    input_token_balances_store: &StoreGetBigInt,
    pool_tvl_store: &StoreGetBigDecimal,
    cumulative_fields_store: &StoreGetBigDecimal,
    is_initialized: bool,
) -> EntityChange {
    let pool_address = &pool.address;

    let mut pool_entity_change: EntityChange =
        EntityChange::new("LiquidityPool", pool_address, 0, Operation::Update);

    if !is_initialized {
        let fees: Vec<String> = vec![];
        pool_entity_change.operation = Operation::Create as i32;

        pool_entity_change
            .change("id", pool_address)
            .change("protocol", UNISWAP_V2_FACTORY.to_string())
            .change("name", &pool.name)
            .change("symbol", &pool.symbol)
            .change("inputTokens", pool.input_tokens())
            .change("outputToken", pool.output_token())
            .change("fees", &fees)
            .change("isSingleSided", false)
            .change("createdTimestamp", BigInt::from(pool.created_timestamp))
            .change(
                "createdBlockNumber",
                BigInt::from(pool.created_block_number),
            );
    }

    pool_entity_change
        .change(
            "totalValueLockedUSD",
            pool_tvl_store
                .get_last(StoreKey::TotalValueLockedUSD.get_unique_pool_key(&pool_address))
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
            "cumulativeProtocolSideRevenueUSD",
            cumulative_fields_store
                .get_last(
                    StoreKey::CumulativeProtocolSideRevenueUSD.get_unique_pool_key(&pool_address),
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
        );

    pool_entity_change
}
