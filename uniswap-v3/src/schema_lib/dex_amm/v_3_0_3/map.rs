use crate::pb::dex_amm::v3_0_3::{PrunedTransaction, EntityCreation};
use crate::pb::dex_amm::v3_0_3::entity_creation::Type;

use crate::tables::Tables;
use crate::schema_lib::dex_amm::v_3_0_3::entity_creations::{
    liquidity_pool::{
        create_liquidity_pool_entity
    },
    liquidity_pool_fee::{
        create_liquidity_pool_fee_entity
    },
    event::{
        create_swap_entity,
        create_deposit_entity,
        create_withdraw_entity,
    },
    token::{
        create_token_entity
    },
    dex_amm_protocol::{
        create_dex_amm_protocol_entity
    },
    tick::{
        create_tick_entity
    },
    position::{
        create_position_entity
    }
};

pub fn map_dex_amm_v_3_0_3_entity_creation(
    tables: &mut Tables,
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    entity_creation: &EntityCreation, 
) {
    match entity_creation.r#type.clone().unwrap() {
        Type::SwapEntityCreation(swap_entity_creation) => {
            create_swap_entity(
                tables,
                &entity_creation.entity_id,
                block_number,
                timestamp,
                pruned_transaction,
                &swap_entity_creation,
            );
        },
        Type::DepositEntityCreation(deposit_entity_creation) => {
            create_deposit_entity(
                tables,
                &entity_creation.entity_id,
                block_number,
                timestamp,
                pruned_transaction,
                &deposit_entity_creation,
            );
        },
        Type::WithdrawEntityCreation(withdraw_entity_creation) => {
            create_withdraw_entity(
                tables,
                &entity_creation.entity_id,
                block_number,
                timestamp,
                pruned_transaction,
                &withdraw_entity_creation,
            );
        },
        Type::DexAmmProtocolEntityCreation(dex_amm_protocol_entity_creation) => {
            create_dex_amm_protocol_entity(
                tables,
                &entity_creation.entity_id,
                &dex_amm_protocol_entity_creation
            );
        },
        Type::LiquidityPoolEntityCreation(liquidity_pool_entity_creation) => {
            create_liquidity_pool_entity(
                tables,
                &entity_creation.entity_id,
                block_number,
                timestamp,
                &liquidity_pool_entity_creation,
            );
        },
        Type::TokenEntityCreation(token_entity_creation) => {
            create_token_entity(
                tables,
                &entity_creation.entity_id,
                &token_entity_creation,
            );
        },
        Type::LiquidityPoolFeeEntityCreation(liquidity_pool_fee_entity_creation) => {
            create_liquidity_pool_fee_entity(
                tables,
                &entity_creation.entity_id,
                &liquidity_pool_fee_entity_creation,
            );
        },
        Type::TickEntityCreation(tick_entity_creation) => {
            create_tick_entity(
                tables,
                &entity_creation.entity_id,
                block_number,
                timestamp,
                &tick_entity_creation,
            );
        },
        Type::PositionEntityCreation(position_entity_creation) => {
            create_position_entity(
                tables,
                &entity_creation.entity_id,
                block_number,
                timestamp,
                pruned_transaction,
                &position_entity_creation,
            );
        },
    };
}
