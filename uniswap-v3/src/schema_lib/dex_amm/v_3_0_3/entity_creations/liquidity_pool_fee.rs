use crate::pb::dex_amm::v3_0_3::LiquidityPoolFeeEntityCreation;
use crate::tables::Tables;

pub fn create_liquidity_pool_fee_entity(
    tables: &mut Tables,
    entity_id: &Vec<u8>,
    liquidity_pool_fee_entity_creation: &LiquidityPoolFeeEntityCreation,
) {
    tables
        .create_row("LiquidityPoolFee", std::str::from_utf8(entity_id).unwrap())
        .set(
            "feePercentage",
            &liquidity_pool_fee_entity_creation
                .fee_percentage
                .clone()
                .unwrap(),
        )
        .set("feeType", &liquidity_pool_fee_entity_creation.fee_type);
}
