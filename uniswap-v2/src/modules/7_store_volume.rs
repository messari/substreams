use std::collections::HashMap;

use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{StoreNew, StoreSetBigDecimal};

use crate::common::traits::StoreSetter;
use crate::pb::uniswap::v2::event::Type::Swap as SwapEvent;
use crate::pb::uniswap::v2::Events;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_volume(swap_events: Events, output_store: StoreSetBigDecimal) {
    let mut volume_map: HashMap<String, BigDecimal> = HashMap::new();

    for event in swap_events.events {
        match event.clone().r#type.unwrap() {
            SwapEvent(swap) => {
                let pool_address = event.clone().pool.to_owned();

                let token_in_price = BigDecimal::one();
                let token_out_price = BigDecimal::one();

                let amount_in = BigInt::from_unsigned_bytes_be(swap.amount_in.as_bytes())
                    .to_decimal(swap.token_in.unwrap().decimals);
                let amount_out = BigInt::from_unsigned_bytes_be(swap.amount_out.as_bytes())
                    .to_decimal(swap.token_out.unwrap().decimals);

                let volume = (amount_in * token_in_price + amount_out * token_out_price)
                    / BigDecimal::from(2);

                if volume_map.contains_key(&pool_address) {
                    volume_map
                        .entry(pool_address)
                        .and_modify(|v| *v = v.clone() + volume.clone());
                } else {
                    volume_map.insert(pool_address, volume);
                };
            }
            _ => {}
        }
    }

    for (address, volume) in volume_map.into_iter() {
        output_store.set_value(StoreKey::Volume.get_unique_pool_key(&address), &volume);
    }
}
