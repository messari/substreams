use substreams::prelude::*;
use substreams::store::{StoreSetBigInt};
use substreams::scalar::BigInt;

use crate::pb::dex_amm::v3_0_3::MappedDataSources;
use crate::pb::store::v1::store_instruction;

#[substreams::handlers::store]
pub fn store_set_bigint(
    mapped_data_sources: MappedDataSources,
    set_bigint_store: StoreSetBigInt,
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