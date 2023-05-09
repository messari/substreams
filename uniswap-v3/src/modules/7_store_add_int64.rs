use substreams::prelude::*;
use substreams::store::{StoreAddInt64};
use crate::pb::dex_amm::v3_0_3::{MappedDataSources, store_instruction, AddInt64, AddManyInt64};

#[substreams::handlers::store]
pub fn store_add_int64(
    mapped_data_sources: MappedDataSources,
    add_int64_store: StoreAddInt64,
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