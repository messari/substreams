use substreams::store::StoreNew;
use substreams::store::StoreSetIfNotExists;
use substreams::store::StoreSetIfNotExistsString;

use crate::common::constants;
use crate::pb::contract::v1::Pools;

#[substreams::handlers::store]
pub fn store_pools(pools_created: Pools, store: StoreSetIfNotExistsString) {
    for pool in pools_created.pools {
        store.set_if_not_exists(0, &pool.address, &"Added".to_string());
    }

    for pool in constants::POOLS_DEPLOYED_BEFORE_FACTORIES {
        store.set_if_not_exists(0, pool.to_string(), &"Added".to_string());
    }
}
