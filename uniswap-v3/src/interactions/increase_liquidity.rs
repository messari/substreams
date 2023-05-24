use substreams::{Hex, store::StoreGet};
use substreams_ethereum::{pb::eth::v2::{self as eth}};
use substreams::store;
use substreams::log;

use crate::dex_amm::v_3_0_3::enums;
use crate::store::sdk;

use crate::store::store_operations;
use crate::pb::store::v1::{StoreOperation, StoreOperations};

use crate::abi::nonFungiblePositionManager as NonFungiblePositionManagerContract;
use substreams_ethereum::NULL_ADDRESS;


pub fn create_store_operations_l1_increase_liquidity(
    store_operations: &mut StoreOperations,
    increase_liquidity_event: NonFungiblePositionManagerContract::events::IncreaseLiquidity, 
    call: &eth::Call, 
    log: &eth::Log,
) {
    store_operations.instructions.push(
        store_operations::add_int_64(
            0, 
            ["mutable-entity-count", "Position", &Hex(&NULL_ADDRESS).to_string(), increase_liquidity_event.token_id.to_string().as_str()].join(":"),
            1
        ) 
    );
}

pub fn prepare_increase_liquidity_entity_changes(
    entity_update_factory: &mut sdk::DexAmmEntityUpdateFactory, 
    transaction_trace: &eth::TransactionTrace,
    call: &eth::Call, 
    log: &eth::Log,
    increase_liquidity_event: NonFungiblePositionManagerContract::events::IncreaseLiquidity, 
) {
    let pool_address = Hex(&NULL_ADDRESS).to_string();
    let position_id = [
            &pool_address,
            increase_liquidity_event.token_id.to_string().as_str(),
        ].join(":");
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
        &position_id,
        0,
        &increase_liquidity_event.liquidity,
    );
    entity_update_factory.store_operations.add_position_deposit_count(
        &position_id,
        0,
        1,
    );
    entity_update_factory.store_operations.add_position_cumulative_deposit_token_amounts(
        &position_id,
        0,
        &vec![increase_liquidity_event.amount0.clone(), increase_liquidity_event.amount1.clone()],
    );
}
