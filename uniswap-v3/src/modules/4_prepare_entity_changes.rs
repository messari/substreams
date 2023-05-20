use substreams::prelude::*;
use substreams::errors::Error;
use substreams_ethereum::{pb::eth::v2::{self as eth}, Event};
use substreams::store;
use substreams::store::*;

use substreams::scalar::{BigDecimal, BigInt};

use crate::{pb::dex_amm::v3_0_3::{
    DataSource, MappedDataSources, PrunedTransaction, 
    EntityCreation}
};
use crate::pb::store::v1::StoreInstruction;
use crate::pb::store::v1::store_instruction;

use crate::abi::pool as PoolContract;
use crate::abi::factory as FactoryContract;

use crate::interactions;
use crate::constants;

use crate::keyer::{get_data_source_key};

fn initialize_pruned_transaction(transaction_trace: eth::TransactionTrace) -> PrunedTransaction {
    PrunedTransaction {
        hash: transaction_trace.hash.clone(),
        from: transaction_trace.from.clone(),
        to: transaction_trace.to.clone(),
        nonce: Some(transaction_trace.nonce.into()),
        gas_limit: Some(transaction_trace.gas_limit.into()),
        gas_used: Some(transaction_trace.gas_used.into()),
        gas_price: Some(constants::BIGINT_ZERO.clone().into()),
        entity_creations: Vec::<EntityCreation>::new(),
    }
}

#[substreams::handlers::map]
pub fn prepare_entity_changes(
    block: eth::Block,
    data_sources_store: store::StoreGetProto<DataSource>,
    set_bigdecimal_l1_store: store::StoreGetBigDecimal,
    append_string_l1_store: store::StoreGetArray<String>,
) -> Result<MappedDataSources, Error>{
    let mut mapped_data_sources = MappedDataSources {
        pruned_transactions: Vec::<PrunedTransaction>::new(),
        store_instructions: Vec::<StoreInstruction>::new(),
    };

    for transaction_trace in block.transaction_traces {
        let mut pruned_transaction: PrunedTransaction = initialize_pruned_transaction(transaction_trace.clone());
 
        for call_view in transaction_trace.calls() {
            if let Some(data_source) = data_sources_store.get_last(get_data_source_key(&call_view.call.address)) {
                match data_source.data_source_type {
                    0 => {
                        for log in &call_view.call.logs {
                            if let Some(swap_event) = PoolContract::events::Swap::match_and_decode(&log) {
                                interactions::swap::prepare_swap_entity_changes(
                                    &mut mapped_data_sources, 
                                    &mut pruned_transaction, 
                                    swap_event, 
                                    call_view.call, 
                                    log, 
                                    &set_bigdecimal_l1_store, 
                                    &append_string_l1_store
                                );
                            } else if let Some(mint_event) = PoolContract::events::Mint::match_and_decode(&log) {
                                interactions::mint::prepare_mint_entity_changes(
                                    &mut mapped_data_sources, 
                                    &mut pruned_transaction, 
                                    mint_event, 
                                    call_view.call, 
                                    log, 
                                    &append_string_l1_store
                                );
                            } else if let Some(burn_event) = PoolContract::events::Burn::match_and_decode(&log) {
                                interactions::burn::prepare_burn_entity_changes(
                                    &mut mapped_data_sources, 
                                    &mut pruned_transaction, 
                                    burn_event, 
                                    call_view.call, 
                                    log, 
                                    &append_string_l1_store
                                );
                            }
                        }
                    }
                    1 => {
                        for log in &call_view.call.logs {
                            if let Some(pool_created_event) = FactoryContract::events::PoolCreated::match_and_decode(&log) {
                                interactions::pool_created::prepare_pool_created_entity_changes(
                                    &mut mapped_data_sources, 
                                    &mut pruned_transaction, 
                                    pool_created_event, 
                                    call_view.call, 
                                    log
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        if pruned_transaction.entity_creations.len() > 0 {
            mapped_data_sources.pruned_transactions.push(pruned_transaction);
        }
    }
    Ok(mapped_data_sources)
}

#[substreams::handlers::store]
pub fn add_bigdecimal_entity_changes(
    mapped_data_sources: MappedDataSources,
    add_bigdecimal_store: store::StoreAddBigDecimal,
) {
    for store_instruction in mapped_data_sources.store_instructions {
        match store_instruction.r#type.unwrap() {
            store_instruction::Type::AddBigDecimal(item) => {
                let item_value: BigDecimal = BigDecimal::try_from(item.value.unwrap().value).unwrap();
                add_bigdecimal_store.add(item.ordinal, item.key, item_value);
            }, 
            store_instruction::Type::AddManyBigDecimal(item) => {
                let item_value: BigDecimal = BigDecimal::try_from(item.value.unwrap().value).unwrap();
                add_bigdecimal_store.add_many(item.ordinal, &item.key, item_value);
            },
            _ => continue,
        }
    }
}

#[substreams::handlers::store]
pub fn add_bigint_entity_changes(
    mapped_data_sources: MappedDataSources,
    add_bigint_store: store::StoreAddBigInt,
) {
    for store_instruction in mapped_data_sources.store_instructions {
        match store_instruction.r#type.unwrap() {
            store_instruction::Type::AddBigInt(item) => {
                let item_value: BigInt = BigInt::try_from(item.value.unwrap().value).unwrap();
                add_bigint_store.add(item.ordinal, item.key, item_value);
            }, 
            store_instruction::Type::AddManyBigInt(item) => {
                let item_value: BigInt = BigInt::try_from(item.value.unwrap().value).unwrap();
                add_bigint_store.add_many(item.ordinal, &item.key, item_value);
            },
            _ => continue,
        }
    }
}

#[substreams::handlers::store]
pub fn add_int64_entity_changes(
    mapped_data_sources: MappedDataSources,
    add_int64_store: store::StoreAddInt64,
) {
    for store_instruction in mapped_data_sources.store_instructions {
        match store_instruction.r#type.unwrap() {
            store_instruction::Type::AddInt64(item) => {
                add_int64_store.add(item.ordinal, item.key, item.value);
            }, 
            store_instruction::Type::AddManyInt64(item) => {
                add_int64_store.add_many(item.ordinal, &item.key, item.value);
            },
            _ => continue,
        }
    }
}

#[substreams::handlers::store]
pub fn append_string_entity_changes(
    mapped_data_sources: MappedDataSources,
    append_string_store: store::StoreAppend<String>,
) {
    for store_instruction in mapped_data_sources.store_instructions {
        match store_instruction.r#type.unwrap() {
            store_instruction::Type::AppendString(item) => {
                append_string_store.append(item.ordinal, item.key, item.value);
            }, 
            _ => continue,
        } 
    }
}

#[substreams::handlers::store]
pub fn set_bigint_entity_changes(
    mapped_data_sources: MappedDataSources,
    set_bigint_store: store::StoreSetBigInt,
) {
    for store_instruction in mapped_data_sources.store_instructions {
        match store_instruction.r#type.unwrap() {
            store_instruction::Type::SetBigInt(item) => {
                let item_value: BigInt = BigInt::try_from(item.value.unwrap().value).unwrap();
                set_bigint_store.set(item.ordinal, item.key, &item_value);
            }, 
            store_instruction::Type::SetManyBigInt(item) => {
                let item_value: BigInt = BigInt::try_from(item.value.unwrap().value).unwrap();
                set_bigint_store.set_many(item.ordinal, &item.key, &item_value);
            },
            _ => continue,
        }
    }
}
