use substreams::store::StoreGet;
use substreams::store::StoreNew;
use substreams::store::{StoreAddBigInt, StoreGetProto};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::abi::pair;
use crate::pb::dex_amm::v1::Pool;
use crate::pool_balance_retriever::PoolBalanceRetriever;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn pool_transfers(block: eth::Block, pool_store: StoreGetProto<Pool>, output: StoreAddBigInt) {
    for log in block.logs() {
        if let Some(event) = pair::events::Transfer::match_and_decode(log) {
            let pool_address = Hex(log.address()).to_string();

            if let Some(_) = pool_store.get_last(StoreKey::Pool.get_unique_pool_key(&pool_address))
            {
                let pool_balance = PoolBalanceRetriever::new(pool_address, &output);

                pool_balance.store_user_balance(event.to, event.value.as_ref());
                pool_balance.store_user_balance(event.from, event.value.neg().as_ref());
            }
        }
    }
}
