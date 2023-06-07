use substreams::prelude::*;
use substreams::errors::Error;
use substreams_ethereum::{pb::eth::v2::{self as eth}, Event};
use substreams::store;
use substreams::pb::substreams::Clock;

use crate::pb::dex_amm::v3_0_3::DataSource;

use crate::pb::store::v1::StoreOperations;
use crate::pb::store::v1::store_operation;

use crate::abi::pool as PoolContract;
use crate::abi::factory as FactoryContract;
use crate::abi::non_fungible_position_manager as NonFungiblePositionManagerContract;

use crate::interactions;
use crate::store::sdk;

use crate::keyer::{get_data_source_key};


#[substreams::handlers::map]
pub fn create_store_operations_l1(
    clock: Clock,
    block: eth::Block,
    data_sources_store: store::StoreGetProto<DataSource>,
) -> Result<StoreOperations, Error>{
    let mut store_operation_factory = sdk::StoreOperationFactory::new(clock);

    for transaction_trace in block.transaction_traces { 
        for call_view in transaction_trace.calls() {
            if let Some(data_source) = data_sources_store.get_last(get_data_source_key(&call_view.call.address)) {
                match data_source.data_source_type {
                    0 => {
                        for log in &call_view.call.logs {
                            if let Some(mint_event) = PoolContract::events::Mint::match_and_decode(&log) {
                                interactions::mint::create_store_operations_l1_mint(&mut store_operation_factory, mint_event, call_view.call);
                            } else if let Some(burn_event) = PoolContract::events::Burn::match_and_decode(&log) {
                                interactions::burn::create_store_operations_l1_burn(&mut store_operation_factory, burn_event, call_view.call);
                            }
                        }
                    }
                    1 => {
                        for log in &call_view.call.logs {
                            if let Some(pool_created_event) = FactoryContract::events::PoolCreated::match_and_decode(&log) {
                                interactions::pool_created::create_store_operations_l1_pool_created(&mut store_operation_factory, pool_created_event);
                            }
                        }
                    }
                    2 => {
                        for log in &call_view.call.logs {
                            if let Some(increase_liquidity_event) = NonFungiblePositionManagerContract::events::IncreaseLiquidity::match_and_decode(&log) {
                                interactions::increase_liquidity::create_store_operations_l1_increase_liquidity(&mut store_operation_factory, increase_liquidity_event);
                            } else if let Some(decrease_liquidity_event) = NonFungiblePositionManagerContract::events::DecreaseLiquidity::match_and_decode(&log) {
                                interactions::decrease_liquidity::create_store_operations_l1_decrease_liquidity(&mut store_operation_factory, decrease_liquidity_event);
                            } else if let Some(transfer_event) = NonFungiblePositionManagerContract::events::Transfer::match_and_decode(&log) {
                                interactions::transfer::create_store_operations_l1_transfer(&mut store_operation_factory, transfer_event);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }


    Ok(
        StoreOperations {
            operations: store_operation_factory.get_operations()
        }
    )
}

#[substreams::handlers::store]
pub fn add_bigdecimal_l1(
    store_operations: StoreOperations,
    add_bigdecimal_store: store::StoreAddBigDecimal,
) {
    for store_operation in store_operations.operations {
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
pub fn add_bigint_l1(
    store_operations: StoreOperations,
    add_bigint_store: store::StoreAddBigInt,
) {
    for store_operation in store_operations.operations {
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
pub fn add_int64_l1(
    store_operations: StoreOperations,
    add_int64_store: store::StoreAddInt64,
) {
    for store_operation in store_operations.operations {
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
pub fn append_string_l1(
    store_operations: StoreOperations,
    append_string_store: store::StoreAppend<String>,
) {
    for store_operation in store_operations.operations {
        match store_operation.r#type.unwrap() {
            store_operation::Type::AppendString(item) => {
                append_string_store.append(item.ordinal, item.key, item.value);
            }, 
            _ => continue,
        } 
    }
}

#[substreams::handlers::store]
pub fn set_bigint_l1(
    store_operations: StoreOperations,
    set_bigint_store: store::StoreSetBigInt,
) {
    for store_operation in store_operations.operations {
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
pub fn set_bigdecimal_l1(
    store_operations: StoreOperations,
    set_bigdecimal_store: store::StoreSetBigDecimal,
) {
    for store_operation in store_operations.operations {
        match store_operation.r#type.unwrap() {
            store_operation::Type::SetBigDecimal(item) => {
                let item_value: BigDecimal = BigDecimal::try_from(item.value.unwrap().value).unwrap();
                set_bigdecimal_store.set(item.ordinal, item.key, &item_value);
            }, 
            _ => continue,
        }
    }
}

#[substreams::handlers::store]
pub fn set_bytes_l1(
    store_operations: StoreOperations,
    set_bytes_store: store::StoreSetRaw,
) {
    for store_operation in store_operations.operations {
        match store_operation.r#type.unwrap() {
            store_operation::Type::SetBytes(item) => {
                set_bytes_store.set(item.ordinal, item.key, &item.value);
            }, 
            _ => continue,
        } 
    }
}
