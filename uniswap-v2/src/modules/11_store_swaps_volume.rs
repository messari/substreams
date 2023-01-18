use std::str::FromStr;

use crate::pb::uniswap::v2::event::Type::Swap as SwapEvent;
use substreams::pb::substreams::Clock;
use substreams::scalar::BigDecimal;
use substreams::store::{StoreGet, StoreGetProto, StoreSet};
use substreams::store::{StoreNew, StoreSetBigDecimal};

use crate::pb::uniswap::v2::{Events, Pool};
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_swaps_volume(
    _clock: Clock,
    pool_store: StoreGetProto<Pool>,
    pool_events: Events,
    output: StoreSetBigDecimal,
) {
    for event in pool_events.events {
        let pool = match pool_store.get_last(StoreKey::Pool.get_unique_pool_key(&event.pool)) {
            Some(pool) => pool,
            None => continue,
        };

        match event.r#type.unwrap() {
            SwapEvent(swap) => {
                let volume: BigDecimal = (BigDecimal::from_str(&swap.amount_in_usd).unwrap()
                    + BigDecimal::from_str(&swap.amount_out_usd).unwrap())
                    / BigDecimal::from(2);

                output.set(
                    0,
                    StoreKey::PoolVolume.get_unique_pool_key(&pool.address),
                    &volume,
                );
            }
            _ => {}
        }
    }
}
