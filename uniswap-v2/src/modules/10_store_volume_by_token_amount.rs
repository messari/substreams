use substreams::scalar::BigInt;
use substreams::store::StoreAddBigInt;
use substreams::store::StoreNew;

use crate::common::traits::StoreAddSnapshot;
use crate::pb::uniswap::v2::event::Type::SwapType;
use crate::pb::uniswap::v2::Events;
use crate::store_key::StoreKey;
use crate::utils;

#[substreams::handlers::store]
pub fn store_volume_by_token_amount(swap_events: Events, output_store: StoreAddBigInt) {
    for event in swap_events.events {
        match event.r#type.unwrap() {
            SwapType(swap) => {
                let ordinal = event.log_ordinal;
                let pool_address = event.pool;

                let day_id = utils::get_day_id(event.timestamp as i64);
                let hour_id = utils::get_hour_id(event.timestamp as i64);

                let token_in = swap.token_in.unwrap();
                let token_out = swap.token_out.unwrap();

                let amount_in = BigInt::try_from(swap.amount_in).unwrap();
                let amount_out = BigInt::try_from(swap.amount_out).unwrap();

                output_store.add_snapshot(
                    ordinal,
                    day_id,
                    StoreKey::DailyVolumeByTokenAmount,
                    vec![&pool_address, &token_in.address],
                    &amount_in,
                );
                output_store.add_snapshot(
                    ordinal,
                    hour_id,
                    StoreKey::HourlyVolumeByTokenAmount,
                    vec![&pool_address, &token_in.address],
                    &amount_in,
                );

                output_store.add_snapshot(
                    ordinal,
                    day_id,
                    StoreKey::DailyVolumeByTokenAmount,
                    vec![&pool_address, &token_out.address],
                    &amount_out,
                );
                output_store.add_snapshot(
                    ordinal,
                    hour_id,
                    StoreKey::HourlyVolumeByTokenAmount,
                    vec![&pool_address, &token_out.address],
                    &amount_out,
                );
            }
            _ => {}
        }
    }
}
