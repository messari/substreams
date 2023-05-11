use substreams::Hex;
use substreams_ethereum::{pb::eth::v2::{self as eth}};

use crate::{pb::dex_amm::v3_0_3::{
    MappedDataSources, PrunedTransaction, StoreInstruction, 
    Update, Swap, AddManyInt64, AddManyBigInt}, 
    utils::UNISWAP_V3_FACTORY_SLICE

};
use crate::pb::dex_amm::v3_0_3::update::Type::{Swap as SwapType};
use crate::pb::dex_amm::v3_0_3::store_instruction;

use crate::abi::pool as PoolContract;
use crate::schema_lib::dex_amm::v_3_0_3::store_keys;

pub fn handle_swap(
    mapped_data_sources: &mut MappedDataSources, 
    pruned_transaction: &mut PrunedTransaction,
    swap_event: PoolContract::events::Swap, 
    call: &eth::Call, 
    log: &eth::Log
) {
    pruned_transaction.updates.push(
        Update {
            r#type: Some(SwapType(Swap { 
                pool: call.address.clone(),
                protocol: UNISWAP_V3_FACTORY_SLICE.to_vec(),
                account: pruned_transaction.from.clone(),
                amounts: vec![swap_event.amount0.clone().into(), swap_event.amount1.clone().into()],
                liquidity: Some(swap_event.liquidity.clone().into()),
                tick: Some(swap_event.tick.clone().into()),
                log_index: Some(log.index.into()),
                log_ordinal: Some(log.ordinal.into()),
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
                            store_keys::get_store_key(store_keys::StoreKey::LiquidityPoolCumulativeSwapCount, &Hex(&call.address).to_string(), 0),
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
                            store_keys::get_store_key(store_keys::StoreKey::LiquidityPoolInputTokenBalance, &Hex(&call.address).to_string(), 0),
                        ],
                        value: Some(swap_event.amount0.clone().into()),
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
                            store_keys::get_store_key(store_keys::StoreKey::LiquidityPoolInputTokenBalance, &Hex(&call.address).to_string(), 1),
                        ],
                        value: Some(swap_event.amount1.clone().into()),
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
                            store_keys::get_store_key(store_keys::StoreKey::LiquidityPoolCumulativeVolumeTokenAmounts, &Hex(&call.address).to_string(), 0),
                        ],
                        value: Some(swap_event.amount0.clone().into()),
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
                            store_keys::get_store_key(store_keys::StoreKey::LiquidityPoolCumulativeVolumeTokenAmounts, &Hex(&call.address).to_string(), 1),
                        ],
                        value: Some(swap_event.amount1.clone().into()),
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
                            store_keys::get_store_key(store_keys::StoreKey::LiquidityPoolActiveLiquidity, &Hex(&call.address).to_string(), 0),
                        ],
                        value: Some(swap_event.liquidity.clone().into()),
                    }
                )
            )
        }
    );
}



// fn create_store_instruction<K, F>(ordinal: u64, key: K, f: F) -> StoreInstruction
// where
//     K: AsRef<str>,
//     F: FnOnce(String) -> store_instruction::Type,
// {
//     StoreInstruction {
//         r#type: Some(f(key.as_ref().to_string())),
//     }
// }

// pub fn add_int_64<K: AsRef<str>>(ordinal: u64, key: K, value: i64) -> StoreInstruction {
//     create_store_instruction(ordinal, key, |key| {
//         store_instruction::Type::AddInt64(AddInt64 { ordinal, key, value })
//     })
// }

// pub fn add_bigint<K: AsRef<str>>(ordinal: u64, key: K, value: String) -> StoreInstruction {
//     create_store_instruction(ordinal, key, |key| {
//         store_instruction::Type::AddBigInt(AddBigInt { ordinal, key, value: value.to_string() })
//     })
// }

// pub fn add_bigdecimal<K: AsRef<str>>(ordinal: u64, key: K, value: String) -> StoreInstruction {
//     create_store_instruction(ordinal, key, |key| {
//         store_instruction::Type::AddBigDecimal(AddBigDecimal { ordinal, key, value: value.to_string() })
//     })
// }
