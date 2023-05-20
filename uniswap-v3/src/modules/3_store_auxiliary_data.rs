use substreams::prelude::*;
use substreams::errors::Error;
use substreams_ethereum::{pb::eth::v2::{self as eth}, Event};
use substreams::store;

use crate::{pb::dex_amm::v3_0_3::{
    DataSource, MappedDataSources, PrunedTransaction, 
    EntityCreation}
};
use crate::pb::store::v1::{StoreInstruction, StoreInstructions};
use crate::pb::store::v1::store_instruction;

use crate::abi::pool as PoolContract;
use crate::abi::factory as FactoryContract;

use crate::interactions;
use crate::constants;

use crate::keyer::{get_data_source_key};

#[substreams::handlers::map]
pub fn create_store_instructions_l1(
    block: eth::Block,
    data_sources_store: store::StoreGetProto<DataSource>,
) -> Result<StoreInstructions, Error>{
    let mut store_instructions = StoreInstructions {
        instructions: Vec::<StoreInstruction>::new(),
    };

    for transaction_trace in block.transaction_traces { 
        for call_view in transaction_trace.calls() {
            if let Some(data_source) = data_sources_store.get_last(get_data_source_key(&call_view.call.address)) {
                match data_source.data_source_type {
                    1 => {
                        for log in &call_view.call.logs {
                            if let Some(pool_created_event) = FactoryContract::events::PoolCreated::match_and_decode(&log) {
                                interactions::pool_created::create_store_instructions_l1_pool_created(&mut store_instructions.instructions, pool_created_event, call_view.call, log);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }


    Ok(store_instructions)
}

#[substreams::handlers::store]
pub fn add_bigdecimal_l1(
    store_instructions: StoreInstructions,
    add_bigdecimal_store: store::StoreAddBigDecimal,
) {
    for store_instruction in store_instructions.instructions {
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
pub fn add_bigint_l1(
    store_instructions: StoreInstructions,
    add_bigint_store: store::StoreAddBigInt,
) {
    for store_instruction in store_instructions.instructions {
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
pub fn add_int64_l1(
    store_instructions: StoreInstructions,
    add_int64_store: store::StoreAddInt64,
) {
    for store_instruction in store_instructions.instructions {
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
pub fn append_string_l1(
    store_instructions: StoreInstructions,
    append_string_store: store::StoreAppend<String>,
) {
    for store_instruction in store_instructions.instructions {
        match store_instruction.r#type.unwrap() {
            store_instruction::Type::AppendString(item) => {
                append_string_store.append(item.ordinal, item.key, item.value);
            }, 
            _ => continue,
        } 
    }
}

#[substreams::handlers::store]
pub fn set_bigint_l1(
    store_instructions: StoreInstructions,
    set_bigint_store: store::StoreSetBigInt,
) {
    for store_instruction in store_instructions.instructions {
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

#[substreams::handlers::store]
pub fn set_bigdecimal_l1(
    store_instructions: StoreInstructions,
    set_bigdecimal_store: store::StoreSetBigDecimal,
) {
    for store_instruction in store_instructions.instructions {
        match store_instruction.r#type.unwrap() {
            store_instruction::Type::SetBigDecimal(item) => {
                let item_value: BigDecimal = BigDecimal::try_from(item.value.unwrap().value).unwrap();
                set_bigdecimal_store.set(item.ordinal, item.key, &item_value);
            }, 
            _ => continue,
        }
    }
}
