use substreams::store::{StoreAdd, StoreAddBigInt, StoreGet, StoreGetProto, StoreNew};
use substreams::Hex;
use substreams_ethereum::{
    pb::eth::v2::{self as eth},
    Event, NULL_ADDRESS,
};

use crate::{abi::pair as PairContract, pb::uniswap::v2::Pool, store_key::StoreKey};

#[substreams::handlers::store]
pub fn store_output_token_supply(
    block: eth::Block,
    pool_store: StoreGetProto<Pool>,
    output_store: StoreAddBigInt,
) {
    for log in block.logs() {
        if let Some(transfer_event) = PairContract::events::Transfer::match_and_decode(log) {
            let pool_address = Hex(log.address()).to_string();

            if let Some(_) = pool_store.get_last(StoreKey::Pool.get_unique_pool_key(&pool_address))
            {
                let is_mint = transfer_event.from == NULL_ADDRESS;
                let is_burn = transfer_event.to == NULL_ADDRESS;

                let mut value = transfer_event.value;

                if !(is_mint || is_burn) {
                    continue;
                }

                if is_burn {
                    value = value.neg()
                }

                output_store.add(
                    log.ordinal(),
                    StoreKey::OutputTokenBalance.get_unique_pool_key(&pool_address),
                    value,
                );
            }
        }
    }
}
