use std::str::FromStr;

use substreams::Hex;
use substreams::scalar::{BigInt, BigDecimal};
use substreams_ethereum::{pb::eth::v2::{self as eth}};

use crate::dex_amm::v_3_0_3::entity_creation::token;
use crate::{pb::dex_amm::v3_0_3::{
    MappedDataSources, PrunedTransaction}, 
    utils::UNISWAP_V3_FACTORY_SLICE

};
use crate::schema_lib::dex_amm::v_3_0_3::enums;

use crate::abi::factory as FactoryContract;
use crate::contract::erc20;
use crate::constants;
use crate::store::store_update;
use crate::pb::store::v1::StoreInstruction;

pub fn create_store_instructions_l1_pool_created(
    store_instructions: &mut Vec<StoreInstruction>,
    pool_created_event: FactoryContract::events::PoolCreated, 
    call: &eth::Call, 
    log: &eth::Log
) {
    store_instructions.push(
        store_update::append_string(
            0, 
            [Hex(&pool_created_event.pool).to_string().as_str(), "inputTokens"].join(":"), 
            Hex(&pool_created_event.token0).to_string(), 
        )
    );
    store_instructions.push(
        store_update::append_string(
            0, 
            [Hex(&pool_created_event.pool).to_string().as_str(), "inputTokens"].join(":"), 
            Hex(&pool_created_event.token1).to_string() 
        )
    );

    store_instructions.push(
        store_update::set_bigdecimal(
            0, 
            [Hex(&pool_created_event.pool).to_string().as_str(), &enums::LiquidityPoolFeeType::FIXED_TRADING_FEE.to_string()].join(":"), 
            convert_fee_to_percentage(&pool_created_event.fee),
        )
    );
    store_instructions.push(
        store_update::set_bigdecimal(
            0, 
            [Hex(&pool_created_event.pool).to_string().as_str(), &enums::LiquidityPoolFeeType::FIXED_LP_FEE.to_string()].join(":"), 
            convert_fee_to_percentage(&pool_created_event.fee),
        )
    );
    store_instructions.push(
        store_update::set_bigdecimal(
            0, 
            [Hex(&pool_created_event.pool).to_string().as_str(), &enums::LiquidityPoolFeeType::FIXED_PROTOCOL_FEE.to_string()].join(":"), 
            constants::BIGDECIMAL_ZERO.clone(),
        )
    );
}

pub fn prepare_pool_created_entity_changes(
    mapped_data_sources: &mut MappedDataSources, 
    pruned_transaction: &mut PrunedTransaction,
    pool_created_event: FactoryContract::events::PoolCreated, 
    call: &eth::Call, 
    log: &eth::Log
) {
    let token0 = erc20::Erc20::new(pool_created_event.token0.clone()).as_struct();
    let token1 = erc20::Erc20::new(pool_created_event.token1.clone()).as_struct();
    pruned_transaction.create_token_entity(
        &pool_created_event.token0,
        &token0.name,
        &token0.symbol,
        token0.decimals as i32,
    );
    pruned_transaction.create_token_entity(
        &pool_created_event.token1,
        &token1.name,
        &token1.symbol,
        token1.decimals as i32,
    );

    let (pool_lp_fee, pool_protocol_fee, pool_trading_fee) = create_pool_fees(&pool_created_event.pool, &pool_created_event.fee);
    pruned_transaction.create_liquidity_pool_fee_entity(
        &pool_created_event.pool, 
        &pool_lp_fee.fee_type, 
        &pool_lp_fee.fee_percentage
    );

    pruned_transaction.create_liquidity_pool_fee_entity(
        &pool_created_event.pool, 
        &pool_protocol_fee.fee_type, 
        &pool_protocol_fee.fee_percentage
    );

    pruned_transaction.create_liquidity_pool_fee_entity(
        &pool_created_event.pool, 
        &pool_trading_fee.fee_type, 
        &pool_trading_fee.fee_percentage
    );

    let input_tokens: Vec<Vec<u8>> = vec![pool_created_event.token0.clone(), pool_created_event.token1.clone()];
    pruned_transaction.create_liquidity_pool_entity(
        &UNISWAP_V3_FACTORY_SLICE.to_vec(),
        &pool_created_event.pool,
        &input_tokens,
        &vec![token0.symbol.clone(), token1.symbol.clone()],
        &vec![constants::BIGDECIMAL_50.clone(), constants::BIGDECIMAL_50.clone()],
        false,
        None,
        Some(&vec![enums::LiquidityPoolFeeType::FIXED_LP_FEE, enums::LiquidityPoolFeeType::FIXED_PROTOCOL_FEE, enums::LiquidityPoolFeeType::FIXED_TRADING_FEE]),
        None,
        None,
        None,
    );
}


pub struct LiquidityPoolFee {
    fee_type: enums::LiquidityPoolFeeType,
    fee_percentage: BigDecimal,
}

impl LiquidityPoolFee {
    pub fn new(fee_type: enums::LiquidityPoolFeeType, fee_percentage: BigDecimal) -> Self {
        Self {
            fee_type,
            fee_percentage,
        }
    }
}

pub fn convert_fee_to_percentage(fee: &BigInt) -> BigDecimal {
    let fee_decimal = BigDecimal::try_from(fee.to_string()).unwrap();
    let percent = fee_decimal / constants::BIGDECIMAL_10000.clone();
    percent
}

pub fn create_pool_fees(pool_address: &Vec<u8>, trading_fee: &BigInt) -> (LiquidityPoolFee, LiquidityPoolFee, LiquidityPoolFee) {
    let trading_fee_percentage = convert_fee_to_percentage(trading_fee);

    // Trading Fee
    let mut pool_trading_fee = LiquidityPoolFee::new(
        enums::LiquidityPoolFeeType::FIXED_TRADING_FEE, 
        trading_fee_percentage.clone(),
    );

    // LP Fee
    let mut pool_lp_fee = LiquidityPoolFee::new(
        enums::LiquidityPoolFeeType::FIXED_LP_FEE,
        trading_fee_percentage.clone(),
    );

    // Protocol Fee
    let mut pool_protocol_fee = LiquidityPoolFee::new(
        enums::LiquidityPoolFeeType::FIXED_PROTOCOL_FEE,
        constants::BIGDECIMAL_ZERO.clone(),
    );

    (pool_lp_fee, pool_protocol_fee, pool_trading_fee)
}
