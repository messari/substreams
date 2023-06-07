use substreams::Hex;
use substreams_ethereum::{pb::eth::v2::{self as eth}};
use substreams_ethereum::NULL_ADDRESS;

use crate::dex_amm::v_3_0_3::enums;
use crate::store::sdk;

use crate::schema_lib::dex_amm::v_3_0_3::keys;

use crate::abi::non_fungible_position_manager as NonFungiblePositionManagerContract;


pub fn create_store_operations_l1_transfer(
    store_operation_factory: &mut sdk::StoreOperationFactory,
    transfer_event: NonFungiblePositionManagerContract::events::Transfer, 
) {
    store_operation_factory.track_position_mutation(
        keys::get_position_key(&Hex(&NULL_ADDRESS).to_string(), &transfer_event.token_id.to_string())
    );
}

pub fn prepare_transfer_entity_changes(
    entity_update_factory: &mut sdk::EntityUpdateFactory, 
    transaction_trace: &eth::TransactionTrace,
    call: &eth::Call, 
    transfer_event: NonFungiblePositionManagerContract::events::Transfer, 
) {
    let position_id = &keys::get_position_key(&Hex(&NULL_ADDRESS).to_string(), &transfer_event.token_id.to_string());
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