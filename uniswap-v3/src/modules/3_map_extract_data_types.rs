use substreams::prelude::*;
use substreams::errors::Error;
use substreams::Hex;
use substreams_ethereum::{
    pb::eth::v2::{self as eth},
    Event, NULL_ADDRESS,
};
use substreams::scalar::{BigDecimal, BigInt};

use substreams::store::{StoreGetProto};
use crate::{pb::uniswap::v3::{DataSource, DataSourceType, PrunedBlock, PrunedTransaction, Call, Event as EventProto, PoolCreated, Swap, Mint, Burn}};
use crate::pb::uniswap::v3::event::Type::{Swap as SwapType, Mint as MintType, Burn as BurnType, PoolCreated as PoolCreatedType};
use crate::abi::pool as PoolContract;
use crate::abi::factory as FactoryContract;
use crate::keyer::{get_data_source_key};

#[substreams::handlers::map]
pub fn map_extract_data_types(
    block: eth::Block,
    data_sources_store: StoreGetProto<DataSource>,
) -> Result<PrunedBlock, Error>{
    let mut pruned_block = PrunedBlock {
        number: block.number,
        timestamp: block.timestamp_seconds(),
        transactions: Vec::<PrunedTransaction>::new(),
    };
    

    for trx in block.transactions() {
        let mut pruned_trx = PrunedTransaction {
            hash: trx.hash.clone(),
            from: trx.from.clone(),
            to: trx.to.clone(),
            nonce: trx.nonce,
            gas_limit: trx.gas_limit,
            gas_used: trx.gas_used,
            gas_price: "0".to_string(),
            calls: Vec::<Call>::new(),
        };
 
        for call_view in trx.calls() {
            if let Some(data_source) = data_sources_store.get_last(get_data_source_key(&call_view.call.address)) {
                let mut call = Call {
                    address: call_view.call.address.clone(),
                    events: Vec::<EventProto>::new(),
                };
                match data_source.data_source_type {
                    0 => {
                        for log in &call_view.call.logs {
                            if let Some(swap_event) = PoolContract::events::Swap::match_and_decode(&log) {
                                call.events.push(
                                    EventProto {
                                        log_index: log.index,
                                        log_ordinal: log.ordinal,
                                        r#type: Some(SwapType(Swap {
                                            sender: swap_event.sender.clone(),
                                            amount0: swap_event.amount0.to_string(),
                                            amount1: swap_event.amount1.to_string(),
                                            liquidity: swap_event.liquidity.to_string(),
                                            sqrt_price_x96: swap_event.sqrt_price_x96.to_string(),
                                            tick: swap_event.tick.to_string(),
                                            recipient: swap_event.recipient.clone(),
                                        }))
                                    }
                                );
                            } else if let Some(mint_event) = PoolContract::events::Mint::match_and_decode(&log) {
                                call.events.push(
                                    EventProto {
                                        log_index: log.index ,
                                        log_ordinal: log.ordinal,
                                        r#type: Some(MintType(Mint {
                                            owner: mint_event.owner.clone(),
                                            sender: mint_event.sender.clone(),
                                            amount: mint_event.amount.to_string(),
                                            amount0: mint_event.amount0.to_string(),
                                            amount1: mint_event.amount1.to_string(),
                                            tick_lower: mint_event.tick_lower.to_string(),
                                            tick_upper: mint_event.tick_upper.to_string(),
                                        }))
                                    }
                                );
                            } else if let Some(burn_event) = PoolContract::events::Burn::match_and_decode(&log) {
                                call.events.push(
                                    EventProto {
                                        log_index: log.index,
                                        log_ordinal: log.ordinal,
                                        r#type: Some(BurnType(Burn {
                                            owner: burn_event.owner.clone(),
                                            amount: burn_event.amount.to_string(),
                                            amount0: burn_event.amount0.to_string(),
                                            amount1: burn_event.amount1.to_string(),
                                            tick_lower: burn_event.tick_lower.to_string(),
                                            tick_upper: burn_event.tick_upper.to_string(),
                                        }))
                                    }
                                );
                            }
                        }
                    }
                    1 => {
                        for log in &call_view.call.logs {
                            if let Some(pool_created_event) = FactoryContract::events::PoolCreated::match_and_decode(&log) {
                                call.events.push(
                                    EventProto {
                                        log_index: log.index,
                                        log_ordinal: log.ordinal,
                                        r#type: Some(PoolCreatedType(PoolCreated {
                                            pool: pool_created_event.pool.clone(),
                                            token0: pool_created_event.token0.clone(),
                                            token1: pool_created_event.token1.clone(),
                                            fee: pool_created_event.fee.to_string(),
                                            tick_spacing: pool_created_event.tick_spacing.to_string(),
                                        }))
                                    }
                                );
                            }
                        }
                    }
                    _ => {}
                }
                if call.events.len() > 0 {
                    pruned_trx.calls.push(call);
                }
            }
        }
        if pruned_trx.calls.len() > 0 {
            pruned_block.transactions.push(pruned_trx);
        }
    }
    Ok(pruned_block)
}
