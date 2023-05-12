use substreams::prelude::*;
use substreams::store::{StoreAppend};

use crate::pb::dex_amm::v3_0_3::MappedDataSources;
use crate::pb::store::v1::store_instruction;

#[substreams::handlers::store]
pub fn store_append_string(
    mapped_data_sources: MappedDataSources,
    append_string_store: StoreAppend<String>,
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