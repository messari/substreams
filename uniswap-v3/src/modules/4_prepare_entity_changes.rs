use std::u8;

use substreams::prelude::*;
use substreams::Hex;
use substreams::errors::Error;
use substreams_ethereum::{pb::eth::v2::{self as eth}, Event};
use substreams::store;
use substreams::store::*;

use substreams::scalar::{BigDecimal, BigInt};

use crate::dex_amm::v_3_0_3::enums;
use crate::dex_amm::v_3_0_3::map;

use crate::{pb::dex_amm::v3_0_3::{
    DataSource, EntityUpdates, PrunedTransaction, 
    EntityCreation}
};
use crate::pb::store::v1::StoreOperation;
use crate::pb::store::v1::store_operation;

use crate::abi::pool as PoolContract;
use crate::abi::factory as FactoryContract;
use crate::abi::nonFungiblePositionManager as NonFungiblePositionManagerContract;

use crate::interactions;
use crate::constants;
use crate::utils;
use crate::store::sdk;

use crate::keyer::{get_data_source_key};

#[substreams::handlers::map]
pub fn prepare_entity_changes(
    block: eth::Block,
    data_sources_store: store::StoreGetProto<DataSource>,
    add_int64_l1_store_deltas: store::Deltas<store::DeltaInt64>,
    set_bigdecimal_l1_store: store::StoreGetBigDecimal,
    append_string_l1_store: store::StoreGetArray<String>,
) -> Result<EntityUpdates, Error>{
    let mut entity_update_factory = sdk::EntityUpdateFactory::new(
        &add_int64_l1_store_deltas
    );

    for transaction_trace in block.transaction_traces {
        for call_view in transaction_trace.calls() {
            if let Some(data_source) = data_sources_store.get_last(get_data_source_key(&call_view.call.address)) {
                match data_source.data_source_type {
                    0 => {
                        for log in &call_view.call.logs {
                            if let Some(swap_event) = PoolContract::events::Swap::match_and_decode(&log) {
                                interactions::swap::prepare_swap_entity_changes(
                                    &mut entity_update_factory, 
                                    &transaction_trace, 
                                    call_view.call, 
                                    log, 
                                    swap_event, 
                                    &set_bigdecimal_l1_store, 
                                    &append_string_l1_store
                                );
                            }
                            // } else if let Some(mint_event) = PoolContract::events::Mint::match_and_decode(&log) {
                            //     interactions::mint::prepare_mint_entity_changes(
                            //         &mut entity_update_factory, 
                            //         &transaction_trace, 
                            //         call_view.call, 
                            //         log, 
                            //         mint_event, 
                            //         &append_string_l1_store
                            //     );
                            // } 
                            // else if let Some(burn_event) = PoolContract::events::Burn::match_and_decode(&log) {
                            //     interactions::burn::prepare_burn_entity_changes(
                            //         &mut entity_update_factory, 
                            //         &transaction_trace, 
                            //         call_view.call, 
                            //         log, 
                            //         burn_event, 
                            //         &append_string_l1_store
                            //     );
                            // }
                        }
                    }
                    // 1 => {
                    //     for log in &call_view.call.logs {
                    //         if let Some(pool_created_event) = FactoryContract::events::PoolCreated::match_and_decode(&log) {
                    //             interactions::pool_created::prepare_pool_created_entity_changes(
                    //                 &mut entity_update_factory, 
                    //                 &transaction_trace, 
                    //                 call_view.call, 
                    //                 log,
                    //                 pool_created_event, 
                    //             );
                    //         }
                    //     }
                    // }
                    // 2 => {
                    //     for log in &call_view.call.logs {
                    //         if let Some(increase_liquidity_event) = NonFungiblePositionManagerContract::events::IncreaseLiquidity::match_and_decode(&log) {
                    //             interactions::increase_liquidity::prepare_increase_liquidity_entity_changes(
                    //                 &mut entity_update_factory, 
                    //                 &transaction_trace, 
                    //                 call_view.call, 
                    //                 log, 
                    //                 increase_liquidity_event, 
                    //             );
                    //         } else if let Some(decrease_liquidity_event) = NonFungiblePositionManagerContract::events::DecreaseLiquidity::match_and_decode(&log) {
                    //             interactions::decrease_liquidity::prepare_decrease_liquidity_entity_changes(
                    //                 &mut entity_update_factory, 
                    //                 &transaction_trace, 
                    //                 call_view.call, 
                    //                 log, 
                    //                 decrease_liquidity_event, 
                    //             );
                    //         } else if let Some(transfer_event) = NonFungiblePositionManagerContract::events::Transfer::match_and_decode(&log) {
                    //             interactions::transfer::prepare_transfer_entity_changes(
                    //                 &mut entity_update_factory, 
                    //                 &transaction_trace, 
                    //                 call_view.call, 
                    //                 log, 
                    //                 transfer_event, 
                    //             );
                    //         }
                    //     }
                    // }
                    _ => {}
                }
            }
        }
    }
    Ok(entity_update_factory.to_entity_updates())
}

#[substreams::handlers::store]
pub fn add_bigdecimal_entity_changes(
    entity_updates: EntityUpdates,
    add_bigdecimal_store: store::StoreAddBigDecimal,
) {
    for store_operation in entity_updates.store_operations {
        match store_operation.r#type.unwrap() {
            store_operation::Type::AddBigDecimal(item) => {
                let item_value: BigDecimal = BigDecimal::try_from(item.value.unwrap().value).unwrap();
                add_bigdecimal_store.add(item.ordinal, item.key, item_value);
            }, 
            store_operation::Type::AddManyBigDecimal(item) => {
                let item_value: BigDecimal = BigDecimal::try_from(item.value.unwrap().value).unwrap();
                add_bigdecimal_store.add_many(item.ordinal, &item.key, item_value);
            },
            _ => continue,
        }
    }
}

#[substreams::handlers::store]
pub fn add_bigint_entity_changes(
    entity_updates: EntityUpdates,
    add_bigint_store: store::StoreAddBigInt,
) {
    for store_operation in entity_updates.store_operations {
        match store_operation.r#type.unwrap() {
            store_operation::Type::AddBigInt(item) => {
                let item_value: BigInt = BigInt::try_from(item.value.unwrap().value).unwrap();
                add_bigint_store.add(item.ordinal, item.key, item_value);
            }, 
            store_operation::Type::AddManyBigInt(item) => {
                let item_value: BigInt = BigInt::try_from(item.value.unwrap().value).unwrap();
                add_bigint_store.add_many(item.ordinal, &item.key, item_value);
            },
            _ => continue,
        }
    }
}

#[substreams::handlers::store]
pub fn add_int64_entity_changes(
    entity_updates: EntityUpdates,
    add_int64_store: store::StoreAddInt64,
) {
    for store_operation in entity_updates.store_operations {
        match store_operation.r#type.unwrap() {
            store_operation::Type::AddInt64(item) => {
                add_int64_store.add(item.ordinal, item.key, item.value);
            }, 
            store_operation::Type::AddManyInt64(item) => {
                add_int64_store.add_many(item.ordinal, &item.key, item.value);
            },
            _ => continue,
        }
    }
}

#[substreams::handlers::store]
pub fn append_string_entity_changes(
    entity_updates: EntityUpdates,
    append_string_store: store::StoreAppend<String>,
) {
    for store_operation in entity_updates.store_operations {
        match store_operation.r#type.unwrap() {
            store_operation::Type::AppendString(item) => {
                append_string_store.append(item.ordinal, item.key, item.value);
            }, 
            _ => continue,
        } 
    }
}

#[substreams::handlers::store]
pub fn set_bigint_entity_changes(
    entity_updates: EntityUpdates,
    set_bigint_store: store::StoreSetBigInt,
) {
    for store_operation in entity_updates.store_operations {
        match store_operation.r#type.unwrap() {
            store_operation::Type::SetBigInt(item) => {
                let item_value: BigInt = BigInt::try_from(item.value.unwrap().value).unwrap();
                set_bigint_store.set(item.ordinal, item.key, &item_value);
            }, 
            store_operation::Type::SetManyBigInt(item) => {
                let item_value: BigInt = BigInt::try_from(item.value.unwrap().value).unwrap();
                set_bigint_store.set_many(item.ordinal, &item.key, &item_value);
            },
            _ => continue,
        }
    }
}

#[substreams::handlers::store]
pub fn set_bytes_entity_changes(
    entity_updates: EntityUpdates,
    set_bytes_store: store::StoreSetRaw,
) {
    for store_operation in entity_updates.store_operations {
        match store_operation.r#type.unwrap() {
            store_operation::Type::SetBytes(item) => {
                set_bytes_store.set(item.ordinal, item.key, &item.value);
            }, 
            _ => continue,
        }
    }
}
