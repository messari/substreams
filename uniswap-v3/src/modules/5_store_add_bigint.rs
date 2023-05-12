use substreams::prelude::*;
use substreams::store::{StoreAddBigInt};
use substreams::scalar::BigInt;

use crate::pb::dex_amm::v3_0_3::MappedDataSources;
use crate::pb::store::v1::store_instruction;

#[substreams::handlers::store]
pub fn store_add_bigint(
    mapped_data_sources: MappedDataSources,
    add_bigint_store: StoreAddBigInt,
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