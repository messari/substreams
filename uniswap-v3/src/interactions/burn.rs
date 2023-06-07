use substreams::{Hex, store::StoreGet};
use substreams_ethereum::{pb::eth::v2::{self as eth}};
use substreams::store;

use crate::utils::UNISWAP_V3_FACTORY_SLICE;
use crate::schema_lib::dex_amm::v_3_0_3::keys;

use crate::abi::pool as PoolContract;
use crate::store::sdk;


pub fn create_store_operations_l1_burn(
    store_operation_factory: &mut sdk::StoreOperationFactory,
    burn_event: PoolContract::events::Burn, 
    call: &eth::Call, 
) {
    let pool_address = Hex(&call.address).to_string();
    store_operation_factory.track_tick_mutation(
        keys::get_tick_key(&pool_address, &burn_event.tick_lower)
    );
    store_operation_factory.track_tick_mutation(
        keys::get_tick_key(&pool_address, &burn_event.tick_upper)
    );
}

pub fn prepare_burn_entity_changes(
    entity_update_factory: &mut sdk::EntityUpdateFactory, 
    transaction_trace: &eth::TransactionTrace,
    call: &eth::Call, 
    log: &eth::Log,
    burn_event: PoolContract::events::Burn, 
    append_string_l1_store: &store::StoreGetArray<String>,
) {
    let liquidity_pool_id: String = Hex(&call.address).to_string(); 
    let tick_lower_id = keys::get_tick_key(&liquidity_pool_id, &burn_event.tick_lower);
    let tick_upper_id = keys::get_tick_key(&liquidity_pool_id, &burn_event.tick_upper);

    let input_tokens = match append_string_l1_store.get_last(["raw", "LiquidityPool", liquidity_pool_id.as_str(), "inputTokens"].join(":")) {
        Some(input_tokens) => input_tokens.into_iter().map(|s| s.into_bytes()).collect(),
        None => {
            panic!("No input tokens found for pool address: {}", liquidity_pool_id)
        }
    };
    entity_update_factory.create_withdraw_entity(
        transaction_trace,
        &call.address,
        &UNISWAP_V3_FACTORY_SLICE.to_vec(),
        &transaction_trace.from.clone(),
        &burn_event.amount,
        &input_tokens,
        &vec![burn_event.amount0.clone(), burn_event.amount1.clone()],
        None,
        Some(&burn_event.tick_lower),
        Some(&burn_event.tick_upper),
        &transaction_trace.hash.clone(),
        log.index,
        log.ordinal,
    );

    entity_update_factory.create_tick_entity_if_not_exists(
        transaction_trace,
        &tick_lower_id,
        &call.address,
        &burn_event.tick_lower,
    );

    entity_update_factory.create_tick_entity_if_not_exists(
        transaction_trace,
        &tick_upper_id,
        &call.address,
        &burn_event.tick_upper,
    );

    entity_update_factory.store_operations.add_liquidity_pool_input_token_balances(
        0,
        &liquidity_pool_id, 
        vec![burn_event.amount0.clone().neg(), burn_event.amount1.clone().neg()]
    );
    entity_update_factory.store_operations.add_liquidity_pool_total_liquidity(
        0,
        &liquidity_pool_id, 
        burn_event.amount.neg()
    );
    entity_update_factory.store_operations.increment_liquidity_pool_cumulative_withdraw_count(
        0,
        &liquidity_pool_id, 
    );

    entity_update_factory.store_operations.add_tick_liquidity_gross(
        0,
        &tick_lower_id, 
        burn_event.amount.neg(),
    );
    entity_update_factory.store_operations.add_tick_liquidity_net(
        0,
        &tick_lower_id, 
        burn_event.amount.neg(),
    );

    entity_update_factory.store_operations.add_tick_liquidity_gross(
        0,
        &tick_upper_id, 
        burn_event.amount.neg(),
    );
    entity_update_factory.store_operations.add_tick_liquidity_net(
        0,
        &tick_upper_id, 
        burn_event.amount,
    );
}
