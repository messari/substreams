use std::ops::Mul;
use substreams::log;
use substreams::{Hex, store::StoreGet};
use substreams_ethereum::{pb::eth::v2::{self as eth}};
use substreams_ethereum::NULL_ADDRESS;
use substreams::store;
use substreams::scalar::{BigDecimal, BigInt};

use crate::pb::store::v1::StoreOperation;
use crate::utils::UNISWAP_V3_FACTORY_SLICE; 
use crate::constants;

use crate::store::store_operations;

use crate::abi::pool as PoolContract;
use crate::sdk;
use crate::utils;
use crate::schema_lib::dex_amm::v_3_0_3::{enums, keys};

pub fn prepare_swap_entity_changes(
    entity_update_factory: &mut sdk::DexAmmEntityUpdateFactory, 
    transaction_trace: &eth::TransactionTrace,
    call: &eth::Call, 
    log: &eth::Log,
    swap_event: PoolContract::events::Swap, 
    set_bigdecimal_l1_store: &store::StoreGetBigDecimal,
    append_string_l1_store: &store::StoreGetArray<String>,
) {
    let liquidity_pool_id: String = Hex(&call.address).to_string(); 

    let liquidity_pool_fee_supply_side = keys::get_liquidity_pool_fee_key(&liquidity_pool_id, &enums::LiquidityPoolFeeType::FIXED_LP_FEE);
    let liquidity_pool_fee_protocol_side = keys::get_liquidity_pool_fee_key(&liquidity_pool_id, &enums::LiquidityPoolFeeType::FIXED_PROTOCOL_FEE);
    let liquidity_pool_fee_total = keys::get_liquidity_pool_fee_key(&liquidity_pool_id, &enums::LiquidityPoolFeeType::FIXED_TRADING_FEE);
    
    let input_tokens: Vec<Vec<u8>> = match append_string_l1_store.get_last(["LiquidityPool", liquidity_pool_id.as_str(), "inputTokens"].join(":")) {
        Some(input_tokens) => input_tokens.into_iter().map(|s| s.into_bytes()).collect(),
        None => {
            panic!("No input tokens found for pool address: {}", liquidity_pool_id)
        }
    };
    let amounts = vec![swap_event.amount0.clone(), swap_event.amount1.clone()];
    entity_update_factory.create_swap_entity(
        transaction_trace,
        &liquidity_pool_id,
        &call.address, 
        &UNISWAP_V3_FACTORY_SLICE.to_vec(), 
        &transaction_trace.from.clone(), 
        &input_tokens,
        &amounts, 
        &swap_event.liquidity, 
        Some(&swap_event.tick), 
        &transaction_trace.hash.clone(),
        log.index, 
        log.ordinal,
    );
    entity_update_factory.store_operations.add_liquidity_pool_cumulative_swap_count(
        &liquidity_pool_id, 
        0, 
        1
    );
    entity_update_factory.store_operations.add_liquidity_pool_input_token_balances(
        &liquidity_pool_id, 
        0, 
        &vec![swap_event.amount0.clone(), swap_event.amount1.clone()]
    );
    entity_update_factory.store_operations.add_liquidity_pool_cumulative_volume_token_amounts(
        &liquidity_pool_id, 
        0, 
        &vec![utils::abs_bigint(&swap_event.amount0), utils::abs_bigint(&swap_event.amount1)]
    );
    entity_update_factory.store_operations.set_liquidity_pool_active_liquidity(
        &liquidity_pool_id, 
        0, 
        &swap_event.liquidity
    );
    entity_update_factory.store_operations.set_liquidity_pool_tick(
        &liquidity_pool_id, 
        0, 
        &swap_event.tick
    );

    let mut supply_side_revenue_token_amounts: Vec<BigInt> = vec![constants::BIGINT_ZERO.clone(); amounts.len()];
    let mut protocol_side_revenue_token_amounts: Vec<BigInt> = vec![constants::BIGINT_ZERO.clone(); amounts.len()];
    let mut total_revenue_token_amounts: Vec<BigInt> = vec![constants::BIGINT_ZERO.clone(); amounts.len()];

    let supply_side_fee_percentage = match set_bigdecimal_l1_store.get_at(0, ["LiquidityPoolFee", &liquidity_pool_fee_supply_side, "feePercentage"].join(":")) {
        Some(supply_side_fee_percentage) => supply_side_fee_percentage,
        None => constants::BIGDECIMAL_ZERO.clone()
    };
    let protocol_side_fee_percentage = match set_bigdecimal_l1_store.get_at(0, ["LiquidityPoolFee", &liquidity_pool_fee_protocol_side, "feePercentage"].join(":")) {
        Some(protocol_side_fee_percentage) => protocol_side_fee_percentage,
        None => constants::BIGDECIMAL_ZERO.clone()
    };
    let total_fee_percentage = match set_bigdecimal_l1_store.get_at(0, ["LiquidityPoolFee", &liquidity_pool_fee_total, "feePercentage"].join(":")) {
        Some(total_fee_percentage) => total_fee_percentage,
        None => constants::BIGDECIMAL_ZERO.clone()
    };

    for i in 0..amounts.len() {
        if amounts[i].gt(&constants::BIGINT_ZERO) {
            supply_side_revenue_token_amounts[i] = calculate_fee(&amounts[i], &supply_side_fee_percentage);
            protocol_side_revenue_token_amounts[i] = calculate_fee(&amounts[i], &protocol_side_fee_percentage);
            total_revenue_token_amounts[i] = calculate_fee(&amounts[i], &total_fee_percentage);
        }
    }

    entity_update_factory.store_operations.add_liquidity_pool_cumulative_supply_side_revenue_token_amounts(
        &liquidity_pool_id, 
        0, 
        &supply_side_revenue_token_amounts
    );
    entity_update_factory.store_operations.add_liquidity_pool_cumulative_protocol_side_revenue_token_amounts(
        &liquidity_pool_id, 
        0, 
        &protocol_side_revenue_token_amounts
    );
    entity_update_factory.store_operations.add_liquidity_pool_cumulative_total_revenue_token_amounts(
        &liquidity_pool_id, 
        0, 
        &total_revenue_token_amounts
    );
}

pub fn calculate_fee(
    amount: &BigInt, 
    fee_percentage: &BigDecimal
) -> BigInt {
    let amount_bd: BigDecimal = utils::bigint_to_bigdecimal(amount);
    let fee = fee_percentage.clone().mul(amount_bd.clone()) / constants::BIGDECIMAL_100.clone();
    utils::bigdecimal_to_bigint(&fee)
}

pub fn create_store_operations_l1_swap(
    store_operations: &mut Vec<StoreOperation>,
    swap_event: PoolContract::events::Swap, 
    call: &eth::Call, 
    log: &eth::Log
) {}
