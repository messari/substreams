use substreams::scalar::{BigDecimal, BigInt, self};
use substreams_ethereum::pb::eth::v2::{self as eth};

use crate::constants;
use crate::pb::common;
use crate::store::store_update;
use crate::pb::store::v1 as store;

use crate::pb::dex_amm::v3_0_3::{MappedDataSources, PrunedTransaction, Update, Swap, Deposit, Withdraw, CreateToken};
use crate::pb::dex_amm::v3_0_3::update::Type;

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
                ["LiquidityPoolCumulativeSwapCount", entity_id].join(":"),
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
                    ["LiquidityPoolInputTokenBalance", i.to_string().as_str(), entity_id].join(":"),
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
                    ["LiquidityPoolCumulativeVolumeTokenAmounts", i.to_string().as_str(), entity_id].join(":"),
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
                ["LiquidityPoolTotalLiquidity", entity_id].join(":"),
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
                ["LiquidityPoolActiveLiquidity", entity_id].join(":"),
                value.clone(),
            )
        );
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
            updates: Vec::<Update>::new(),
        }
    }

    pub fn create_liquidity_pool(
        &mut self,
        protocol: &Vec<u8>,
        pool_address: &Vec<u8>,
        input_tokens: &Vec<Vec<u8>>,
        is_single_sided: bool,
        reward_tokens: Option<&Vec<Vec<u8>>>,
        fees: Option<&Vec<Vec<u8>>>,
        tick: Option<&scalar::BigInt>,
        liquidity_token: Option<&Vec<u8>>, 
        liquidity_token_type: Option<&str>

    ) {
        self.updates.push(
            Update {
                r#type: Some(Type::CreateLiquidityPool(
                    crate::pb::dex_amm::v3_0_3::CreateLiquidityPool {
                        protocol: protocol.clone(),
                        pool_address: pool_address.clone(),
                        input_tokens: input_tokens.clone(),
                        is_single_sided: is_single_sided,
                        reward_tokens: match reward_tokens {
                            Some(reward_tokens) => reward_tokens.clone(),
                            None => vec![],
                        },
                        fees: match fees {
                            Some(fees) => fees.clone(),
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

    pub fn create_token(
        &mut self,
        token_address: &Vec<u8>,
        name: &str,
        symbol: &str,
        decimals: u64,
    ) {
        self.updates.push(
            Update {
                r#type: Some(Type::CreateToken(
                    CreateToken {
                        token_address: token_address.clone(),
                        name: name.to_string(),
                        symbol: symbol.to_string(),
                        decimals: decimals,
                    }
                ))
            }
        );
    }

    pub fn create_swap(
        &mut self,
        pool: &Vec<u8>,
        protocol: &Vec<u8>,
        account: &Vec<u8>,
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
        self.updates.push(
            Update {
                r#type: Some(Type::Swap(
                    Swap {
                        pool: pool.clone(),
                        protocol: protocol.clone(),
                        account: account.clone(),
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

    pub fn create_deposit(
        &mut self,
        pool: &Vec<u8>,
        protocol: &Vec<u8>,
        account: &Vec<u8>,
        liquidity: &scalar::BigInt,
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
        self.updates.push(
            Update {
                r#type: Some(Type::Deposit(
                    Deposit {
                        pool: pool.clone(),
                        protocol: protocol.clone(),
                        account: account.clone(),
                        liquidity: Some(liquidity.clone().into()),
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

    pub fn create_withdraw(
        &mut self,
        pool: &Vec<u8>,
        protocol: &Vec<u8>,
        account: &Vec<u8>,
        liquidity: &scalar::BigInt,
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
        self.updates.push(
            Update {
                r#type: Some(Type::Withdraw(
                    Withdraw {
                        pool: pool.clone(),
                        protocol: protocol.clone(),
                        account: account.clone(),
                        liquidity: Some(liquidity.clone().into()),
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
