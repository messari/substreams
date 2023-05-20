use substreams::scalar::{BigDecimal, BigInt, self};
use substreams_ethereum::pb::eth::v2::{self as eth};

use crate::abi::pool;
use crate::constants;
use crate::pb::common;
use crate::store::store_update;

use crate::pb::store::v1 as store;
use crate::pb::dex_amm::v3_0_3::{MappedDataSources, PrunedTransaction, EntityCreation, SwapEntityCreation, DepositEntityCreation, WithdrawEntityCreation, TokenEntityCreation, LiquidityPoolFeeEntityCreation};
use crate::pb::dex_amm::v3_0_3::entity_creation::Type;
use crate::schema_lib::dex_amm::v_3_0_3::enums;

impl MappedDataSources {
    pub fn new() -> Self {
        MappedDataSources {
            pruned_transactions: vec![],
            store_instructions: vec![],
        }
    }

    pub fn add_liquidity_pool_cumulative_swap_count(
        &mut self,
        entity_id: &str,
        ordinal: u64, 
        value: i64,
    ) {
        self.store_instructions.push(
            store_update::add_int_64(
                ordinal,
                ["entity-change", "LiquidityPool", entity_id, "cumulativeSwapCount"].join(":"),
                value,
            )
        );
    }

    pub fn add_liquidity_pool_cumulative_deposit_count(
        &mut self,
        entity_id: &str,
        ordinal: u64, 
        value: i64,
    ) {
        self.store_instructions.push(
            store_update::add_int_64(
                ordinal,
                ["entity-change", "LiquidityPool", entity_id, "cumulativeDepositCount"].join(":"),
                value,
            )
        );
    }

    pub fn add_liquidity_pool_cumulative_withdraw_count(
        &mut self,
        entity_id: &str,
        ordinal: u64, 
        value: i64,
    ) {
        self.store_instructions.push(
            store_update::add_int_64(
                ordinal,
                ["entity-change", "LiquidityPool", entity_id, "cumulativeWithdrawCount"].join(":"),
                value,
            )
        );
    }
    
    pub fn add_liquidity_pool_input_token_balances(
        &mut self,
        entity_id: &str,
        ordinal: u64, 
        value: &Vec<BigInt>,
    ) {
        for (i, v) in value.into_iter().enumerate() {
            self.store_instructions.push(
                store_update::add_bigint(
                    ordinal,
                    ["entity-change", "LiquidityPool", entity_id, "inputTokenBalances", i.to_string().as_str(), value.len().to_string().as_str()].join(":"),
                    v.clone(),
                )
            );
        }
    }

    pub fn add_liquidity_pool_cumulative_volume_token_amounts(
        &mut self,
        entity_id: &str,
        ordinal: u64, 
        value: &Vec<BigInt>,
    ) {
        for (i, v) in value.into_iter().enumerate() {
            self.store_instructions.push(
                store_update::add_bigint(
                    ordinal,
                    ["entity-change", "LiquidityPool", entity_id, "cumulativeVolumeTokenAmounts", i.to_string().as_str(), value.len().to_string().as_str()].join(":"),
                    v.clone(),
                )
            );
        }
    }

    pub fn add_liquidity_pool_cumulative_supply_side_revenue_token_amounts(
        &mut self,
        entity_id: &str,
        ordinal: u64, 
        value: &Vec<BigInt>,
    ) {
        for (i, v) in value.into_iter().enumerate() {
            self.store_instructions.push(
                store_update::add_bigint(
                    ordinal,
                    ["entity-change", "LiquidityPool", entity_id, "cumulativeSupplySideRevenueTokenAmounts", i.to_string().as_str(), value.len().to_string().as_str()].join(":"),
                    v.clone(),
                )
            );
        }
    }

    pub fn add_liquidity_pool_cumulative_protocol_side_revenue_token_amounts(
        &mut self,
        entity_id: &str,
        ordinal: u64, 
        value: &Vec<BigInt>,
    ) {
        for (i, v) in value.into_iter().enumerate() {
            self.store_instructions.push(
                store_update::add_bigint(
                    ordinal,
                    ["entity-change", "LiquidityPool", entity_id, "cumulativeProtocolSideRevenueTokenAmounts", i.to_string().as_str(), value.len().to_string().as_str()].join(":"),
                    v.clone(),
                )
            );
        }
    }

    pub fn add_liquidity_pool_cumulative_total_revenue_token_amounts(
        &mut self,
        entity_id: &str,
        ordinal: u64, 
        value: &Vec<BigInt>,
    ) {
        for (i, v) in value.into_iter().enumerate() {
            self.store_instructions.push(
                store_update::add_bigint(
                    ordinal,
                    ["entity-change", "LiquidityPool", entity_id, "cumulativeTotalRevenueTokenAmounts", i.to_string().as_str(), value.len().to_string().as_str()].join(":"),
                    v.clone(),
                )
            );
        }
    }

    pub fn add_liquidity_pool_total_liquidity(
        &mut self,
        entity_id: &str,
        ordinal: u64, 
        value: &BigInt,
    ) {
        self.store_instructions.push(
            store_update::add_bigint(
                ordinal,
                ["entity-change", "LiquidityPool", entity_id, "totalLiquidity"].join(":"),
                value.clone(),
            )
        );
    }

    pub fn set_liquidity_pool_active_liquidity(
        &mut self,
        entity_id: &str,
        ordinal: u64, 
        value: &BigInt,
    ) {
        self.store_instructions.push(
            store_update::set_bigint(
                ordinal,
                ["entity-change", "LiquidityPool", entity_id, "activeLiquidity"].join(":"),
                value.clone(),
            )
        );
    }

    pub fn set_liquidity_pool_tick(
        &mut self,
        entity_id: &str,
        ordinal: u64, 
        value: &BigInt,
    ) {
        self.store_instructions.push(
            store_update::set_bigint(
                ordinal,
                ["entity-change", "LiquidityPool", entity_id, "tick"].join(":"),
                value.clone(),
            )
        );
    }

    pub fn append_liquidity_pool_input_tokens(
        &mut self,
        entity_id: &str,
        ordinal: u64, 
        value: &Vec<Vec<u8>>,
    ) {
        for address in value.into_iter() {
            self.store_instructions.push(
                store_update::append_bytes(
                    ordinal,
                    ["entity-change", "LiquidityPool", entity_id, "inputTokens"].join(":"),
                    address.clone(),
                )
            );
        }
    }
}

impl PrunedTransaction {
    pub fn new(transaction_trace: eth::TransactionTrace) -> Self {
        PrunedTransaction {
            hash: transaction_trace.hash.clone(),
            from: transaction_trace.from.clone(),
            to: transaction_trace.to.clone(),
            nonce: Some(transaction_trace.nonce.into()),
            gas_limit: Some(transaction_trace.gas_limit.into()),
            gas_used: Some(transaction_trace.gas_used.into()),
            gas_price: Some(constants::BIGINT_ZERO.clone().into()),
            entity_creations: Vec::<EntityCreation>::new(),
        }
    }

    pub fn create_liquidity_pool_entity(
        &mut self,
        protocol: &Vec<u8>,
        pool_address: &Vec<u8>,
        input_tokens: &Vec<Vec<u8>>,
        input_token_symbols: &Vec<String>,
        input_token_weights: &Vec<scalar::BigDecimal>,
        is_single_sided: bool,
        reward_tokens: Option<&Vec<Vec<u8>>>,
        fees: Option<&Vec<enums::LiquidityPoolFeeType>>,
        tick: Option<&scalar::BigInt>,
        liquidity_token: Option<&Vec<u8>>, 
        liquidity_token_type: Option<&str>

    ) {
        let mut input_token_weights_list = Vec::<common::v1::BigDecimal>::new();
        for input_token_weight in input_token_weights {
            input_token_weights_list.push(input_token_weight.clone().into());
        }   
        self.entity_creations.push(
            EntityCreation {
                r#type: Some(Type::LiquidityPoolEntityCreation(
                    crate::pb::dex_amm::v3_0_3::LiquidityPoolEntityCreation {
                        protocol: protocol.clone(),
                        pool_address: pool_address.clone(),
                        input_tokens: input_tokens.clone(),
                        input_token_symbols: input_token_symbols.clone(),
                        input_token_weights: input_token_weights_list,
                        is_single_sided: is_single_sided,
                        reward_tokens: match reward_tokens {
                            Some(reward_tokens) => reward_tokens.clone(),
                            None => vec![],
                        },
                        fees: match fees {
                            Some(fees) => fees.iter()
                                .map(|fee| {
                                    let mut pool_fee = pool_address.clone(); // Clone the pool_address for each fee
                                    pool_fee.extend(&fee.to_string().into_bytes()); // Extend it with the fee bytes
                                    pool_fee // Return it
                                })
                                .collect(),
                            None => vec![],
                        },
                        tick: match tick {
                            Some(tick) => Some(tick.clone().into()),
                            None => None,
                        },
                        liquidity_token: match liquidity_token {
                            Some(liquidity_token) => Some(liquidity_token.clone().into()),
                            None => None,
                        },
                        liquidity_token_type: match liquidity_token_type {
                            Some(liquidity_token_type) => Some(liquidity_token_type.to_string()),
                            None => None,
                        },
                    }
                ))
            }
        );
    }

    pub fn create_liquidity_pool_fee_entity(
        &mut self,
        pool_address: &Vec<u8>,
        fee_type: &enums::LiquidityPoolFeeType,
        fee_percentage: &BigDecimal,
    ) {
        self.entity_creations.push(
            EntityCreation {
                r#type: Some(Type::LiquidityPoolFeeEntityCreation(
                    LiquidityPoolFeeEntityCreation {
                        pool_address: pool_address.clone(),
                        fee_type: fee_type.to_string(),
                        fee_percentage: Some(fee_percentage.clone().into()),
                    }
                ))
            }
        );
    }

    pub fn create_token_entity(
        &mut self,
        token_address: &Vec<u8>,
        name: &str,
        symbol: &str,
        decimals: i32,
    ) {
        self.entity_creations.push(
            EntityCreation {
                r#type: Some(Type::TokenEntityCreation(
                    TokenEntityCreation {
                        token_address: token_address.clone(),
                        name: name.to_string(),
                        symbol: symbol.to_string(),
                        decimals: decimals,
                    }
                ))
            }
        );
    }

    pub fn create_swap_entity(
        &mut self,
        pool: &Vec<u8>,
        protocol: &Vec<u8>,
        account: &Vec<u8>,
        input_tokens: &Vec<Vec<u8>>,
        amounts: &Vec<scalar::BigInt>,
        liquidity: &scalar::BigInt,
        tick: Option<&scalar::BigInt>,
        log_index: u32,
        log_ordinal: u64,
    ) {
        let mut amounts_list = Vec::<common::v1::BigInt>::new();
        for amount in amounts {
            amounts_list.push(amount.clone().into());
        }   
        self.entity_creations.push(
            EntityCreation {
                r#type: Some(Type::SwapEntityCreation(
                    SwapEntityCreation {
                        pool: pool.clone(),
                        protocol: protocol.clone(),
                        account: account.clone(),
                        input_tokens: input_tokens.clone(),
                        amounts: amounts_list,
                        liquidity: Some(liquidity.clone().into()),
                        tick: match tick {
                            Some(tick) => Some(tick.clone().into()),
                            None => None,
                        },
                        log_index: Some(log_index.into()),
                        log_ordinal: Some(log_ordinal.into()),
                    }
                ))
            }
        );
    }

    pub fn create_deposit_entity(
        &mut self,
        pool: &Vec<u8>,
        protocol: &Vec<u8>,
        account: &Vec<u8>,
        liquidity: &scalar::BigInt,
        input_tokens: &Vec<Vec<u8>>,
        input_token_amounts: &Vec<scalar::BigInt>,
        position: Option<&Vec<u8>>,
        tick_lower: Option<&scalar::BigInt>,
        tick_upper: Option<&scalar::BigInt>,
        log_index: u32,
        log_ordinal: u64,
    ) {
        let mut input_token_amounts_list = Vec::<common::v1::BigInt>::new();
        for input_token_amount in input_token_amounts {
            input_token_amounts_list.push(input_token_amount.clone().into());
        }   
        self.entity_creations.push(
            EntityCreation {
                r#type: Some(Type::DepositEntityCreation(
                    DepositEntityCreation {
                        pool: pool.clone(),
                        protocol: protocol.clone(),
                        account: account.clone(),
                        liquidity: Some(liquidity.clone().into()),
                        input_tokens: input_tokens.clone(),
                        input_token_amounts: input_token_amounts_list,
                        position: match position {
                            Some(position) => Some(position.clone()),
                            None => None,
                        },
                        tick_lower: match tick_lower {
                            Some(tick_lower) => Some(tick_lower.clone().into()),
                            None => None,
                        },
                        tick_upper: match tick_upper {
                            Some(tick_upper) => Some(tick_upper.clone().into()),
                            None => None,
                        },
                        log_index: Some(log_index.into()),
                        log_ordinal: Some(log_ordinal.into()),
                    }
                ))
            }
        );
    }

    pub fn create_withdraw_entity(
        &mut self,
        pool: &Vec<u8>,
        protocol: &Vec<u8>,
        account: &Vec<u8>,
        liquidity: &scalar::BigInt,
        input_tokens: &Vec<Vec<u8>>,
        input_token_amounts: &Vec<scalar::BigInt>,
        position: Option<&Vec<u8>>,
        tick_lower: Option<&scalar::BigInt>,
        tick_upper: Option<&scalar::BigInt>,
        log_index: u32,
        log_ordinal: u64,
    ) {
        let mut input_token_amounts_list = Vec::<common::v1::BigInt>::new();
        for input_token_amount in input_token_amounts {
            input_token_amounts_list.push(input_token_amount.clone().into());
        }   
        self.entity_creations.push(
            EntityCreation {
                r#type: Some(Type::WithdrawEntityCreation(
                    WithdrawEntityCreation {
                        pool: pool.clone(),
                        protocol: protocol.clone(),
                        account: account.clone(),
                        liquidity: Some(liquidity.clone().into()),
                        input_tokens: input_tokens.clone(),
                        input_token_amounts: input_token_amounts_list,
                        position: match position {
                            Some(position) => Some(position.clone()),
                            None => None,
                        },
                        tick_lower: match tick_lower {
                            Some(tick_lower) => Some(tick_lower.clone().into()),
                            None => None,
                        },
                        tick_upper: match tick_upper {
                            Some(tick_upper) => Some(tick_upper.clone().into()),
                            None => None,
                        },
                        log_index: Some(log_index.into()),
                        log_ordinal: Some(log_ordinal.into()),
                    }
                ))
            }
        );
    }
}
