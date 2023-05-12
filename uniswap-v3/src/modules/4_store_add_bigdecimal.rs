use substreams::prelude::*;
use substreams::store::StoreAddBigDecimal;
use substreams::scalar::BigDecimal;

use crate::pb::dex_amm::v3_0_3::MappedDataSources;
use crate::pb::store::v1::store_instruction;

#[substreams::handlers::store]
pub fn store_add_bigdecimal(
    mapped_data_sources: MappedDataSources,
    add_bigdecimal_store: StoreAddBigDecimal,
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
