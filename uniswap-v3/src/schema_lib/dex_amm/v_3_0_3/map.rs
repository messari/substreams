use substreams::prelude::*;

use substreams::store::{DeltaBigDecimal, DeltaInt64, DeltaBigInt, StoreGetRaw, StoreGet, StoreGetArray};
use substreams_entity_change::pb::entity::{EntityChange, EntityChanges};

use crate::pb::dex_amm::v3_0_3::{MappedDataSources, PrunedTransaction, Update};
use crate::pb::dex_amm::v3_0_3::update::Type;

use crate::tables::Tables;
use crate::schema_lib::dex_amm::v_3_0_3::entity_changes::{
    create_liquidity_pool::{
        create_liquidity_pool_entity_change
    },
    create_event::{
        create_swap_entity_change,
        create_deposit_entity_change,
        create_withdraw_entity_change,
    },
    create_token::{
        create_token_entity_change
    },
    create_dex_amm_protocol::{
        create_dex_amm_protocol_entity_change
    },
};

pub fn map_dex_amm_v_3_0_3_entity_changes(
    tables: &mut Tables,
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    update: &Update, 
    add_bigdecimal_store_deltas: &Deltas<DeltaBigDecimal>,
    add_bigint_store_deltas: &Deltas<DeltaBigInt>,
    add_int64_store_deltas: &Deltas<DeltaInt64>,
    store_append_string: &StoreGetArray<String>,
) {
    match update.r#type.clone().unwrap() {
        Type::Swap(swap) => {
            create_swap_entity_change(
                tables,
                block_number,
                timestamp,
                pruned_transaction,
                &swap,
                store_append_string
            );
        },
        Type::Deposit(deposit) => {
            create_deposit_entity_change(
                tables,
                block_number,
                timestamp,
                pruned_transaction,
                &deposit,
                store_append_string
            );
        },
        Type::Withdraw(withdraw) => {
            create_withdraw_entity_change(
                tables,
                block_number,
                timestamp,
                pruned_transaction,
                &withdraw,
                store_append_string
            );
        },
        Type::CreateDexAmmProtocol(create_dex_amm_protocol) => {
            create_dex_amm_protocol_entity_change(
                tables,
                block_number,
                timestamp,
                pruned_transaction,
                &create_dex_amm_protocol
            );
        },
        Type::CreateLiquidityPool(create_liquidity_pool) => {
            create_liquidity_pool_entity_change(
                tables,
                block_number,
                timestamp,
                pruned_transaction,
                &create_liquidity_pool,
            );
        },
        Type::CreateToken(create_token) => {
            create_token_entity_change(
                tables,
                block_number,
                timestamp,
                pruned_transaction,
                &create_token,
            );
        }
    };
}