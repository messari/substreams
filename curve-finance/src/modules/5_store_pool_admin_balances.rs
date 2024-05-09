use substreams::scalar::BigInt;
use substreams::store::StoreGet;
use substreams::store::StoreNew;
use substreams::store::StoreSet;
use substreams::store::{StoreGetString, StoreSetBigInt};

use substreams_ethereum::pb::eth::{rpc::RpcResponse, v2 as eth};
use substreams_ethereum::rpc::RpcBatch;
use substreams_helper::hex::Hexable;

use crate::abi::pool_contract::functions::AdminBalances;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_pool_admin_balances(
    blk: eth::Block,
    pools_store: StoreGetString,
    output_store: StoreSetBigInt,
) {
    for calls in blk.calls() {
        let address = calls.call.address.clone();

        if pools_store.get_last(&address.to_hex()).is_none() {
            continue;
        }

        let admin_balances = get_pool_admin_balances(address.clone());

        output_store.set(
            calls.call.end_ordinal,
            StoreKey::Pool.get_unique_key(&address.to_hex()),
            &BigInt::one(),
        );
        output_store.set(
            calls.call.end_ordinal,
            StoreKey::AdminBalanceToken0.get_unique_key(&address.to_hex()),
            RpcBatch::decode::<_, AdminBalances>(&admin_balances[0])
                .unwrap_or(BigInt::zero())
                .as_ref(),
        );
        output_store.set(
            calls.call.end_ordinal,
            StoreKey::AdminBalanceToken1.get_unique_key(&address.to_hex()),
            RpcBatch::decode::<_, AdminBalances>(&admin_balances[1])
                .unwrap_or(BigInt::zero())
                .as_ref(),
        );
        output_store.set(
            calls.call.end_ordinal,
            StoreKey::AdminBalanceToken2.get_unique_key(&address.to_hex()),
            RpcBatch::decode::<_, AdminBalances>(&admin_balances[2])
                .unwrap_or(BigInt::zero())
                .as_ref(),
        );
        output_store.set(
            calls.call.end_ordinal,
            StoreKey::AdminBalanceToken3.get_unique_key(&address.to_hex()),
            RpcBatch::decode::<_, AdminBalances>(&admin_balances[3])
                .unwrap_or(BigInt::zero())
                .as_ref(),
        );
    }
}

fn get_pool_admin_balances(contract: Vec<u8>) -> Vec<RpcResponse> {
    let batch = RpcBatch::new();

    let responses = batch
        .add(
            AdminBalances {
                arg0: BigInt::from(0),
            },
            contract.clone(),
        )
        .add(
            AdminBalances {
                arg0: BigInt::from(1),
            },
            contract.clone(),
        )
        .add(
            AdminBalances {
                arg0: BigInt::from(2),
            },
            contract.clone(),
        )
        .add(
            AdminBalances {
                arg0: BigInt::from(3),
            },
            contract.clone(),
        )
        .execute()
        .unwrap()
        .responses;

    responses
}
