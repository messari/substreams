use substreams::pb::substreams::store_delta::Operation;
use substreams::scalar::BigInt;
use substreams::store;
use substreams::store::{DeltaBytes, StoreAdd};
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams::store::StoreAddBigInt;

use crate::aggregator::Aggregator;
use crate::block_handler::BlockHandler;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn days_and_hours_pre_aggregations(block: eth::Block, unique_deltas: store::Deltas<DeltaBytes>, mut pre_aggregation_store: store::StoreAddBigInt) {
    let block_handler = BlockHandler::new(&block);
    let mut aggregator = Aggregator::new(&mut pre_aggregation_store, block_handler.days_timestamp(), block_handler.hours_timestamp());

    aggregator.store_day_and_hour_sum_contributions(StoreKey::Transactions, &block_handler.transactions());
    aggregator.store_day_and_hour_sum_contributions(StoreKey::Supply, &block_handler.supply());
    aggregator.store_day_sum_contribution(StoreKey::Blocks, &BigInt::one());

    for unique_delta in unique_deltas.deltas.into_iter() {
        if unique_delta.key.starts_with("d") && unique_delta.operation == Operation::Create {
            aggregator.store_day_sum_contribution(StoreKey::UniqueAuthors, &BigInt::one());
        }

        if unique_delta.key.starts_with("h") && unique_delta.operation == Operation::Create {
            aggregator.store_hour_sum_contribution(StoreKey::UniqueAuthors, &BigInt::one());
        }
    }
}
