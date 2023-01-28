use substreams::store::StoreGet;
use substreams::store::StoreNew;
use substreams::store::{StoreAddBigInt, StoreGetProto};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::abi::pair;
use crate::balance_updater::PoolBalanceUpdater;
use crate::pb::uniswap::v2::Pool;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_pool_user_balance(
    block: eth::Block,
    pool_store: StoreGetProto<Pool>,
    output: StoreAddBigInt,
) {
    for log in block.logs() {
        if let Some(event) = pair::events::Transfer::match_and_decode(log) {
            let pool_address = Hex(log.address()).to_string();

            if let Some(_) = pool_store.get_last(StoreKey::Pool.get_unique_pool_key(&pool_address))
            {
                let pool_balance = PoolBalanceUpdater::new(pool_address, &output);

                pool_balance.update_user_balance(&event.to, &event.value);
                pool_balance.update_user_balance(&event.from, event.value.neg().as_ref());
                pool_balance.update_output_token_supply(&event.from, &event.to, &event.value);
            }
        }
    }
}
