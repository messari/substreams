use substreams::scalar::{BigInt};
use substreams::Hex;

use crate::pb::dex_amm::v3_0_3::{PrunedTransaction, TickEntityCreation};
use crate::pb::entity;
use crate::schema_lib::dex_amm::v_3_0_3::keys;
use crate::tables::{Tables, Row};
use crate::constants;

pub fn create_tick_entity(
    tables: &mut Tables,
    entity_id: &Vec<u8>,
    block_number: &u64,
    timestamp: &i64,
    tick_creation: &TickEntityCreation,
) {
    let row: &mut Row = tables.create_row("Tick", Hex(entity_id).to_string());
    row
        .set("index", &tick_creation.index.clone().unwrap())
        .set("pool", &tick_creation.pool)
        .set("createdTimestamp", BigInt::from(*timestamp))
        .set("createdBlockNumber", BigInt::from(*block_number))
        .set("prices", vec![constants::BIGDECIMAL_ZERO.clone(), constants::BIGDECIMAL_ZERO.clone()])
        .set("liquidityGross", constants::BIGINT_ZERO.clone())
        .set("liquidityGrossUSD", constants::BIGDECIMAL_ZERO.clone())
        .set("liquidityNet", constants::BIGINT_ZERO.clone())
        .set("liquidityNetUSD", constants::BIGDECIMAL_ZERO.clone());
}
