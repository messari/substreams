use substreams::Hex;
use crate::pb::dex_amm::v3_0_3::{PrunedTransaction, LiquidityPoolFeeEntityCreation};
use crate::tables::{Tables};

pub fn create_liquidity_pool_fee_entity(
    tables: &mut Tables,
    entity_id: &Vec<u8>,
    block_number: &u64,
    timestamp: &i64,
    pruned_transaction: &PrunedTransaction,
    liquidity_pool_fee_entity_creation: &LiquidityPoolFeeEntityCreation,
) {
    tables.create_row("LiquidityPoolFee", Hex(entity_id).to_string())
        .set("feePercentage", &liquidity_pool_fee_entity_creation.fee_percentage.clone().unwrap())
        .set("feeType", &liquidity_pool_fee_entity_creation.fee_type);
}
