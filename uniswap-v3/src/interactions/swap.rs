use std::ops::Mul;
use substreams::log;
use substreams::{Hex, store::StoreGet};
use substreams_ethereum::{pb::eth::v2::{self as eth}};
use substreams_ethereum::NULL_ADDRESS;
use substreams::store;
use substreams::scalar::{BigDecimal, BigInt};

use crate::{pb::{dex_amm::v3_0_3::{
    MappedDataSources, PrunedTransaction}, store::v1::StoreInstruction}, 
    utils::UNISWAP_V3_FACTORY_SLICE, constants
};

use crate::store::store_update;

use crate::abi::pool as PoolContract;
use crate::sdk;
use crate::utils;
use crate::schema_lib::dex_amm::v_3_0_3::enums;

pub fn prepare_swap_entity_changes(
    mapped_data_sources: &mut MappedDataSources, 
    pruned_transaction: &mut PrunedTransaction,
    swap_event: PoolContract::events::Swap, 
    call: &eth::Call, 
    log: &eth::Log,
    set_bigdecimal_l1_store: &store::StoreGetBigDecimal,
    append_string_l1_store: &store::StoreGetArray<String>,
) {
    let pool_address_string: String = Hex(&call.address).to_string();
    let input_tokens = match append_string_l1_store.get_last([pool_address_string.as_str(), "inputTokens"].join(":")) {
        Some(input_tokens) => input_tokens.into_iter().map(|s| s.into_bytes()).collect(),
        None => {
            panic!("No input tokens found for pool address: {}", pool_address_string)
        }
    };
    let amounts = vec![swap_event.amount0.clone(), swap_event.amount1.clone()];
    pruned_transaction.create_swap_entity(
        &call.address, 
        &UNISWAP_V3_FACTORY_SLICE.to_vec(), 
        &pruned_transaction.from.clone(), 
        &input_tokens,
        &amounts, 
        &swap_event.liquidity, 
        Some(&swap_event.tick), 
        log.index, 
        log.ordinal,
    );
    mapped_data_sources.add_liquidity_pool_cumulative_swap_count(
        &pool_address_string, 
        0, 
        1
    );
    mapped_data_sources.add_liquidity_pool_input_token_balances(
        &pool_address_string, 
        0, 
        &vec![swap_event.amount0.clone(), swap_event.amount1.clone()]
    );
    mapped_data_sources.add_liquidity_pool_cumulative_volume_token_amounts(
        &pool_address_string, 
        0, 
        &vec![utils::abs_bigint(&swap_event.amount0), utils::abs_bigint(&swap_event.amount1)]
    );
    mapped_data_sources.set_liquidity_pool_active_liquidity(
        &pool_address_string, 
        0, 
        &swap_event.liquidity
    );
    mapped_data_sources.set_liquidity_pool_tick(
        &pool_address_string, 
        0, 
        &swap_event.tick
    );

    let mut supply_side_revenue_token_amounts: Vec<BigInt> = vec![constants::BIGINT_ZERO.clone(); amounts.len()];
    let mut protocol_side_revenue_token_amounts: Vec<BigInt> = vec![constants::BIGINT_ZERO.clone(); amounts.len()];
    let mut total_revenue_token_amounts: Vec<BigInt> = vec![constants::BIGINT_ZERO.clone(); amounts.len()];

    let supply_side_fee_percentage = match set_bigdecimal_l1_store.get_last([pool_address_string.as_str(), &enums::LiquidityPoolFeeType::FIXED_TRADING_FEE.to_string()].join(":")) {
        Some(supply_side_fee_percentage) => supply_side_fee_percentage,
        None => panic!("No supply side fee percentage found for pool address: {}", pool_address_string)
    };
    let protocol_side_fee_percentage = match set_bigdecimal_l1_store.get_last([pool_address_string.as_str(), &enums::LiquidityPoolFeeType::FIXED_PROTOCOL_FEE.to_string()].join(":")) {
        Some(protocol_side_fee_percentage) => protocol_side_fee_percentage,
        None => panic!("No protocol side fee percentage found for pool address: {}", pool_address_string)
    };
    let total_fee_percentage = match set_bigdecimal_l1_store.get_last([pool_address_string.as_str(), &enums::LiquidityPoolFeeType::FIXED_TRADING_FEE.to_string()].join(":")) {
        Some(total_fee_percentage) => total_fee_percentage,
        None => panic!("No total fee percentage found for pool address: {}", pool_address_string)
    };

    for i in 0..amounts.len() {
        if amounts[i].gt(&constants::BIGINT_ZERO) {
            supply_side_revenue_token_amounts[i] = calculate_fee(&amounts[i], &supply_side_fee_percentage);
            protocol_side_revenue_token_amounts[i] = calculate_fee(&amounts[i], &protocol_side_fee_percentage);
            total_revenue_token_amounts[i] = calculate_fee(&amounts[i], &total_fee_percentage);
        }
    }

    mapped_data_sources.add_liquidity_pool_cumulative_supply_side_revenue_token_amounts(
        &pool_address_string, 
        0, 
        &supply_side_revenue_token_amounts
    );
    mapped_data_sources.add_liquidity_pool_cumulative_protocol_side_revenue_token_amounts(
        &pool_address_string, 
        0, 
        &protocol_side_revenue_token_amounts
    );
    mapped_data_sources.add_liquidity_pool_cumulative_total_revenue_token_amounts(
        &pool_address_string, 
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

pub fn create_store_instructions_l1_swap(
    store_instructions: &mut Vec<StoreInstruction>,
    swap_event: PoolContract::events::Swap, 
    call: &eth::Call, 
    log: &eth::Log
) {}
