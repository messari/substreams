use std::str::FromStr;

use substreams::Hex;
use substreams::scalar::{BigInt, BigDecimal};
use substreams_ethereum::{pb::eth::v2::{self as eth}};

use crate::{pb::dex_amm::v3_0_3::{
    MappedDataSources, PrunedTransaction, 
    Update, CreateLiquidityPool, CreateToken}, 
    utils::UNISWAP_V3_FACTORY_SLICE

};
use crate::pb::dex_amm::v3_0_3::update::Type::{CreateLiquidityPool as CreateLiquidityPoolType, CreateToken as CreateTokenType};
use crate::schema_lib::dex_amm::v_3_0_3::enums;

use crate::abi::factory as FactoryContract;
use crate::contract::erc20;
use crate::constants;

pub fn handle_pool_created(
    mapped_data_sources: &mut MappedDataSources, 
    pruned_transaction: &mut PrunedTransaction,
    pool_created_event: FactoryContract::events::PoolCreated, 
    call: &eth::Call, 
    log: &eth::Log
) {
    let (pool_lp_fee, pool_protocol_fee, pool_trading_fee) = create_pool_fees(&pool_created_event.pool, &pool_created_event.fee);
    pruned_transaction.create_liquidity_pool_fee(
        &pool_created_event.pool, 
        &pool_lp_fee.fee_type, 
        &pool_lp_fee.fee_percentage
    );

    pruned_transaction.create_liquidity_pool_fee(
        &pool_created_event.pool, 
        &pool_protocol_fee.fee_type, 
        &pool_protocol_fee.fee_percentage
    );

    pruned_transaction.create_liquidity_pool_fee(
        &pool_created_event.pool, 
        &pool_trading_fee.fee_type, 
        &pool_trading_fee.fee_percentage
    );

    let input_tokens: Vec<Vec<u8>> = vec![pool_created_event.token0.clone(), pool_created_event.token1.clone()];
    pruned_transaction.create_liquidity_pool(
        &UNISWAP_V3_FACTORY_SLICE.to_vec(),
        &pool_created_event.pool,
        &input_tokens,
        false,
        None,
        Some(&vec![enums::LiquidityPoolFeeType::FIXED_LP_FEE, enums::LiquidityPoolFeeType::FIXED_PROTOCOL_FEE, enums::LiquidityPoolFeeType::FIXED_TRADING_FEE]),
        None,
        None,
        None,
    );

    let token0 = erc20::Erc20::new(pool_created_event.token0.clone()).as_struct();
    let token1 = erc20::Erc20::new(pool_created_event.token1.clone()).as_struct();
    pruned_transaction.create_token(
        &pool_created_event.token0,
        &token0.name,
        &token0.symbol,
        token0.decimals as i32,
    );
    pruned_transaction.create_token(
        &pool_created_event.token1,
        &token1.name,
        &token1.symbol,
        token1.decimals as i32,
    );

    mapped_data_sources.append_liquidity_pool_input_tokens(
        &Hex(&pool_created_event.pool).to_string(), 
        0, 
        &input_tokens
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

pub fn convert_fee_to_percent(fee: &BigInt) -> BigDecimal {
    let fee_decimal = BigDecimal::try_from(fee.to_string()).unwrap();
    let percent = fee_decimal / constants::BIGDECIMAL_100.clone();
    percent
}

pub fn create_pool_fees(pool_address: &Vec<u8>, trading_fee: &BigInt) -> (LiquidityPoolFee, LiquidityPoolFee, LiquidityPoolFee) {
    let trading_fee_percentage = convert_fee_to_percent(trading_fee);

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
