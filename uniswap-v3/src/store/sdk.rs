use core::panic;
use std::collections::{HashMap, HashSet};

use substreams::pb::substreams::Clock;
use substreams::scalar::{BigDecimal, BigInt, self};
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams::log;

use crate::constants;
use crate::pb::common;
use crate::pb::dex_amm::v3_0_3::PositionEntityCreation;
use crate::store::store_operations;

use substreams::store;
use crate::pb::dex_amm::v3_0_3::{EntityUpdates, PrunedTransaction, EntityCreation, SwapEntityCreation, DepositEntityCreation, WithdrawEntityCreation, TokenEntityCreation, LiquidityPoolFeeEntityCreation};
use crate::pb::dex_amm::v3_0_3::entity_creation::Type;
use crate::pb::store::v1::{StoreOperation};
use crate::schema_lib::dex_amm::v_3_0_3::enums;
use crate::schema_lib::dex_amm::v_3_0_3::keys;

impl PrunedTransaction {
    pub fn new(transaction_trace: &eth::TransactionTrace) -> Self {
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
}

struct EntityAccount {
    pub seen: bool,
    pub should_create: bool,
}

impl EntityAccount {
    pub fn new(old_value: i64) -> Self {
        EntityAccount {
            seen: false,
            should_create: old_value == 0 as i64,
        }
    }
}

impl std::fmt::Display for EntityAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EntityAccount {{ seen: {}, should_create: {} }}", self.seen, self.should_create)
    }
}

// Implementation of core logic for creating entities
pub struct EntityUpdateFactory {
    pub store_operations: StoreOperationFactory,
    pruned_transactions_map: HashMap<Vec<u8>, PrunedTransaction>,
    entity_accounting_table: HashMap<String, EntityAccount>,
}

impl EntityUpdateFactory {
    pub fn new(clock: Clock, int64_store_deltas: &store::Deltas<store::DeltaInt64>) -> Self {
        EntityUpdateFactory {
            pruned_transactions_map: HashMap::new(),
            store_operations: StoreOperationFactory::new(clock),
            entity_accounting_table: Self::get_entity_accounting_table(int64_store_deltas),
        }
    }

    fn get_entity_accounting_table(
        int64_store_deltas: &store::Deltas<store::DeltaInt64>,
    ) -> HashMap<String, EntityAccount> {
        let mut entity_accounting_table: HashMap<String, EntityAccount> = HashMap::new();
        for delta in &int64_store_deltas.deltas {
            let key_list = delta.key.split(":").collect::<Vec<_>>();
            if key_list[0] == "entity-mutation-count" && delta.ordinal == 0 {
                if !entity_accounting_table.contains_key(&key_list[1..].join(":")) {
                    entity_accounting_table.insert(
                        key_list[1..].join(":"),
                        EntityAccount::new(delta.old_value),
                    );
                }
                // log::println(format!("{}: {}: {}", key_list[1..].join(":"), delta.old_value, delta.ordinal));
            }
        }
        entity_accounting_table
    }

    fn get_or_create_pruned_transaction(&mut self, transaction_trace: &eth::TransactionTrace) -> &mut PrunedTransaction {
        self.pruned_transactions_map.entry(transaction_trace.hash.clone())
            .or_insert_with(|| PrunedTransaction::new(transaction_trace))
    }

    fn should_create_entity(&mut self, entity_id: &str) -> bool {
        match self.entity_accounting_table.get_mut(entity_id.clone()) {
            Some(entity_account) => {
                if entity_account.seen == true {
                    return false;
                }
                entity_account.seen = true;
                return entity_account.should_create;
            }
            None => {
                let key_list = entity_id.split(":").collect::<Vec<_>>();
                panic!("Creation of mutable entity {} not accounted for in entity_accounting_table. Please add to int64_store, before prepare_entity_changes module if you would like to create this entity. ID: {}", key_list[0], key_list[1])
            }
        }
    }

    pub fn to_entity_updates(&mut self) -> EntityUpdates {
        for (entity_id, entity_account) in self.entity_accounting_table.iter() {
            if entity_account.seen == false {
                for (key, value) in self.entity_accounting_table.iter() {
                    log::println(format!("{}: {}", key, value));
                }
                panic!("Mutable entity {} was not seen. Please add to byte_store, before prepare_entity_changes module if you would like to create this entity. {}", entity_id, entity_account);
            }
        }

        EntityUpdates {
            pruned_transactions: self.pruned_transactions_map.values().cloned().collect(),
            store_operations: self.store_operations.get_operations(),
        }
    }
}

// Implementation of entity creation logic for DEX protocols.
impl EntityUpdateFactory {
    pub fn create_dex_amm_protocol_entity_if_not_exists(
        &mut self,
        transaction_trace: &eth::TransactionTrace,
        entity_id: &str,
        protocol_address: &Vec<u8>,
        name: &str,
        slug: &str,
        schema_version: &str,
        substream_version: &str,
        methodology_version: &str,
        network: &enums::Network,
        r#type: &enums::ProtocolType,
    ) { 
        let complete_id = ["DexAmmProtocol", &entity_id].join(":");
        if !self.should_create_entity(&complete_id) {
            return;
        }
        let pruned_transaction: &mut PrunedTransaction = self.get_or_create_pruned_transaction(transaction_trace);
        pruned_transaction.entity_creations.push(
            EntityCreation {
                entity_id: entity_id.as_bytes().to_vec(),
                r#type: Some(Type::DexAmmProtocolEntityCreation(
                    crate::pb::dex_amm::v3_0_3::DexAmmProtocolEntityCreation {
                        protocol_address: protocol_address.clone(),
                        name: name.to_string(),
                        slug: slug.to_string(),
                        schema_version: schema_version.to_string(),
                        substream_version: substream_version.to_string(),
                        methodology_version: methodology_version.to_string(),
                        network: network.to_string(),
                        r#type: r#type.to_string(),
                    }
                )),
            }
        );
    }

    pub fn create_liquidity_pool_entity_if_not_exists(
        &mut self,
        transaction_trace: &eth::TransactionTrace,
        entity_id: &str,
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
        let complete_id = ["LiquidityPool", &entity_id].join(":");
        if !self.should_create_entity(&complete_id) {
            return;
        }

        let pruned_transaction: &mut PrunedTransaction = self.get_or_create_pruned_transaction(transaction_trace);
        let mut input_token_weights_list = Vec::<common::v1::BigDecimal>::new();
        for input_token_weight in input_token_weights {
            input_token_weights_list.push(input_token_weight.clone().into());
        }   
        pruned_transaction.entity_creations.push(
            EntityCreation {
                entity_id: entity_id.as_bytes().to_vec(),
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

    pub fn create_tick_entity_if_not_exists(
        &mut self,
        transaction_trace: &eth::TransactionTrace,
        entity_id: &str,
        pool_address: &Vec<u8>,
        index: &scalar::BigInt,
    ) {
        let complete_id = ["Tick", &entity_id].join(":");
        if !self.should_create_entity(&complete_id) {
            return;
        }

        let pruned_transaction: &mut PrunedTransaction = self.get_or_create_pruned_transaction(transaction_trace);
        pruned_transaction.entity_creations.push(
            EntityCreation {
                entity_id: entity_id.as_bytes().to_vec(),
                r#type: Some(Type::TickEntityCreation(
                    crate::pb::dex_amm::v3_0_3::TickEntityCreation {
                        index: Some(index.clone().into()),
                        pool: pool_address.clone(),
                    }
                ))
            }
        );
    }

    pub fn create_liquidity_pool_fee_entity_if_not_exists(
        &mut self,
        transaction_trace: &eth::TransactionTrace,
        entity_id: &str,
        pool_address: &Vec<u8>,
        fee_type: &enums::LiquidityPoolFeeType,
        fee_percentage: &BigDecimal,
    ) {
        let complete_id = ["LiquidityPoolFee", &entity_id].join(":");
        if !self.should_create_entity(&complete_id) {
            return;
        }
        let pruned_transaction: &mut PrunedTransaction = self.get_or_create_pruned_transaction(transaction_trace);
        pruned_transaction.entity_creations.push(
            EntityCreation {
                entity_id: entity_id.as_bytes().to_vec(),
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

    pub fn create_token_entity_if_not_exists(
        &mut self,
        transaction_trace: &eth::TransactionTrace,
        entity_id: &str,
        token_address: &Vec<u8>,
        name: &str,
        symbol: &str,
        decimals: i32,
    ) {
        let complete_id = ["Token", &entity_id].join(":");
        if !self.should_create_entity(&complete_id) {
            return;
        }
        let pruned_transaction: &mut PrunedTransaction = self.get_or_create_pruned_transaction(transaction_trace);
        pruned_transaction.entity_creations.push(
            EntityCreation {
                entity_id: entity_id.as_bytes().to_vec(),
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

    pub fn create_position_entity_if_not_exists(
        &mut self,
        transaction_trace: &eth::TransactionTrace,
        entity_id: &str,
        account: &Vec<u8>,
        pool_address: &Vec<u8>,
        n_tokens: i32,
        tick_lower: Option<&scalar::BigInt>,
        tick_upper: Option<&scalar::BigInt>,
        liquidity_token: Option<&Vec<u8>>,
        liquidity_token_type: Option<&enums::TokenType>,
        n_reward_tokens: Option<i32>,
    ) {
        let complete_id = ["Position", &entity_id].join(":");
        if !self.should_create_entity(&complete_id) {
            return;
        }
        let pruned_transaction: &mut PrunedTransaction = self.get_or_create_pruned_transaction(transaction_trace);
        pruned_transaction.entity_creations.push(
            EntityCreation {
                entity_id: entity_id.as_bytes().to_vec(),
                r#type: Some(Type::PositionEntityCreation(
                    PositionEntityCreation {
                        account: account.clone(),
                        pool: pool_address.clone(),
                        n_tokens: n_tokens,
                        tick_lower: match tick_lower {
                            Some(tick_lower) => Some(tick_lower.clone().into()),
                            None => None,
                        },
                        tick_upper: match tick_upper {
                            Some(tick_upper) => Some(tick_upper.clone().into()),
                            None => None,
                        },
                        liquidity_token: match liquidity_token {
                            Some(liquidity_token) => Some(liquidity_token.clone()),
                            None => None,
                        },
                        liquidity_token_type: match liquidity_token_type {
                            Some(liquidity_token_type) => Some(liquidity_token_type.to_string()),
                            None => None,
                        },
                        n_reward_tokens: match n_reward_tokens {
                            Some(n_reward_tokens) => Some(n_reward_tokens),
                            None => None,
                        },
                    }
                ))
            }
        );
    }

    pub fn create_swap_entity(
        &mut self,
        transaction_trace: &eth::TransactionTrace,
        pool: &Vec<u8>,
        protocol: &Vec<u8>,
        account: &Vec<u8>,
        input_tokens: &Vec<Vec<u8>>,
        amounts: &Vec<scalar::BigInt>,
        liquidity: &scalar::BigInt,
        tick: Option<&scalar::BigInt>,
        hash: &Vec<u8>,
        log_index: u32,
        log_ordinal: u64,
    ) {
        let mut amounts_list = Vec::<common::v1::BigInt>::new();
        for amount in amounts {
            amounts_list.push(amount.clone().into());
        }   
        let pruned_transaction: &mut PrunedTransaction = self.get_or_create_pruned_transaction(transaction_trace);
        pruned_transaction.entity_creations.push(
            EntityCreation {
                entity_id: keys::get_event_key(hash, log_index),
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
        transaction_trace: &eth::TransactionTrace,
        pool: &Vec<u8>,
        protocol: &Vec<u8>,
        account: &Vec<u8>,
        liquidity: &scalar::BigInt,
        input_tokens: &Vec<Vec<u8>>,
        input_token_amounts: &Vec<scalar::BigInt>,
        position: Option<&Vec<u8>>,
        tick_lower: Option<&scalar::BigInt>,
        tick_upper: Option<&scalar::BigInt>,
        hash: &Vec<u8>,
        log_index: u32,
        log_ordinal: u64,
    ) {
        let mut input_token_amounts_list = Vec::<common::v1::BigInt>::new();
        for input_token_amount in input_token_amounts {
            input_token_amounts_list.push(input_token_amount.clone().into());
        }   
        let pruned_transaction: &mut PrunedTransaction = self.get_or_create_pruned_transaction(transaction_trace);
        pruned_transaction.entity_creations.push(
            EntityCreation {
                entity_id: keys::get_event_key(hash, log_index),
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
        transaction_trace: &eth::TransactionTrace,
        pool: &Vec<u8>,
        protocol: &Vec<u8>,
        account: &Vec<u8>,
        liquidity: &scalar::BigInt,
        input_tokens: &Vec<Vec<u8>>,
        input_token_amounts: &Vec<scalar::BigInt>,
        position: Option<&Vec<u8>>,
        tick_lower: Option<&scalar::BigInt>,
        tick_upper: Option<&scalar::BigInt>,
        hash: &Vec<u8>,
        log_index: u32,
        log_ordinal: u64,
    ) {
        let mut input_token_amounts_list = Vec::<common::v1::BigInt>::new();
        for input_token_amount in input_token_amounts {
            input_token_amounts_list.push(input_token_amount.clone().into());
        }   
        let pruned_transaction: &mut PrunedTransaction = self.get_or_create_pruned_transaction(transaction_trace);
        pruned_transaction.entity_creations.push(
            EntityCreation {
                entity_id: keys::get_event_key(hash, log_index),
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

fn entity_change_key(entity_type: &str, entity_id: &str, field: &str) -> String {
    ["entity-change", entity_type, entity_id, field].join(":")
}

fn entity_array_change_key(entity_type: &str, entity_id: &str, field: &str, index: usize, array_size: usize) -> String {
    ["entity-change", entity_type, entity_id, field, index.to_string().as_str(), array_size.to_string().as_str()].join(":")
}

fn entity_last_updated_key(entity_type: &str, entity_id: &str) -> String {
    ["last-updated", entity_type, entity_id].join(":")
}

fn raw_store_key(key: &str) -> String {
    ["raw", key].join(":")
}

fn track_entity_mutation_key(entity_type: &str, entity_id: &str) -> String {
    ["entity-mutation-count", entity_type, entity_id].join(":")
}

pub struct StoreOperationFactory {
    timestamp: i64,
    operations: Vec<StoreOperation>,
    entity_updates: HashSet<String>,
}

// Implement core methods for StoreOperations
impl StoreOperationFactory {
    pub fn new(clock: Clock) -> Self {
        StoreOperationFactory {
            timestamp: clock.timestamp.unwrap().seconds,
            operations: Vec::<StoreOperation>::new(),
            entity_updates: HashSet::<String>::new(),
        }
    }
    
    pub fn get_operations(&self) -> Vec<StoreOperation> {
        self.operations.clone()
    }

    fn track_entity_update_timestamp(&mut self, entity_type: &str, entity_id: &str) {
        if self.entity_updates.insert([entity_type, entity_id].join(":")) {
            self.operations.push(
                store_operations::set_int64(
                    0,
                    entity_last_updated_key(entity_type, entity_id),
                    self.timestamp,
                )
            );
        }
    }
}

// Implement raw store operations
impl StoreOperationFactory {
    pub fn add_raw_int64<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        key: K,
        value: i64,
    ) {
        self.operations.push(
            store_operations::add_int64(
                ordinal,
                raw_store_key(key.as_ref()),
                value,
            )
        );
    }

    pub fn set_raw_int64<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        key: K,
        value: i64,
    ) {
        self.operations.push(
            store_operations::set_int64(
                ordinal,
                raw_store_key(key.as_ref()),
                value,
            )
        );
    }

    pub fn add_raw_bigint<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        key: K,
        value: scalar::BigInt,
    ) {
        self.operations.push(
            store_operations::add_bigint(
                ordinal,
                raw_store_key(key.as_ref()),
                value,
            )
        );
    }

    pub fn set_raw_bigint<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        key: K,
        value: scalar::BigInt,
    ) {
        self.operations.push(
            store_operations::set_bigint(
                ordinal,
                raw_store_key(key.as_ref()),
                value,
            )
        );
    }

    pub fn add_raw_bigdecimal<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        key: K,
        value: scalar::BigDecimal,
    ) {
        self.operations.push(
            store_operations::add_bigdecimal(
                ordinal,
                raw_store_key(key.as_ref()),
                value,
            )
        );
    }

    pub fn set_raw_bigdecimal<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        key: K,
        value: scalar::BigDecimal,
    ) {
        self.operations.push(
            store_operations::set_bigdecimal(
                ordinal,
                raw_store_key(key.as_ref()),
                value,
            )
        );
    }

    pub fn set_raw_bytes<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        key: K,
        value: Vec<u8>,
    ) {
        self.operations.push(
            store_operations::set_bytes(
                ordinal,
                raw_store_key(key.as_ref()),
                value,
            )
        );
    }

    pub fn append_raw_bytes<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        key: K,
        value: Vec<u8>,
    ) {
        self.operations.push(
            store_operations::append_bytes(
                ordinal,
                raw_store_key(key.as_ref()),
                value,
            )
        );
    }

    pub fn set_raw_string<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        key: K,
        value: String,
    ) {
        self.operations.push(
            store_operations::set_string(
                ordinal,
                raw_store_key(key.as_ref()),
                value,
            )
        );
    }

    pub fn append_raw_string<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        key: K,
        value: String,
    ) {
        self.operations.push(
            store_operations::append_string(
                ordinal,
                raw_store_key(key.as_ref()),
                value,
            )
        );
    }
}



// Implement store operations for DEX protocols. 
impl StoreOperationFactory {
    pub fn increment_liquidity_pool_cumulative_swap_count<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
    ) {
        self.track_entity_update_timestamp("LiquidityPool", entity_id.as_ref());
        self.operations.push(
            store_operations::add_int64(
                ordinal,
                entity_change_key("LiquidityPool", entity_id.as_ref(), "cumulativeSwapCount"),
                1,
            )
        );
    }

    pub fn increment_liquidity_pool_cumulative_deposit_count<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
    ) {
        self.track_entity_update_timestamp("LiquidityPool", entity_id.as_ref());
        self.operations.push(
            store_operations::add_int64(
                ordinal,
                entity_change_key("LiquidityPool", entity_id.as_ref(), "cumulativeDepositCount"),
                1,
            )
        );
    }

    pub fn increment_liquidity_pool_cumulative_withdraw_count<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
    ) {
        self.track_entity_update_timestamp("LiquidityPool", entity_id.as_ref());
        self.operations.push(
            store_operations::add_int64(
                ordinal,
                entity_change_key("LiquidityPool", entity_id.as_ref(), "cumulativeWithdrawCount"),
                1,
            )
        );
    }
    
    pub fn add_liquidity_pool_input_token_balances<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
        value: Vec<BigInt>,
    ) {
        self.track_entity_update_timestamp("LiquidityPool", entity_id.as_ref());
        let array_size = value.len();
        for (i, v) in value.into_iter().enumerate() {
            self.operations.push(
                store_operations::add_bigint(
                    ordinal,
                    entity_array_change_key("LiquidityPool", entity_id.as_ref(), "inputTokenBalances", i , array_size),
                    v.clone(),
                )
            );
        }
    }

    pub fn add_liquidity_pool_cumulative_volume_token_amounts<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
        value: Vec<BigInt>,
    ) {
        self.track_entity_update_timestamp("LiquidityPool", entity_id.as_ref());
        let array_size = value.len();
        for (i, v) in value.into_iter().enumerate() {
            self.operations.push(
                store_operations::add_bigint(
                    ordinal,
                    entity_array_change_key("LiquidityPool", entity_id.as_ref(), "cumulativeVolumeTokenAmounts", i , array_size),
                    v.clone(),
                )
            );
        }
    }

    pub fn add_liquidity_pool_cumulative_supply_side_revenue_token_amounts<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
        value: Vec<BigInt>,
    ) {
        self.track_entity_update_timestamp("LiquidityPool", entity_id.as_ref());
        let array_size = value.len();
        for (i, v) in value.into_iter().enumerate() {
            self.operations.push(
                store_operations::add_bigint(
                    ordinal,
                    entity_array_change_key("LiquidityPool", entity_id.as_ref(), "cumulativeSupplySideRevenueTokenAmounts", i , array_size),
                    v.clone(),
                )
            );
        }
    }

    pub fn add_liquidity_pool_cumulative_protocol_side_revenue_token_amounts<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
        value: Vec<BigInt>,
    ) {
        self.track_entity_update_timestamp("LiquidityPool", entity_id.as_ref());
        let array_size = value.len();
        for (i, v) in value.into_iter().enumerate() {
            self.operations.push(
                store_operations::add_bigint(
                    ordinal,
                    entity_array_change_key("LiquidityPool", entity_id.as_ref(), "cumulativeProtocolSideRevenueTokenAmounts", i , array_size),
                    v.clone(),
                )
            );
        }
    }

    pub fn add_liquidity_pool_cumulative_total_revenue_token_amounts<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
        value: Vec<BigInt>,
    ) {
        self.track_entity_update_timestamp("LiquidityPool", entity_id.as_ref());
        let array_size = value.len();
        for (i, v) in value.into_iter().enumerate() {
            self.operations.push(
                store_operations::add_bigint(
                    ordinal,
                    entity_array_change_key("LiquidityPool", entity_id.as_ref(), "cumulativeTotalRevenueTokenAmounts", i , array_size),
                    v.clone(),
                )
            );
        }
    }

    pub fn add_liquidity_pool_total_liquidity<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K, 
        value: BigInt,
    ) {
        self.track_entity_update_timestamp("LiquidityPool", entity_id.as_ref());
        self.operations.push(
            store_operations::add_bigint(
                ordinal,
                entity_change_key("LiquidityPool", entity_id.as_ref(), "totalLiquidity"),
                value.clone(),
            )
        );
    }

    pub fn set_liquidity_pool_active_liquidity<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
        value: BigInt,
    ) {
        self.track_entity_update_timestamp("LiquidityPool", entity_id.as_ref());
        self.operations.push(
            store_operations::set_bigint(
                ordinal,
                entity_change_key("LiquidityPool", entity_id.as_ref(), "activeLiquidity"),
                value.clone(),
            )
        );
    }

    pub fn set_liquidity_pool_tick<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
        value: BigInt,
    ) {
        self.track_entity_update_timestamp("LiquidityPool", entity_id.as_ref());
        self.operations.push(
            store_operations::set_bigint(
                ordinal,
                entity_change_key("LiquidityPool", entity_id.as_ref(), "tick"),
                value.clone(),
            )
        );
    }

    pub fn append_liquidity_pool_input_tokens<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
        value: Vec<Vec<u8>>,
    ) {
        self.track_entity_update_timestamp("LiquidityPool", entity_id.as_ref());
        for address in value.into_iter() {
            self.operations.push(
                store_operations::append_bytes(
                    ordinal,
                    entity_change_key("LiquidityPool", entity_id.as_ref(), "inputTokens"),
                    address.clone(),
                )
            );
        }
    }

    pub fn increment_dex_amm_protocol_total_pool_count<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
    ) {
        self.track_entity_update_timestamp("DexAmmProtocol", entity_id.as_ref());
        self.operations.push(
            store_operations::add_int64(
                ordinal,
                entity_change_key("DexAmmProtocol", entity_id.as_ref(), "totalPoolCount"),
                1,
            )
        );
    }

    pub fn increment_dex_amm_protocol_open_position_count<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
    ) {
        self.track_entity_update_timestamp("DexAmmProtocol", entity_id.as_ref());
        self.operations.push(
            store_operations::add_int64(
                ordinal,
                entity_change_key("DexAmmProtocol", entity_id.as_ref(), "openPositionCount"),
                1,
            )
        );
    }

    pub fn increment_dex_amm_protocol_cumulative_position_count<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K, 
    ) {
        self.track_entity_update_timestamp("DexAmmProtocol", entity_id.as_ref());
        self.operations.push(
            store_operations::add_int64(
                ordinal,
                entity_change_key("DexAmmProtocol", entity_id.as_ref(), "cumulativePositionCount"),
                1,
            )
        );
    }

    pub fn increment_position_deposit_count<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
    ) {
        self.track_entity_update_timestamp("Position", entity_id.as_ref());
        self.operations.push(
            store_operations::add_int64(
                ordinal,
                entity_change_key("Position", entity_id.as_ref(), "depositCount"),
                1,
            )
        );
    }

    pub fn increment_position_withdraw_count<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
    ) {
        self.track_entity_update_timestamp("Position", entity_id.as_ref());
        self.operations.push(
            store_operations::add_int64(
                ordinal,
                entity_change_key("Position", entity_id.as_ref(), "withdrawCount"),
                1,
            )
        );
    }
    
    pub fn add_position_cumulative_deposit_token_amounts<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K, 
        value: Vec<BigInt>,
    ) {
        self.track_entity_update_timestamp("Position", entity_id.as_ref());
        let array_size = value.len();
        for (i, v) in value.into_iter().enumerate() {
            self.operations.push(
                store_operations::add_bigint(
                    ordinal,
                    entity_array_change_key("Position", entity_id.as_ref(), "cumulativeDepositTokenAmounts", i , array_size),
                    v,
                )
            );
        }
    }

    pub fn add_position_cumulative_withdraw_token_amounts<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
        value: Vec<BigInt>,
    ) {
        self.track_entity_update_timestamp("Position", entity_id.as_ref());
        let array_size = value.len();
        for (i, v) in value.into_iter().enumerate() {
            self.operations.push(
                store_operations::add_bigint(
                    ordinal,
                    entity_array_change_key("Position", entity_id.as_ref(), "cumulativeWithdrawTokenAmounts", i , array_size),
                    v,
                )
            );
        }
    }

    pub fn add_position_liquidity<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
        value: BigInt,
    ) {
        self.track_entity_update_timestamp("Position", entity_id.as_ref());
        self.operations.push(
            store_operations::add_bigint(
                ordinal,
                entity_change_key("Position", entity_id.as_ref(), "liquidity"),
                value,
            )
        );
    }

    pub fn add_tick_liquidity_gross<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
        value: BigInt,
    ) {
        self.track_entity_update_timestamp("Tick", entity_id.as_ref());
        self.operations.push(
            store_operations::add_bigint(
                ordinal,
                entity_change_key("Tick", entity_id.as_ref(), "liquidityGross"),
                value,
            )
        );
    }

    pub fn add_tick_liquidity_net<K: AsRef<str>>(
        &mut self,
        ordinal: u64, 
        entity_id: K,
        value: BigInt,
    ) {
        self.track_entity_update_timestamp("Tick", entity_id.as_ref());
        self.operations.push(
            store_operations::add_bigint(
                ordinal,
                entity_change_key("Tick", entity_id.as_ref(), "liquidityNet"),
                value,
            )
        );
    }

    // Track Mutable Entity Mutations
    pub fn track_dex_amm_protocol_mutation<K: AsRef<str>>(
        &mut self, 
        entity_id: K,
    ) {
        self.operations.push(
            store_operations::add_int64(
                0,
                track_entity_mutation_key("DexAmmProtocol", entity_id.as_ref()),
                1,
            )
        );
    }

    pub fn track_liquidity_pool_mutation<K: AsRef<str>>(
        &mut self, 
        entity_id: K,
    ) {
        self.operations.push(
            store_operations::add_int64(
                0,
                track_entity_mutation_key("LiquidityPool", entity_id.as_ref()),
                1,
            )
        );
    }

    pub fn track_liquidity_pool_fee_mutation<K: AsRef<str>>(
        &mut self, 
        entity_id: K,
    ) {
        self.operations.push(
            store_operations::add_int64(
                0,
                track_entity_mutation_key("LiquidityPoolFee", entity_id.as_ref()),
                1,
            )
        );
    }

    pub fn track_position_mutation<K: AsRef<str>>(
        &mut self, 
        entity_id: K,
    ) {
        self.operations.push(
            store_operations::add_int64(
                0,
                track_entity_mutation_key("Position", entity_id.as_ref()),
                1,
            )
        );
    }

    pub fn track_tick_mutation<K: AsRef<str>>(
        &mut self, 
        entity_id: K,
    ) {
        self.operations.push(
            store_operations::add_int64(
                0,
                track_entity_mutation_key("Tick", entity_id.as_ref()),
                1,
            )
        );
    }

    pub fn track_token_mutation<K: AsRef<str>>(
        &mut self, 
        entity_id: K,
    ) {
        self.operations.push(
            store_operations::add_int64(
                0,
                track_entity_mutation_key("Token", entity_id.as_ref()),
                1,
            )
        );
    }
}
