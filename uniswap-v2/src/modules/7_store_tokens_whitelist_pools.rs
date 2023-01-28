use substreams::store::{Appender, StoreAppend};

use crate::common::constants;
use crate::pb::uniswap::v2::Pools;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn store_tokens_whitelist_pools(pools_created: Pools, output: StoreAppend<String>) {
    for pool in pools_created.pools {
        let input_tokens = pool.input_tokens.unwrap().items;

        for token in input_tokens {
            if constants::WHITELIST_TOKENS.contains(&token.address.as_str()) {
                output.append(
                    0,
                    StoreKey::TokenWhitelist.get_unique_pool_key(&token.address),
                    pool.address.clone(),
                )
            }
        }
    }
}
