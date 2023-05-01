use std::str::FromStr;

use substreams::prelude::*;
use substreams::errors::Error;
use substreams::store::{DeltaBigDecimal, DeltaInt64, DeltaBigInt, StoreGetRaw, StoreGet};
use substreams_entity_change::pb::entity::{EntityChange, EntityChanges, entity_change::Operation};

use crate::pb::uniswap::v3::{PrunedBlock, PrunedTransaction};
use crate::pb::uniswap::v3::event::Type::{Swap as SwapType, Mint as MintType, Burn as BurnType, PoolCreated as PoolCreatedType};

use crate::keyer::get_event_key;
use crate::utils;
pub use substreams_ethereum::{NULL_ADDRESS};


#[substreams::handlers::map]
pub fn map_graph_out(
    pruned_block: PrunedBlock,
    add_bigdecimal_store_deltas: Deltas<DeltaBigDecimal>,
    add_bigint_store_deltas: Deltas<DeltaBigInt>,
    add_int64_store_deltas: Deltas<DeltaInt64>,
    set_bytes_store_deltas: StoreGetRaw,

) -> Result<EntityChanges, ()> {
    let mut entity_changes: Vec<EntityChange> = Vec::new();

    // Can parallelize across transactions?
    for pruned_transaction in &pruned_block.transactions {
        entity_changes.extend(get_event_entity_changes(&pruned_block, &pruned_transaction));
        // entity_changes.push(get_account_entity_changes(&pruned_transaction));
        // entity_changes.push(get_position_entity_changes(&pruned_transaction));
        // entity_changes.push(get_liquidity_pool_entity_changes(&pruned_transaction));
        // entity_changes.push(get_token_entity_changes(&pruned_transaction));
        // entity_changes.push(get_dex_amm_protocol_entity_changes(&pruned_transaction));
    }

    Ok(EntityChanges { entity_changes })
}

fn get_event_entity_changes(pruned_block: &PrunedBlock, pruned_transaction: &PrunedTransaction) -> Vec<EntityChange> {
    let mut entity_changes: Vec<EntityChange> = Vec::new();
    for call in &pruned_transaction.calls {
        for event in &call.events {
            match &event.r#type {
                Some(SwapType(swap)) => {
                    let mut swap_entity_change: EntityChange =
                        EntityChange::new("Swap", &get_event_key(&pruned_transaction.hash, &event.log_index), 0, Operation::Create);
                    
                        swap_entity_change
                            .change("hash", &pruned_transaction.hash)
                            .change("nonce", pruned_transaction.nonce)
                            .change("gasLimit", pruned_transaction.gas_limit)
                            .change("gasUsed", pruned_transaction.gas_used)
                            .change("gasPrice", &pruned_transaction.gas_price)
                            .change("logIndex", event.log_index as u64)
                            .change("protocol", utils::UNISWAP_V3_FACTORY.to_string())
                            .change("account", &pruned_transaction.from)
                            .change("pool", &call.address)
                            .change("blockNumber", pruned_block.number)
                            .change("timestamp", pruned_block.timestamp)
                            .change("tick", &swap.tick)
                            .change("tokenIn", NULL_ADDRESS.to_vec())
                            .change("amountIn", BigInt::from(0))
                            .change("amountInUSD", BigDecimal::from(0))
                            .change("tokenOut", NULL_ADDRESS.to_vec())
                            .change("amountOut", BigInt::from(0))
                            .change("amountOutUSD", BigDecimal::from(0));
                        
                        entity_changes.push(swap_entity_change);
                    
                }
                Some(MintType(mint)) => {
                    let mut deposit_entity_change: EntityChange =
                    EntityChange::new("Deposit", &get_event_key(&pruned_transaction.hash, &event.log_index), 0, Operation::Create);

                    deposit_entity_change
                        .change("hash", &pruned_transaction.hash)
                        .change("nonce", pruned_transaction.nonce)
                        .change("gasLimit", pruned_transaction.gas_limit)
                        .change("gasUsed", pruned_transaction.gas_used)
                        .change("gasPrice", &pruned_transaction.gas_price)
                        .change("logIndex", event.log_index)
                        .change("protocol", utils::UNISWAP_V3_FACTORY.to_string())
                        .change("account", &pruned_transaction.from)
                        // .change("position", &position)
                        .change("pool", &call.address)
                        .change("tickLower", mint.tick_lower)
                        .change("tickUpper", mint.tick_upper)
                        .change("blockNumber", pruned_block.number)
                        .change("timestamp", pruned_block.timestamp)
                        .change("liquidity", mint.amount)
                        .change("inputTokens", [NULL_ADDRESS.to_vec()])
                        .change("inputTokenAmounts", vec![mint.amount0, mint.amount1])
                        .change("amountUSD", BigDecimal::from(0));

                    entity_changes.push(deposit_entity_change);
                }
                _ => {}
            }
        }
    }
    entity_changes
}