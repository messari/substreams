mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
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

const TRACKED_CONTRACT: [u8; 20] = hex!("0d4a11d5eeaac28ec3f61d100daf4d40471f1852");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    Ok(contract::Events {
        approvals: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Approval::match_and_decode(log) {
                            return Some(contract::Approval {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                owner: event.owner,
                                spender: event.spender,
                                value: event.value.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        burns: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Burn::match_and_decode(log) {
                            return Some(contract::Burn {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                amount0: event.amount0.to_string(),
                                amount1: event.amount1.to_string(),
                                sender: event.sender,
                                to: event.to,
                            });
                        }

                        None
                })
            })
            .collect(),
        mints: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Mint::match_and_decode(log) {
                            return Some(contract::Mint {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                amount0: event.amount0.to_string(),
                                amount1: event.amount1.to_string(),
                                sender: event.sender,
                            });
                        }

                        None
                })
            })
            .collect(),
        swaps: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Swap::match_and_decode(log) {
                            return Some(contract::Swap {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
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
            .collect(),
        syncs: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Sync::match_and_decode(log) {
                            return Some(contract::Sync {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                reserve0: event.reserve0.to_string(),
                                reserve1: event.reserve1.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        transfers: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Transfer::match_and_decode(log) {
                            return Some(contract::Transfer {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                from: event.from,
                                to: event.to,
                                value: event.value.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn db_out(events: contract::Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = DatabaseChangeTables::new();

    // Loop over all the abis events to create changes
    events.approvals.into_iter().for_each(|evt| {
        tables
            .create_row("approval", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("owner", Hex(&evt.owner).to_string())
            .set("spender", Hex(&evt.spender).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.burns.into_iter().for_each(|evt| {
        tables
            .create_row("burn", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount0", BigDecimal::from_str(&evt.amount0).unwrap())
            .set("amount1", BigDecimal::from_str(&evt.amount1).unwrap())
            .set("sender", Hex(&evt.sender).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.mints.into_iter().for_each(|evt| {
        tables
            .create_row("mint", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount0", BigDecimal::from_str(&evt.amount0).unwrap())
            .set("amount1", BigDecimal::from_str(&evt.amount1).unwrap())
            .set("sender", Hex(&evt.sender).to_string());
    });
    events.swaps.into_iter().for_each(|evt| {
        tables
            .create_row("swap", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount0_in", BigDecimal::from_str(&evt.amount0_in).unwrap())
            .set("amount0_out", BigDecimal::from_str(&evt.amount0_out).unwrap())
            .set("amount1_in", BigDecimal::from_str(&evt.amount1_in).unwrap())
            .set("amount1_out", BigDecimal::from_str(&evt.amount1_out).unwrap())
            .set("sender", Hex(&evt.sender).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.syncs.into_iter().for_each(|evt| {
        tables
            .create_row("sync", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("reserve0", BigDecimal::from_str(&evt.reserve0).unwrap())
            .set("reserve1", BigDecimal::from_str(&evt.reserve1).unwrap());
    });
    events.transfers.into_iter().for_each(|evt| {
        tables
            .create_row("transfer", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });

    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = EntityChangesTables::new();

    // Loop over all the abis events to create changes
    events.approvals.into_iter().for_each(|evt| {
        tables
            .create_row("approval", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("owner", Hex(&evt.owner).to_string())
            .set("spender", Hex(&evt.spender).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.burns.into_iter().for_each(|evt| {
        tables
            .create_row("burn", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount0", BigDecimal::from_str(&evt.amount0).unwrap())
            .set("amount1", BigDecimal::from_str(&evt.amount1).unwrap())
            .set("sender", Hex(&evt.sender).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.mints.into_iter().for_each(|evt| {
        tables
            .create_row("mint", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount0", BigDecimal::from_str(&evt.amount0).unwrap())
            .set("amount1", BigDecimal::from_str(&evt.amount1).unwrap())
            .set("sender", Hex(&evt.sender).to_string());
    });
    events.swaps.into_iter().for_each(|evt| {
        tables
            .create_row("swap", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount0_in", BigDecimal::from_str(&evt.amount0_in).unwrap())
            .set("amount0_out", BigDecimal::from_str(&evt.amount0_out).unwrap())
            .set("amount1_in", BigDecimal::from_str(&evt.amount1_in).unwrap())
            .set("amount1_out", BigDecimal::from_str(&evt.amount1_out).unwrap())
            .set("sender", Hex(&evt.sender).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.syncs.into_iter().for_each(|evt| {
        tables
            .create_row("sync", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("reserve0", BigDecimal::from_str(&evt.reserve0).unwrap())
            .set("reserve1", BigDecimal::from_str(&evt.reserve1).unwrap());
    });
    events.transfers.into_iter().for_each(|evt| {
        tables
            .create_row("transfer", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });

    Ok(tables.to_entity_changes())
}
