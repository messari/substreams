use substreams::store::StoreNew;
use substreams::store::StoreSet;
use substreams::store::StoreSetProto;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::abi::factory;
use crate::pb::dex_amm::v1::Pool;
use crate::pool_retriever::PoolRetriever;
use crate::store_key::StoreKey;

#[substreams::handlers::store]
pub fn initialize_pool(block: eth::Block, store: StoreSetProto<Pool>) {
    for log in block.logs() {
        if let Some(event) = factory::events::PairCreated::match_and_decode(log) {
            let pool_address = event.pair;
            let pool_retriever = PoolRetriever::new(&pool_address);

            let pool = Pool {
                name: pool_retriever.get_name(),
                address: pool_retriever.get_address(),
                symbol: pool_retriever.get_symbol(),
                input_tokens: pool_retriever.get_input_tokens(),
                output_token: pool_retriever.get_output_token(),
                is_single_sided: false,
                created_timestamp: block.timestamp_seconds(),
                created_block_number: block.number,

                ..Default::default()
            };

            store.set(
                log.ordinal(),
                StoreKey::Pool.get_unique_pool_key(&pool.address),
                &pool,
            )
        }
    }
}
