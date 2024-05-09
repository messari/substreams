use std::collections::HashSet;

use substreams::scalar::BigInt;
use substreams::store::{DeltaBigInt, Deltas, StoreGet, StoreGetBigInt};
use substreams_entity_change::pb::entity::{entity_change, EntityChange, EntityChanges};
use substreams_ethereum::pb::eth::rpc::RpcResponse;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::rpc::RpcBatch;
use substreams_helper::hex::Hexable;

use crate::abi::pool_contract::functions::{AdminFee, Fee};
use crate::store_key::StoreKey;

#[substreams::handlers::map]
pub fn map_fees_entity(
    blk: eth::Block,
    admin_balances: StoreGetBigInt,
    admin_balance_deltas: Deltas<DeltaBigInt>,
) -> Result<EntityChanges, ()> {
    let mut entity_changes: Vec<EntityChange> = vec![];
    let mut unique_pools_set: HashSet<Vec<u8>> = HashSet::new();

    for calls in blk.calls() {
        let address: Vec<u8> = calls.call.address.clone();

        if admin_balances
            .get_last(StoreKey::Pool.get_unique_key(&address.to_hex()))
            .is_none()
        {
            continue;
        }

        unique_pools_set.insert(address);
    }

    for pool in &unique_pools_set {
        let id = [pool.to_hex(), blk.number.to_string()].join("-");

        let mut fees_entity_change =
            EntityChange::new("Fees", id.as_str(), 0, entity_change::Operation::Update);

        let pool_fees = get_pool_fees(pool);

        fees_entity_change
            .change("id", id)
            .change("pool", pool.to_hex())
            .change(
                "fee",
                RpcBatch::decode::<_, Fee>(&pool_fees[0])
                    .map(|x| x.to_u64())
                    .unwrap_or_default(),
            )
            .change(
                "adminFee",
                RpcBatch::decode::<_, Fee>(&pool_fees[1])
                    .map(|x| x.to_u64())
                    .unwrap_or_default(),
            )
            .change(
                "adminBalances",
                get_admin_balances(&admin_balance_deltas, &pool.to_hex()),
            )
            .change("blockNumber", blk.number)
            .change("timestamp", blk.timestamp_seconds());

        entity_changes.push(fees_entity_change);
    }

    Ok(EntityChanges { entity_changes })
}

fn get_pool_fees(contract: &Vec<u8>) -> Vec<RpcResponse> {
    let batch = RpcBatch::new();

    let responses = batch
        .add(Fee {}, contract.to_vec())
        .add(AdminFee {}, contract.to_vec())
        .execute()
        .unwrap()
        .responses;

    responses
}

fn get_admin_balances(admin_balance_deltas: &Deltas<DeltaBigInt>, address: &String) -> Vec<BigInt> {
    let mut balances = vec![BigInt::zero(); 4];

    for delta in admin_balance_deltas.deltas.iter() {
        match delta {
            ref x if x.key == StoreKey::AdminBalanceToken0.get_unique_key(address) => {
                balances[0] = delta.new_value.clone() - delta.old_value.clone();
            }
            ref x if x.key == StoreKey::AdminBalanceToken1.get_unique_key(address) => {
                balances[1] = delta.new_value.clone() - delta.old_value.clone();
            }
            ref x if x.key == StoreKey::AdminBalanceToken2.get_unique_key(address) => {
                balances[2] = delta.new_value.clone() - delta.old_value.clone();
            }
            ref x if x.key == StoreKey::AdminBalanceToken3.get_unique_key(address) => {
                balances[3] = delta.new_value.clone() - delta.old_value.clone();
            }

            _ => {}
        }
    }

    balances
}
