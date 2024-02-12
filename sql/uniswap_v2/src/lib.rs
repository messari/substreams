mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::prelude::*;
use substreams::store;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const FACTORY_TRACKED_CONTRACT: [u8; 20] = hex!("5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f");

fn map_factory_events(blk: &eth::Block, events: &mut contract::Events) {
    events.factory_pair_createds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == FACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::factory_contract::events::PairCreated::match_and_decode(log) {
                        return Some(contract::FactoryPairCreated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            pair: event.pair,
                            param3: event.param3.to_string(),
                            token0: event.token0,
                            token1: event.token1,
                        });
                    }

                    None
                })
        })
        .collect());
}

fn is_declared_dds_address(addr: &Vec<u8>, ordinal: u64, dds_store: &store::StoreGetInt64) -> bool {
    //    substreams::log::info!("Checking if address {} is declared dds address", Hex(addr).to_string());
    if dds_store.get_at(ordinal, Hex(addr).to_string()).is_some() {
        return true;
    }
    return false;
}

fn map_pools_events(
    blk: &eth::Block,
    dds_store: &store::StoreGetInt64,
    events: &mut contract::Events,
) {

    events.pools_approvals.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pools_contract::events::Approval::match_and_decode(log) {
                        return Some(contract::PoolsApproval {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            owner: event.owner,
                            spender: event.spender,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pools_burns.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pools_contract::events::Burn::match_and_decode(log) {
                        return Some(contract::PoolsBurn {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                            sender: event.sender,
                            to: event.to,
                        });
                    }

                    None
                })
        })
        .collect());

    events.pools_mints.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pools_contract::events::Mint::match_and_decode(log) {
                        return Some(contract::PoolsMint {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());

    events.pools_swaps.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pools_contract::events::Swap::match_and_decode(log) {
                        return Some(contract::PoolsSwap {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            amount0_in: event.amount0_in.to_string(),
                            amount0_out: event.amount0_out.to_string(),
                            amount1_in: event.amount1_in.to_string(),
                            amount1_out: event.amount1_out.to_string(),
                            sender: event.sender,
                            to: event.to,
                        });
                    }

                    None
                })
        })
        .collect());

    events.pools_syncs.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pools_contract::events::Sync::match_and_decode(log) {
                        return Some(contract::PoolsSync {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            reserve0: event.reserve0.to_string(),
                            reserve1: event.reserve1.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pools_transfers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pools_contract::events::Transfer::match_and_decode(log) {
                        return Some(contract::PoolsTransfer {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            from: event.from,
                            to: event.to,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
}


fn db_factory_out(events: &contract::Events, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis events to create table changes
    events.factory_pair_createds.iter().for_each(|evt| {
        tables
            .create_row("factory_pair_created", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("pair", Hex(&evt.pair).to_string())
            .set("param3", BigDecimal::from_str(&evt.param3).unwrap())
            .set("token0", Hex(&evt.token0).to_string())
            .set("token1", Hex(&evt.token1).to_string());
    });
}
fn db_pools_out(events: &contract::Events, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis events to create table changes
    events.pools_approvals.iter().for_each(|evt| {
        tables
            .create_row("pools_approval", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("owner", Hex(&evt.owner).to_string())
            .set("spender", Hex(&evt.spender).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.pools_burns.iter().for_each(|evt| {
        tables
            .create_row("pools_burn", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("amount0", BigDecimal::from_str(&evt.amount0).unwrap())
            .set("amount1", BigDecimal::from_str(&evt.amount1).unwrap())
            .set("sender", Hex(&evt.sender).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.pools_mints.iter().for_each(|evt| {
        tables
            .create_row("pools_mint", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("amount0", BigDecimal::from_str(&evt.amount0).unwrap())
            .set("amount1", BigDecimal::from_str(&evt.amount1).unwrap())
            .set("sender", Hex(&evt.sender).to_string());
    });
    events.pools_swaps.iter().for_each(|evt| {
        tables
            .create_row("pools_swap", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("amount0_in", BigDecimal::from_str(&evt.amount0_in).unwrap())
            .set("amount0_out", BigDecimal::from_str(&evt.amount0_out).unwrap())
            .set("amount1_in", BigDecimal::from_str(&evt.amount1_in).unwrap())
            .set("amount1_out", BigDecimal::from_str(&evt.amount1_out).unwrap())
            .set("sender", Hex(&evt.sender).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.pools_syncs.iter().for_each(|evt| {
        tables
            .create_row("pools_sync", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("reserve0", BigDecimal::from_str(&evt.reserve0).unwrap())
            .set("reserve1", BigDecimal::from_str(&evt.reserve1).unwrap());
    });
    events.pools_transfers.iter().for_each(|evt| {
        tables
            .create_row("pools_transfer", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
}


fn graph_factory_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.factory_pair_createds.iter().for_each(|evt| {
        tables
            .create_row("factory_pair_created", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("pair", Hex(&evt.pair).to_string())
            .set("param3", BigDecimal::from_str(&evt.param3).unwrap())
            .set("token0", Hex(&evt.token0).to_string())
            .set("token1", Hex(&evt.token1).to_string());
    });
}
fn graph_pools_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.pools_approvals.iter().for_each(|evt| {
        tables
            .create_row("pools_approval", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("owner", Hex(&evt.owner).to_string())
            .set("spender", Hex(&evt.spender).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.pools_burns.iter().for_each(|evt| {
        tables
            .create_row("pools_burn", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("amount0", BigDecimal::from_str(&evt.amount0).unwrap())
            .set("amount1", BigDecimal::from_str(&evt.amount1).unwrap())
            .set("sender", Hex(&evt.sender).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.pools_mints.iter().for_each(|evt| {
        tables
            .create_row("pools_mint", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("amount0", BigDecimal::from_str(&evt.amount0).unwrap())
            .set("amount1", BigDecimal::from_str(&evt.amount1).unwrap())
            .set("sender", Hex(&evt.sender).to_string());
    });
    events.pools_swaps.iter().for_each(|evt| {
        tables
            .create_row("pools_swap", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("amount0_in", BigDecimal::from_str(&evt.amount0_in).unwrap())
            .set("amount0_out", BigDecimal::from_str(&evt.amount0_out).unwrap())
            .set("amount1_in", BigDecimal::from_str(&evt.amount1_in).unwrap())
            .set("amount1_out", BigDecimal::from_str(&evt.amount1_out).unwrap())
            .set("sender", Hex(&evt.sender).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.pools_syncs.iter().for_each(|evt| {
        tables
            .create_row("pools_sync", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("reserve0", BigDecimal::from_str(&evt.reserve0).unwrap())
            .set("reserve1", BigDecimal::from_str(&evt.reserve1).unwrap());
    });
    events.pools_transfers.iter().for_each(|evt| {
        tables
            .create_row("pools_transfer", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", &evt.evt_address)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
}
#[substreams::handlers::store]
fn store_factory_pools_created(blk: eth::Block, store: StoreSetInt64) {
    for rcpt in blk.receipts() {
        for log in rcpt
            .receipt
            .logs
            .iter()
            .filter(|log| log.address == FACTORY_TRACKED_CONTRACT)
        {
            if let Some(event) = abi::factory_contract::events::PairCreated::match_and_decode(log) {
                store.set(log.ordinal, Hex(event.pair).to_string(), &1);
            }
        }
    }
}

#[substreams::handlers::map]
fn map_events(
    blk: eth::Block,
    store_pools: StoreGetInt64,
) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_factory_events(&blk, &mut events);
    map_pools_events(&blk, &store_pools, &mut events);
    Ok(events)
}

#[substreams::handlers::map]
fn db_out(events: contract::Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = DatabaseChangeTables::new();
    db_factory_out(&events, &mut tables);
    db_pools_out(&events, &mut tables);
    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();
    graph_factory_out(&events, &mut tables);
    graph_pools_out(&events, &mut tables);
    Ok(tables.to_entity_changes())
}
