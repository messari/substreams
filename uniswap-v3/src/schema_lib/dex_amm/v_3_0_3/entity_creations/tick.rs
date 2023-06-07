use substreams::scalar::BigInt;

use crate::pb::dex_amm::v3_0_3::TickEntityCreation;

use crate::constants;
use crate::tables::{Row, Tables};

pub fn create_tick_entity(
    tables: &mut Tables,
    entity_id: &Vec<u8>,
    block_number: &u64,
    timestamp: &i64,
    tick_creation: &TickEntityCreation,
) {
    let row: &mut Row = tables.create_row("Tick", std::str::from_utf8(entity_id).unwrap());
    row.set("index", &tick_creation.index.clone().unwrap())
        .set("pool", &tick_creation.pool)
        .set("createdTimestamp", BigInt::from(*timestamp))
        .set("createdBlockNumber", BigInt::from(*block_number))
        .set(
            "prices",
            vec![
                constants::BIGDECIMAL_ZERO.clone(),
                constants::BIGDECIMAL_ZERO.clone(),
            ],
        )
        .set("liquidityGross", constants::BIGINT_ZERO.clone())
        .set("liquidityGrossUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("liquidityNet", constants::BIGINT_ZERO.clone())
        .set("liquidityNetUSD", constants::BIGDECIMAL_ZERO.clone());
}
