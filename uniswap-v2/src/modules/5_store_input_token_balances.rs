use std::ops::Add;
use std::str::FromStr;

use substreams::scalar::BigInt;
use substreams::store::StoreNew;
use substreams::store::StoreSet;
use substreams::store::StoreSetBigInt;

use crate::pb::uniswap::v2::event::Type::SyncType;
use crate::pb::uniswap::v2::Events;
use crate::store_key::StoreKey;
use crate::utils;

#[substreams::handlers::store]
pub fn store_input_token_balances(pool_events: Events, output_store: StoreSetBigInt) {
    for event in pool_events.events {
        match event.r#type.unwrap() {
            SyncType(sync_event) => {
                let pool_address = event.pool;

                let day_id = utils::get_day_id(event.timestamp as i64);

                let amount0 = BigInt::from_str(sync_event.reserve0.as_str()).unwrap();
                let amount1 = BigInt::from_str(sync_event.reserve1.as_str()).unwrap();

                output_store.set(
                    event.log_ordinal,
                    StoreKey::TotalBalance.get_unique_pool_key(&pool_address),
                    &amount0.clone().add(amount1.clone()),
                );
                output_store.set(
                    event.log_ordinal,
                    StoreKey::LatestBlockNumber.unique_id(),
                    &BigInt::from(event.block_number),
                );
                output_store.set(
                    event.log_ordinal,
                    StoreKey::LatestTimestamp
                        .get_unique_snapshot_tracking_key(&pool_address, &day_id.to_string()),
                    &BigInt::from(event.timestamp as i64),
                );

                output_store.set(
                    event.log_ordinal,
                    StoreKey::Token0Balance.get_unique_pool_key(&pool_address),
                    &amount0,
                );
                output_store.set(
                    event.log_ordinal,
                    StoreKey::Token1Balance.get_unique_pool_key(&pool_address),
                    &amount1,
                );
            }
            _ => {}
        }
    }
}
