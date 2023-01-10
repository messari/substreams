use substreams::store::StoreNew;
use substreams::store::StoreSet;
use substreams::store::StoreSetProto;

use crate::pb::uniswap::v2::{Pool, Pools};
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn initialize_pool(pools_created: Pools, store: StoreSetProto<Pool>) {
    for pool in pools_created.pools {
        store.set(0, StoreKey::Pool.get_unique_pool_key(&pool.address), &pool)
    }
}
