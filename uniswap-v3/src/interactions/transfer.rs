use substreams::{Hex, store::StoreGet};
use substreams_ethereum::{pb::eth::v2::{self as eth}};
use substreams::store;
use substreams::log;
use substreams_ethereum::NULL_ADDRESS;

use crate::dex_amm::v_3_0_3::enums;
use crate::store::sdk;

use crate::store::store_operations;
use crate::pb::store::v1::{StoreOperation, StoreOperations};

use crate::abi::nonFungiblePositionManager as NonFungiblePositionManagerContract;


pub fn create_store_operations_l1_transfer(
    store_operations: &mut StoreOperations,
    transfer_event: NonFungiblePositionManagerContract::events::Transfer, 
    call: &eth::Call, 
    log: &eth::Log,
) {
    store_operations.instructions.push(
        store_operations::add_int_64(
            0, 
            ["mutable-entity-count", "Position", &Hex(&NULL_ADDRESS).to_string(), transfer_event.token_id.to_string().as_str()].join(":"),
            1
        ) 
    );
}

pub fn prepare_transfer_entity_changes(
    entity_update_factory: &mut sdk::DexAmmEntityUpdateFactory, 
    transaction_trace: &eth::TransactionTrace,
    call: &eth::Call, 
    log: &eth::Log,
    transfer_event: NonFungiblePositionManagerContract::events::Transfer, 
) {
    let pool_address = Hex(&NULL_ADDRESS).to_string();
    let position_id = [
            &pool_address,
            transfer_event.token_id.to_string().as_str(),
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
}