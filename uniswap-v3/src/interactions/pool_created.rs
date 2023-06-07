use substreams::scalar::{BigDecimal, BigInt};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{self as eth};

use crate::utils::UNISWAP_V3_FACTORY_SLICE;

use crate::schema_lib::dex_amm::v_3_0_3::enums;

use crate::abi::factory as FactoryContract;
use crate::constants;
use crate::contract::erc20;
use crate::schema_lib::dex_amm::v_3_0_3::keys;
use crate::store::sdk;

pub fn create_store_operations_l1_pool_created(
    store_operation_factory: &mut sdk::StoreOperationFactory,
    pool_created_event: FactoryContract::events::PoolCreated,
) {
    let liquidity_pool_id_string = Hex(&pool_created_event.pool).to_string();
    let liquidity_pool_supply_side_fee_id = keys::get_liquidity_pool_fee_key(
        &liquidity_pool_id_string,
        &enums::LiquidityPoolFeeType::FIXED_LP_FEE,
    );
    let liquidity_pool_protocol_side_fee_id = keys::get_liquidity_pool_fee_key(
        &liquidity_pool_id_string,
        &enums::LiquidityPoolFeeType::FIXED_PROTOCOL_FEE,
    );
    let liquidity_pool_total_fee_id = keys::get_liquidity_pool_fee_key(
        &liquidity_pool_id_string,
        &enums::LiquidityPoolFeeType::FIXED_TRADING_FEE,
    );
    let token0_id_string = Hex(&pool_created_event.token0).to_string();
    let token1_id_string = Hex(&pool_created_event.token1).to_string();

    store_operation_factory.append_raw_string(
        0,
        [
            "LiquidityPool",
            liquidity_pool_id_string.as_str(),
            "inputTokens",
        ]
        .join(":"),
        token0_id_string.clone(),
    );
    store_operation_factory.append_raw_string(
        0,
        [
            "LiquidityPool",
            liquidity_pool_id_string.as_str(),
            "inputTokens",
        ]
        .join(":"),
        token1_id_string.clone(),
    );
    store_operation_factory.set_raw_bigdecimal(
        0,
        [
            "LiquidityPoolFee",
            &liquidity_pool_supply_side_fee_id,
            "feePercentage",
        ]
        .join(":"),
        convert_fee_to_percentage(&pool_created_event.fee),
    );
    store_operation_factory.set_raw_bigdecimal(
        0,
        [
            "LiquidityPoolFee",
            &liquidity_pool_protocol_side_fee_id,
            "feePercentage",
        ]
        .join(":"),
        convert_fee_to_percentage(&pool_created_event.fee),
    );
    store_operation_factory.set_raw_bigdecimal(
        0,
        [
            "LiquidityPoolFee",
            &liquidity_pool_total_fee_id,
            "feePercentage",
        ]
        .join(":"),
        constants::BIGDECIMAL_ZERO.clone(),
    );

    store_operation_factory
        .track_dex_amm_protocol_mutation(Hex(&UNISWAP_V3_FACTORY_SLICE).to_string());
    store_operation_factory.track_liquidity_pool_mutation(liquidity_pool_id_string);
    store_operation_factory.track_liquidity_pool_fee_mutation(liquidity_pool_supply_side_fee_id);
    store_operation_factory.track_liquidity_pool_fee_mutation(liquidity_pool_protocol_side_fee_id);
    store_operation_factory.track_liquidity_pool_fee_mutation(liquidity_pool_total_fee_id);
    store_operation_factory.track_token_mutation(token0_id_string);
    store_operation_factory.track_token_mutation(token1_id_string);
}

pub fn prepare_pool_created_entity_changes(
    entity_update_factory: &mut sdk::EntityUpdateFactory,
    transaction_trace: &eth::TransactionTrace,
    pool_created_event: FactoryContract::events::PoolCreated,
) {
    let protocol_entity_id = Hex(&UNISWAP_V3_FACTORY_SLICE.to_vec()).to_string();
    let liquidity_pool_id = Hex(&pool_created_event.pool).to_string();
    let liquidity_pool_fee_supply_side = keys::get_liquidity_pool_fee_key(
        &liquidity_pool_id,
        &enums::LiquidityPoolFeeType::FIXED_LP_FEE,
    );
    let liquidity_pool_fee_protocol_side = keys::get_liquidity_pool_fee_key(
        &liquidity_pool_id,
        &enums::LiquidityPoolFeeType::FIXED_PROTOCOL_FEE,
    );
    let liquidity_pool_fee_total = keys::get_liquidity_pool_fee_key(
        &liquidity_pool_id,
        &enums::LiquidityPoolFeeType::FIXED_TRADING_FEE,
    );
    let token0_id = Hex(&pool_created_event.token0).to_string();
    let token1_id = Hex(&pool_created_event.token1).to_string();

    entity_update_factory.create_dex_amm_protocol_entity_if_not_exists(
        &transaction_trace,
        &protocol_entity_id,
        &UNISWAP_V3_FACTORY_SLICE.to_vec(),
        "Uniswap V3",
        "uniswap-v3",
        "3.0.3",
        "1.0.0",
        "1.0.0",
        &enums::Network::MAINNET,
        &enums::ProtocolType::EXCHANGE,
    );

    let token0 = erc20::Erc20::new(pool_created_event.token0.clone()).as_struct();
    let token1 = erc20::Erc20::new(pool_created_event.token1.clone()).as_struct();
    entity_update_factory.create_token_entity_if_not_exists(
        &transaction_trace.clone(),
        &token0_id,
        &pool_created_event.token0,
        &token0.name,
        &token0.symbol,
        token0.decimals as i32,
    );
    entity_update_factory.create_token_entity_if_not_exists(
        &transaction_trace.clone(),
        &token1_id,
        &pool_created_event.token1,
        &token1.name,
        &token1.symbol,
        token1.decimals as i32,
    );
    let input_tokens: Vec<Vec<u8>> = vec![
        pool_created_event.token0.clone(),
        pool_created_event.token1.clone(),
    ];
    entity_update_factory.create_liquidity_pool_entity_if_not_exists(
        &transaction_trace.clone(),
        &liquidity_pool_id,
        &UNISWAP_V3_FACTORY_SLICE.to_vec(),
        &pool_created_event.pool,
        &input_tokens,
        &vec![token0.symbol.clone(), token1.symbol.clone()],
        &vec![
            constants::BIGDECIMAL_50.clone(),
            constants::BIGDECIMAL_50.clone(),
        ],
        false,
        None,
        Some(&vec![
            enums::LiquidityPoolFeeType::FIXED_LP_FEE,
            enums::LiquidityPoolFeeType::FIXED_PROTOCOL_FEE,
            enums::LiquidityPoolFeeType::FIXED_TRADING_FEE,
        ]),
        None,
        None,
        None,
    );

    let (pool_lp_fee, pool_protocol_fee, pool_trading_fee) =
        create_pool_fees(&pool_created_event.fee);
    entity_update_factory.create_liquidity_pool_fee_entity_if_not_exists(
        &transaction_trace.clone(),
        &liquidity_pool_fee_supply_side,
        &pool_created_event.pool,
        &pool_lp_fee.fee_type,
        &pool_lp_fee.fee_percentage,
    );

    entity_update_factory.create_liquidity_pool_fee_entity_if_not_exists(
        &transaction_trace.clone(),
        &liquidity_pool_fee_protocol_side,
        &pool_created_event.pool,
        &pool_protocol_fee.fee_type,
        &pool_protocol_fee.fee_percentage,
    );

    entity_update_factory.create_liquidity_pool_fee_entity_if_not_exists(
        &transaction_trace.clone(),
        &liquidity_pool_fee_total,
        &pool_created_event.pool,
        &pool_trading_fee.fee_type,
        &pool_trading_fee.fee_percentage,
    );

    entity_update_factory
        .store_operations
        .increment_dex_amm_protocol_total_pool_count(0, &protocol_entity_id);
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

pub fn create_pool_fees(
    trading_fee: &BigInt,
) -> (LiquidityPoolFee, LiquidityPoolFee, LiquidityPoolFee) {
    let trading_fee_percentage = convert_fee_to_percentage(trading_fee);

    // Trading Fee
    let pool_trading_fee = LiquidityPoolFee::new(
        enums::LiquidityPoolFeeType::FIXED_TRADING_FEE,
        trading_fee_percentage.clone(),
    );

    // LP Fee
    let pool_lp_fee = LiquidityPoolFee::new(
        enums::LiquidityPoolFeeType::FIXED_LP_FEE,
        trading_fee_percentage.clone(),
    );

    // Protocol Fee
    let pool_protocol_fee = LiquidityPoolFee::new(
        enums::LiquidityPoolFeeType::FIXED_PROTOCOL_FEE,
        constants::BIGDECIMAL_ZERO.clone(),
    );

    (pool_lp_fee, pool_protocol_fee, pool_trading_fee)
}
