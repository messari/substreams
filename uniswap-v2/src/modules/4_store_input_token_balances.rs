use std::ops::Add;

use substreams::scalar::BigInt;
use substreams::store::{StoreGet, StoreGetProto, StoreNew, StoreSet, StoreSetBigInt};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::store_key::StoreKey;
use crate::utils::get_day_id;
use crate::{abi::pair as PairContract, pb::uniswap::v2::Pool};

#[substreams::handlers::store]
pub fn store_input_token_balances(
    block: eth::Block,
    pool_store: StoreGetProto<Pool>,
    output_store: StoreSetBigInt,
) {
    for log in block.logs() {
        if let Some(sync_event) = PairContract::events::Sync::match_and_decode(log) {
            let pool_address = Hex(log.address()).to_string();

            let amount0 = sync_event.reserve0;
            let amount1 = sync_event.reserve1;

            let day_id = get_day_id(block.timestamp_seconds() as i64);

            if let Some(_) = pool_store.get_last(StoreKey::Pool.get_unique_pool_key(&pool_address))
            {
                output_store.set(
                    log.ordinal(),
                    StoreKey::TotalBalance.get_unique_pool_key(&pool_address),
                    &amount0.clone().add(amount1.clone()),
                );
                output_store.set(
                    log.ordinal(),
                    StoreKey::LatestBlockNumber.unique_id(),
                    &BigInt::from(block.number),
                );
                output_store.set(
                    log.ordinal(),
                    StoreKey::LatestTimestamp
                        .get_unique_snapshot_tracking_key(&pool_address, &day_id.to_string()),
                    &BigInt::from(block.timestamp_seconds()),
                );

                output_store.set(
                    log.ordinal(),
                    StoreKey::Token0Balance.get_unique_pool_key(&pool_address),
                    &amount0,
                );
                output_store.set(
                    log.ordinal(),
                    StoreKey::Token1Balance.get_unique_pool_key(&pool_address),
                    &amount1,
                );
            }
        }
    }
}
