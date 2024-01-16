mod abi;
mod pb;
use substreams::Hex;
use hex_literal::hex;
use pb::vault::v1 as vault;
use substreams_ethereum::Event;
use substreams_ethereum::pb::eth::v2 as eth;

use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;

use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;


const TRACKED_CONTRACT: [u8; 20] = hex!("ba12222222228d8ba445958a75a0704d566bf2c8");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<vault::Events, substreams::errors::Error> {
    Ok(vault::Events {
        pool_registrations: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs
                    .iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if
                            let Some(event) =
                                abi::vault::events::PoolRegistered::match_and_decode(log)
                        {
                            return Some(vault::PoolRegistered {
                                tx_hash: Hex(&view.transaction.hash).to_string(),
                                log_index: log.block_index,
                                block_time: Some(
                                    blk.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_owned()
                                ),
                                block_number: blk.number,
                                pool_id: event.pool_id.to_vec(),
                                pool_address: event.pool_address,
                                specialization: event.specialization.to_i32() as u32,
                                from_address: view.transaction.clone().from,
                                to_address: view.transaction.clone().to,
                            });
                        }

                        None
                    })
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn db_out(events: vault::Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = DatabaseChangeTables::new();

    // Loop over all the abis events to create changes
    events.pool_registrations.into_iter().for_each(|evt: vault::PoolRegistered| {
        tables
            .create_row("pool_registrations", [("tx_hash", evt.clone().tx_hash), ("log_index", evt.clone().log_index.to_string())])
            .set("block_time", evt.block_time.as_ref().unwrap())
            .set("block_number", evt.block_number)
            .set("pool_id", Hex(&evt.pool_id).to_string())
            .set("pool_address", Hex(&evt.pool_address).to_string())
            .set("specialization", evt.clone().specialization)
            .set("from_address", Hex(&evt.from_address).to_string())
            .set("to_address", Hex(&evt.to_address).to_string());
    });

    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(events: vault::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = EntityChangesTables::new();

    // Loop over all the abis events to create changes
    events.pool_registrations.into_iter().for_each(|evt: vault::PoolRegistered| {
        tables
            .create_row("pool_registrations", format!("{}-{}", evt.tx_hash, evt.log_index))
            .set("block_time", evt.block_time.as_ref().unwrap())
            .set("block_number", evt.block_number)
            .set("pool_id", Hex(&evt.pool_id).to_string())
            .set("pool_address", Hex(&evt.pool_address).to_string())
            .set("specialization", evt.clone().specialization)
            .set("from_address", Hex(&evt.from_address).to_string())
            .set("to_address", Hex(&evt.to_address).to_string());
    });

    Ok(tables.to_entity_changes())
}
