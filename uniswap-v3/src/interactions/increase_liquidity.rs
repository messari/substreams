use substreams::Hex;
use substreams_ethereum::{pb::eth::v2::{self as eth}};

use crate::dex_amm::v_3_0_3::enums;
use crate::store::sdk;

use crate::schema_lib::dex_amm::v_3_0_3::keys;

use crate::abi::non_fungible_position_manager as NonFungiblePositionManagerContract;
use substreams_ethereum::NULL_ADDRESS;


pub fn create_store_operations_l1_increase_liquidity(
    store_operation_factory: &mut sdk::StoreOperationFactory,
    increase_liquidity_event: NonFungiblePositionManagerContract::events::IncreaseLiquidity, 
) {
    store_operation_factory.track_position_mutation(
        keys::get_position_key(&Hex(&NULL_ADDRESS).to_string(), &increase_liquidity_event.token_id.to_string())
    );
}

pub fn prepare_increase_liquidity_entity_changes(
    entity_update_factory: &mut sdk::EntityUpdateFactory, 
    transaction_trace: &eth::TransactionTrace,
    call: &eth::Call, 
    increase_liquidity_event: NonFungiblePositionManagerContract::events::IncreaseLiquidity, 
) {
    let position_id = &keys::get_position_key(&Hex(&NULL_ADDRESS).to_string(), &increase_liquidity_event.token_id.to_string());
    entity_update_factory.create_position_entity_if_not_exists(
        &transaction_trace,
        &position_id,
        &transaction_trace.from.clone(),
        &call.address,
        2,
        None,
        None,
        None,
        Some(&enums::TokenType::ERC721),
        None,
    );

    entity_update_factory.store_operations.add_position_liquidity(
        0,
        &position_id,
        increase_liquidity_event.liquidity,
    );
    entity_update_factory.store_operations.increment_position_deposit_count(
        0,
        &position_id,
    );
    entity_update_factory.store_operations.add_position_cumulative_deposit_token_amounts(
        0,
        &position_id,
        vec![increase_liquidity_event.amount0.clone(), increase_liquidity_event.amount1.clone()],
    );
}
