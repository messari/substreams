use substreams::store::StoreNew;
use substreams::store::StoreSet;
use substreams::store::StoreSetInt64;
use substreams_ethereum::pb::eth::v2::{self as eth};

use crate::block_handler::BlockHandler;
use crate::utils::i64_to_str;

#[substreams::handlers::store]
pub fn unique_tracking(block: eth::Block, unique_tracking_store: StoreSetInt64) {
    let block_handler = BlockHandler::new(&block);

    if let Some(author) = block_handler.author() {
        // For value here, instead if leaving this empty we could put the block timestamp. This would be good
        // when taking into account stats on daily/hourly aggregates. The issue is that if the first block of
        // the chain starts in the middle or towards the end of the day/hour then the first few aggregate
        // stats in the chain will get skewed. By adding in the timestamp we could check to see if this is a
        // "first block scenario" and fix the stats accordingly. (The fix would be to either not include it,
        // or to do a naive approximation to get the stats for the whole day, ie you could double the
        // aggregate if you have only half a days data and keep the variance the same)
        unique_tracking_store.set(0, format!("t:{}", author), &1); // (Any number can be used here for differentiation as long as it's not set as blank..)
        unique_tracking_store.set(0, format!("d:{}:{}", i64_to_str(block_handler.days_timestamp()), author), &1);
        unique_tracking_store.set(0, format!("h:{}:{}", i64_to_str(block_handler.hours_timestamp()), author), &1);
    }

    unique_tracking_store.set(0, "latest_day_timestamp", &block_handler.days_timestamp());
    unique_tracking_store.set(0, "latest_hour_timestamp", &block_handler.hours_timestamp());
    unique_tracking_store.set(0, "block_timestamp", &block_handler.timestamp());
}
