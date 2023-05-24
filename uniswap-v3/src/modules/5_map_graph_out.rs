use ethabi::{Hash};
use substreams::prelude::*;
use substreams::pb::substreams::Clock;
use substreams::store::{StoreGetBigDecimal, DeltaBigDecimal, StoreGetInt64, DeltaInt64, StoreGetBigInt, DeltaBigInt, StoreGetArray, DeltaArray};
use substreams_entity_change::pb::entity::{EntityChanges};

use core::panic;
use std::collections::HashMap;
use std::fmt::Debug;


use crate::tables::{Tables, Row};
use crate::pb::dex_amm::v3_0_3::{EntityUpdates};
use crate::schema_lib::dex_amm::v_3_0_3::map::map_dex_amm_v_3_0_3_entity_creation;
use crate::constants; 

#[substreams::handlers::map]
pub fn map_graph_out(
    clock: Clock,
    entity_updates: EntityUpdates,
    add_bigdecimal_store: StoreGetBigDecimal,
    add_bigdecimal_store_deltas: Deltas<DeltaBigDecimal>,
    add_bigint_store: StoreGetBigInt,
    add_bigint_store_deltas: Deltas<DeltaBigInt>,
    set_bigint_store: StoreGetBigInt,
    set_bigint_store_deltas: Deltas<DeltaBigInt>,
    set_bytes_store: StoreGetRaw,
    set_bytes_store_deltas: Deltas<DeltaBytes>,
    add_int64_store: StoreGetInt64,
    add_int64_store_deltas: Deltas<DeltaInt64>,
    append_string_store: StoreGetArray<String>,
    append_string_store_deltas: Deltas<DeltaArray<String>>,
) -> Result<EntityChanges, ()> {
    let mut tables: Tables = Tables::new();

    let block_number = clock.number;
    let timestamp = clock.timestamp.unwrap().seconds;
    
    for pruned_transaction in entity_updates.pruned_transactions {
        for entity_creation in &pruned_transaction.entity_creations {
            map_dex_amm_v_3_0_3_entity_creation(
                &mut tables,
                &block_number,
                &timestamp,
                &pruned_transaction,
                &entity_creation, 
            );
        }
    }

    map_store_values(
        &mut tables,
        &add_bigint_store,
        &add_bigint_store_deltas,
        &set_bigint_store,
        &set_bigint_store_deltas,
        &add_bigdecimal_store,
        &add_bigdecimal_store_deltas,
        &add_int64_store,
        &add_int64_store_deltas,
        &append_string_store,
        &append_string_store_deltas,
    );

    Ok(tables.to_entity_changes())
}


pub fn map_store_values(
    tables: &mut Tables,
    add_bigint_store: &StoreGetBigInt,
    add_bigint_store_deltas: &Deltas<DeltaBigInt>,
    set_bigint_store: &StoreGetBigInt,
    set_bigint_store_deltas: &Deltas<DeltaBigInt>,
    add_bigdecimal_store: &StoreGetBigDecimal,
    add_bigdecimal_store_deltas: &Deltas<DeltaBigDecimal>,
    add_int64_store: &StoreGetInt64,
    add_int64_store_deltas: &Deltas<DeltaInt64>,
    append_string_store: &StoreGetArray<String>,
    append_string_store_deltas: &Deltas<DeltaArray<String>>,
) {
    fn split_key(key: &str) -> (Vec<&str>, Option<usize>, Option<usize>) {
        let parts = key.split(":").collect::<Vec<_>>();
        let index = parts.get(4).and_then(|s| s.parse::<usize>().ok());
        let array_size = parts.get(5).and_then(|s| s.parse::<usize>().ok());
        (parts[0..4].to_vec(), index, array_size)
    }

    for delta in &add_bigint_store_deltas.deltas {
        let (key_list, index, array_size) = split_key(&delta.key);
        if key_list[0] != "entity-change" {
            continue;
        }

        match (index, array_size) {
            (None, None) => {
                tables.update_row(key_list[1], key_list[2]).set(key_list[3], delta.new_value.clone());

            },
            (Some(_), Some(array_size)) => {
                let mut value_list = vec![constants::BIGINT_ZERO.clone(); array_size];
                for i in 0..array_size {
                    let key = [key_list[0], key_list[1], key_list[2], key_list[3], i.to_string().as_str(), array_size.to_string().as_str()].join(":");
                    if let Some(value) = add_bigint_store.get_last(key.as_str()) {
                        value_list[i] = value.clone();
                    } else {
                        panic!("Missing delta for key: {}", key)
                    }
                }

                tables.update_row(key_list[1], key_list[2]).set(key_list[3], value_list.clone());

            },
            _ => {
                panic!("Invalid key: {}", delta.key);
            }
        }
    }

    for delta in &set_bigint_store_deltas.deltas {
        let (key_list, index, array_size) = split_key(&delta.key);
        if key_list[0] != "entity-change" {
            continue;
        }

        match (index, array_size) {
            (None, None) => {
                tables.update_row(key_list[1], key_list[2]).set(key_list[3], delta.new_value.clone());

            },
            (Some(_), Some(array_size)) => {
                let mut value_list = vec![constants::BIGINT_ZERO.clone(); array_size];
                for i in 0..array_size {
                    let key = [key_list[0], key_list[1], key_list[2], key_list[3], i.to_string().as_str(), array_size.to_string().as_str()].join(":");
                    if let Some(value) = set_bigint_store.get_last(key.as_str()) {
                        value_list[i] = value.clone();
                    } else {
                        panic!("Missing delta for key: {}", key)
                    }
                }

                tables.update_row(key_list[1], key_list[2]).set(key_list[3], value_list.clone());

            },
            _ => {
                panic!("Invalid key: {}", delta.key);
            }
        }
    }

    for delta in &add_bigdecimal_store_deltas.deltas {
        let (key_list, index, array_size) = split_key(&delta.key);
        if key_list[0] != "entity-change" {
            continue;
        }

        match (index, array_size) {
            (None, None) => {
                tables.update_row(key_list[1], key_list[2]).set(key_list[3], delta.new_value.clone());

            },
            (Some(_), Some(array_size)) => {
                let mut value_list = vec![constants::BIGDECIMAL_ZERO.clone(); array_size];
                for i in 0..array_size {
                    let key = [key_list[0], key_list[1], key_list[2], key_list[3], i.to_string().as_str(), array_size.to_string().as_str()].join(":");
                    if let Some(value) = add_bigdecimal_store.get_last(key.as_str()) {
                        value_list[i] = value.clone();
                    } else {
                        panic!("Missing delta for key: {}", key)
                    }
                }

                tables.update_row(key_list[1], key_list[2]).set(key_list[3], value_list.clone());

            },
            _ => {
                panic!("Invalid key: {}", delta.key);
            }
        }
    }

    for delta in &append_string_store_deltas.deltas {
        let (key_list, index, array_size) = split_key(&delta.key);
        if key_list[0] != "entity-change" {
            continue;
        }

        match (index, array_size) {
            (None, None) => {
                tables.update_row(key_list[1], key_list[2]).set(key_list[3], &delta.new_value.iter().map(|v| v.clone().into_bytes()).collect::<Vec<_>>());

            },
            (Some(_), Some(array_size)) => {
                panic!("Not implemented");
            },
            _ => {
                panic!("Invalid key: {}", delta.key);
            }
        }
    }

    for delta in &add_int64_store_deltas.deltas {
        let (key_list, index, array_size) = split_key(&delta.key);
        if key_list[0] != "entity-change" {
            continue;
        }

        match (index, array_size) {
            (None, None) => {
                tables.update_row(key_list[1], key_list[2]).set(key_list[3], delta.new_value as i32);

            },
            (Some(_), Some(array_size)) => {
                panic!("Not implemented");
            },
            _ => {
                panic!("Invalid key: {}", delta.key);
            }
        }
    }
}
