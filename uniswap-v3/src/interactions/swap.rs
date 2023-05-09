use substreams::prelude::*;
use substreams::errors::Error;
use substreams::Hex;
use substreams_ethereum::{pb::eth::v2::{self as eth}, Event, NULL_ADDRESS};
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{StoreGetProto};

use crate::{pb::dex_amm::v3_0_3::{
    DataSource, MappedDataSources, PrunedTransaction, StoreInstruction, 
    Update, Swap, Deposit, Withdraw, CreateLiquidityPool, AddInt64, AddManyInt64, AddBigInt, AddManyBigInt}, 
    utils::UNISWAP_V3_FACTORY_SLICE, dex_amm::v_3_0_3::map

};
use crate::pb::dex_amm::v3_0_3::update::Type::{Swap as SwapType, Deposit as DepositType, Withdraw as WithdrawType, CreateLiquidityPool as CreateLiquidityPoolType};
use crate::pb::dex_amm::v3_0_3::store_instruction;

use crate::abi::pool as PoolContract;
use crate::schema_lib::dex_amm::v_3_0_3::store_keys;

use crate::keyer::{get_data_source_key};

pub fn handle_swap(
    mapped_data_sources: &mut MappedDataSources, 
    pruned_transaction: &mut PrunedTransaction,
    swap_event: PoolContract::events::Swap, 
    call: eth::CallView, 
    log: eth::Log
) {
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

    // Update SwapCounts
    mapped_data_sources.store_instructions.push(
        StoreInstruction {
            r#type: Some(
                store_instruction::Type::AddManyInt64(
                    AddManyInt64 {
                        ordinal: 0,
                        key: vec![
                            store_keys::get_store_key(store_keys::StoreKey::LiquidityPoolCumulativeSwapCount, &Hex(&call_view.call.address).to_string(), 0),
                        ],
                        value: 1,
                    }
                )
            )
        }
    );

    // Update Balance Changes
    mapped_data_sources.store_instructions.push(
        StoreInstruction {
            r#type: Some(
                store_instruction::Type::AddManyBigInt(
                    AddManyBigInt {
                        ordinal: 0,
                        key: vec![
                            store_keys::get_store_key(store_keys::StoreKey::LiquidityPoolInputTokenBalance, &Hex(&call_view.call.address).to_string(), 0),
                        ],
                        value: swap_event.amount0.to_string(),
                    }
                )
            )
        }
    );
    mapped_data_sources.store_instructions.push(
        StoreInstruction {
            r#type: Some(
                store_instruction::Type::AddManyBigInt(
                    AddManyBigInt {
                        ordinal: 0,
                        key: vec![
                            store_keys::get_store_key(store_keys::StoreKey::LiquidityPoolInputTokenBalance, &Hex(&call_view.call.address).to_string(), 1),
                        ],
                        value: swap_event.amount1.to_string(),
                    }
                )
            )
        }
    );

    // Update Volumes
    mapped_data_sources.store_instructions.push(
        StoreInstruction {
            r#type: Some(
                store_instruction::Type::AddManyBigInt(
                    AddManyBigInt {
                        ordinal: 0,
                        key: vec![
                            store_keys::get_store_key(store_keys::StoreKey::LiquidityPoolCumulativeVolumeTokenAmounts, &Hex(&call_view.call.address).to_string(), 0),
                        ],
                        value: swap_event.amount0.to_string(),
                    }
                )
            )
        }
    );
    mapped_data_sources.store_instructions.push(
        StoreInstruction {
            r#type: Some(
                store_instruction::Type::AddManyBigInt(
                    AddManyBigInt {
                        ordinal: 0,
                        key: vec![
                            store_keys::get_store_key(store_keys::StoreKey::LiquidityPoolCumulativeVolumeTokenAmounts, &Hex(&call_view.call.address).to_string(), 1),
                        ],
                        value: swap_event.amount1.to_string(),
                    }
                )
            )
        }
    );

    // Upate Active Liquidity
    mapped_data_sources.store_instructions.push(
        StoreInstruction {
            r#type: Some(
                store_instruction::Type::AddManyBigInt(
                    AddManyBigInt {
                        ordinal: 0,
                        key: vec![
                            store_keys::get_store_key(store_keys::StoreKey::LiquidityPoolActiveLiquidity, &Hex(&call_view.call.address).to_string(), 0),
                        ],
                        value: swap_event.liquidity.to_string(),
                    }
                )
            )
        }
    );
}



fn create_store_instruction<K, F>(ordinal: u64, key: K, f: F) -> StoreInstruction
where
    K: AsRef<str>,
    F: FnOnce(String) -> store_instruction::Type,
{
    StoreInstruction {
        r#type: Some(f(key.as_ref().to_string())),
    }
}

pub fn add_int_64<K: AsRef<str>>(ordinal: u64, key: K, value: i64) -> StoreInstruction {
    create_store_instruction(ordinal, key, |key| {
        store_instruction::Type::AddInt64(AddInt64 { ordinal, key, value })
    })
}

pub fn add_bigint<K: AsRef<str>>(ordinal: u64, key: K, value: BigInt) -> StoreInstruction {
    create_store_instruction(ordinal, key, |key| {
        store_instruction::Type::AddBigInt(AddBigInt { ordinal, key, value })
    })
}

pub fn add_bigdecimal<K: AsRef<str>>(ordinal: u64, key: K, value: BigDecimal) -> StoreInstruction {
    create_store_instruction(ordinal, key, |key| {
        store_instruction::Type::AddBigDecimal(AddBigDecimal { ordinal, key, value })
    })
}
