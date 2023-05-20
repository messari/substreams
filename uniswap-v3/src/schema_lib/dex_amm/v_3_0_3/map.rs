use substreams::prelude::*;

use substreams::store::{DeltaBigDecimal, DeltaInt64, DeltaBigInt, StoreGetArray};

use crate::pb::dex_amm::v3_0_3::{PrunedTransaction, EntityCreation};
use crate::pb::dex_amm::v3_0_3::entity_creation::Type;

use crate::tables::Tables;
use crate::schema_lib::dex_amm::v_3_0_3::entity_creation::{
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
                block_number,
                timestamp,
                pruned_transaction,
                &swap_entity_creation,
            );
        },
        Type::DepositEntityCreation(deposit_entity_creation) => {
            create_deposit_entity(
                tables,
                block_number,
                timestamp,
                pruned_transaction,
                &deposit_entity_creation,
            );
        },
        Type::WithdrawEntityCreation(withdraw_entity_creation) => {
            create_withdraw_entity(
                tables,
                block_number,
                timestamp,
                pruned_transaction,
                &withdraw_entity_creation,
            );
        },
        Type::DexAmmProtocolEntityCreation(dex_amm_protocol_entity_creation) => {
            create_dex_amm_protocol_entity(
                tables,
                block_number,
                timestamp,
                pruned_transaction,
                &dex_amm_protocol_entity_creation
            );
        },
        Type::LiquidityPoolEntityCreation(liquidity_pool_entity_creation) => {
            create_liquidity_pool_entity(
                tables,
                block_number,
                timestamp,
                pruned_transaction,
                &liquidity_pool_entity_creation,
            );
        },
        Type::TokenEntityCreation(token_entity_creation) => {
            create_token_entity(
                tables,
                block_number,
                timestamp,
                pruned_transaction,
                &token_entity_creation,
            );
        },
        Type::LiquidityPoolFeeEntityCreation(liquidity_pool_fee_entity_creation) => {
            create_liquidity_pool_fee_entity(
                tables,
                block_number,
                timestamp,
                pruned_transaction,
                &liquidity_pool_fee_entity_creation,
            );
        },
    };
}
