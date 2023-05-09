use substreams::prelude::*;
use substreams::errors::Error;
use substreams::Hex;
use substreams_ethereum::{pb::eth::v2::{self as eth}, Event, NULL_ADDRESS};
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{StoreGetProto};

use crate::{pb::dex_amm::v3_0_3::{
    DataSource, MappedDataSources, PrunedTransaction, StoreInstruction, 
    Update, Swap, Deposit, Withdraw, CreateLiquidityPool, AddInt64, AddManyInt64}, 
    utils::UNISWAP_V3_FACTORY_SLICE, dex_amm::v_3_0_3::map

};
use crate::pb::dex_amm::v3_0_3::update::Type::{Swap as SwapType, Deposit as DepositType, Withdraw as WithdrawType, CreateLiquidityPool as CreateLiquidityPoolType};
use crate::pb::dex_amm::v3_0_3::store_instruction;

use crate::abi::pool as PoolContract;
use crate::abi::factory as FactoryContract;
use crate::schema_lib::dex_amm::v_3_0_3::store_keys;

use crate::keyer::{get_data_source_key};


fn initialize_pruned_transaction(transaction_trace: eth::TransactionTrace) -> PrunedTransaction {
    PrunedTransaction {
        hash: transaction_trace.hash.clone(),
        from: transaction_trace.from.clone(),
        to: transaction_trace.to.clone(),
        nonce: transaction_trace.nonce,
        gas_limit: transaction_trace.gas_limit,
        gas_used: transaction_trace.gas_used,
        gas_price: "0".to_string(),
        updates: Vec::<Update>::new(),
    }
}

#[substreams::handlers::map]
pub fn map_extract_data_types(
    block: eth::Block,
    data_sources_store: StoreGetProto<DataSource>,
) -> Result<MappedDataSources, Error>{
    let mut mapped_data_sources = MappedDataSources {
        pruned_transactions: Vec::<PrunedTransaction>::new(),
        store_instructions: Vec::<StoreInstruction>::new(),
    };

    for transaction_trace in block.transaction_traces {
        let mut pruned_transaction: PrunedTransaction = initialize_pruned_transaction(transaction_trace.clone());
 
        for call_view in transaction_trace.calls() {
            if let Some(data_source) = data_sources_store.get_last(get_data_source_key(&call_view.call.address)) {
                match data_source.data_source_type {
                    0 => {
                        for log in &call_view.call.logs {
                            if let Some(swap_event) = PoolContract::events::Swap::match_and_decode(&log) {
                                pruned_transaction.updates.push(
                                    Update {
                                        r#type: Some(SwapType(Swap { 
                                            pool: call_view.call.address.clone(),
                                            protocol: UNISWAP_V3_FACTORY_SLICE.to_vec(),
                                            account: pruned_transaction.from.clone(),
                                            amounts: vec![swap_event.amount0.to_string(), swap_event.amount1.to_string()],
                                            liquidity: swap_event.liquidity.to_string(),
                                            tick: swap_event.tick.to_string(),
                                            log_index: log.index,
                                            log_ordinal: log.ordinal,
                                        }))
                                    }
                                );
                            } else if let Some(mint_event) = PoolContract::events::Mint::match_and_decode(&log) {
                                pruned_transaction.updates.push(
                                    Update {
                                        r#type: Some(DepositType(Deposit {
                                            pool: call_view.call.address.clone(),
                                            protocol: UNISWAP_V3_FACTORY_SLICE.to_vec(),
                                            account: pruned_transaction.from.clone(),
                                            position: None,
                                            liquidity: mint_event.amount.to_string(),
                                            input_token_amounts: vec![mint_event.amount0.to_string(), mint_event.amount1.to_string()],
                                            tick_lower: Some(mint_event.tick_lower.to_string()),
                                            tick_upper:Some( mint_event.tick_upper.to_string()),
                                            log_index: log.index,
                                            log_ordinal: log.ordinal,
                                        }))
                                    }
                                );
                            } else if let Some(burn_event) = PoolContract::events::Burn::match_and_decode(&log) {
                                pruned_transaction.updates.push(
                                    Update {
                                        r#type: Some(WithdrawType(Withdraw {
                                            pool: call_view.call.address.clone(),
                                            protocol: UNISWAP_V3_FACTORY_SLICE.to_vec(),
                                            account: pruned_transaction.from.clone(),
                                            position: None,
                                            liquidity: burn_event.amount.to_string(),
                                            input_token_amounts: vec![burn_event.amount0.to_string(), burn_event.amount1.to_string()],
                                            tick_lower: Some(burn_event.tick_lower.to_string()),
                                            tick_upper: Some(burn_event.tick_upper.to_string()),
                                            log_index: log.index,
                                            log_ordinal: log.ordinal,
                                        }))
                                    }
                                );
                            }
                        }
                    }
                    1 => {
                        for log in &call_view.call.logs {
                            if let Some(pool_created_event) = FactoryContract::events::PoolCreated::match_and_decode(&log) {
                                pruned_transaction.updates.push(
                                    Update {
                                        r#type: Some(CreateLiquidityPoolType(CreateLiquidityPool {
                                            protocol: UNISWAP_V3_FACTORY_SLICE.to_vec(),
                                            pool_address: pool_created_event.pool.clone(),
                                            input_tokens: vec![pool_created_event.token0.clone(), pool_created_event.token1.clone()],
                                            reward_tokens: vec![],
                                            fees: vec![],
                                            is_single_sided: false,
                                            tick: None,
                                            liquidity_token: None,
                                            liquidity_token_type: None,
                                        }))
                                    }
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        if pruned_transaction.updates.len() > 0 {
            mapped_data_sources.pruned_transactions.push(pruned_transaction);
        }
    }
    Ok(mapped_data_sources)
}
