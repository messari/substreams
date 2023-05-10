use substreams::prelude::*;
use substreams::pb::substreams::Clock;
use substreams::errors::Error;
use substreams::store::{DeltaBigDecimal, DeltaInt64, DeltaBigInt, StoreGetRaw, StoreGet, StoreGetArray};
use substreams_entity_change::pb::entity::{EntityChange, EntityChanges};

use crate::tables::Tables;
use crate::pb::dex_amm::v3_0_3::{MappedDataSources, PrunedTransaction};
use crate::schema_lib::dex_amm::v_3_0_3::map::map_dex_amm_v_3_0_3_entity_changes;

#[substreams::handlers::map]
pub fn map_graph_out(
    clock: Clock,
    mapped_data_sources: MappedDataSources,
    add_bigdecimal_store_deltas: Deltas<DeltaBigDecimal>,
    add_bigint_store_deltas: Deltas<DeltaBigInt>,
    add_int64_store_deltas: Deltas<DeltaInt64>,
    store_append_string: StoreGetArray<String>,
) -> Result<EntityChanges, ()> {
    let mut tables: Tables = Tables::new();

    let block_number = clock.number;
    let timestamp = clock.timestamp.unwrap().seconds;

    for pruned_transaction in mapped_data_sources.pruned_transactions {
        for update in &pruned_transaction.updates {
            map_dex_amm_v_3_0_3_entity_changes(
                &mut tables,
                &block_number,
                &timestamp,
                &pruned_transaction,
                &update, 
                &add_bigdecimal_store_deltas,
                &add_bigint_store_deltas,
                &add_int64_store_deltas,
                &store_append_string
            );
        }
    }

    Ok(tables.to_entity_changes())
}
